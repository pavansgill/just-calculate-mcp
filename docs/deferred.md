# Deferred Tool Implementations

Items from the math catalog that are not included in the current implementation pass, with the reason and what's needed to unblock each.

---

## Eigenvalue / Eigenvector Decomposition

**What:** Given an n×n matrix, return eigenvalues and eigenvectors.

**Why deferred:** `nalgebra` (already a planned dependency for `linear_algebra.rs`) fully supports this. The blocker is output format — eigenvalues can be complex numbers even for real matrices, and encoding a variable-length list of possibly-complex pairs as a clean MCP `String` response needs a decision before implementation.

**To unblock:** Decide on output format (e.g. JSON string, or require the `complex_number` tool as a companion), then add `eigenvalues` as an operation in `matrix_ops`.

---

## Amortization Schedule (Full Table)

**What:** Given loan principal, rate, and term, return a period-by-period table: payment number, principal paid, interest paid, remaining balance.

**Why deferred:** The `loan` tool already covers single-value outputs (PMT, remaining balance at period N, total interest). A full schedule is a multi-row table, which doesn't fit the `String` return pattern cleanly — every other tool returns a single computed value or a short summary line.

**To unblock:** Adopt a richer return type (e.g. return JSON as a string) or add a structured output mode to the server. Once that decision is made, the math itself is trivial.

---

## Bond Pricing, Yield to Maturity, Duration

**What:** `bond_price` (given coupon rate, face value, YTM, periods), `yield_to_maturity` (Newton–Raphson iteration given market price), Macaulay duration, modified duration.

**Why deferred:** YTM requires iterative root-finding (no closed form). The approach works but it's more involved than the other finance tools and warrants its own careful implementation and test coverage.

**To unblock:** Implement `yield_to_maturity` using Newton–Raphson or bisection (same algorithm planned for `numerical_methods` in `calculus.rs`) — coordinate with that module to reuse the root-finding helper from `shared.rs` once it exists.

---

## Arbitrary Function Calculus (Lambda / Expression Calculus)

**What:** Derivative and definite integral of a user-supplied symbolic expression (e.g. `sin(x) + x^2`).

**Why deferred:** MCP tools receive JSON. There is no way to pass a callable function over JSON. The `calculus.rs` tools work around this by accepting polynomial coefficient arrays or arrays of (x, y) data points, which covers the most common practical cases.

**To unblock:** Would require embedding a symbolic math / expression-parser library (e.g. `meval` or a CAS). Worth revisiting if there is clear demand for symbolic differentiation.

---

## Probability Distribution Full Suite (PDF / CDF / Inverse-CDF)

**What:** For normal, binomial, Poisson, exponential, uniform, geometric distributions: probability density, cumulative probability, and quantile (inverse CDF).

**Why deferred:** The `statrs` crate (planned dependency) provides all of this. The deferred part is edge-case validation — distribution parameters have strict domain requirements (e.g. Poisson λ > 0, binomial 0 < p < 1, n integer) that need careful input validation and test coverage before shipping.

**To unblock:** Implement `probability.rs` with thorough input guards and tests. The math is fully provided by `statrs`; no algorithmic work needed.

---

## Frequency Tables and Cumulative Sum Output

**What:** `frequency_table(values)` returning each distinct value and its count/proportion; `cumulative_sum(values)` returning the running total array.

**Why deferred:** Both return a sequence of values, not a single number. Like the amortization schedule, the multi-value output format needs a decision.

**To unblock:** Same as amortization schedule — agree on output format (JSON string, newline-separated, etc.), then these are trivial to implement in `statistics.rs`.
