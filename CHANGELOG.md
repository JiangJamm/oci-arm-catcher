# Changelog

## Unreleased

- The bash / PowerShell scripts, their helpers and tests moved to `legacy/`,
  with their own README. They still read the `.env` in the repo root. The old
  root layout is kept at the `scripts-final` tag.
- README: one copyable block per install method.

## 0.1.1 — 2026-10-27

- Colour output on terminals: dim headers, red error codes, yellow countdown,
  green success. `NO_COLOR` turns it off.
- `discover` prints `SSH_KEY_FILE="$HOME/.ssh/..."`, so the line works on any
  machine.
- `OCI_ARM_CATCHER_ENDPOINT` points the client at another base URL (mock API
  for tests and the README demos).
- Published to crates.io: `cargo install oci-arm-catcher`.

## 0.1.0 — 2026-10-27

First release of the Rust binary.

- One self-contained binary for macOS, Linux and Windows that talks to the OCI
  API directly — no OCI CLI, no Python.
- `run` (default) retries `LaunchInstance` until capacity appears, rotating
  availability domains; read-only `check` and `discover`.
- Errors are classified by HTTP status: 4xx stops, 429 / 5xx / network retry.
  After a network error the same request is resent with the same
  `opc-retry-token`, so a lost response cannot create a second instance.
- Reads the classic scripts' `.env`; flag > environment variable > `.env`.
