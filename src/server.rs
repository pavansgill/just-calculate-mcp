use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::tools::arithmetic::{ArithmeticInput, RoundingInput, NumberPropertiesInput, compute, round, number_properties};
use crate::tools::calculus::{NumericalMethodsInput, PolynomialCalcInput, numerical_methods, polynomial_calc};
use crate::tools::combinatorics::{CombinatoricsInput, combinatorics};
use crate::tools::constants::{MathConstantInput, math_constant};
use crate::tools::engineering::{ComplexNumberInput, DecibelInput, ElectricalInput, InterpolationInput, complex_number, decibel, electrical, interpolation};
use crate::tools::finance::{BusinessMathInput, DepreciationInput, InterestRateInput, InvestmentReturnInput, LoanInput, PresentFutureValueInput, business_math, depreciation, interest_rate, investment_return, loan, present_future_value};
use crate::tools::geometry::{Area2dInput, CoordinateConvertInput, Distance2dInput, Perimeter2dInput, SurfaceArea3dInput, Volume3dInput, area_2d, coordinate_convert, distance_2d, perimeter_2d, surface_area_3d, volume_3d};
use crate::tools::linear_algebra::{MatrixOpsInput, VectorOpsInput, matrix_ops, vector_ops};
use crate::tools::logarithm::{LogarithmInput, logarithm};
use crate::tools::number_theory::{ModularArithmeticInput, NumberTheoryInput, modular_arithmetic, number_theory};
use crate::tools::powers::{ExponentialInput, PowerRootInput, exponential, power_root};
use crate::tools::probability::{DistributionInput, OddsConvertInput, distribution, odds_convert};
use crate::tools::programmer::{BaseConvertInput, BitwiseOpsInput, NumberReprInput, base_convert, bitwise_ops, number_repr};
use crate::tools::special_functions::{SpecialFunctionsInput, special_functions};
use crate::tools::statistics::{CorrelationInput, DescriptiveStatsInput, LinearRegressionInput, PercentileInput, correlation, descriptive_stats, linear_regression, percentile};
use crate::tools::trigonometry::{AngleConvertInput, HyperbolicInput, Trig2ArgInput, TrigInput, angle_convert, hyperbolic, trigonometry, trigonometry_2arg};
use crate::tools::unit_convert::{UnitConvertInput, unit_convert};

#[derive(Debug, Clone)]
pub struct Calculator {
    tool_router: ToolRouter<Self>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl Calculator {
    // ── Arithmetic ────────────────────────────────────────────────────────────
    #[tool(description = "Perform a simple arithmetic operation (+, -, *, /, %) on two numbers")]
    pub fn simple_arithmetic(&self, Parameters(input): Parameters<ArithmeticInput>) -> String { compute(input) }

    #[tool(description = "Round a number: floor, ceil, round, truncate, round_dp (decimal places), round_sf (significant figures)")]
    pub fn rounding(&self, Parameters(input): Parameters<RoundingInput>) -> String { round(input) }

    #[tool(description = "Number properties: abs, sign, reciprocal, min, max, clamp, gcd, lcm")]
    pub fn number_properties(&self, Parameters(input): Parameters<NumberPropertiesInput>) -> String { number_properties(input) }

    // ── Powers / Roots / Exponentials ─────────────────────────────────────────
    #[tool(description = "Powers and roots: square, cube, sqrt, cbrt, nth_root, pow (x^y)")]
    pub fn power_root(&self, Parameters(input): Parameters<PowerRootInput>) -> String { power_root(input) }

    #[tool(description = "Exponential functions: exp (e^x), expm1 (e^x - 1), exp2 (2^x), exp10 (10^x)")]
    pub fn exponential(&self, Parameters(input): Parameters<ExponentialInput>) -> String { exponential(input) }

    // ── Logarithms ────────────────────────────────────────────────────────────
    #[tool(description = "Logarithms: ln, log10, log2, log_base (arbitrary base), log1p, logit")]
    pub fn logarithm(&self, Parameters(input): Parameters<LogarithmInput>) -> String { logarithm(input) }

    // ── Trigonometry ──────────────────────────────────────────────────────────
    #[tool(description = "Trigonometry: sin, cos, tan, sec, csc, cot and their inverses asin, acos, atan, asec, acsc, acot. Accepts degrees or radians.")]
    pub fn trigonometry(&self, Parameters(input): Parameters<TrigInput>) -> String { trigonometry(input) }

    #[tool(description = "Two-argument trig functions: atan2(y, x) for quadrant-aware angle, hypot(x, y) for √(x²+y²)")]
    pub fn trigonometry_2arg(&self, Parameters(input): Parameters<Trig2ArgInput>) -> String { trigonometry_2arg(input) }

    #[tool(description = "Hyperbolic functions: sinh, cosh, tanh, sech, csch, coth and their inverses asinh, acosh, atanh, asech, acsch, acoth")]
    pub fn hyperbolic(&self, Parameters(input): Parameters<HyperbolicInput>) -> String { hyperbolic(input) }

    #[tool(description = "Angle unit conversion: deg↔rad, deg↔grad, rad↔grad, normalize to 0-360° or -180-180°")]
    pub fn angle_convert(&self, Parameters(input): Parameters<AngleConvertInput>) -> String { angle_convert(input) }

    // ── Statistics ────────────────────────────────────────────────────────────
    #[tool(description = "Descriptive statistics on a list of numbers: mean, median, mode, geometric_mean, harmonic_mean, rms, variance, std_dev, range, iqr, mad, skewness, kurtosis, sum, min, max, count, midrange, coefficient_of_variation")]
    pub fn descriptive_stats(&self, Parameters(input): Parameters<DescriptiveStatsInput>) -> String { descriptive_stats(input) }

    #[tool(description = "Percentile, quartile (Q1/Q2/Q3), and z-score for a list of numbers")]
    pub fn percentile(&self, Parameters(input): Parameters<PercentileInput>) -> String { percentile(input) }

    #[tool(description = "Correlation between two datasets: Pearson r, Spearman rho, or covariance")]
    pub fn correlation(&self, Parameters(input): Parameters<CorrelationInput>) -> String { correlation(input) }

    #[tool(description = "Linear regression: given x and y arrays, returns slope, intercept, R², and optionally predicts y for a new x")]
    pub fn linear_regression(&self, Parameters(input): Parameters<LinearRegressionInput>) -> String { linear_regression(input) }

    // ── Combinatorics ─────────────────────────────────────────────────────────
    #[tool(description = "Combinatorics: factorial (n!), double_factorial (n!!), permutations (nPr), combinations (nCr), binomial_coeff, Catalan numbers")]
    pub fn combinatorics(&self, Parameters(input): Parameters<CombinatoricsInput>) -> String { combinatorics(input) }

    // ── Number theory ─────────────────────────────────────────────────────────
    #[tool(description = "Number theory: gcd, lcm, is_prime, prime_factors, next_prime, nth_prime, divisors, divisor_count, sum_of_divisors, euler_totient, Fibonacci, Lucas, digit_sum, digital_root")]
    pub fn number_theory(&self, Parameters(input): Parameters<NumberTheoryInput>) -> String { number_theory(input) }

    #[tool(description = "Modular arithmetic: mod_pow (a^b mod m), mod_inverse, mod_add, mod_multiply")]
    pub fn modular_arithmetic(&self, Parameters(input): Parameters<ModularArithmeticInput>) -> String { modular_arithmetic(input) }

    // ── Finance ───────────────────────────────────────────────────────────────
    #[tool(description = "Time value of money: present_value, future_value, NPV (net present value), annuity_pv, annuity_payment")]
    pub fn present_future_value(&self, Parameters(input): Parameters<PresentFutureValueInput>) -> String { present_future_value(input) }

    #[tool(description = "Interest calculations: simple_interest, compound_interest, effective_annual_rate (EAR), continuous_compounding, APR↔APY conversion")]
    pub fn interest_rate(&self, Parameters(input): Parameters<InterestRateInput>) -> String { interest_rate(input) }

    #[tool(description = "Loan calculations: periodic payment (PMT), remaining balance after N payments, total interest paid over loan life")]
    pub fn loan(&self, Parameters(input): Parameters<LoanInput>) -> String { loan(input) }

    #[tool(description = "Investment returns: ROI (return on investment), CAGR (compound annual growth rate), annualized return")]
    pub fn investment_return(&self, Parameters(input): Parameters<InvestmentReturnInput>) -> String { investment_return(input) }

    #[tool(description = "Asset depreciation: straight_line, declining_balance, double_declining balance, sum_of_years_digits")]
    pub fn depreciation(&self, Parameters(input): Parameters<DepreciationInput>) -> String { depreciation(input) }

    #[tool(description = "Business math: percent_change, markup, gross_margin, discount, sales_tax, tip, break_even units")]
    pub fn business_math(&self, Parameters(input): Parameters<BusinessMathInput>) -> String { business_math(input) }

    // ── Geometry ──────────────────────────────────────────────────────────────
    #[tool(description = "2D area: rectangle, square, triangle, triangle_heron, circle, sector, trapezoid, parallelogram, ellipse, regular_polygon")]
    pub fn area_2d(&self, Parameters(input): Parameters<Area2dInput>) -> String { area_2d(input) }

    #[tool(description = "2D perimeter/circumference: rectangle, square, triangle, circle, ellipse_approx, regular_polygon")]
    pub fn perimeter_2d(&self, Parameters(input): Parameters<Perimeter2dInput>) -> String { perimeter_2d(input) }

    #[tool(description = "3D volume: box_, sphere, cylinder, cone, pyramid, prism, torus")]
    pub fn volume_3d(&self, Parameters(input): Parameters<Volume3dInput>) -> String { volume_3d(input) }

    #[tool(description = "3D surface area: sphere, cylinder, cone, box_")]
    pub fn surface_area_3d(&self, Parameters(input): Parameters<SurfaceArea3dInput>) -> String { surface_area_3d(input) }

    #[tool(description = "2D distance and relations: euclidean, manhattan, midpoint, slope, pythagorean, euclidean_3d")]
    pub fn distance_2d(&self, Parameters(input): Parameters<Distance2dInput>) -> String { distance_2d(input) }

    #[tool(description = "Coordinate system conversion: cartesian↔polar (2D), cartesian↔spherical (3D)")]
    pub fn coordinate_convert(&self, Parameters(input): Parameters<CoordinateConvertInput>) -> String { coordinate_convert(input) }

    // ── Programmer ────────────────────────────────────────────────────────────
    #[tool(description = "Base conversion: decimal↔binary, decimal↔hex, decimal↔octal, and arbitrary base conversions")]
    pub fn base_convert(&self, Parameters(input): Parameters<BaseConvertInput>) -> String { base_convert(input) }

    #[tool(description = "Bitwise operations: and, or, xor, not, left_shift, right_shift, rotate_left, rotate_right, popcount, bit_length, leading_zeros, trailing_zeros")]
    pub fn bitwise_ops(&self, Parameters(input): Parameters<BitwiseOpsInput>) -> String { bitwise_ops(input) }

    #[tool(description = "Number representations: two's complement (encode/decode), Unicode codepoint↔char, IEEE-754 float bit breakdown")]
    pub fn number_repr(&self, Parameters(input): Parameters<NumberReprInput>) -> String { number_repr(input) }

    // ── Unit conversion ───────────────────────────────────────────────────────
    #[tool(description = "Unit conversion: length, mass, temperature, time, speed, data size, pressure, energy, power, force, angle. Provide value, from_unit, and to_unit.")]
    pub fn unit_convert(&self, Parameters(input): Parameters<UnitConvertInput>) -> String { unit_convert(input) }

    // ── Engineering ───────────────────────────────────────────────────────────
    #[tool(description = "Complex number arithmetic: add, subtract, multiply, divide, conjugate, magnitude, phase, polar↔rectangular conversion")]
    pub fn complex_number(&self, Parameters(input): Parameters<ComplexNumberInput>) -> String { complex_number(input) }

    #[tool(description = "Electrical laws: Ohm's law (V=IR, solve for any variable) and power law (P=VI, P=V²/R, P=I²R)")]
    pub fn electrical(&self, Parameters(input): Parameters<ElectricalInput>) -> String { electrical(input) }

    #[tool(description = "Decibel conversions: power ratio↔dB, voltage ratio↔dB")]
    pub fn decibel(&self, Parameters(input): Parameters<DecibelInput>) -> String { decibel(input) }

    #[tool(description = "Interpolation: linear_interpolate, linear_extrapolate between two known points, lerp (linear interpolation with t parameter)")]
    pub fn interpolation(&self, Parameters(input): Parameters<InterpolationInput>) -> String { interpolation(input) }

    // ── Linear algebra ────────────────────────────────────────────────────────
    #[tool(description = "Vector operations: add, subtract, scalar_multiply, magnitude, normalize, dot_product, cross_product_3d, angle_between, projection")]
    pub fn vector_ops(&self, Parameters(input): Parameters<VectorOpsInput>) -> String { vector_ops(input) }

    #[tool(description = "Matrix operations: add, multiply, scalar_multiply, transpose, determinant (2×2/3×3), inverse_2x2, trace, solve linear system (2×2 or 3×3)")]
    pub fn matrix_ops(&self, Parameters(input): Parameters<MatrixOpsInput>) -> String { matrix_ops(input) }

    // ── Calculus ──────────────────────────────────────────────────────────────
    #[tool(description = "Polynomial calculus: evaluate, differentiate, integrate_indefinite, integrate_definite. Polynomial given as coefficient array [c0, c1, c2, ...] where index = power.")]
    pub fn polynomial_calc(&self, Parameters(input): Parameters<PolynomialCalcInput>) -> String { polynomial_calc(input) }

    #[tool(description = "Numerical methods on data points: derivative_at_point (finite differences), integrate_data (trapezoidal rule), root_bisection_data (sign-change detection)")]
    pub fn numerical_methods(&self, Parameters(input): Parameters<NumericalMethodsInput>) -> String { numerical_methods(input) }

    // ── Probability ───────────────────────────────────────────────────────────
    #[tool(description = "Probability distributions: pdf/pmf, cdf, inverse_cdf for normal, binomial, Poisson, exponential, uniform, geometric distributions")]
    pub fn distribution(&self, Parameters(input): Parameters<DistributionInput>) -> String { distribution(input) }

    #[tool(description = "Odds conversion: probability↔odds ratio, fractional↔decimal odds")]
    pub fn odds_convert(&self, Parameters(input): Parameters<OddsConvertInput>) -> String { odds_convert(input) }

    // ── Special functions ─────────────────────────────────────────────────────
    #[tool(description = "Special mathematical functions: Gamma Γ(x), log-Gamma ln(Γ(x)), Beta B(x,y), error function erf, erfc, sigmoid/logistic")]
    pub fn special_functions(&self, Parameters(input): Parameters<SpecialFunctionsInput>) -> String { special_functions(input) }

    // ── Constants ─────────────────────────────────────────────────────────────
    #[tool(description = "Mathematical and physical constants: pi, e, tau, phi (golden ratio), sqrt2, sqrt3, ln2, ln10, c (speed of light), g (gravity), h (Planck), avogadro, boltzmann")]
    pub fn math_constant(&self, Parameters(input): Parameters<MathConstantInput>) -> String { math_constant(input) }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Calculator {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("A calculator MCP server covering arithmetic, trigonometry, statistics, finance, geometry, programming utilities, and more.")
    }
}
