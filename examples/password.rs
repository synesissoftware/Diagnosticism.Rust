use diagnosticism::diagnostics::Password;

use std::fmt as std_fmt;


struct Credentials {
    username : String,
    #[allow(dead_code)]
    password : String,
}

impl std_fmt::Debug for Credentials {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &Password::default())
            .finish()
    }
}


fn main() {
    let credentials = Credentials {
        username : "JohnSmith@email.com".into(),
        password : "abc123".into(),
    };

    println!("credentials: {credentials:#?}");
}


// ///////////////////////////// end of file //////////////////////////// //
