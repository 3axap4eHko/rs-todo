# Todo API

A simple, layered **Todo** REST API written in **Rust** using **Actix-web**.

## Features

- CRUD operations for Todo items
- Layered architecture:
  - **domain**: core models, errors, repository trait
  - **app**: business logic in `TodoService`
  - **infra**: in-memory repository implementation
  - **interfaces**: HTTP handlers, configuration, and routing
- Environment-based configuration (PORT, LOG_LEVEL)
- Logging via `env_logger`
- Comprehensive unit tests and HTTP handler tests

## Prerequisites

- Rust toolchain (1.70+)
- Cargo
- (Optional) [`dotenv-cli`](https://crates.io/crates/dotenv)

## Getting Started

1. **Clone the repository**
   ```sh
   git clone https://github.com/your-username/todo-api.git
   cd todo-api
   ```

2. **Configure environment**
   Copy `.env.example` to `.env` and adjust if needed:
   ```env
   PORT=8080
   LOG_LEVEL=info
   ```

3. **Build & run**
   ```sh
   cargo run
   ```
   The server will start on `http://localhost:8080`.

## API Endpoints

| Method | Path             | Description                         | Body                     |
| ------ | ---------------- | ----------------------------------- | ------------------------ |
| GET    | `/`              | Health check (returns `"OK"`)     | —                        |
| GET    | `/todos`         | List all todos                      | —                        |
| GET    | `/todos/{id}`    | Get a single todo by ID             | —                        |
| POST   | `/todos`         | Create a new todo                   | `{ "title": "..." }` |
| PUT    | `/todos/{id}`    | Update an existing todo             | `{ "title": ".", "completed": true }` |
| DELETE | `/todos/{id}`    | Delete a todo                       | —                        |

## Project Structure

```
├── src
│   ├── app            # Business logic
│   ├── domain         # Core models, errors, and repository trait
│   ├── infra          # In-memory repository implementation
│   ├── interfaces     # HTTP handlers, extractors, router, config
│   ├── main.rs        # Server bootstrap
│   └── lib.rs         # Library exports
├── Cargo.toml         # Dependencies & metadata
└── README.md          # This file
```

## Testing

Run the full test suite:
```sh
cargo test
```

## Logging

By default, logs are printed at the level specified by the `LOG_LEVEL` environment variable. For example:
```sh
LOG_LEVEL=debug cargo run
```

## License

Dual-licensed under **MIT** OR **Apache-2.0**. See [LICENSE](LICENSE) for details.

