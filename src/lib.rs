const STEP_SIZE: f64 = 1e-7;

struct HamiltonianSystem<F>
where
    F: Fn(&[f64]) -> f64,
{
    x: Vec<f64>,
    p: Vec<f64>,
    potential_energy: F,
    mass: f64,
}

impl<F> HamiltonianSystem<F>
where
    F: Fn(&[f64]) -> f64,
{
    pub fn new(initial_x: Vec<f64>, initial_p: Vec<f64>, potential_energy: F, mass: f64) -> Self {
        Self {
            x: initial_x,
            p: initial_p,
            potential_energy,
            mass,
        }
    }

    pub fn leapfrog(&mut self, n_steps: usize, delta_t: f64) {
        assert!(n_steps > 0);
        let grad = finite_difference(&self.x, &self.potential_energy);

        for ix in 0..self.p.len() {
            self.p[ix] -= 0.5 * delta_t * grad[ix];
        }

        for n_step in 0..n_steps {
            for ix in 0..self.x.len() {
                self.x[ix] += (delta_t / self.mass) * self.p[ix];
            }

            let grad = finite_difference(&self.x, &self.potential_energy);
            if n_step == n_steps - 1 {
                for ix in 0..self.p.len() {
                    self.p[ix] -= 0.5 * delta_t * grad[ix];
                }
            } else {
                for ix in 0..self.p.len() {
                    self.p[ix] -= delta_t * grad[ix];
                }
            }
        }
    }
}

pub fn finite_difference<F>(x: &[f64], f: F) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let mut gradient = vec![0.0; x.len()];
    let mut x_prime = x.to_vec();
    for dim_ix in 0..x.len() {
        let original = x_prime[dim_ix];
        x_prime[dim_ix] = original + 0.5 * STEP_SIZE;
        let f_plus = f(&x_prime);
        x_prime[dim_ix] = original - 0.5 * STEP_SIZE;
        let f_minus = f(&x_prime);
        x_prime[dim_ix] = original;
        let numerator = f_plus - f_minus;
        gradient[dim_ix] = numerator / STEP_SIZE;
    }
    gradient
}

#[cfg(test)]
mod tests {
    use crate::finite_difference;

    #[test]
    fn test_finite_difference_power() {
        // f(x) = x^7, f'(x) = 7 * x^6
        let f = |x: &[f64]| x.iter().map(|x_i| x_i.powi(7)).sum::<f64>();
        let x = [1.0];
        let grad = finite_difference(&x, f);
        let expected = 7.0;
        assert!(
            (grad[0] - expected).abs() < 1e-6,
            "Expected {}, got {}",
            expected,
            grad[0]
        );
    }

    #[test]
    fn test_finite_difference_multivariate() {
        // f(x, y) = x^2 + y^3, grad f = [2x, 3y^2]
        let f = |x: &[f64]| x[0].powi(2) + x[1].powi(3);
        let x = [2.0, 3.0];
        let grad = finite_difference(&x, f);
        let expected = [4.0, 27.0];

        for i in 0..x.len() {
            assert!(
                (grad[i] - expected[i]).abs() < 1e-6,
                "At index {}, expected {}, got {}",
                i,
                expected[i],
                grad[i]
            );
        }
    }
}
