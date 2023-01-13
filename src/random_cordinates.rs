use rand::Rng;

pub fn random_cordinates_two_dim(steps: u64) -> Vec<[f64; 2]> {
    let mut ran = rand::thread_rng();

    let x_walk: Vec<u64> = (0..steps).map(|_| ran.gen_range(0..=1)).collect();
    let y_walk: Vec<u64> = (0..steps).map(|_| ran.gen_range(0..=1)).collect();

    let walk: &mut Vec<[f64; 2]> = &mut vec![[0.0, 0.0]];

    for i in 0..steps as usize {
        let x: u64 = x_walk[i];
        let y: u64 = y_walk[i];

        let step_before_x = walk[i][0];
        let step_before_y = walk[i][1];

        let new_step_x = step_before_x + bool_as_number(x);
        let new_step_y = step_before_y + bool_as_number(y);

        let new_step = [new_step_x, new_step_y];
        walk.push(new_step)
    }

    walk.clone()
}

fn bool_as_number(value: u64) -> f64 {
    if value == 1 {
        1.0
    } else {
        -1.0
    }
}

pub fn random_cordinates_one_dim(steps: u64) -> Vec<[f64; 2]> {
    let mut ran = rand::thread_rng();

    let x_walk: Vec<u64> = (0..steps).map(|_| ran.gen_range(0..=1)).collect();

    let walk: &mut Vec<[f64; 2]> = &mut vec![[0.0, 0.0]];

    for i in 0..steps as usize {
        let x: u64 = x_walk[i];

        let step_before_x = walk[i][0];

        let new_step_x = step_before_x + bool_as_number(x);

        let new_step = [new_step_x, 0.0];
        walk.push(new_step)
    }

    walk.clone()
}
