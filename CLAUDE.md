# CLAUDE.md

Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

## 5. Skill Usage Constraints

**Use skills only when relevant, and never let them expand scope.**

- Before acting, check for relevant or explicitly requested skills.
- Load current skill instructions before relying on memory.
- Use the smallest skill set that covers the task; process skills come before implementation skills.
- Do not use skills to justify extra features, files, refactors, tests, or workflows.
- Priority: direct user instructions → project instructions → skill instructions → default behavior.

## 6. Development Stack Constraints

**Default to the approved stack unless the user explicitly says otherwise.**

- Frontend: React, Vite, TypeScript. Use shadcn/ui when adding reusable UI components.
- Backend: Go and Gin. Use sqlc, PostgreSQL, and Redis when the feature needs persistence, queries, caching, or queues.
- Keep API contracts typed and explicit; do not invent parallel REST/RPC styles without approval.
- Use explicit SQL with sqlc for database access; avoid ORMs unless the user explicitly requests one.
- Use database migrations for schema changes; do not hand-edit live schemas.
- Put environment-specific values in configuration or environment variables, not source code.
- Do not introduce alternative frameworks, ORMs, databases, cache layers, UI kits, or build tools without asking first.

## 7. Directory Structure

- `backend/` — Go source. Entry points live under `cmd/`; shared application code belongs under `internal/`; migrations and API specs live under `migrations/` and `api/` when needed.
- `frontend/` — React / Vite / TypeScript source. Application code lives under `src/`; static files live under `public/` when needed.
- `frontend/vite.config.ts` — Vite configuration, including the local `/api` proxy to the backend.
- `build.sh` — one-shot script that builds both frontend and backend from the repo root.
- `dist/frontend.tar.gz` — frontend build artifact, created from `frontend/dist/`.
- `dist/backend` — backend build artifact for non-Windows targets.
- `dist/backend.exe` — backend build artifact when `GOOS=windows`.
- `dist/` is build output — do not commit it; do not edit by hand.

Rules:
- Colocate related code by feature, not by file type.
- Generated code (sqlc, OpenAPI) lives in clearly marked folders; do not hand-edit.
- Keep release artifacts under `dist/`; do not add alternate output directories without updating `build.sh`.

## 8. Build

Run `./build.sh` from the repo root on Linux, macOS, or Windows Git Bash. It produces both artifacts under `dist/` and defaults to a Linux AMD64 backend binary.

- Frontend: `pnpm install` + `pnpm build` → `dist/frontend.tar.gz`.
- Backend default: `CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -trimpath -buildvcs=false -ldflags='-s -w'` → `dist/backend`.
- Windows target: `GOOS=windows GOARCH=amd64 ./build.sh` → `dist/backend.exe`.
- Optional: `BACKEND_UPX=1 ./build.sh` runs UPX if installed, for further compression.
- With mise on Windows, run `mise exec -- bash ./build.sh`. Do not run `mise exec -- ./build.sh`; Windows will try to execute the shell script as a native Win32 program.

Overridable env: `GOOS`, `GOARCH`, `CGO_ENABLED`, `BACKEND_PKG`, `BACKEND_BIN`, `BACKEND_UPX`. Set `GOOS` and `GOARCH` together when cross-compiling to a non-default target.

Do not run `go build` or `pnpm build` manually for releases — use the script so artifact paths and flags stay consistent.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.
