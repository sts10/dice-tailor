fn main() {
    assert_eq!(get_loss(6, 8000), 8000 - 7776);
    assert_eq!(log_base(6, 7776.0), 5.0);

    let list_length: i32 = 7776;

    let mut lowest_loss: i32 = i32::MAX;
    let mut best_sides_to_use = 0;
    for sides in 2..36 {
        let this_loss = get_loss(sides, list_length);
        if this_loss < lowest_loss {
            lowest_loss = this_loss;
            best_sides_to_use = sides;
        }
    }
    let cut_list_length = list_length - lowest_loss;
    let optimal_number_of_dice = log_base(best_sides_to_use, cut_list_length as f64);
    if lowest_loss == 0 {
        println!(
            "Can keep list at {}. Use {} {}-sided dice.",
            list_length, optimal_number_of_dice, best_sides_to_use
        );
    } else {
        println!(
            "Recommend cutting list length to {}. Can use {} {}-sided dice.",
            list_length - lowest_loss,
            optimal_number_of_dice,
            best_sides_to_use
        );
    }
}

fn get_loss(dice_sides: i32, list_length: i32) -> i32 {
    let mut loss = 0;
    for n in 0..100 {
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

fn log_base(base: i32, n: f64) -> f64 {
    let base_as_float: f64 = base as f64;
    (n.ln() / base_as_float.ln()) as f64
}
