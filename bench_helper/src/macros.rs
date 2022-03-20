#[macro_export]
macro_rules! loop_program {
    ($($e:tt)*) => {{
        format!(
            r#"
var i = 0;
while (i < {num_iter}) {{
  i = i + 1;
}}
"#,
$(
    $e
)*
        )
    }};
}

#[macro_export]
macro_rules! equality_program {
    ($($e:tt)*) => {{
        format!(
            r#"
var i = 0;
while (i < {num_iter}){{
  i = i + 1;
  1 == 1; 1 == 2; 1 == nil; 1 == "str"; 1 == true;
  nil == nil; nil == 1; nil == "str"; nil == true;
  true == true; true == 1; true == false; true == "str"; true == nil;
  "str" == "str"; "str" == "stru"; "str" == 1; "str" == nil; "str" == true;
}}
"#,
$(
    $e
)*
        )
    }};
}

#[macro_export]
macro_rules! instantiation_program{
  ($($e:tt)*) => {{
        format!(
r#"
class Foo {{
  init() {{}}
}}

var i = 0;
while (i < {num_iter}) {{
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  i = i + 1;
}}
"#,
$(
    $e
)*
        )
    }};

}

#[macro_export]
macro_rules! fib_program{
  ($($e:tt)*) => {{
        format!(
r#"
fun fib(n) {{
    if (n < 2) return n;
      return fib(n - 2) + fib(n - 1);
}}
print fib({num_iter});
"#,
$(
    $e
)*
        )
    }};

}

#[macro_export]
macro_rules! properties_program {
  ($($e:tt)*) => {{
        format!(
r#"
class Foo {{
  init() {{
    this.field0 = 1;
    this.field1 = 1;
    this.field2 = 1;
    this.field3 = 1;
    this.field4 = 1;
    this.field5 = 1;
    this.field6 = 1;
    this.field7 = 1;
    this.field8 = 1;
    this.field9 = 1;
    this.field10 = 1;
    this.field11 = 1;
    this.field12 = 1;
    this.field13 = 1;
    this.field14 = 1;
    this.field15 = 1;
    this.field16 = 1;
    this.field17 = 1;
    this.field18 = 1;
    this.field19 = 1;
    this.field20 = 1;
    this.field21 = 1;
    this.field22 = 1;
    this.field23 = 1;
    this.field24 = 1;
    this.field25 = 1;
    this.field26 = 1;
    this.field27 = 1;
    this.field28 = 1;
    this.field29 = 1;
  }}

  method0() {{ return this.field0; }}
  method1() {{ return this.field1; }}
  method2() {{ return this.field2; }}
  method3() {{ return this.field3; }}
  method4() {{ return this.field4; }}
  method5() {{ return this.field5; }}
  method6() {{ return this.field6; }}
  method7() {{ return this.field7; }}
  method8() {{ return this.field8; }}
  method9() {{ return this.field9; }}
  method10() {{ return this.field10; }}
  method11() {{ return this.field11; }}
  method12() {{ return this.field12; }}
  method13() {{ return this.field13; }}
  method14() {{ return this.field14; }}
  method15() {{ return this.field15; }}
  method16() {{ return this.field16; }}
  method17() {{ return this.field17; }}
  method18() {{ return this.field18; }}
  method19() {{ return this.field19; }}
  method20() {{ return this.field20; }}
  method21() {{ return this.field21; }}
  method22() {{ return this.field22; }}
  method23() {{ return this.field23; }}
  method24() {{ return this.field24; }}
  method25() {{ return this.field25; }}
  method26() {{ return this.field26; }}
  method27() {{ return this.field27; }}
  method28() {{ return this.field28; }}
  method29() {{ return this.field29; }}
}}

var i = 0;
while (i < {num_iter}) {{
  foo.method0();
  foo.method1();
  foo.method2();
  foo.method3();
  foo.method4();
  foo.method5();
  foo.method6();
  foo.method7();
  foo.method8();
  foo.method9();
  foo.method10();
  foo.method11();
  foo.method12();
  foo.method13();
  foo.method14();
  foo.method15();
  foo.method16();
  foo.method17();
  foo.method18();
  foo.method19();
  foo.method20();
  foo.method21();
  foo.method22();
  foo.method23();
  foo.method24();
  foo.method25();
  foo.method26();
  foo.method27();
  foo.method28();
  foo.method29();
  i = i + 1;
}}
"#,
$(
    $e
)*
        )
    }};

}

#[macro_export]
macro_rules! invocation_program {
  ($($e:tt)*) => {{
        format!(
r"
class Foo {{
  method0() {{}}
  method1() {{}}
  method2() {{}}
  method3() {{}}
  method4() {{}}
  method5() {{}}
  method6() {{}}
  method7() {{}}
  method8() {{}}
  method9() {{}}
  method10() {{}}
  method11() {{}}
  method12() {{}}
  method13() {{}}
  method14() {{}}
  method15() {{}}
  method16() {{}}
  method17() {{}}
  method18() {{}}
  method19() {{}}
  method20() {{}}
  method21() {{}}
  method22() {{}}
  method23() {{}}
  method24() {{}}
  method25() {{}}
  method26() {{}}
  method27() {{}}
  method28() {{}}
  method29() {{}}
}}

var i = 0;
while (i < {num_iter}) {{
  foo.method0();
  foo.method1();
  foo.method2();
  foo.method3();
  foo.method4();
  foo.method5();
  foo.method6();
  foo.method7();
  foo.method8();
  foo.method9();
  foo.method10();
  foo.method11();
  foo.method12();
  foo.method13();
  foo.method14();
  foo.method15();
  foo.method16();
  foo.method17();
  foo.method18();
  foo.method19();
  foo.method20();
  foo.method21();
  foo.method22();
  foo.method23();
  foo.method24();
  foo.method25();
  foo.method26();
  foo.method27();
  foo.method28();
  foo.method29();
  i = i + 1;
}}
",$(
    $e
)*
        )
    }};

}
