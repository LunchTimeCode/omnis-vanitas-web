use rand::Rng;

pub fn random_cordinates() -> Vec<[f64; 2]> {
    let mut ran = rand::thread_rng();

    let result: Vec<[f64; 2]> = (0..100)
        .map(|_| {
            let random_point: [f64; 2] = [ran.gen::<f64>(), ran.gen::<f64>()];
            random_point
        })
        .collect();
    result
}
