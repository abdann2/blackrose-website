# Features to implement:
## Registration/Login REST endpoints:
1. Input Validation for these REST endpoints using the `validator` crate
See [this axum example](https://github.com/tokio-rs/axum/blob/main/examples/validator/src/main.rs) for direction.
    1. Define `FromRequest` implementation on `UserCredentials` and `UserRegistrationCredentials`. This would use the `DbInterface` state to do the checking of the things done in `registration_handler` and `login_handler`. In addition, the `Validate` trait would be derived on the aforementioned structs.
    2. Refactor `registration_handler` and `login_handler` to use `UserCredentials(credentials)` and `UserRegistrationCredentials(credentials)`. The handlers would then just create the corresponding `Responses`; no checks required.
