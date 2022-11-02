use clap::Parser;

/// Figure out how to efficiently assign dice calues to each word in a word list.
#[derive(Parser, Debug)]
#[clap(version, about, name = "dice-tailor")]
struct Args {
    /// Length of given list
    #[clap(short = 'l', long = "length")]
    list_length: i32,

    /// Fix number of dice sides
    #[clap(short = 's', long = "sides")]
    sides: Option<i32>,
}

fn main() {
    assert_eq!(get_loss(6, 8000), 8000 - 7776);
    assert_eq!(log_base(6, 7776.0), 5.0);

    let opt = Args::parse();
    let list_length: i32 = opt.list_length;

    let mut lowest_loss: i32 = i32::MAX;
    let mut best_sides_to_use = 0;

    match opt.sides {
        Some(sides) => {
            // Fixed number of dice sides provided by user, so only need to check
            // that amount of dice sides
            let this_loss = get_loss(sides, list_length);
            if this_loss < lowest_loss {
                lowest_loss = this_loss;
                best_sides_to_use = sides;
            }
        }
        None => {
            // No number of dice sides given by user, so we'll check all dice from 2 sides
            // to 36 sides and see which results in the lowest loss of words
            // from the given list.
            for sides in 2..36 {
                let this_loss = get_loss(sides, list_length);
                if this_loss < lowest_loss {
                    lowest_loss = this_loss;
                    best_sides_to_use = sides;
                }
            }
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
