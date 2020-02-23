This is my first project with Rust; it'a an example of how to setup a simple set of CRUD API endpoints.  
For suggestion or improvements on the code, open an issue or create a pull request!  

## INITIAL SETUP

### DIESEL SETUP
Open a shell terminal, move to this repository's local directory and install Diesel CLI tool:  

```sh
cargo install diesel_cli
```
If you get errors about missing libraries, you need to install the missing **dev** libraries on your system. If you can't install those (or you don't want to mess with mysql/postgresql dev libraries) you can tell diesel to use only sqlite:  

```sh
cargo install diesel_cli --no-default-features --features sqlite
```
Note that in this way, you can use the diesel command line tool only on sqlite db.

Now create the db and run the included migration:  
```sh
diesel setup
```

### ROCKET SETUP
For Rocket to work, you have to use the Nightly build of Rust.  
If you are running on Stable channel, you can update to Nightly globally for all your projects:  

```
rustup default nightly
```
Or just for this project, by running the following command in this project directory:
```
rustup override set nightly
```

### RUN
You can run the project with `cargo run`.

## TESTING THE ENDPOINTS
The db provided has only one table (`users`) with five fields (`id`, `firstname`, `lastname`, `age`, `email`).

### LIST ALL
```sh
curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/users
```

### CREATE
```sh
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{
    "firstname": "John",
    "lastname": "Doe",
    "age": 24,
    "email": "jdoe@hotmail.com"
  }' \
  http://localhost:8000/users
```

### RETRIEVE
```sh
curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/users/1
```

### UPDATE
```sh
curl --header "Content-Type: application/json" \
  --request PUT \
  --data '{
    "firstname": "Robert",
    "lastname": "Jones",
    "age": 34,
    "email": "r.jones@gmail.com"
  }' \
  http://localhost:8000/users/1
```

### DELETE
```sh
curl --header "Content-Type: application/json" \
  --request DELETE \
  http://localhost:8000/users/1
```