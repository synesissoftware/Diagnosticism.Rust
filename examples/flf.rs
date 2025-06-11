use diagnosticism::{
    fileline,
    filelinefunction,
    filelinefunction_fully_qualified_name as filelinefunction_fqn,
    function_fully_qualified_name,
    function_name_only,
};

struct Context {}

impl Context {

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
}

impl Context {

    fn m_fl(&self) {
        eprintln!("{}", file!());
    }

    fn m_fl_ln(&self) {
        eprintln!("{}", fileline!());
    }

    fn m_fun(&self) {
        eprintln!("{}", function_name_only!());
    }

    fn m_fun_fqn(&self) {
        eprintln!("{}", function_fully_qualified_name!());
    }

    fn m_fl_ln_fun(&self) {
        eprintln!("{}", filelinefunction!());
    }

    fn m_fl_ln_fun_fqn(&self) {
        eprintln!("{}", filelinefunction_fqn!());
    }
}


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

    {
        eprintln!("");
        eprintln!("---------------");
        eprintln!("");
        eprintln!("free functions:");

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
        eprintln!("calling `fl_ln_fun_fqn()`:");

        fl_ln_fun_fqn();

        eprintln!("");
    }

    {
        eprintln!("");
        eprintln!("--------------------");
        eprintln!("");
        eprintln!("associated functions:");

        eprintln!("");
        eprintln!("calling `Context::fl()`:");

        Context::fl();

        eprintln!("");
        eprintln!("calling `Context::fun()`:");

        Context::fun();

        eprintln!("");
        eprintln!("calling `Context::fun_fqn()`:");

        Context::fun_fqn();

        eprintln!("");
        eprintln!("calling `Context::fl_ln()`:");

        Context::fl_ln();

        eprintln!("");
        eprintln!("calling `Context::fl_ln_fun()`:");

        Context::fl_ln_fun();

        eprintln!("");
        eprintln!("calling `Context::fl_ln_fun_fqn()`:");

        Context::fl_ln_fun_fqn();

        eprintln!("");
    }

    {
        eprintln!("");
        eprintln!("--------");
        eprintln!("");
        eprintln!("methods:");

        let ctxt = Context{};

        eprintln!("");
        eprintln!("calling `Context#fl()`:");

        ctxt.m_fl();

        eprintln!("");
        eprintln!("calling `Context#m_fun()`:");

        ctxt.m_fun();

        eprintln!("");
        eprintln!("calling `Context#m_fun_fqn()`:");

        ctxt.m_fun_fqn();

        eprintln!("");
        eprintln!("calling `Context#m_fl_ln()`:");

        ctxt.m_fl_ln();

        eprintln!("");
        eprintln!("calling `Context#m_fl_ln_fun()`:");

        ctxt.m_fl_ln_fun();

        eprintln!("");
        eprintln!("calling `Context#m_fl_ln_fun_fqn()`:");

        ctxt.m_fl_ln_fun_fqn();

        eprintln!("");
    }
}
