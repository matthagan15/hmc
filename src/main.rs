fn leapfrog<F>(initial_conditions: Vec<(f64, f64)>, objective_fn: F) -> Vec<(f64, f64)>
where
    F: Fn(Vec<f64>) -> f64,
{
    todo!()
}

fn leapfrog_1d<F>(
    initial_pos: f64,
    initial_velocity: f64,
    objective_fn: F,
    num_steps: usize,
    step_size: f64,
) -> (f64, f64)
where
    F: Fn(f64) -> f64,
{
    todo!()
}

fn main() {
    println!("Hello, world!");
}
