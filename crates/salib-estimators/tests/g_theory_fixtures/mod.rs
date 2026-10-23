//! Shared crossed p × i × r grids for G-theory tests.
#![allow(dead_code)]

use ndarray::Array3;

/// Brennan (2001), *Generalizability Theory*, Synthetic Data Set No. 3:
/// 10 persons × 4 items × 2 occasions, with occasions on the rater axis.
///
/// Transcribed from GeneralizIT `tests/fixtures/design3_fixtures.py` at
/// 8af4faefa4027e5e7e5b45495a5540b2093dc173, a secondary source: the book
/// itself has not been acquired. Printed targets:
/// σ²(p) .5528, σ²(i) .4417, σ²(o) .0074, σ²(pi) .5750, σ²(po) .1009,
/// σ²(io) .1565, σ²(pio) .9352; Eρ² .64, Φ .55.
pub fn synthetic3() -> Array3<f64> {
    let cols: [[[f64; 10]; 4]; 2] = [
        [
            [2., 4., 5., 5., 4., 4., 2., 3., 0., 6.],
            [6., 5., 5., 9., 3., 4., 6., 4., 5., 8.],
            [7., 6., 4., 8., 5., 4., 6., 4., 4., 7.],
            [5., 7., 6., 6., 6., 7., 5., 5., 5., 6.],
        ],
        [
            [2., 6., 5., 5., 4., 6., 2., 6., 5., 6.],
            [5., 7., 4., 7., 5., 4., 7., 6., 5., 8.],
            [5., 5., 5., 7., 6., 7., 7., 6., 5., 8.],
            [5., 7., 5., 6., 4., 8., 5., 4., 3., 6.],
        ],
    ];
    Array3::from_shape_fn((10, 4, 2), |(p, i, o)| cols[o][i][p])
}

/// Self-derived 4p × 3i × 2r grid whose EMS estimates of σ²(r) and σ²(ir)
/// are negative. Exact mean squares (fixtures/gate1_pir_4x3x2.json) give
/// σ²(p) 9/4, σ²(i) 3/4, σ²(r) −1/24, σ²(pi) 1/4, σ²(pr) 3/8, σ²(ir) −1/72,
/// σ²(pir) 7/72.
pub fn gate1() -> Array3<f64> {
    let data: [[[f64; 2]; 3]; 4] = [
        [[4.0, 3.0], [6.0, 5.0], [5.0, 4.0]],
        [[7.0, 6.0], [8.0, 8.0], [6.0, 5.0]],
        [[3.0, 4.0], [5.0, 6.0], [4.0, 5.0]],
        [[8.0, 7.0], [9.0, 8.0], [7.0, 7.0]],
    ];
    Array3::from_shape_fn((4, 3, 2), |(p, i, r)| data[p][i][r])
}

/// Deterministic xorshift64 for reproducible random grids.
pub struct XorShift(pub u64);

impl XorShift {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1_u64 << 53) as f64
    }
}
