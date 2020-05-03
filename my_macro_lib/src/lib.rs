///
pub trait Login {
    fn login(&self, user_name: &str, password: &str) -> bool;
}

/// The macro which can implement the `Login` trait.
#[macro_export]
macro_rules! impl_login {

    //
    // Pass in a `Struct` type name, we implement the `Login` trait
    // for it automatic.
    //
    ($target_struct: ty) => {{
        impl Login for $target_struct {
            fn login(&self, user_name: &str, password: &str) -> bool {
                println!(
                    "login method get called with user_name: '{}' and password: '{}'",
                    user_name, password
                );
                true
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_login() {
        struct MyTestStruct {}

        // After calling the `impl_login!` macro, the `MyTestStruct` should
        // be already implemented the `Login` trait which allows us to call
        // `.login()` and return a `true`.
        impl_login!(MyTestStruct);

        let test_struct = MyTestStruct {};
        let user_name = "wison";
        let password = "my_password";
        let login_result = test_struct.login(&user_name, &password);
        assert_eq!(login_result, true);
    }
}
