# AI Coding Template

A minimal full-stack scaffold for AI-assisted coding projects.

## Stack

- Frontend: React, Vite, TypeScript
- Backend: Go, Gin
- Data layer convention: sqlc, PostgreSQL, Redis
- Build: `build.sh`
- Environment variables: `mise.toml`

## Layout

```text
backend/              Go backend
  cmd/server/         Server entry point
  internal/server/    Gin router and HTTP tests
frontend/             React frontend
  src/                Application source
build.sh              One-shot release build
mise.toml             Environment variables only
dist/                 Build output, ignored by git
```

## Development

Run the backend:

```bash
cd backend
go run ./cmd/server
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

The default backend target is Linux AMD64. Artifacts are written to `dist/`:

- `dist/frontend.tar.gz`
- `dist/backend`

To build another target, override `GOOS` and `GOARCH` together.

## Next Steps For Apps

- Add database migrations under `backend/migrations/`.
- Add sqlc queries and generated code under `backend/internal/db/`.
- Add Redis wiring under `backend/internal/` when a workflow needs caching or queues.
- Add API contracts under `backend/api/` when external clients need stable schemas.
