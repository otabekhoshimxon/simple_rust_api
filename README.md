# Simple REST API

A simple REST API project built with Rust and [Axum](https://github.com/tokio-rs/axum). The project performs CRUD operations on users and stores data in memory.

**Tech stack:**
- **Rust** — programming language
- **Axum** — HTTP server/router framework
- **Tokio** — async runtime
- **Serde** — JSON serialize/deserialize
- **Utoipa + Swagger UI** — automatic OpenAPI documentation
- **Tracing** — logging

**Endpoints:**
| Method | Path | Description |
|---|---|---|
| POST | `/api/users` | Create a new user |
| GET | `/api/users` | Get all users |
| GET | `/api/users/{id}` | Get a user by ID |
| DELETE | `/api/users/{id}` | Delete a user |

Swagger UI: `http://localhost:3000/swagger-ui`

## 1. Installing Rust

Rust is installed via `rustup` (Rust and Cargo are installed together).

### Windows

1. Go to [https://rustup.rs](https://rustup.rs), download `rustup-init.exe` and run it.
2. Or via PowerShell:
   ```powershell
   winget install Rustlang.Rustup
   ```
3. After installation finishes, reopen your terminal and verify:
   ```powershell
   rustc --version
   cargo --version
   ```

### Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```

On Debian/Ubuntu you may also need build tools first:
```bash
sudo apt update && sudo apt install build-essential -y
```

### macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```

If Xcode Command Line Tools are not installed yet:
```bash
xcode-select --install
```

## 2. Running the project

1. Clone the repository (or open the existing folder):
   ```bash
   git clone <repo-url>
   cd simple_rest_api
   ```

2. Fetch dependencies and run the project:
   ```bash
   cargo run
   ```

   On the first run, Cargo automatically downloads all required packages (listed in `Cargo.toml`) — an internet connection is required.

3. Once the server starts successfully, you'll see the following in the terminal:
   ```
   🚀 Server running on: http://localhost:3000
   📚 Swagger UI available at: http://localhost:3000/swagger-ui
   ```

4. Open Swagger UI in your browser to try out the API:
   ```
   http://localhost:3000/swagger-ui
   ```

### Useful commands

```bash
cargo build           # Compile the project (debug mode)
cargo build --release # Optimized (production) build
cargo run              # Compile and run immediately
cargo check             # Only check for errors (fast)
```

## 3. Quick API examples

```bash
# Create a user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"ali","email":"ali@example.com","password":"secret123"}'

# Get all users
curl http://localhost:3000/api/users

# Get a single user
curl http://localhost:3000/api/users/1

# Delete a user
curl -X DELETE http://localhost:3000/api/users/1
```

> **Note:** Data is stored in memory (RAM) — all records are lost when the server stops. This project is written for demo/learning purposes; for production use, a real database (e.g. Postgres) and proper password hashing (argon2/bcrypt) would be needed.

## 4. Deployment (GitHub Actions + systemd)

`.github/workflows/deploy.yml` builds a release binary on every push to `master` and deploys it to a Linux (Ubuntu/Debian, x86_64) server over SSH, running it as a `systemd` service.

### One-time server setup

Run these once on the target server:

```bash
sudo useradd -r -s /usr/sbin/nologin deploy   # dedicated user to run the service
sudo mkdir -p /opt/simple-rest-api
sudo chown deploy:deploy /opt/simple-rest-api
```

Copy `deploy/simple-rest-api.service` from this repo to `/etc/systemd/system/simple-rest-api.service` on the server, then:

```bash
sudo systemctl daemon-reload
sudo systemctl enable simple-rest-api
```

The service expects the binary at `/opt/simple-rest-api/simple_rust_api` (the workflow uploads it there on deploy).

### Required GitHub Secrets

Set these under **Settings → Secrets and variables → Actions**:

| Secret | Description |
|---|---|
| `SSH_HOST` | Server IP address or domain |
| `SSH_USER` | SSH user with permission to run `sudo systemctl` and write to `/opt/simple-rest-api` (e.g. via passwordless sudo for these commands) |
| `SSH_PRIVATE_KEY` | Private key matching a public key authorized on the server (`~/.ssh/authorized_keys`) |
| `SSH_PORT` | *(optional)* SSH port, defaults to `22` |

### How it works

1. **build** job compiles `cargo build --release` on `ubuntu-latest` and uploads the binary as an artifact.
2. **deploy** job downloads the artifact, `scp`s it to `/tmp/deploy` on the server, then over SSH stops the service, moves the new binary into place, and restarts it.

Trigger manually anytime via the **Actions** tab (`workflow_dispatch`), or it runs automatically on every push to `master`.
