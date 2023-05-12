# Here is the REST API documentation
This is split up into sections based on the resource in question.
## How to make requests via the terminal
If you want to send HTTP requests via the terminal, you can use `curl`. Here's an example where we request the root resource:

`curl -s -w '\n' -H 'Content-Type: application/html' http://localhost:4000`

This will print out the result:
```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Black Rose</title>
</head>

<body>
    <header>
    Hello! Welcome to Black Rose!

    </header>
    <h1>

    </h1>
</body>

</html>
```

### Notes on application types
If you're using a JSON endpoint, use `application/json`. If you're using a HTML endpoint, use `application/html`.
### Notes on authentication
We use JSON web tokens for user authentication. These are obtained by logging in via the `/login` JSON endpoint. In essence, every request made to a protected endpoint must be made using an "access_token". Here is an example of trying to access a hypothetical protected `/account` endpoint:

`curl -s -w '\n' -H 'Content-Type: application/json' -H 'Authorization: Bearer <INSERT_TOKEN_HERE>' http://localhost:4000/login`

Which would return the requested resource assuming the access token is correct.

# AUTHENTICATON ENDPOINTS

TODO