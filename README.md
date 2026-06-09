# just-calculate-mcp

[![CI](https://github.com/pavansgill/just-calculate-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/pavansgill/just-calculate-mcp/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

A Rust [Model Context Protocol](https://modelcontextprotocol.io) server that gives
an AI assistant a real calculator — **45 math tools across 17 domains**, from
arithmetic and trigonometry to finance, linear algebra, statistics, and bitwise
ops — without drowning the client in 45 tool schemas.

That second part is the interesting bit, and the reason this repo exists as much
as a **demonstration of two tool-management patterns** as a calculator. If you're
here for the math, jump to [the catalog](#the-catalog). If you're here because
your own MCP server has too many tools, read on — that problem is what this
project is really about.

> Licensed under **Apache-2.0**. Use it, fork it, ship it in your own product —
> commercial or not. The patent grant and "as-is, no warranty" terms are there so
> nobody gets patent-ambushed and nobody gets sued. It's a gift to the world; go
> build something with it. See [the license section](#license).

---

## The problem: tool overload

MCP clients send **every** tool's schema to the model on **every** request. A
calculator that covers "all of math" can easily hit 40+ tools, and at that point:

- the tool list alone burns a large slice of the context window, every turn;
- many clients degrade — selection accuracy drops as the list grows;
- yet the model still needs to *discover* what's available and *how* to call it.

You can't win by just deleting tools (you lose capability) or by cramming
everything into one mega-tool (you lose validated, self-describing schemas). So
this server keeps all 45 tools **and** shows a tool list of only **two** by
default. It does that with a small **gating layer**, and it deliberately ships
**two complementary ways** through it so you can see both patterns side by side.

---

## The two approaches

By default the client sees exactly two tools — `toggle` and `act` — plus one
discovery resource. Everything else is hidden until asked for.

### 1. `toggle` — progressive disclosure

Reveal a whole category (or a single tool), use it with its **real, validated
schema**, then hide it again. Revealed tools appear/disappear from the live tool
list via the standard `notifications/tools/list_changed` event.

```jsonc
toggle { "target": "geometry", "on": true }   // reveals area_2d, volume_3d, … (6 tools)
toggle { "target": "area_2d",  "on": true }   // or just one tool
toggle { "target": "all",      "on": true }   // reveal everything
toggle { "target": "geometry", "on": false }  // hide again when done
```

**Best when** you'll make several related calls — you get the genuine input
schema, argument validation, and client-side autocomplete, at the cost of one
round-trip to turn the category on.

### 2. `act` — one-shot proxy

Call any tool by name in a single step, **without** revealing it. Works even
while the tool is toggled off.

```jsonc
act { "tool": "area_2d", "args": { "shape": "circle", "a": 5 } }
// → area = 78.53981633974483
```

**Best when** you want a quick one-off — zero round-trips, zero footprint in the
tool list. The trade-off is that the model leans on the guide (below) for the
argument shape instead of an inline schema.

### Discovery: the `calc://guide` resource

Neither approach is useful if the model can't find out what exists. So the server
exposes an MCP **resource** at `calc://guide` — a generated catalog of every
category, every tool, and how to use both meta-tools. The model reads it once and
knows the whole surface area, without 45 schemas ever entering the context.

**The payoff:** default context drops from ~45 schemas to **2 tools + 1 on-demand
resource**, while the full calculator stays reachable two different ways.

---

## Quick start

Requires a [Rust toolchain](https://rustup.rs).

```bash
git clone https://github.com/pavansgill/just-calculate-mcp
cd just-calculate-mcp
cargo build --release
```

Register it with any MCP client. For Claude Code / Claude Desktop, point at the
binary (or use `cargo run`):

```jsonc
{
  "mcpServers": {
    "just-calculate": {
      "command": "/path/to/just-calculate-mcp/target/release/just-calculate-mcp"
    }
  }
}
```

The server speaks JSON-RPC 2.0 over stdio. On connect, the client will see
`toggle` and `act`, and can read `calc://guide` to discover the rest.

### Smoke test from the shell

The `act` proxy reaches any tool in one shot without toggling:

```bash
printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}\n{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"act","arguments":{"tool":"simple_arithmetic","args":{"number_a":10,"number_b":3,"operator":"/"}}}}\n' \
  | ./target/release/just-calculate-mcp
# → 10 / 3 = 3.3333333333333335
```

---

## The catalog

45 tools across 17 categories. Read `calc://guide` at runtime for the live list
and per-tool argument shapes.

| Domain | Tools |
| --- | --- |
| **arithmetic** | simple_arithmetic, rounding, number_properties |
| **powers** | power_root, exponential |
| **logarithm** | logarithm |
| **trigonometry** | trigonometry, trigonometry_2arg, hyperbolic, angle_convert |
| **statistics** | descriptive_stats, percentile, correlation, linear_regression |
| **combinatorics** | combinatorics |
| **number_theory** | number_theory, modular_arithmetic |
| **finance** | present_future_value, interest_rate, loan, investment_return, depreciation, business_math |
| **geometry** | area_2d, perimeter_2d, volume_3d, surface_area_3d, distance_2d, coordinate_convert |
| **programmer** | base_convert, bitwise_ops, number_repr |
| **unit_convert** | unit_convert |
| **engineering** | complex_number, electrical, decibel, interpolation |
| **linear_algebra** | vector_ops, matrix_ops |
| **calculus** | polynomial_calc, numerical_methods |
| **probability** | distribution, odds_convert |
| **special_functions** | special_functions |
| **constants** | math_constant |

---

## How it's built

```
src/
├── main.rs           — wires Calculator to the stdio transport
├── server.rs         — Calculator struct, the #[tool_router] block, the
│                       hand-written ServerHandler, and the toggle/act gating
└── tools/
    ├── registry.rs   — category → tool-name map; the single source of truth
    │                   that drives both `toggle` and the calc://guide resource
    ├── shared.rs     — small shared helpers (gcd, lcm, factorial, …)
    └── <category>.rs — pure math logic + input struct, one file per domain
```

The split is deliberate: `tools/` is **pure logic with no MCP types**, and
`server.rs` is **all the MCP wiring**. The gating layer holds two routers — a
toggleable `visible` set (everything hidden at startup except the two meta-tools)
and a `full` set used only by `act` — both generated from the same
`#[tool_router]` macro, so there's no second dispatch table to keep in sync.

Built on [rmcp](https://crates.io/crates/rmcp) v1.7. The gating uses rmcp's
native `disable_route` / `enable_route` and `list_changed` notifications — it's
standard MCP, not a hack.

See [CLAUDE.md](CLAUDE.md) for the full architecture notes and the steps to add a
new math category.

---

## Development

```bash
cargo test     # unit tests + an in-memory client/server integration test
cargo build    # verify it compiles
```

Adding a tool is four small steps — write the pure logic in `tools/<category>.rs`,
add a `#[tool]` method in `server.rs`, **register its name in
`tools/registry.rs`** (so `toggle` and the guide can see it), and add tests.
Details in [CLAUDE.md](CLAUDE.md).

---

## License

Copyright 2026 Pavan Singh Gill. Licensed under the
[Apache License, Version 2.0](LICENSE).

In plain terms:

- **Use it for anything** — personal, commercial, in a closed product, anywhere.
- **Patent grant + retaliation** — contributors grant you a patent license for
  what they contribute, and anyone who weaponizes a patent against the project
  loses their license. Because the code is published openly, it also stands as
  **prior art**, which is what actually stops someone patenting these ideas out
  from under everyone.
- **No warranty, no liability** — it's provided as-is. Don't sue me if your
  rocket misses the moon because of a rounding error.

Contributions are welcome and, per Apache-2.0 §5, come in under the same terms.
This is part of my open-source work — built in the open, given freely. Enjoy it.
