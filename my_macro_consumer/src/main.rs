fn main() {
    use my_macro_lib::{impl_login, Login};

    struct UserService {}

    // After calling `impl_login`, the `UserService` struct should be
    // already implemented the `Login` trait which allows us to call
    // `UserService_INSTANCE.login()`
    impl_login!(UserService);

    let temp_service = UserService {};
    let login_result = temp_service.login(&"Wison", &"my_test_password");

    println!("login_result: {}", login_result);
}
