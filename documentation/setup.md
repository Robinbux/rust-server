# Setup for running the application

To run the application locally, you must install the following tools:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started)

Clone the repository and switch to the Assessment branch
```zsh
git clone https://github.com/Robinbux/rust-server && cd rust-server && git checkout SE-19
```
Start Docker Desktop, then start the container with `docker-compose up -d` 

Finally run the server with `cargo run`

You can now connect to the site on `localhost:8087/home`