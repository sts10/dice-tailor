pub fn get_loss(dice_sides: i32, list_length: i32) -> i32 {
    let mut loss = 0;
    for n in 1..100 {
        if dice_sides.pow(n) > list_length {
            // We went over the list length.
            // Go back one n
            let new_list_length = dice_sides.pow(n - 1);
            loss = list_length - new_list_length;
            break;
        }
    }
    loss
}

pub fn log_base(base: i32, n: f64) -> f64 {
    let base_as_float: f64 = base as f64;
    (n.ln() / base_as_float.ln()) as f64
}
