use std::sync::Arc;

use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::ToolCallContext, wrapper::Parameters},
    model::{
        AnnotateAble, CallToolRequestParams, CallToolResult, Content, JsonObject,
        ListResourcesResult, ListToolsResult, PaginatedRequestParams, RawResource,
        ReadResourceRequestParams, ReadResourceResult, ResourceContents, ServerCapabilities,
        ServerInfo,
    },
    service::{MaybeSendFuture, NotificationContext, Peer, RequestContext},
    tool, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::tools::arithmetic::{
    compute, number_properties, round, ArithmeticInput, NumberPropertiesInput, RoundingInput,
};
use crate::tools::calculus::{
    numerical_methods, polynomial_calc, NumericalMethodsInput, PolynomialCalcInput,
};
use crate::tools::combinatorics::{combinatorics, CombinatoricsInput};
use crate::tools::constants::{math_constant, MathConstantInput};
use crate::tools::engineering::{
    complex_number, decibel, electrical, interpolation, ComplexNumberInput, DecibelInput,
    ElectricalInput, InterpolationInput,
};
use crate::tools::finance::{
    business_math, depreciation, interest_rate, investment_return, loan, present_future_value,
    BusinessMathInput, DepreciationInput, InterestRateInput, InvestmentReturnInput, LoanInput,
    PresentFutureValueInput,
};
use crate::tools::geometry::{
    area_2d, coordinate_convert, distance_2d, perimeter_2d, surface_area_3d, volume_3d,
    Area2dInput, CoordinateConvertInput, Distance2dInput, Perimeter2dInput, SurfaceArea3dInput,
    Volume3dInput,
};
use crate::tools::linear_algebra::{matrix_ops, vector_ops, MatrixOpsInput, VectorOpsInput};
use crate::tools::logarithm::{logarithm, LogarithmInput};
use crate::tools::number_theory::{
    modular_arithmetic, number_theory, ModularArithmeticInput, NumberTheoryInput,
};
use crate::tools::powers::{exponential, power_root, ExponentialInput, PowerRootInput};
use crate::tools::probability::{distribution, odds_convert, DistributionInput, OddsConvertInput};
use crate::tools::programmer::{
    base_convert, bitwise_ops, number_repr, BaseConvertInput, BitwiseOpsInput, NumberReprInput,
};
use crate::tools::registry;
use crate::tools::special_functions::{special_functions, SpecialFunctionsInput};
use crate::tools::statistics::{
    correlation, descriptive_stats, linear_regression, percentile, CorrelationInput,
    DescriptiveStatsInput, LinearRegressionInput, PercentileInput,
};
use crate::tools::trigonometry::{
    angle_convert, hyperbolic, trigonometry, trigonometry_2arg, AngleConvertInput, HyperbolicInput,
    Trig2ArgInput, TrigInput,
};
use crate::tools::unit_convert::{unit_convert, UnitConvertInput};

/// URI of the usage-guide resource (explains `toggle` vs `act`).
const GUIDE_URI: &str = "calc://guide";

#[derive(Clone)]
pub struct Calculator {
    /// The toggleable set used by `list_tools` and normal `call_tool`. All
    /// domain tools start disabled; only `toggle` + `act` are visible until a
    /// client toggles a category on. Behind `RwLock` because toggling mutates
    /// it through `&self`; behind `Arc` so clones share the same state.
    visible: Arc<RwLock<ToolRouter<Self>>>,
    /// The full router with every tool enabled, used only by `act` so a one-off
    /// proxy call works regardless of what is toggled on. Reuses the exact same
    /// route closures — no second dispatch table to maintain.
    full: Arc<ToolRouter<Self>>,
    /// This client's peer, captured on initialization, so `toggle` can emit a
    /// single `tools/list_changed` per command (rather than one per route).
    peer: Arc<std::sync::OnceLock<Peer<RoleServer>>>,
}

impl Calculator {
    pub fn new() -> Self {
        let full_router = Self::tool_router();

        // Start with every non-meta tool hidden — only `toggle`/`act` are
        // visible by default. Driving this off the router's own keys means a
        // newly added tool is hidden automatically. No peer is bound yet, so
        // these disables are silent.
        let mut visible_router = full_router.clone();
        let routed: Vec<String> = visible_router.map.keys().map(|k| k.to_string()).collect();
        for name in routed {
            if !registry::META_TOOLS.contains(&name.as_str()) {
                visible_router.disable_route(name);
            }
        }

        Self {
            visible: Arc::new(RwLock::new(visible_router)),
            full: Arc::new(full_router),
            peer: Arc::new(std::sync::OnceLock::new()),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ToggleInput {
    #[schemars(
        description = "What to toggle: a category (e.g. \"geometry\"), a single tool name (e.g. \"area_2d\"), or \"all\". Read the calc://guide resource for the catalog."
    )]
    pub target: String,
    #[serde(default = "default_true")]
    #[schemars(
        description = "true to reveal the tool(s), false to hide them again. Defaults to true."
    )]
    pub on: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActInput {
    #[schemars(
        description = "Name of the calculator tool to invoke (e.g. \"area_2d\"). See the calc://guide resource for names and their arguments."
    )]
    pub tool: String,
    #[serde(default)]
    #[schemars(
        description = "Arguments object for that tool, matching its schema, e.g. {\"shape\":\"circle\",\"radius\":3}."
    )]
    pub args: serde_json::Value,
}

#[tool_router]
impl Calculator {
    // ── Meta-tools (always visible; dispatched directly in call_tool) ─────────
    // NOTE: these two bodies are never executed. `call_tool` intercepts
    // "toggle"/"act" by name and handles them itself (see ServerHandler impl)
    // to avoid a read-vs-write deadlock on `visible`. They exist only so the
    // `#[tool_router]` macro publishes their input schemas into the tool list.
    #[tool(
        description = "Reveal or hide calculator tools so they (don't) appear in the tool list. Pass a category like \"geometry\", a single tool name, or \"all\", plus on=true/false. Use this when you'll make several related calls. Read the calc://guide resource first to see what's available."
    )]
    pub fn toggle(&self, Parameters(_input): Parameters<ToggleInput>) -> String {
        "toggle is handled by the dispatcher".to_string()
    }

    #[tool(
        description = "One-shot proxy: call any calculator tool by name without revealing it in the tool list. Provide {tool, args}. Works even if the tool is toggled off. Use for quick one-off calculations; use toggle instead for repeated calls. See the calc://guide resource for tool names and argument shapes."
    )]
    pub fn act(&self, Parameters(_input): Parameters<ActInput>) -> String {
        "act is handled by the dispatcher".to_string()
    }

    // ── Arithmetic ────────────────────────────────────────────────────────────
    #[tool(description = "Perform a simple arithmetic operation (+, -, *, /, %) on two numbers")]
    pub fn simple_arithmetic(&self, Parameters(input): Parameters<ArithmeticInput>) -> String {
        compute(input)
    }

    #[tool(
        description = "Round a number: floor, ceil, round, truncate, round_dp (decimal places), round_sf (significant figures)"
    )]
    pub fn rounding(&self, Parameters(input): Parameters<RoundingInput>) -> String {
        round(input)
    }

    #[tool(description = "Number properties: abs, sign, reciprocal, min, max, clamp, gcd, lcm")]
    pub fn number_properties(
        &self,
        Parameters(input): Parameters<NumberPropertiesInput>,
    ) -> String {
        number_properties(input)
    }

    // ── Powers / Roots / Exponentials ─────────────────────────────────────────
    #[tool(description = "Powers and roots: square, cube, sqrt, cbrt, nth_root, pow (x^y)")]
    pub fn power_root(&self, Parameters(input): Parameters<PowerRootInput>) -> String {
        power_root(input)
    }

    #[tool(
        description = "Exponential functions: exp (e^x), expm1 (e^x - 1), exp2 (2^x), exp10 (10^x)"
    )]
    pub fn exponential(&self, Parameters(input): Parameters<ExponentialInput>) -> String {
        exponential(input)
    }

    // ── Logarithms ────────────────────────────────────────────────────────────
    #[tool(description = "Logarithms: ln, log10, log2, log_base (arbitrary base), log1p, logit")]
    pub fn logarithm(&self, Parameters(input): Parameters<LogarithmInput>) -> String {
        logarithm(input)
    }

    // ── Trigonometry ──────────────────────────────────────────────────────────
    #[tool(
        description = "Trigonometry: sin, cos, tan, sec, csc, cot and their inverses asin, acos, atan, asec, acsc, acot. Accepts degrees or radians."
    )]
    pub fn trigonometry(&self, Parameters(input): Parameters<TrigInput>) -> String {
        trigonometry(input)
    }

    #[tool(
        description = "Two-argument trig functions: atan2(y, x) for quadrant-aware angle, hypot(x, y) for √(x²+y²)"
    )]
    pub fn trigonometry_2arg(&self, Parameters(input): Parameters<Trig2ArgInput>) -> String {
        trigonometry_2arg(input)
    }

    #[tool(
        description = "Hyperbolic functions: sinh, cosh, tanh, sech, csch, coth and their inverses asinh, acosh, atanh, asech, acsch, acoth"
    )]
    pub fn hyperbolic(&self, Parameters(input): Parameters<HyperbolicInput>) -> String {
        hyperbolic(input)
    }

    #[tool(
        description = "Angle unit conversion: deg↔rad, deg↔grad, rad↔grad, normalize to 0-360° or -180-180°"
    )]
    pub fn angle_convert(&self, Parameters(input): Parameters<AngleConvertInput>) -> String {
        angle_convert(input)
    }

    // ── Statistics ────────────────────────────────────────────────────────────
    #[tool(
        description = "Descriptive statistics on a list of numbers: mean, median, mode, geometric_mean, harmonic_mean, rms, variance, std_dev, range, iqr, mad, skewness, kurtosis, sum, min, max, count, midrange, coefficient_of_variation"
    )]
    pub fn descriptive_stats(
        &self,
        Parameters(input): Parameters<DescriptiveStatsInput>,
    ) -> String {
        descriptive_stats(input)
    }

    #[tool(description = "Percentile, quartile (Q1/Q2/Q3), and z-score for a list of numbers")]
    pub fn percentile(&self, Parameters(input): Parameters<PercentileInput>) -> String {
        percentile(input)
    }

    #[tool(
        description = "Correlation between two datasets: Pearson r, Spearman rho, or covariance"
    )]
    pub fn correlation(&self, Parameters(input): Parameters<CorrelationInput>) -> String {
        correlation(input)
    }

    #[tool(
        description = "Linear regression: given x and y arrays, returns slope, intercept, R², and optionally predicts y for a new x"
    )]
    pub fn linear_regression(
        &self,
        Parameters(input): Parameters<LinearRegressionInput>,
    ) -> String {
        linear_regression(input)
    }

    // ── Combinatorics ─────────────────────────────────────────────────────────
    #[tool(
        description = "Combinatorics: factorial (n!), double_factorial (n!!), permutations (nPr), combinations (nCr), binomial_coeff, Catalan numbers"
    )]
    pub fn combinatorics(&self, Parameters(input): Parameters<CombinatoricsInput>) -> String {
        combinatorics(input)
    }

    // ── Number theory ─────────────────────────────────────────────────────────
    #[tool(
        description = "Number theory: gcd, lcm, is_prime, prime_factors, next_prime, nth_prime, divisors, divisor_count, sum_of_divisors, euler_totient, Fibonacci, Lucas, digit_sum, digital_root"
    )]
    pub fn number_theory(&self, Parameters(input): Parameters<NumberTheoryInput>) -> String {
        number_theory(input)
    }

    #[tool(
        description = "Modular arithmetic: mod_pow (a^b mod m), mod_inverse, mod_add, mod_multiply"
    )]
    pub fn modular_arithmetic(
        &self,
        Parameters(input): Parameters<ModularArithmeticInput>,
    ) -> String {
        modular_arithmetic(input)
    }

    // ── Finance ───────────────────────────────────────────────────────────────
    #[tool(
        description = "Time value of money: present_value, future_value, NPV (net present value), annuity_pv, annuity_payment"
    )]
    pub fn present_future_value(
        &self,
        Parameters(input): Parameters<PresentFutureValueInput>,
    ) -> String {
        present_future_value(input)
    }

    #[tool(
        description = "Interest calculations: simple_interest, compound_interest, effective_annual_rate (EAR), continuous_compounding, APR↔APY conversion"
    )]
    pub fn interest_rate(&self, Parameters(input): Parameters<InterestRateInput>) -> String {
        interest_rate(input)
    }

    #[tool(
        description = "Loan calculations: periodic payment (PMT), remaining balance after N payments, total interest paid over loan life"
    )]
    pub fn loan(&self, Parameters(input): Parameters<LoanInput>) -> String {
        loan(input)
    }

    #[tool(
        description = "Investment returns: ROI (return on investment), CAGR (compound annual growth rate), annualized return"
    )]
    pub fn investment_return(
        &self,
        Parameters(input): Parameters<InvestmentReturnInput>,
    ) -> String {
        investment_return(input)
    }

    #[tool(
        description = "Asset depreciation: straight_line, declining_balance, double_declining balance, sum_of_years_digits"
    )]
    pub fn depreciation(&self, Parameters(input): Parameters<DepreciationInput>) -> String {
        depreciation(input)
    }

    #[tool(
        description = "Business math: percent_change, markup, gross_margin, discount, sales_tax, tip, break_even units"
    )]
    pub fn business_math(&self, Parameters(input): Parameters<BusinessMathInput>) -> String {
        business_math(input)
    }

    // ── Geometry ──────────────────────────────────────────────────────────────
    #[tool(
        description = "2D area: rectangle, square, triangle, triangle_heron, circle, sector, trapezoid, parallelogram, ellipse, regular_polygon"
    )]
    pub fn area_2d(&self, Parameters(input): Parameters<Area2dInput>) -> String {
        area_2d(input)
    }

    #[tool(
        description = "2D perimeter/circumference: rectangle, square, triangle, circle, ellipse_approx, regular_polygon"
    )]
    pub fn perimeter_2d(&self, Parameters(input): Parameters<Perimeter2dInput>) -> String {
        perimeter_2d(input)
    }

    #[tool(description = "3D volume: box_, sphere, cylinder, cone, pyramid, prism, torus")]
    pub fn volume_3d(&self, Parameters(input): Parameters<Volume3dInput>) -> String {
        volume_3d(input)
    }

    #[tool(description = "3D surface area: sphere, cylinder, cone, box_")]
    pub fn surface_area_3d(&self, Parameters(input): Parameters<SurfaceArea3dInput>) -> String {
        surface_area_3d(input)
    }

    #[tool(
        description = "2D distance and relations: euclidean, manhattan, midpoint, slope, pythagorean, euclidean_3d"
    )]
    pub fn distance_2d(&self, Parameters(input): Parameters<Distance2dInput>) -> String {
        distance_2d(input)
    }

    #[tool(
        description = "Coordinate system conversion: cartesian↔polar (2D), cartesian↔spherical (3D)"
    )]
    pub fn coordinate_convert(
        &self,
        Parameters(input): Parameters<CoordinateConvertInput>,
    ) -> String {
        coordinate_convert(input)
    }

    // ── Programmer ────────────────────────────────────────────────────────────
    #[tool(
        description = "Base conversion: decimal↔binary, decimal↔hex, decimal↔octal, and arbitrary base conversions"
    )]
    pub fn base_convert(&self, Parameters(input): Parameters<BaseConvertInput>) -> String {
        base_convert(input)
    }

    #[tool(
        description = "Bitwise operations: and, or, xor, not, left_shift, right_shift, rotate_left, rotate_right, popcount, bit_length, leading_zeros, trailing_zeros"
    )]
    pub fn bitwise_ops(&self, Parameters(input): Parameters<BitwiseOpsInput>) -> String {
        bitwise_ops(input)
    }

    #[tool(
        description = "Number representations: two's complement (encode/decode), Unicode codepoint↔char, IEEE-754 float bit breakdown"
    )]
    pub fn number_repr(&self, Parameters(input): Parameters<NumberReprInput>) -> String {
        number_repr(input)
    }

    // ── Unit conversion ───────────────────────────────────────────────────────
    #[tool(
        description = "Unit conversion: length, mass, temperature, time, speed, data size, pressure, energy, power, force, angle. Provide value, from_unit, and to_unit."
    )]
    pub fn unit_convert(&self, Parameters(input): Parameters<UnitConvertInput>) -> String {
        unit_convert(input)
    }

    // ── Engineering ───────────────────────────────────────────────────────────
    #[tool(
        description = "Complex number arithmetic: add, subtract, multiply, divide, conjugate, magnitude, phase, polar↔rectangular conversion"
    )]
    pub fn complex_number(&self, Parameters(input): Parameters<ComplexNumberInput>) -> String {
        complex_number(input)
    }

    #[tool(
        description = "Electrical laws: Ohm's law (V=IR, solve for any variable) and power law (P=VI, P=V²/R, P=I²R)"
    )]
    pub fn electrical(&self, Parameters(input): Parameters<ElectricalInput>) -> String {
        electrical(input)
    }

    #[tool(description = "Decibel conversions: power ratio↔dB, voltage ratio↔dB")]
    pub fn decibel(&self, Parameters(input): Parameters<DecibelInput>) -> String {
        decibel(input)
    }

    #[tool(
        description = "Interpolation: linear_interpolate, linear_extrapolate between two known points, lerp (linear interpolation with t parameter)"
    )]
    pub fn interpolation(&self, Parameters(input): Parameters<InterpolationInput>) -> String {
        interpolation(input)
    }

    // ── Linear algebra ────────────────────────────────────────────────────────
    #[tool(
        description = "Vector operations: add, subtract, scalar_multiply, magnitude, normalize, dot_product, cross_product_3d, angle_between, projection"
    )]
    pub fn vector_ops(&self, Parameters(input): Parameters<VectorOpsInput>) -> String {
        vector_ops(input)
    }

    #[tool(
        description = "Matrix operations: add, multiply, scalar_multiply, transpose, determinant (2×2/3×3), inverse_2x2, trace, solve linear system (2×2 or 3×3)"
    )]
    pub fn matrix_ops(&self, Parameters(input): Parameters<MatrixOpsInput>) -> String {
        matrix_ops(input)
    }

    // ── Calculus ──────────────────────────────────────────────────────────────
    #[tool(
        description = "Polynomial calculus: evaluate, differentiate, integrate_indefinite, integrate_definite. Polynomial given as coefficient array [c0, c1, c2, ...] where index = power."
    )]
    pub fn polynomial_calc(&self, Parameters(input): Parameters<PolynomialCalcInput>) -> String {
        polynomial_calc(input)
    }

    #[tool(
        description = "Numerical methods on data points: derivative_at_point (finite differences), integrate_data (trapezoidal rule), root_bisection_data (sign-change detection)"
    )]
    pub fn numerical_methods(
        &self,
        Parameters(input): Parameters<NumericalMethodsInput>,
    ) -> String {
        numerical_methods(input)
    }

    // ── Probability ───────────────────────────────────────────────────────────
    #[tool(
        description = "Probability distributions: pdf/pmf, cdf, inverse_cdf for normal, binomial, Poisson, exponential, uniform, geometric distributions"
    )]
    pub fn distribution(&self, Parameters(input): Parameters<DistributionInput>) -> String {
        distribution(input)
    }

    #[tool(description = "Odds conversion: probability↔odds ratio, fractional↔decimal odds")]
    pub fn odds_convert(&self, Parameters(input): Parameters<OddsConvertInput>) -> String {
        odds_convert(input)
    }

    // ── Special functions ─────────────────────────────────────────────────────
    #[tool(
        description = "Special mathematical functions: Gamma Γ(x), log-Gamma ln(Γ(x)), Beta B(x,y), error function erf, erfc, sigmoid/logistic"
    )]
    pub fn special_functions(
        &self,
        Parameters(input): Parameters<SpecialFunctionsInput>,
    ) -> String {
        special_functions(input)
    }

    // ── Constants ─────────────────────────────────────────────────────────────
    #[tool(
        description = "Mathematical and physical constants: pi, e, tau, phi (golden ratio), sqrt2, sqrt3, ln2, ln10, c (speed of light), g (gravity), h (Planck), avogadro, boltzmann"
    )]
    pub fn math_constant(&self, Parameters(input): Parameters<MathConstantInput>) -> String {
        math_constant(input)
    }
}

/// Deserialize a tool-call's `arguments` object into a typed input struct.
fn parse_args<T: DeserializeOwned>(args: Option<JsonObject>) -> Result<T, McpError> {
    let value = serde_json::Value::Object(args.unwrap_or_default());
    serde_json::from_value(value).map_err(|e| McpError::invalid_params(e.to_string(), None))
}

impl Calculator {
    /// Handle the `toggle` meta-tool: reveal or hide the resolved tools in the
    /// `visible` router. Mutating it fires `notifications/tools/list_changed`
    /// automatically via the peer notifier bound in `on_initialized`.
    async fn handle_toggle(
        &self,
        request: CallToolRequestParams,
    ) -> Result<CallToolResult, McpError> {
        let input: ToggleInput = parse_args(request.arguments)?;
        let names = registry::resolve(&input.target);
        if names.is_empty() {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Unknown target '{}'. Use a category, a single tool name, or \"all\" — see the {GUIDE_URI} resource.",
                input.target
            ))]));
        }

        {
            let mut router = self.visible.write().await;
            for name in names.iter().copied() {
                if input.on {
                    router.enable_route(name);
                } else {
                    router.disable_route(name);
                }
            }
        } // drop the write guard

        // One notification per command, not one per route.
        if let Some(peer) = self.peer.get() {
            let _ = peer.notify_tool_list_changed().await;
        }

        let verb = if input.on { "Enabled" } else { "Disabled" };
        Ok(CallToolResult::success(vec![Content::text(format!(
            "{verb} {} tool(s): {}",
            names.len(),
            names.join(", ")
        ))]))
    }

    /// Handle the `act` meta-tool: proxy a one-off call to any domain tool via
    /// the always-enabled `full` router, regardless of toggle state.
    async fn handle_act(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let input: ActInput = parse_args(request.arguments)?;
        if !registry::is_tool(&input.tool) {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "'{}' is not a callable calculator tool. See the {GUIDE_URI} resource for valid names.",
                input.tool
            ))]));
        }

        let arguments = match input.args {
            serde_json::Value::Object(map) => Some(map),
            serde_json::Value::Null => None,
            // Some MCP bridges double-encode objects as JSON strings; unwrap transparently.
            serde_json::Value::String(s) => match serde_json::from_str::<serde_json::Value>(&s) {
                Ok(serde_json::Value::Object(map)) => Some(map),
                _ => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "act \"args\" must be a JSON object (or omitted), got string: \"{s}\""
                    ))]));
                }
            },
            other => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "act \"args\" must be a JSON object (or omitted), got: {other}"
                ))]));
            }
        };

        let mut inner = CallToolRequestParams::new(input.tool);
        inner.arguments = arguments;
        let tcc = ToolCallContext::new(self, inner, context);
        self.full.call(tcc).await
    }
}

impl ServerHandler for Calculator {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_tool_list_changed()
                .enable_resources()
                .build(),
        )
        .with_instructions(
            "A calculator covering arithmetic, trig, statistics, finance, geometry, programming \
utilities, and more — but only two tools are visible by default: `toggle` and `act`. First read \
the calc://guide resource to see all categories. Then either `toggle` a category on to reveal its \
tools (validated schemas; best for repeated calls), or use `act` to invoke a tool once without \
revealing it.",
        )
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let router = self.visible.read().await;
        Ok(ListToolsResult::with_all_items(router.list_all()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        // Intercept the meta-tools by name *before* locking `visible` for
        // dispatch — `toggle` needs a write lock, which would deadlock against
        // a read guard held across `router.call().await`.
        match request.name.as_ref() {
            "toggle" => self.handle_toggle(request).await,
            "act" => self.handle_act(request, context).await,
            _ => {
                let router = self.visible.read().await;
                if registry::is_tool(&request.name) && !router.has_route(&request.name) {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Tool '{0}' is currently off. Turn it on with  toggle {{\"target\":\"{0}\",\"on\":true}}  (or toggle its whole category), or call it once via  act {{\"tool\":\"{0}\",\"args\":{{…}}}}.",
                        request.name
                    ))]));
                }
                let tcc = ToolCallContext::new(self, request, context);
                router.call(tcc).await
            }
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resource = RawResource::new(GUIDE_URI, "Calculator usage guide")
            .with_title("How to use this calculator (toggle & act)")
            .with_description(
                "Catalog of every tool category and how to reveal (toggle) or proxy (act) tools.",
            )
            .with_mime_type("text/markdown")
            .no_annotation();
        Ok(ListResourcesResult::with_all_items(vec![resource]))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        if request.uri == GUIDE_URI {
            Ok(ReadResourceResult::new(vec![ResourceContents::text(
                registry::guide_markdown(),
                GUIDE_URI,
            )]))
        } else {
            Err(McpError::resource_not_found(
                format!("Unknown resource: {}", request.uri),
                None,
            ))
        }
    }

    fn on_initialized(
        &self,
        context: NotificationContext<RoleServer>,
    ) -> impl std::future::Future<Output = ()> + MaybeSendFuture + '_ {
        // Capture the peer so `toggle` can emit `tools/list_changed` itself.
        let _ = self.peer.set(context.peer.clone());
        std::future::ready(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use rmcp::{
        service::NotificationContext as ClientNotificationContext, ClientHandler, RoleClient,
        ServiceExt,
    };
    use tokio::sync::Notify;

    /// Minimal client that counts `tools/list_changed` notifications.
    #[derive(Clone)]
    struct CountingClient {
        count: Arc<AtomicUsize>,
        notify: Arc<Notify>,
    }

    impl ClientHandler for CountingClient {
        fn on_tool_list_changed(
            &self,
            _context: ClientNotificationContext<RoleClient>,
        ) -> impl std::future::Future<Output = ()> + MaybeSendFuture + '_ {
            self.count.fetch_add(1, Ordering::SeqCst);
            self.notify.notify_one();
            std::future::ready(())
        }
    }

    fn args(v: serde_json::Value) -> Option<JsonObject> {
        v.as_object().cloned()
    }

    fn text_of(result: &CallToolResult) -> String {
        serde_json::to_string(&result.content).unwrap()
    }

    #[tokio::test]
    async fn toggle_gates_visibility_notifies_once_and_act_is_independent() {
        let (server_transport, client_transport) = tokio::io::duplex(8192);

        let server_handle =
            tokio::spawn(async move { Calculator::new().serve(server_transport).await });

        let client = CountingClient {
            count: Arc::new(AtomicUsize::new(0)),
            notify: Arc::new(Notify::new()),
        };
        let count = client.count.clone();
        let notified = client.notify.clone();
        let client = client.serve(client_transport).await.unwrap();
        let peer = client.peer();

        // Default: only the two meta-tools are visible.
        let tools = peer.list_tools(None).await.unwrap();
        assert_eq!(tools.tools.len(), 2);
        let mut names: Vec<_> = tools.tools.iter().map(|t| t.name.as_ref()).collect();
        names.sort_unstable();
        assert_eq!(names, vec!["act", "toggle"]);

        // `act` reaches a tool that is currently toggled OFF.
        let mut req = CallToolRequestParams::new("act");
        req.arguments = args(serde_json::json!({
            "tool": "trigonometry",
            "args": {"operation": "sin", "value": 90, "angle_unit": "degrees"}
        }));
        let res = peer.call_tool(req).await.unwrap();
        assert_ne!(res.is_error, Some(true));
        assert!(
            text_of(&res).contains("= 1"),
            "act result: {}",
            text_of(&res)
        );

        // Toggle a whole category on.
        let mut req = CallToolRequestParams::new("toggle");
        req.arguments = args(serde_json::json!({"target": "geometry", "on": true}));
        peer.call_tool(req).await.unwrap();

        // Exactly one list_changed notification per toggle command.
        tokio::time::timeout(Duration::from_secs(5), notified.notified())
            .await
            .expect("expected tools/list_changed");
        assert_eq!(count.load(Ordering::SeqCst), 1);

        // The geometry tools are now visible with their real schemas.
        let tools = peer.list_tools(None).await.unwrap();
        assert_eq!(tools.tools.len(), 8);
        assert!(tools.tools.iter().any(|t| t.name == "area_2d"));

        // A disabled tool called directly returns a helpful error, not a result.
        let mut req = CallToolRequestParams::new("logarithm");
        req.arguments = args(serde_json::json!({"operation": "ln", "value": 1.0}));
        let res = peer.call_tool(req).await.unwrap();
        assert_eq!(res.is_error, Some(true));
        assert!(text_of(&res).contains("currently off"));

        // Toggle back off → list shrinks to the meta-tools again.
        let mut req = CallToolRequestParams::new("toggle");
        req.arguments = args(serde_json::json!({"target": "geometry", "on": false}));
        peer.call_tool(req).await.unwrap();
        tokio::time::timeout(Duration::from_secs(5), notified.notified())
            .await
            .expect("expected tools/list_changed on disable");
        let tools = peer.list_tools(None).await.unwrap();
        assert_eq!(tools.tools.len(), 2);

        client.cancel().await.unwrap();
        server_handle.abort();
    }

    #[tokio::test]
    async fn guide_resource_is_readable() {
        let (server_transport, client_transport) = tokio::io::duplex(8192);
        let server_handle =
            tokio::spawn(async move { Calculator::new().serve(server_transport).await });
        let client = ().serve(client_transport).await.unwrap();
        let peer = client.peer();

        let resources = peer.list_resources(None).await.unwrap();
        assert_eq!(resources.resources.len(), 1);
        assert_eq!(resources.resources[0].uri, GUIDE_URI);

        let read = peer
            .read_resource(ReadResourceRequestParams::new(GUIDE_URI))
            .await
            .unwrap();
        let text = match &read.contents[0] {
            ResourceContents::TextResourceContents { text, .. } => text.clone(),
            _ => panic!("expected text resource"),
        };
        assert!(text.contains("toggle"));
        assert!(text.contains("geometry"));

        client.cancel().await.unwrap();
        server_handle.abort();
    }
}
