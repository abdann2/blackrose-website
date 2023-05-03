# Features to implement:
## Registration/Login REST endpoints:
1. Email registration confirmation
See *Zero to Production, chapter 7* for reference on high level concepts and best practices.
    1. Will need to make the following schema changes in a migration called `token_registration` to accomodate this feature:
        1. Add a table called `registration_tokens` which references a user on foreign key and stores the registration token
        2. Alter the `users` table by adding a new not nullable bool field called `email_confirmed`
    2. Generate a unique token and store it in the database via the schema above.
        1. The token string should be wrapped in a `secrecy::Secret` to prevent accidental logging of the token
    3. Send registration confirmation email
        1. Make an `Email` struct that is stored in the shared state. This will require refactoring the shared state from `DbInterface` to a larger type which contains both `Email` and `DbInterface`. The `Email` struct should contain a single field: `mailer`, which is an instance of `mailer` from the `lettre` crate.
        2. Make a constructor to make the `Email` struct, taking a `email`, `password`, and `email_domain`.
        3. Have the `registration_handler` build a `lettre::Message`, of which the body contains the token in a url query to the /registration/confirm resource
        3. Access the `Email` shared state and send the message.
        4. Return success response
    4. Make a handler for 'registration/confirm' which expands a query for the `token`. If the token is found successfully in the `registration_tokens` table, return a success response and update the user's `email_confirmed` as True. Delete the entry in `registration_tokens` when done.
    5. Modify `login_handler` to not allow logins for users who have not confirmed their emails
2. Input Validation for these REST endpoints using the `validator` crate
See [this axum example](https://github.com/tokio-rs/axum/blob/main/examples/validator/src/main.rs) for direction.
    1. Define `FromRequest` implementation on `UserCredentials` and `UserRegistrationCredentials`. This would use the `DbInterface` state to do the checking of the things done in `registration_handler` and `login_handler`. In addition, the `Validate` trait would be derived on the aforementioned structs.
    2. Refactor `registration_handler` and `login_handler` to use `UserCredentials(credentials)` and `UserRegistrationCredentials(credentials)`. The handlers would then just create the corresponding `Responses`; no checks required.
