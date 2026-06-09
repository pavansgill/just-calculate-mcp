//! Single source of truth for tool categories.
//!
//! Pure data + pure functions, no rmcp types — this drives both the `toggle`
//! tool (resolving a target to concrete tool names) and the `calc://guide`
//! resource (rendering the catalog). Keeping them generated from one table
//! means the guide can never drift from what `toggle` actually accepts.
//!
//! When a new tool is added in `server.rs`, add its name to the matching
//! category here so `toggle`/`act`/the guide stay in sync.

/// Category name → the tool names registered under it (must match the
/// `#[tool]` method names in `server.rs`).
pub const CATEGORIES: &[(&str, &[&str])] = &[
    ("arithmetic", &["simple_arithmetic", "rounding", "number_properties"]),
    ("powers", &["power_root", "exponential"]),
    ("logarithm", &["logarithm"]),
    ("trigonometry", &["trigonometry", "trigonometry_2arg", "hyperbolic", "angle_convert"]),
    ("statistics", &["descriptive_stats", "percentile", "correlation", "linear_regression"]),
    ("combinatorics", &["combinatorics"]),
    ("number_theory", &["number_theory", "modular_arithmetic"]),
    ("finance", &[
        "present_future_value", "interest_rate", "loan", "investment_return",
        "depreciation", "business_math",
    ]),
    ("geometry", &[
        "area_2d", "perimeter_2d", "volume_3d", "surface_area_3d",
        "distance_2d", "coordinate_convert",
    ]),
    ("programmer", &["base_convert", "bitwise_ops", "number_repr"]),
    ("unit_convert", &["unit_convert"]),
    ("engineering", &["complex_number", "electrical", "decibel", "interpolation"]),
    ("linear_algebra", &["vector_ops", "matrix_ops"]),
    ("calculus", &["polynomial_calc", "numerical_methods"]),
    ("probability", &["distribution", "odds_convert"]),
    ("special_functions", &["special_functions"]),
    ("constants", &["math_constant"]),
];

/// The always-visible meta-tools. Never gated by `toggle`, never callable via `act`.
pub const META_TOOLS: &[&str] = &["toggle", "act"];

/// Every domain tool name (excludes the meta-tools).
pub fn all_tool_names() -> Vec<&'static str> {
    CATEGORIES
        .iter()
        .flat_map(|(_, tools)| tools.iter().copied())
        .collect()
}

/// Resolve a `toggle` target to concrete tool names.
///
/// Accepts (case-insensitive): a category name, a single tool name, or `"all"`.
/// Returns an empty vec for anything unrecognised.
pub fn resolve(target: &str) -> Vec<&'static str> {
    let t = target.trim().to_lowercase();
    if t == "all" {
        return all_tool_names();
    }
    if let Some((_, tools)) = CATEGORIES.iter().find(|(c, _)| *c == t) {
        return tools.to_vec();
    }
    all_tool_names().into_iter().filter(|n| *n == t).collect()
}

/// Whether `name` is a known domain tool (used to validate `act` targets).
pub fn is_tool(name: &str) -> bool {
    all_tool_names().contains(&name)
}

/// Render the `calc://guide` resource as Markdown, generated from [`CATEGORIES`].
pub fn guide_markdown() -> String {
    let mut s = String::new();
    s.push_str("# just-calculate-mcp — usage guide\n\n");
    s.push_str(
        "Only two tools are visible by default: **`toggle`** and **`act`**. \
This keeps the tool list tiny while still giving you the full calculator. \
There are two ways to reach the underlying tools:\n\n",
    );
    s.push_str(
        "1. **`toggle`** — reveal a category (or a single tool) so its *real, \
validated* schema appears in the tool list, then call it normally. \
Turn it back off when done.\n   - Enable a whole domain:  `toggle {\"target\":\"geometry\",\"on\":true}`\n   - Enable one tool:        `toggle {\"target\":\"area_2d\",\"on\":true}`\n   - Enable everything:      `toggle {\"target\":\"all\",\"on\":true}`\n   - Disable again:          `toggle {\"target\":\"geometry\",\"on\":false}`\n\n",
    );
    s.push_str(
        "2. **`act`** — a one-shot proxy. Call any tool *without* revealing it; \
works even while the tool is toggled off.\n   - `act {\"tool\":\"area_2d\",\"args\":{\"shape\":\"circle\",\"radius\":3}}`\n\n",
    );
    s.push_str(
        "Use `toggle` when you'll make several related calls (you get schema \
validation and autocomplete); use `act` for a quick one-off.\n\n",
    );
    s.push_str("## Categories and tools\n\n");
    for (category, tools) in CATEGORIES {
        s.push_str(&format!("- **{category}**: {}\n", tools.join(", ")));
    }
    s.push_str(&format!(
        "\n{} tools across {} categories.\n",
        all_tool_names().len(),
        CATEGORIES.len()
    ));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_category_returns_all_its_tools() {
        let geo = resolve("geometry");
        assert_eq!(geo.len(), 6);
        assert!(geo.contains(&"area_2d"));
        assert!(geo.contains(&"coordinate_convert"));
    }

    #[test]
    fn resolve_single_tool() {
        assert_eq!(resolve("area_2d"), vec!["area_2d"]);
    }

    #[test]
    fn resolve_is_case_insensitive_and_trims() {
        assert_eq!(resolve("  GEOMETRY ").len(), 6);
        assert_eq!(resolve("Area_2D"), vec!["area_2d"]);
    }

    #[test]
    fn resolve_all_returns_every_tool() {
        assert_eq!(resolve("all").len(), all_tool_names().len());
    }

    #[test]
    fn resolve_unknown_is_empty() {
        assert!(resolve("not_a_thing").is_empty());
        assert!(resolve("").is_empty());
    }

    #[test]
    fn meta_tools_are_not_domain_tools() {
        for m in META_TOOLS {
            assert!(!is_tool(m), "{m} must not be in the domain tool set");
        }
    }

    #[test]
    fn tool_names_are_unique() {
        let mut all = all_tool_names();
        let total = all.len();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), total, "duplicate tool name in CATEGORIES");
    }

    #[test]
    fn guide_lists_every_category() {
        let guide = guide_markdown();
        for (c, _) in CATEGORIES {
            assert!(guide.contains(c), "guide missing category {c}");
        }
    }
}
