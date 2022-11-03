use clap::Parser;
use dicetailor::*;

/// Fit dice values to each word in a word list.
#[derive(Parser, Debug)]
#[clap(version, about, name = "dice-tailor")]
struct Args {
    /// Set as a constant the number of dice sides (Optional)
    #[clap(short = 's', long = "sides")]
    sides: Option<i32>,

    /// Length of initial list
    #[clap(name = "Length of Initial List", required = true)]
    list_length: i32,
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
