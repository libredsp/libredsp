pub struct PiecewiseLinearInterpolator {
    nodes: Vec<(f64, f64)>,
}

impl PiecewiseLinearInterpolator {
    pub fn new(mut nodes: Vec<(f64, f64)>) -> Self {
        assert!(nodes.len() >= 2, "Requires at least two nodes.");

        nodes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        /* Check the nodes are unique */
        for i in 1..nodes.len() {
            assert!(
                (nodes[i].0 - nodes[i - 1].0).abs() > 1e-12,
                "Duplicate x-values are not allowed."
            );
        }

        Self { nodes }
    }

    fn clamp(&self, x: f64) -> Option<f64> {
        if x <= self.nodes[0].0 {
            return Some(self.nodes[0].1);
        }
        if x >= self.nodes[self.nodes.len() - 1].0 {
            return Some(self.nodes[self.nodes.len() - 1].1);
        }
        None
    }
    pub fn evaluate(&self, x: f64) -> f64 {
        if let Some(y) = self.clamp(x) {
            return y;
        }
        
        for i in 0..self.nodes.len() - 1 {
            let (x0, y0) = self.nodes[i];
            let (x1, y1) = self.nodes[i + 1];

            if x >= x0 && x <= x1 {
                let t = (x - x0) / (x1 - x0);
                return y0 + t * (y1 - y0);
            }
        }

        unreachable!()
    }
}
