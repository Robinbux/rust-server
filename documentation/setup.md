# Setup for running the application

To run the application locally, you must install the following tools:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started)
- [Diesel ORM](http://diesel.rs)

---

1. Clone the repository and switch to the Assessment branch
    ```zsh
    git clone https://github.com/Robinbux/rust-server && cd rust-server && git checkout SE-19
    ```
2. Start Docker Desktop, then start the container.
    ```zsh
    docker-compose up -d
    ```

3. Run the Diesel migration.
    ```zsh
    diesel migration run
    ```

4. Run the server.
    ```zsh
    cargo run
    ```

5. You can now connect to the site on `localhost:8087/home`