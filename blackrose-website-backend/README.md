# Instructions on how to run the backend locally:
These instructions assume you have cloned this repository locally.
First, we have to setup the database. Then, we have to install rust.
## Setting up the database (PostgresQL)
### Setting up postgres with a docker container (recommended)
You don't need to use a docker container, but I find it the easiest. I've also included a docker-compose.yml file for ease of use.

Instructions for how to install docker can be found (here)[https://docs.docker.com/get-docker/].

Once you've installed docker, open a terminal and navigate to the `db` folder in the `blackrose-website-backend` folder. Open the `docker-compose.yml` file and type values for `POSTGRES_USER` and `POSTGRES_PASSWORD`. This sets a username and password for accessing the local database. Save the file. Finally, run the following command in the same folder:

`docker-compose up -d`

This should start the container. Optionally, you can check if it's working by opening a terminal and running the following command (note that this requires you to have the `postgresql-client` package installed if you're on Debian Linux):

`psql postgres://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:8002`

Where `<POSTGRES_USER>` and `<POSTGRES_PASSWORD>` are the same as the ones you set above. If it worked, you should see a greeting that you've entered postgres. Exit out of this screen by typing `\q` and hitting enter.

#### Note on the database docker container

This database will continue to run unless stopped manually. You can stop it by running the following command in your terminal:

`docker stop blackrose-website-db`

If successful, it should print `blackrose-website-db`. If you want to restart it, you can just run the following command:

`docker start blackrose-website-db`

## Installing Rust

Go to the [rust foundation website](https://www.rust-lang.org/tools/install) and follow the instructions to install Rust via `rustup`.

## Configuring the backend

Assuming everything has worked out now, you need to install set the following environment variables in a file called `.env` in the `blackrose-website-backend` folder:
```
DATABASE_URL=postgres://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:8002
SECRET=
EMAIL_RELAY=
EMAIL=
EMAIL_PASSWORD=
BASE_URL=http://localhost:4000
```
Here's what each variable means:
`DATABASE_URL` is the URL used to access the database. Replace `<POSTGRES_USER>` and `<POSTGRES_PASSWORD>` with the values you set for the database.
`SECRET` is a random string. You can generate one at this website [here](https://randomkeygen.com/).
`EMAIL_RELAY` is a URL to an SMTP email relay. This is required if you're testing email registration and want to send emails. Learn about setting up an SMTP account [here](https://www.hostinger.com/tutorials/how-to-use-free-google-smtp-server)
`EMAIL` is the actual email that will be used to send emails. See the previous thing here.
`EMAIL_PASSWORD` is the password for the actual email described above.
`BASE_URL` is the base URL for the local backend. This is only required if you're testing email registration and want to send emails.

## Running the backend
After that, you should be good to run the backend by changing your directory to `blackrose-website-backend` and running the following command:

`cargo run`

It should compile and run the backend. To test if it's working, you could send a request to one of the endpoints specified in the [REST API documentation here](REST_API.md) file.


# Backend Features to implement:
## Application-wide changes
1. use a pool of connections rather than a single asyncpg connection, you fucking idiot (see the bb8 crate)
## Registration API Validation
1. Consider rewriting using the more maintained, polished [garde](https://docs.rs/garde/latest/garde/)
## Blog API
1. Unprotected GET endpoint that maps to the BlogPost model
2. Unprotected GET endpoint that maps to the Comment model
3. User-protected POST endpoint that maps to the Comment model
## Admin API
1. Admin-protected GET and POST endpoints that map to our BlogPost, Comment, and User models.