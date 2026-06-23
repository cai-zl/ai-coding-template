# AI Coding Template

A minimal full-stack scaffold for AI-assisted coding projects.

## Stack

- Frontend: React, Vite, TypeScript
- Backend: Rust, axum, tokio
- ORM: sea-orm 1.1
- Database: PostgreSQL
- Cache: Redis
- Build: `build.sh`
- Environment variables: `mise.toml`

## Layout

```text
backend/              Rust backend
  src/
    main.rs           Server entry point
    config.rs         Env config
    state.rs          AppState (sea-orm DB + redis connection)
    routes/           Axum handlers
    entity/           sea-orm entities
    migration/        sea-orm migrations
  tests/              Integration tests
frontend/             React frontend
  src/                Application source
build.sh              One-shot release build
mise.toml             Environment variables only
dist/                 Build output, ignored by git
```

## Development

Prepare environment variables:

```bash
cp backend/.env.example backend/.env
# edit backend/.env with your DATABASE_URL and REDIS_URL
```

Run the backend (it applies migrations on startup):

```bash
cd backend
cargo run
```

Run the frontend:

```bash
cd frontend
pnpm install
pnpm dev
```

The Vite dev server proxies `/api` to `http://localhost:8080`.

## Build

From the repo root:

```bash
./build.sh
```

On Windows with mise:

```bash
mise exec -- bash ./build.sh
```

Artifacts are written to `dist/`:

- `dist/frontend.tar.gz`
- `dist/backend[.exe]`

Optional overrides:

- `BACKEND_TARGET=debug` — use debug profile instead of release
- `BACKEND_RUST_TARGET=x86_64-unknown-linux-gnu` — cross-compile via rustup target
- `BACKEND_UPX=1` — UPX-compress the binary if `upx` is installed

## Sample API

The scaffold ships a `Todo` resource demonstrating the full stack:

- `GET    /api/todos`           — list
- `GET    /api/todos/:id`       — get (cached in redis)
- `POST   /api/todos`           — create
- `PUT    /api/todos/:id`       — update
- `DELETE /api/todos/:id`       — delete

Schema is defined in `backend/src/migration/`.

## Next Steps For Apps

- Add new sea-orm entities under `backend/src/entity/`.
- Add new migrations under `backend/src/migration/` and register them in `Migrator`.
- Add new route modules under `backend/src/routes/`.
- Add redis usage patterns alongside the cache pattern in `backend/src/routes/todo.rs`.
