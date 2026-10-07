# Experimental design methods

These methods analyze balanced grids, measurement designs, and two-level
experiments. Discrepancy measures compare how evenly sampling points fill
the input space.

## ANOVA

Fisher (1925) *Statistical Methods for Research Workers*. [paper](https://www.usablebuildings.co.uk/UsableBuildings/Unprotected/ClassicsFisher1925.pdf) · [reference](../bibliography.md#fisher1925)

### Theory

Analysis of variance partitions the total sum of squares of a balanced design into components attributable to each factor and their interactions. For a two-way layout with factors $A$ (rows, $a$ levels) and $B$ (columns, $b$ levels):

$$SS_{\text{total}} = SS_A + SS_B + SS_{AB} + SS_{\text{error}}$$

where $SS_A = b \sum_{i=1}^{a} (\bar{Y}_{i\cdot} - \bar{Y}_{\cdot\cdot})^2$ measures the row (factor $A$) effect, $SS_B = a \sum_{j=1}^{b} (\bar{Y}_{\cdot j} - \bar{Y}_{\cdot\cdot})^2$ measures the column (factor $B$) effect, and $SS_{AB}$ captures the interaction residual.

Each mean square $MS = SS / df$ is tested against an appropriate denominator via the $F$-statistic:

$$F_A = \frac{MS_A}{MS_{AB}}, \quad F_B = \frac{MS_B}{MS_{AB}}$$

The API computes upper-tail $p$-values from an $F$ distribution using these
ratios. Their inferential interpretation requires a model that justifies
the denominator and distributional assumptions; the grid alone does not.

The three-way extension adds factors $A \times B \times C$ with all two-way and three-way interaction terms. The implementation uses the three-way interaction $MS_{ABC}$ as the error denominator for all $F$-tests (unreplicated balanced design — no pure-error term exists).

### Code

```rust
use salib::estimators::estimate_anova_two_way;
use ndarray::arr2;

// 3 x 4 balanced grid: 3 levels of factor A, 4 levels of factor B
let grid = arr2(&[
    [23.0, 27.0, 21.0, 25.0],
    [30.0, 34.0, 28.0, 32.0],
    [18.0, 22.0, 16.0, 20.0],
]);

let result = estimate_anova_two_way(grid.view()).unwrap();
println!("{result}");
```

Three-way ANOVA:

```rust
use salib::estimators::estimate_anova_three_way;
use ndarray::Array3;

// A x B x C balanced grid
let grid = Array3::<f64>::from_shape_fn((3, 4, 2), |(i, j, k)| {
    10.0 * i as f64 + 3.0 * j as f64 + 1.5 * k as f64
});

let result = estimate_anova_three_way(grid.view()).unwrap();
println!("{result}");
```

Bootstrap confidence intervals on variance fractions:

```rust
use salib::estimators::estimate_anova_two_way_with_bootstrap;
use salib::RngState;
use ndarray::arr2;

let grid = arr2(&[
    [23.0, 27.0, 21.0, 25.0],
    [30.0, 34.0, 28.0, 32.0],
    [18.0, 22.0, 16.0, 20.0],
]);
let mut rng = RngState::from_seed([0u8; 32]);

let result = estimate_anova_two_way_with_bootstrap(
    grid.view(), 2000, 0.05, &mut rng
).unwrap();
```

These grids have one observation per cell, so pure error and the highest-order
interaction cannot be estimated separately. The API assigns the remaining sum
of squares to that interaction and reports residual zero. Its chosen $F$
denominators are implementation conventions, not generally valid tests for
every fixed, random, or mixed-effects design. Use the sums of squares
descriptively unless your statistical model justifies those tests. See the
[NIST two-way ANOVA formulas](https://www.itl.nist.gov/div898/handbook/prc/section4/prc437.htm)
for the distinction between interaction and replicated pure error.

## G-Theory

Brennan (2001) *Generalizability Theory*, Springer. [paper](https://link.springer.com/book/10.1007/978-1-4757-3456-0) · [reference](../bibliography.md#brennan2001)

### Theory

Generalizability theory extends classical reliability (Cronbach's alpha) to designs with multiple facets. A crossed $p \times i \times r$ design (persons $\times$ items $\times$ raters) decomposes the total variance into seven components:

$$\sigma^2_{\text{total}} = \sigma^2_p + \sigma^2_i + \sigma^2_r + \sigma^2_{pi} + \sigma^2_{pr} + \sigma^2_{ir} + \sigma^2_{pir}$$

Each component is estimated from the corresponding mean square via the standard EMS equations (Brennan 2001 Chapter 2). The two reliability coefficients are:

**Generalizability coefficient** (relative decisions):

$$\rho^2 = \frac{\sigma^2_p}{\sigma^2_p + \sigma^2_\delta}$$

where $\sigma^2_\delta = \sigma^2_{pi}/n_i + \sigma^2_{pr}/n_r + \sigma^2_{pir}/(n_i \cdot n_r)$ is the relative error variance.

**Dependability coefficient** (absolute decisions):

$$\Phi = \frac{\sigma^2_p}{\sigma^2_p + \sigma^2_\Delta}$$

where $\sigma^2_\Delta$ includes all non-person variance components scaled by their facet sample sizes.

The D-study projects $\rho^2$ and $\Phi$ to hypothetical designs with different numbers of items and raters — answering "how many raters/items do I need to reach $\rho^2 \geq 0.80$?"

### Code

```rust
use salib::estimators::{estimate_g_theory_pir, GTheoryDesign};
use ndarray::Array3;

// p x i x r balanced grid: 10 persons, 5 items, 3 raters
let grid = Array3::<f64>::from_shape_fn((10, 5, 3), |(p, i, r)| {
    50.0 + 6.0 * p as f64 + 2.0 * i as f64 + 0.5 * r as f64
});

let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
println!("{result}");
```

D-study projection:

```rust
use salib::estimators::project_g_theory_d_study;

// What if we used 8 items and 4 raters instead?
let projected = project_g_theory_d_study(&result, 8, 4).unwrap();
println!("G = {:.4}, Phi = {:.4}", projected.g_coefficient, projected.phi_coefficient);
```

Bootstrap confidence intervals:

```rust
use salib::estimators::{estimate_g_theory_pir_with_bootstrap, GTheoryDesign};
use salib::RngState;

let mut rng = RngState::from_seed([0u8; 32]);
let result = estimate_g_theory_pir_with_bootstrap(
    grid.view(), GTheoryDesign::Crossed, 2000, 0.05, &mut rng
).unwrap();
```

Use a D-study to estimate how reliability would change with more or fewer
items and raters, using variance components from an existing G-study.

Only fully crossed $p \times i \times r$ designs are implemented. The formulas
here treat items and raters as random facets and persons as the objects of
measurement. Fixed facets require different error terms. D-study projections
reuse the estimated variance components; they do not account for changes in
the population or measurement process. See [Brennan's overview, §2](https://www.na-mic.org/w/img_auth.php/1/15/Generalizability_theory_and_example.pdf).

## Discrepancy

Hickernell (1998) *Mathematics of Computation* 67(221), 299–322. [paper](https://doi.org/10.1090/S0025-5718-98-00894-1) · [reference](../bibliography.md#hickernell1998)

### Theory

Discrepancy measures how evenly a point set fills $[0,1]^d$. Different
discrepancies correspond to different integration-error bounds. For example,
the classical Koksma–Hlawka inequality uses star discrepancy:

$$\left|\int_{[0,1]^d} f(\mathbf{x})\,d\mathbf{x} - \frac{1}{N}\sum_{i=1}^{N} f(\mathbf{x}_i)\right| \leq D^*(P_N) \cdot V(f)$$

where $D^*(P_N)$ is star discrepancy and $V(f)$ is Hardy–Krause variation,
assumed finite. This API returns L2-star, centered, wrap-around, and modified
discrepancies; none can simply be substituted for $D^*$ in that inequality.
[Hickernell (1998)](https://doi.org/10.1090/S0025-5718-98-00894-1) develops
generalized bounds with matching variation measures. All four API results
are square roots of the corresponding squared-discrepancy formulas.

Four discrepancy measures are computed simultaneously:

| Measure | Description |
|---------|-------------|
| **Centered (CD)** | Uses distances from the cube center and pairwise coordinate distances. |
| **Wrap-around (WD)** | Uses periodic pairwise coordinate distances. |
| **Modified (MD)** | A distinct kernel formula; compare values only within the same measure. |
| **L2-star** | L2 norm of the error in volumes of boxes anchored at the origin. |

### Code

```rust
use salib::estimators::compute_discrepancy;
use ndarray::Array2;

// Illustrative 100-point set in three dimensions
let sample = Array2::<f64>::from_shape_fn((100, 3), |(i, j)| {
    // Replace this placeholder with your sampling design.
    ((i * 7 + j * 13) % 100) as f64 / 100.0
});

let result = compute_discrepancy(sample.view()).unwrap();
println!("{result}");
```

Evaluate and compare the quality of sampling designs — LHS vs Sobol' sequence vs Halton sequence vs random. Lower discrepancy values indicate better space-filling. All input values must lie in $[0, 1]$; rescale if needed.

## Fractional Factorial

Box, Hunter & Hunter (1978) *Statistics for Experimenters*, Wiley. [paper](https://openlibrary.org/works/OL2181531W/Statistics_for_experimenters) · [reference](../bibliography.md#box1978)

### Theory

A $2^{k-p}$ fractional factorial runs a fraction of the full factorial design to estimate main effects and (when resolution permits) two-factor interactions. The implementation uses Plackett–Burman designs, which screen up to $N-1$
factors in supported multiples of four runs. These are not generally regular
$2^{k-p}$ fractions. Main effects may be confounded with interactions; see
[NIST's Plackett–Burman guide](https://www.itl.nist.gov/div898/handbook/pri/section3/pri335.htm).

For each factor $i$, the main effect is the difference in mean response between the high (+1) and low (-1) levels:

$$\text{effect}_i = \bar{Y}_{x_i = +1} - \bar{Y}_{x_i = -1}$$

The coded levels $\{-1, +1\}$ are mapped to the physical factor bounds via $x_{\text{phys}} = \text{lo} + \frac{x_{\text{coded}} + 1}{2}(\text{hi} - \text{lo})$.

### Code

```rust
use salib::estimators::estimate_fractional_factorial;
use salib::samplers::build_plackett_burman;
use salib::*;

let problem = ProblemBuilder::new()
    .factor("x1", Distribution::Uniform { lo: 0.0, hi: 1.0 })
    .factor("x2", Distribution::Uniform { lo: 0.0, hi: 1.0 })
    .factor("x3", Distribution::Uniform { lo: 0.0, hi: 1.0 })
    .build()
    .unwrap();

let design = build_plackett_burman(problem.dim()).unwrap();

let effects = estimate_fractional_factorial(&design, &problem, |x| {
    3.0 * x[0] + 1.0 * x[1] + 0.5 * x[2]
});

println!("{effects}");
```

Plackett–Burman designs screen many factors with relatively few runs. They
are useful for expensive experiments when main effects are the priority.
Interpret those effects with the design's interaction confounding in mind.

## Choosing among experimental design methods

| Method | Input | Output | Best for |
|--------|-------|--------|----------|
| ANOVA | Balanced grid | Variance fractions + $F$-tests | Factor significance in structured experiments. |
| G-Theory | Crossed $p \times i \times r$ grid | Variance components + $\rho^2$, $\Phi$ | Measurement reliability, D-study projections. |
| Discrepancy | Point set in $[0, 1]^d$ | CD, WD, MD, L2* | Evaluating space-filling quality of sampling plans. |
| Fractional Factorial | Plackett-Burman design | Main effects | Screening with minimal runs in physical experiments. |
