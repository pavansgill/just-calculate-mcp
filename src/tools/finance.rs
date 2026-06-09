use schemars::JsonSchema;
use serde::Deserialize;

macro_rules! require {
    ($opt:expr, $name:expr) => {
        match $opt {
            Some(v) => v,
            None => return format!("Error: '{}' is required for this operation", $name),
        }
    };
}

// ── Present / future value ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PresentFutureValueInput {
    #[schemars(
        description = "Operation: present_value, future_value, npv, annuity_pv, annuity_payment"
    )]
    pub operation: String,
    #[schemars(description = "Periodic interest rate (e.g. 0.05 for 5%)")]
    pub rate: Option<f64>,
    #[schemars(description = "Number of periods")]
    pub periods: Option<f64>,
    #[schemars(
        description = "Present value (present_value op: ignored; future_value: initial amount; annuity_pv: payment amount)"
    )]
    pub present_value: Option<f64>,
    #[schemars(description = "Future value (present_value: target; future_value: ignored)")]
    pub future_value: Option<f64>,
    #[schemars(
        description = "Cash flows list for npv (first element is initial investment as negative)"
    )]
    pub cash_flows: Option<Vec<f64>>,
}

pub fn present_future_value(input: PresentFutureValueInput) -> String {
    match input.operation.as_str() {
        "present_value" => {
            let r = require!(input.rate, "rate");
            let n = require!(input.periods, "periods");
            let fv = require!(input.future_value, "future_value");
            format!("PV = {}", fv / (1.0 + r).powf(n))
        }
        "future_value" => {
            let r = require!(input.rate, "rate");
            let n = require!(input.periods, "periods");
            let pv = require!(input.present_value, "present_value");
            format!("FV = {}", pv * (1.0 + r).powf(n))
        }
        "npv" => {
            let r = require!(input.rate, "rate");
            let cf = match &input.cash_flows {
                Some(v) if !v.is_empty() => v.clone(),
                _ => return "Error: npv requires non-empty cash_flows".to_string(),
            };
            let npv: f64 = cf.iter().enumerate().map(|(t, c)| c / (1.0 + r).powi(t as i32)).sum();
            format!("NPV = {npv}")
        }
        "annuity_pv" => {
            let r = require!(input.rate, "rate");
            let n = require!(input.periods, "periods");
            let pmt = require!(input.present_value, "present_value (payment per period)");
            if r == 0.0 { return format!("Annuity PV = {}", pmt * n); }
            format!("Annuity PV = {}", pmt * (1.0 - (1.0 + r).powf(-n)) / r)
        }
        "annuity_payment" => {
            let r = require!(input.rate, "rate");
            let n = require!(input.periods, "periods");
            let pv = require!(input.present_value, "present_value (loan amount)");
            if r == 0.0 { return format!("Payment = {}", pv / n); }
            format!("Payment = {}", pv * r / (1.0 - (1.0 + r).powf(-n)))
        }
        op => format!("Error: Unknown operation '{op}'. Supported: present_value, future_value, npv, annuity_pv, annuity_payment"),
    }
}

// ── Interest rate ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InterestRateInput {
    #[schemars(
        description = "Operation: simple_interest, compound_interest, effective_annual_rate, continuous_compounding, apr_to_apy, apy_to_apr"
    )]
    pub operation: String,
    #[schemars(description = "Principal amount")]
    pub principal: Option<f64>,
    #[schemars(description = "Annual interest rate (e.g. 0.05 for 5%)")]
    pub rate: Option<f64>,
    #[schemars(description = "Time in years")]
    pub time: Option<f64>,
    #[schemars(
        description = "Compounding periods per year (compound_interest, effective_annual_rate, apr_to_apy)"
    )]
    pub compounds_per_year: Option<f64>,
}

pub fn interest_rate(input: InterestRateInput) -> String {
    match input.operation.as_str() {
        "simple_interest" => {
            let p = require!(input.principal, "principal");
            let r = require!(input.rate, "rate");
            let t = require!(input.time, "time");
            let interest = p * r * t;
            format!("simple_interest = {interest}, total = {}", p + interest)
        }
        "compound_interest" => {
            let p = require!(input.principal, "principal");
            let r = require!(input.rate, "rate");
            let t = require!(input.time, "time");
            let n = input.compounds_per_year.unwrap_or(1.0);
            let amount = p * (1.0 + r / n).powf(n * t);
            format!("amount = {amount}, interest = {}", amount - p)
        }
        "effective_annual_rate" => {
            let r = require!(input.rate, "rate (nominal)");
            let n = require!(input.compounds_per_year, "compounds_per_year");
            let ear = (1.0 + r / n).powf(n) - 1.0;
            format!("EAR = {ear} ({:.4}%)", ear * 100.0)
        }
        "continuous_compounding" => {
            let p = require!(input.principal, "principal");
            let r = require!(input.rate, "rate");
            let t = require!(input.time, "time");
            let amount = p * (r * t).exp();
            format!("amount = {amount}, interest = {}", amount - p)
        }
        "apr_to_apy" => {
            let r = require!(input.rate, "rate (APR)");
            let n = input.compounds_per_year.unwrap_or(12.0);
            let apy = (1.0 + r / n).powf(n) - 1.0;
            format!("APY = {apy} ({:.4}%)", apy * 100.0)
        }
        "apy_to_apr" => {
            let apy = require!(input.rate, "rate (APY)");
            let n = input.compounds_per_year.unwrap_or(12.0);
            let apr = n * ((1.0 + apy).powf(1.0 / n) - 1.0);
            format!("APR = {apr} ({:.4}%)", apr * 100.0)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: simple_interest, compound_interest, effective_annual_rate, continuous_compounding, apr_to_apy, apy_to_apr"),
    }
}

// ── Loan ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoanInput {
    #[schemars(description = "Operation: payment_pmt, remaining_balance, total_interest_paid")]
    pub operation: String,
    #[schemars(description = "Loan principal")]
    pub principal: f64,
    #[schemars(description = "Periodic interest rate (e.g. 0.005 for 0.5% monthly)")]
    pub periodic_rate: f64,
    #[schemars(description = "Total number of payments")]
    pub total_payments: u32,
    #[schemars(description = "Payment number already made (required for remaining_balance)")]
    pub payments_made: Option<u32>,
}

pub fn loan(input: LoanInput) -> String {
    let p = input.principal;
    let r = input.periodic_rate;
    let n = input.total_payments as f64;

    let pmt = if r == 0.0 {
        p / n
    } else {
        p * r / (1.0 - (1.0 + r).powf(-n))
    };

    match input.operation.as_str() {
        "payment_pmt" => format!("periodic payment = {pmt}"),
        "remaining_balance" => {
            let k = input.payments_made.unwrap_or(0) as f64;
            let balance = if r == 0.0 {
                p - pmt * k
            } else {
                p * (1.0 + r).powf(k) - pmt * ((1.0 + r).powf(k) - 1.0) / r
            };
            format!("remaining balance after {k} payments = {balance}")
        }
        "total_interest_paid" => {
            let total = pmt * n;
            format!("total paid = {total}, total interest = {}", total - p)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: payment_pmt, remaining_balance, total_interest_paid"),
    }
}

// ── Investment return ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InvestmentReturnInput {
    #[schemars(description = "Operation: roi, cagr, annualized_return")]
    pub operation: String,
    #[schemars(description = "Initial investment / beginning value")]
    pub initial_value: f64,
    #[schemars(description = "Final value")]
    pub final_value: f64,
    #[schemars(description = "Number of years (required for cagr, annualized_return)")]
    pub years: Option<f64>,
}

pub fn investment_return(input: InvestmentReturnInput) -> String {
    let iv = input.initial_value;
    let fv = input.final_value;
    if iv == 0.0 {
        return "Error: initial_value cannot be zero".to_string();
    }

    match input.operation.as_str() {
        "roi" => {
            let roi = (fv - iv) / iv * 100.0;
            format!("ROI = {roi:.4}%")
        }
        "cagr" => {
            let years = match input.years {
                Some(y) if y > 0.0 => y,
                _ => return "Error: years must be > 0".to_string(),
            };
            let cagr = (fv / iv).powf(1.0 / years) - 1.0;
            format!("CAGR = {cagr:.6} ({:.4}%)", cagr * 100.0)
        }
        "annualized_return" => {
            let years = match input.years {
                Some(y) if y > 0.0 => y,
                _ => return "Error: years must be > 0".to_string(),
            };
            let total_return = (fv - iv) / iv;
            let ann = (1.0 + total_return).powf(1.0 / years) - 1.0;
            format!("annualized_return = {ann:.6} ({:.4}%)", ann * 100.0)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: roi, cagr, annualized_return"),
    }
}

// ── Depreciation ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DepreciationInput {
    #[schemars(
        description = "Operation: straight_line, declining_balance, double_declining, sum_of_years_digits"
    )]
    pub operation: String,
    #[schemars(description = "Initial asset cost")]
    pub cost: f64,
    #[schemars(description = "Salvage value at end of useful life")]
    pub salvage: f64,
    #[schemars(description = "Useful life in years")]
    pub life_years: u32,
    #[schemars(description = "Year number to calculate depreciation for (1-indexed)")]
    pub year: Option<u32>,
    #[schemars(description = "Depreciation rate for declining_balance (e.g. 0.2 for 20%/year)")]
    pub rate: Option<f64>,
}

pub fn depreciation(input: DepreciationInput) -> String {
    let cost = input.cost;
    let salvage = input.salvage;
    let life = input.life_years as f64;
    let year = input.year.unwrap_or(1) as f64;

    if cost < salvage {
        return "Error: cost must be >= salvage".to_string();
    }
    if input.life_years == 0 {
        return "Error: life_years must be > 0".to_string();
    }

    match input.operation.as_str() {
        "straight_line" => {
            let annual = (cost - salvage) / life;
            format!("straight_line annual depreciation = {annual}")
        }
        "declining_balance" => {
            let r = match input.rate { Some(x) => x, None => return "Error: declining_balance requires rate".to_string() };
            let book_start = cost * (1.0 - r).powf(year - 1.0);
            let dep = book_start * r;
            format!("year {year} depreciation = {dep}, book value = {}", book_start - dep)
        }
        "double_declining" => {
            let rate = 2.0 / life;
            let book_start = cost * (1.0 - rate).powf(year - 1.0);
            let dep = (book_start * rate).min(book_start - salvage);
            format!("year {year} double_declining depreciation = {dep}, book value = {}", book_start - dep)
        }
        "sum_of_years_digits" => {
            let n = input.life_years as f64;
            let sum_digits = n * (n + 1.0) / 2.0;
            let remaining = n - year + 1.0;
            let dep = (remaining / sum_digits) * (cost - salvage);
            format!("year {year} SYD depreciation = {dep}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: straight_line, declining_balance, double_declining, sum_of_years_digits"),
    }
}

// ── Business math ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BusinessMathInput {
    #[schemars(
        description = "Operation: percent_change, markup, gross_margin, discount, sales_tax, tip, break_even"
    )]
    pub operation: String,
    #[schemars(
        description = "Primary value (old value for percent_change, cost for markup/gross_margin, original price for discount/sales_tax/tip, fixed_costs for break_even)"
    )]
    pub value_a: f64,
    #[schemars(
        description = "Secondary value (new value for percent_change, selling price for markup/gross_margin, rate for discount/sales_tax/tip, contribution_margin_per_unit for break_even)"
    )]
    pub value_b: f64,
}

pub fn business_math(input: BusinessMathInput) -> String {
    let a = input.value_a;
    let b = input.value_b;
    match input.operation.as_str() {
        "percent_change" => {
            if a == 0.0 { return "Error: percent_change requires non-zero old value".to_string(); }
            format!("percent_change = {:.4}%", (b - a) / a * 100.0)
        }
        "markup" => {
            if a == 0.0 { return "Error: cost cannot be zero".to_string(); }
            format!("markup = {:.4}% (cost={a}, selling_price={b})", (b - a) / a * 100.0)
        }
        "gross_margin" => {
            if b == 0.0 { return "Error: selling_price cannot be zero".to_string(); }
            format!("gross_margin = {:.4}% (cost={a}, selling_price={b})", (b - a) / b * 100.0)
        }
        "discount" => {
            let rate = b / 100.0;
            let discounted = a * (1.0 - rate);
            format!("discounted_price = {discounted} (saved {})", a * rate)
        }
        "sales_tax" => {
            let tax = a * b / 100.0;
            format!("total = {} (tax={})", a + tax, tax)
        }
        "tip" => {
            let tip = a * b / 100.0;
            format!("tip = {tip}, total = {}", a + tip)
        }
        "break_even" => {
            if b == 0.0 { return "Error: contribution_margin_per_unit cannot be zero".to_string(); }
            format!("break_even_units = {} (fixed_costs={a}, margin_per_unit={b})", (a / b).ceil())
        }
        op => format!("Error: Unknown operation '{op}'. Supported: percent_change, markup, gross_margin, discount, sales_tax, tip, break_even"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value() {
        let r = present_future_value(PresentFutureValueInput {
            operation: "future_value".to_string(),
            rate: Some(0.1),
            periods: Some(1.0),
            present_value: Some(1000.0),
            future_value: None,
            cash_flows: None,
        });
        assert!(r.contains("1100"), "{r}");
    }
    #[test]
    fn test_simple_interest() {
        let r = interest_rate(InterestRateInput {
            operation: "simple_interest".to_string(),
            principal: Some(1000.0),
            rate: Some(0.05),
            time: Some(2.0),
            compounds_per_year: None,
        });
        assert!(r.contains("100"), "{r}");
    }
    #[test]
    fn test_roi() {
        let r = investment_return(InvestmentReturnInput {
            operation: "roi".to_string(),
            initial_value: 1000.0,
            final_value: 1200.0,
            years: None,
        });
        assert!(r.contains("20"), "{r}");
    }
    #[test]
    fn test_percent_change() {
        let r = business_math(BusinessMathInput {
            operation: "percent_change".to_string(),
            value_a: 100.0,
            value_b: 125.0,
        });
        assert!(r.contains("25"), "{r}");
    }
    #[test]
    fn test_break_even() {
        let r = business_math(BusinessMathInput {
            operation: "break_even".to_string(),
            value_a: 10000.0,
            value_b: 50.0,
        });
        assert!(r.contains("200"), "{r}");
    }
}
