use std::collections::HashMap;
use std::io::Write;

use crate::anyhow;
use crate::ast::*;
use crate::ErrorOrCtxJmp;
use crate::Interpreter;
use crate::Result;

pub type ResolveResult = Result<()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    None,
    Function,
    ClassMethod,
    Initializer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve_stmt<W: Write>(
        &mut self,
        stmt: &Stmt,
        interpreter: &mut Interpreter<W>,
    ) -> ResolveResult {
        match stmt {
            Stmt::Print(e) | Stmt::Expr(e) => self.resolve_expr(e, interpreter)?,
            Stmt::VariableDecl(VariableDecl { name, definition }) => {
                self.declare(name);
                match definition {
                    Some(initalizer_expr) => {
                        self.resolve_expr(initalizer_expr, interpreter)?;
                    }
                    None => {}
                }
                self.define(name);
            }
            Stmt::Block(stmts) => {
                self.begin_scope();
                self.resolve(stmts, interpreter)?;
                self.end_scope();
            }
            Stmt::Conditional(Conditional {
                cond,
                if_branch,
                else_branch,
            }) => {
                self.resolve_expr(cond, interpreter)?;
                self.resolve_stmt(if_branch, interpreter)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch, interpreter)?;
                }
            }
            Stmt::Loop(Loop { cond, body }) => {
                self.resolve_expr(cond, interpreter)?;
                self.resolve_stmt(body, interpreter)?;
            }
            Stmt::FunctionDecl(f) => {
                self.declare(&f.name);
                self.define(&f.name);
                self.resolve_function(&f.params, &f.body, FunctionType::Function, interpreter)?;
            }
            Stmt::Return(expr) => {
                if self.current_function == FunctionType::None {
                    return Err(ErrorOrCtxJmp::Error(anyhow!(
                        "Error at 'return': Can't return from top-level code."
                    )));
                }

                if self.current_function == FunctionType::Initializer && expr != &Expr::Nil {
                    return Err(ErrorOrCtxJmp::Error(anyhow!(
                        "Error at 'return': Can't return a value from an initializer."
                    )));
                }

                self.resolve_expr(expr, interpreter)?;
            }
            Stmt::ClassDecl(ClassDecl {
                name,
                super_class,
                methods,
            }) => {
                self.declare(name);
                self.define(name);

                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;

                if let Some(super_class) = super_class {
                    if let Expr::Ident(ref sc) = super_class {
                        if sc.ident == name.ident {
                            return Err(ErrorOrCtxJmp::Error(anyhow!(
                                "Error at '{}': A class can't inherit from itself.",
                                name
                            )));
                        }
                    }
                    self.resolve_expr(super_class, interpreter)?;
                    self.begin_scope();
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert("super".to_string(), true);
                }

                self.begin_scope();
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert("this".to_string(), true);
                for method in methods {
                    let declaration = if method.name.ident == "init" {
                        FunctionType::Initializer
                    } else {
                        FunctionType::ClassMethod
                    };
                    self.resolve_function(&method.params, &method.body, declaration, interpreter)?;
                }
                self.end_scope();

                if super_class.is_some() {
                    self.end_scope();
                }

                self.current_class = enclosing_class;
            }
        }
        Ok(())
    }

    pub fn resolve_expr<W: Write>(
        &mut self,
        expr: &Expr,
        interpreter: &mut Interpreter<W>,
    ) -> ResolveResult {
        match expr {
            Expr::Nil | Expr::Int(_) | Expr::Float(_) | Expr::Boolean(_) | Expr::String(_) => {}
            Expr::Ident(id) => {
                if !self.scopes.is_empty() {
                    match self.scopes.last().unwrap().get(&id.ident as &str) {
                        Some(b) if !(*b) => {
                            return Err(ErrorOrCtxJmp::Error(anyhow!(
                                "Error at '{}': Can't read local variable in its own initializer.",
                                &id.ident
                            )))
                        }
                        _ => {}
                    };
                }
                self.resolve_local(id, interpreter)?
            }
            Expr::Unary(_, e) => {
                self.resolve_expr(e, interpreter)?;
            }
            Expr::Binary(_, e1, e2) | Expr::Logical(_, e1, e2) => {
                self.resolve_expr(e1, interpreter)?;
                self.resolve_expr(e2, interpreter)?;
            }
            Expr::Assign(ident, e) => {
                self.resolve_expr(e, interpreter)?;
                if let Expr::Ident(ref id) = **ident {
                    self.resolve_local(id, interpreter)?;
                } else {
                    return Err(ErrorOrCtxJmp::Error(anyhow!(
                        "Error at '=': Invalid assignment target."
                    )));
                };
            }
            Expr::Call(callee, args) => {
                self.resolve_expr(callee, interpreter)?;
                for arg in args {
                    self.resolve_expr(&arg.value, interpreter)?;
                }
            }
            Expr::Lambda(params, body) => {
                self.resolve_function(params, body, FunctionType::Function, interpreter)?
            }
            Expr::Get(object, _fields) => {
                self.resolve_expr(object, interpreter)?;
            }
            Expr::Set(object, _, value) => {
                self.resolve_expr(object, interpreter)?;
                self.resolve_expr(value, interpreter)?;
            }
            Expr::This(this) => {
                if self.current_class == ClassType::None {
                    return Err(ErrorOrCtxJmp::Error(anyhow!(
                        "Error at 'this': Can't use 'this' outside of a class."
                    )));
                }
                self.resolve_local(this, interpreter)?
            }
            Expr::Super(super_class, _method) => {
                if self.current_class == ClassType::None {
                    return Err(ErrorOrCtxJmp::Error(anyhow!(
                        "Error at 'super': Can't use 'super' outside of a class."
                    )));
                }
                self.resolve_local(super_class, interpreter)?;
            }
        }
        Ok(())
    }

    pub fn resolve<W: Write>(
        &mut self,
        stmts: &[Stmt],
        interpreter: &mut Interpreter<W>,
    ) -> ResolveResult {
        for stmt in stmts {
            self.resolve_stmt(stmt, interpreter)?;
        }
        Ok(())
    }

    pub fn resolve_local<W: Write>(
        &mut self,
        id: &Identifier,
        interpreter: &mut Interpreter<W>,
    ) -> ResolveResult {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            match scope.get(&id.ident as &str) {
                Some(_) => {
                    interpreter.resolve(id.clone(), i);
                    return Ok(());
                }
                None => {
                    continue;
                }
            }
        }

        if id.ident == "super" {
            return Err(ErrorOrCtxJmp::Error(anyhow!(
                "Error at 'super': Can't use 'super' in a class with no superclass."
            )));
        } else {
            return Err(ErrorOrCtxJmp::Error(anyhow!(
                "Undefined variable '{}'.",
                id.ident
            )));
        }
    }

    fn resolve_function<W: Write>(
        &mut self,
        params: &[Identifier],
        body: &[Stmt],
        ftype: FunctionType,
        interpreter: &mut Interpreter<W>,
    ) -> ResolveResult {
        let enclosing_function = self.current_function;
        self.current_function = ftype;
        self.begin_scope();

        for param in params {
            self.declare(param);
            self.define(param);
        }

        self.resolve(body, interpreter)?;

        self.end_scope();
        self.current_function = enclosing_function;
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Identifier) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.ident.clone(), false);
        }
    }

    fn define(&mut self, name: &Identifier) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.ident.clone(), true);
        }
    }
}
