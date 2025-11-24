# NuggetSync

<div align="center">

**The minimal, zero-knowledge synchronization backend for [NuggetVPN](https://github.com/Rigby-Foundation/nuggetvpn).**

*"Stupid, but reliable.”*

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=flat&logo=docker&logoColor=white)

</div>

---

## About

NuggetSync Server is a lightweight backend service designed for one purpose: to securely store encrypted VPN configuration profiles for NuggetVPN users.

It is built with a **Zero-Trust, Zero-Knowledge** philosophy. The server **never** sees your data in plaintext. It acts as a dumb, reliable storage locker for encrypted binary blobs.

### Key Features

* **True Zero-Knowledge:** All encryption and decryption happen on the client (NuggetVPN app) using a master password that never leaves your device. The server only stores encrypted noise.
* **Blazing Fast:** Built with Rust, [Axum](https://github.com/tokio-rs/axum), and [Tokio](https://tokio.rs/). Designed for high performance and low resource usage.
* **Self-Hostable:** Easily deployable via Docker on any cheap VPS or home server. You own your data infrastructure.
* **Database Agnostic:** Uses [Diesel ORM](https://diesel.rs/), supporting PostgreSQL by default (SQLite and MySQL support can be added).

## Architecture & Security Model

Understanding *what this server DOES NOT do* is key to understanding its security.

### What the Client Does (NuggetVPN App)

1.  Takes your user-provided **Master Password**.
2.  Uses **Argon2id** (a memory-hard password hashing function) on your device to derive a strong encryption key.
3.  Encrypts your profile data using **XChaCha20-Poly1305** (a modern, authenticated encryption algorithm).
4.  Sends the resulting encrypted binary blob (along with the salt and nonce needed for decryption) to the server.

### What the Server Does (This Repo)

1.  **Authenticates** your request using a secure Bearer token (Redis-backed session).
2.  **Validates** that the uploaded blob is within acceptable size limits (e.g., < 1MB).
3.  **Stores** the encrypted blob in the database against your user ID.
4.  **Retrieves** the blob when asked.

**Crucially:** The server **cannot** decrypt your data. It does not have your master password, and it does not have the derived encryption key. If the server is compromised, attackers only get useless encrypted data.

---

## Development & Setup

### Prerequisites

* **Rust & Cargo:** The latest stable version ([rustup.rs](https://rustup.rs/)).
* **PostgreSQL:** A running instance of PostgreSQL database.
* **Diesel CLI:** For running migrations (`cargo install diesel_cli --no-default-features --features postgres`).

### Local Setup

1.  **Clone the repository:**

    ```bash
    git clone [https://github.com/Rigby-Foundation/nuggetsync.git](https://github.com/Rigby-Foundation/nuggetsync.git)
    cd nuggetsync
    ```

2.  **Environment Setup:**
    Create a `.env` file in the root directory and configure your database connection and JWT secret.

    ```env
    # .env
    DATABASE_URL=postgres://user:password@localhost:5432/nugget_sync_db
    REDIS_URL=redis://127.0.0.1:6379
    RUST_LOG=nugget_sync_server=debug,tower_http=debug
    ```

3.  **Database Setup:**

    ```bash
    # Create the database (if it doesn't exist)
    diesel setup

    # Run migrations to create tables
    diesel migration run
    ```

4.  **Run the Server:**

    ```bash
    cargo run
    ```

The server will start listening on `http://0.0.0.0:3001`.

### Docker Deployment (Recommended for Prod)

A `Dockerfile` is provided for easy deployment.

1.  **Build the image:**

    ```bash
    docker build -t nugget-sync-server .
    ```

2.  **Run the container:**
    Make sure to provide the necessary environment variables, preferably via an `.env` file or Docker secrets. You'll need a PostgreSQL container running in the same network or an external DB URL.

    ```bash
    # Example using --env-file (ensure DATABASE_URL points to a reachable DB instance)
    docker run -d \
      -p 3001:3001 \
      --env-file .env \
      --name nugget-sync \
      nugget-sync-server
    ```

### Docker Compose

A `docker-compose.yml` is provided for a complete stack setup (App + Postgres + Redis).

```bash
docker-compose up -d --build
```

## Contributing

We welcome contributions! Whether it's a bug report, feature request, or a pull request, your help is appreciated.

Please ensure any PRs follow the existing code style and include tests where appropriate.

## License

This project is open-source and available under the [GPL-3.0 License](LICENSE).

---

<div align="center">
  Built with 🧡 and a healthy dose of paranoia by the <a href="https://github.com/Rigby-Foundation">Rigby Foundation</a>.
</div>