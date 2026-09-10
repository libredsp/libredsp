pub struct LMS {
    w: Vec<f64>,
    u: Vec<f64>,
    idx: usize,
    mu: f64,
}

impl LMS {
    pub fn new(num_taps: usize, mu: f64) -> Self {
        assert!(num_taps > 0);
        assert!(mu > 0.0);

        Self {
            w: vec![0.0; num_taps],
            u: vec![0.0; num_taps],
            idx: 0,
            mu,
        }
    }

    pub fn update(&mut self, x: f64, d: f64) -> f64 {
        self.u[self.idx] = x;

        // y[n] = w^T u
        let mut y = 0.0;
        let mut k = self.idx;

        for i in 0..self.w.len() {
            y += self.w[i] * self.u[k];
            k = if k == 0 { self.w.len() - 1 } else { k - 1 };
        }

        // e[n] = d[n] - y[n]
        let e = d - y;

        // w <- w + 2μ e u
        let step = 2.0 * self.mu * e;
        let mut k = self.idx;

        for i in 0..self.w.len() {
            self.w[i] += step * self.u[k];
            k = if k == 0 { self.w.len() - 1 } else { k - 1 };
        }

        self.idx = if self.idx + 1 == self.w.len() {
            0
        } else {
            self.idx + 1
        };

        e
    }

    pub fn coefficients(&self) -> &[f64] {
        &self.w
    }
}
