use rand::Rng;

pub fn random_cordinates_two_dim(from: f64, to: f64, steps: u64) -> Vec<[f64; 2]> {
    let mut ran = rand::thread_rng();

    let result: Vec<[f64; 2]> = (0..steps)
        .map(|_| {
            let random_point: [f64; 2] = [ran.gen_range(from..to),ran.gen_range(from..to)];
            random_point
        })
        .collect();
    result
}

pub fn random_cordinates_one_dim(from: f64, to: f64, steps: u64) -> Vec<[f64; 2]> {
    let mut ran = rand::thread_rng();

    let result: Vec<[f64; 2]> = (0..steps)
        .map(|_| {
            let random_point: [f64; 2] = [ran.gen_range(from..to), 0.0];
            random_point
        })
        .collect();
    result
}
