# Math Function Reference Catalog

A grouped reference of calculations used across finance, science, engineering,
programming, and everyday math. Reference only — not a list of implemented tools.

## 1. Arithmetic & Elementary Operations
**Basic**
- Addition, subtraction, multiplication, division
- Modulo (remainder), integer (floor) division
- Negation, absolute value, reciprocal (1/x)

**Rounding & truncation**
- Floor, ceiling, round (half-up / half-even / banker's), truncate
- Round to N decimal places / N significant figures

**Sign & comparison**
- Sign (signum), min, max, clamp (constrain to range)

**Fractions & ratios**
- GCD, LCM, simplify fraction
- Fraction ↔ decimal, ratio scaling, proportion solving (a/b = c/x)

## 2. Powers, Roots & Exponentials
**Powers**
- Square (x²), cube (x³), general power (xʸ)
- Powers of 2 (2ˣ), powers of 10 (10ˣ)

**Roots**
- Square root, cube root, n-th root

**Exponentials**
- Natural exponential eˣ (exp)
- expm1 (eˣ − 1, accurate for small x)
- 2ˣ, 10ˣ

## 3. Logarithms
- Natural log (ln)
- Common log (log₁₀), binary log (log₂)
- Log to arbitrary base (log_b x)
- log1p (ln(1 + x), accurate for small x)
- Logit (ln(p / (1 − p)))

## 4. Trigonometry
**Forward (primary)**
- sin, cos, tan

**Reciprocal**
- sec (1/cos), csc (1/sin), cot (1/tan)

**Inverse (arc)**
- asin, acos, atan
- atan2(y, x) — quadrant-aware angle

**Inverse reciprocal**
- asec, acsc, acot

**Hyperbolic**
- sinh, cosh, tanh
- sech, csch, coth

**Inverse hyperbolic**
- asinh, acosh, atanh
- asech, acsch, acoth

**Angle utilities**
- Degrees ↔ radians ↔ gradians
- Normalize angle (to 0–360° / 0–2π / −180–180°)
- hypot(x, y) — √(x² + y²) without overflow

## 5. Statistics
**Central tendency**
- Arithmetic mean, median, mode
- Geometric mean, harmonic mean, weighted mean
- Root mean square (RMS), midrange

**Dispersion**
- Variance (population & sample), standard deviation (population & sample)
- Range, interquartile range (IQR)
- Mean / median absolute deviation (MAD)
- Coefficient of variation, standard error of the mean

**Position & shape**
- Percentile, quartile, general quantile
- Z-score (standardize), skewness, kurtosis

**Relationships**
- Covariance, Pearson correlation
- Spearman rank correlation
- Linear regression (slope, intercept, R²)

**Aggregation helpers**
- Sum, product, cumulative sum, count, frequency table

## 6. Probability & Combinatorics
- Factorial (n!), double factorial (n!!)
- Permutations (nPr), combinations (nCr)
- Binomial coefficient, multinomial coefficient
- Catalan numbers
- Odds ↔ probability conversion
- Distribution PDF / CDF / inverse-CDF: normal, binomial, Poisson,
  exponential, uniform, geometric

## 7. Number Theory & Integer Math
- GCD, LCM (also in §1)
- Primality test, prime factorization
- Next prime, n-th prime, list primes (sieve)
- Divisor list, divisor count, sum of divisors
- Euler's totient φ(n)
- Modular arithmetic: mod, modular exponentiation, modular inverse
- Fibonacci numbers, Lucas numbers
- Digit sum, digital root, digit reversal

## 8. Financial & Business
**Time value of money**
- Present value (PV), future value (FV)
- Net present value (NPV), internal rate of return (IRR)
- Annuity payment / present value of annuity

**Interest**
- Simple interest, compound interest
- Effective annual rate (EAR), nominal ↔ effective rate
- APR ↔ APY, continuous compounding

**Loans & amortization**
- Periodic payment (PMT), remaining balance, total interest paid
- Amortization schedule

**Returns & growth**
- Return on investment (ROI)
- Compound annual growth rate (CAGR), annualized return

**Depreciation**
- Straight-line, declining balance, double-declining, sum-of-years'-digits

**Bonds**
- Bond price, yield to maturity (YTM), Macaulay / modified duration

**Everyday business math**
- Percent change, markup / margin, discount
- Sales tax, tip, break-even point, currency conversion

## 9. Geometry & Mensuration
**2D area**
- Rectangle, square, triangle (base-height & Heron's), circle, sector
- Trapezoid, parallelogram, ellipse, regular polygon

**2D perimeter / circumference**
- Polygon perimeter, circle circumference, ellipse perimeter (approx.)

**3D volume**
- Cube/box, sphere, cylinder, cone, pyramid, prism, torus

**3D surface area**
- Sphere, cylinder, cone, box

**Distance & relations**
- Euclidean distance (2D / 3D), Manhattan distance
- Pythagorean theorem, law of sines, law of cosines
- Slope of a line, midpoint
- Cartesian ↔ polar, Cartesian ↔ cylindrical / spherical

## 10. Linear Algebra
**Vectors**
- Addition, subtraction, scalar multiplication
- Magnitude / norm (L1, L2, L∞), normalize (unit vector)
- Dot product, cross product, angle between, projection

**Matrices**
- Addition, multiplication, scalar multiplication, transpose
- Determinant, inverse, trace, rank
- Identity matrix, solve linear system (Ax = b)
- Eigenvalues / eigenvectors

## 11. Calculus (Numerical)
- Numerical derivative (finite differences)
- Definite integral (trapezoidal / Simpson's rule)
- Series summation, Taylor / Maclaurin approximation
- Root finding (Newton–Raphson, bisection)

## 12. Programmer / Computer Science
**Base conversion**
- Decimal ↔ binary, hexadecimal, octal
- Arbitrary base ↔ arbitrary base

**Bitwise operations**
- AND, OR, XOR, NOT
- Left shift, right shift (logical & arithmetic), rotate
- Population count (popcount), bit length, leading/trailing zeros

**Representations**
- Two's complement, signed ↔ unsigned interpretation
- IEEE-754 float bit breakdown
- ASCII ↔ character code

## 13. Unit & Quantity Conversions
- Length, area, volume, mass / weight
- Temperature (Celsius / Fahrenheit / Kelvin)
- Time, speed, acceleration
- Pressure, energy, power, force
- Data size (bytes ↔ KB/MB/GB, SI vs binary)
- Angle (deg / rad / grad), scientific notation ↔ decimal

## 14. Engineering & Applied
- Complex numbers: add, multiply, conjugate, magnitude, phase, polar ↔ rectangular
- Ohm's law (V = IR), electrical power (P = VI)
- Decibel (dB) conversions, gain/ratio
- Linear interpolation / extrapolation
- Resonant frequency, RC/RL time constants
- Unit-prefix scaling (kilo, milli, micro, …)

## 15. Special Functions
- Gamma Γ(x), log-gamma, beta B(x, y)
- Error function erf, complementary erfc
- Sigmoid / logistic, softmax
- Floor/ceil-based step functions

## 16. Mathematical Constants
- π (pi), e (Euler's number), τ (tau = 2π)
- Golden ratio φ, Euler–Mascheroni γ
- √2, √3, ln 2, ln 10
- Physical: speed of light c, gravitational g, Planck h, Avogadro Nₐ
