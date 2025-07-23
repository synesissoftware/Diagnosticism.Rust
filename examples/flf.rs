use diagnosticism::{
    fileline,
    filelinefunction,
    filelinefunction_fully_qualified_name as filelinefunction_fqn,
    function_fully_qualified_name,
    function_name_only,
};

fn fl() {
    eprintln!("{}", file!());
}

fn fl_ln() {
    eprintln!("{}", fileline!());
}

fn fun() {
    eprintln!("{}", function_name_only!());
}

fn fun_fqn() {
    eprintln!("{}", function_fully_qualified_name!());
}

fn fl_ln_fun() {
    eprintln!("{}", filelinefunction!());
}

fn fl_ln_fun_fqn() {
    eprintln!("{}", filelinefunction_fqn!());
}

fn main() {
    eprintln!("");
    eprintln!("calling `fl()`:");

    fl();

    eprintln!("");
    eprintln!("calling `fun()`:");

    fun();

    eprintln!("");
    eprintln!("calling `fun_fqn()`:");

    fun_fqn();

    eprintln!("");
    eprintln!("calling `fl_ln()`:");

    fl_ln();

    eprintln!("");
    eprintln!("calling `fl_ln_fun()`:");

    fl_ln_fun();

    eprintln!("");
    eprintln!("calling `fl_ln_fun_fqn()()`:");

    fl_ln_fun_fqn();
}
