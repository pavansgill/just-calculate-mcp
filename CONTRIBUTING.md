# Contributing to just-calculate-mcp

Thanks for your interest — contributions are welcome, whether that's a new math
tool, a bug fix, or a docs improvement. This project is a calculator exposed over
the [Model Context Protocol](https://modelcontextprotocol.io), and it's meant to
keep growing without overwhelming MCP clients.

By contributing, you agree that your contributions are licensed under the
project's [Apache License 2.0](LICENSE) (see section 5 of the license).

## Getting started

You'll need a [Rust toolchain](https://rustup.rs) (stable).

```bash
git clone https://github.com/pavansgill/just-calculate-mcp
cd just-calculate-mcp
cargo build
cargo test
```

## Before you open a PR

CI runs three checks on every pull request, and they must pass. Run them locally
first — they're the exact commands the workflow uses:

```bash
cargo fmt --all -- --check          # formatting
cargo clippy --all-targets -- -D warnings   # lints (warnings are errors)
cargo test --all                    # unit + integration tests
```

If `cargo fmt --all -- --check` complains, just run `cargo fmt --all` to fix it.

## Architecture in one minute

The split between pure logic and MCP wiring is intentional — please keep it.

```
src/
├── main.rs           — wires Calculator to the stdio transport
├── server.rs         — Calculator struct, the #[tool_router] block, the
│                       hand-written ServerHandler, and the toggle/act gating
└── tools/
    ├── registry.rs   — category → tool-name map; the single source of truth
    │                   driving both `toggle` and the calc://guide resource
    ├── shared.rs     — small shared helpers (gcd, lcm, factorial, …)
    └── <category>.rs — pure math logic + input struct, one file per domain
```

- `tools/` is **pure logic with no MCP types**.
- `server.rs` is **all the MCP wiring**. Don't put logic here; don't put rmcp
  types in `tools/`.

## Adding a new math tool

1. **Logic** — create or extend `src/tools/<category>.rs`: define the input
   struct (`#[derive(Deserialize, JsonSchema)]`) and a `compute()`-style function
   that returns a `String` result.
2. **Module** — add `pub mod <category>;` to `src/tools/mod.rs` if the file is new.
3. **Wiring** — add a `#[tool]` method to the `#[tool_router]` impl block in
   `src/server.rs` that just delegates to your logic function.
4. **Register** — add the tool's name to the matching category in
   `src/tools/registry.rs`. **This step is required:** the gating layer hides
   every tool that isn't a registered meta-tool target, so an unregistered tool
   has no `toggle` entry and is missing from the `calc://guide` resource —
   effectively unreachable except via `act`.
5. **Tests** — add unit tests inside the new module.

### Naming

Snake case, and be specific: `simple_arithmetic`, `standard_deviation`,
`area_2d`. Avoid generic names like `calculate` or `math`.

## Commits and pull requests

- Branch off `main`; open your PR against `main`.
- Keep commits focused, with clear messages (we loosely follow
  `type: summary` — e.g. `feat:`, `fix:`, `docs:`, `refactor:`, `ci:`).
- Make sure CI is green and add tests for new behavior.
- A maintainer will review before merge.

## Reporting bugs and proposing features

Open a GitHub issue describing what you expected, what happened, and (for bugs)
the steps or tool call that triggered it. For a new math domain, a short note on
the operations you'd like and example inputs/outputs is plenty to start.
