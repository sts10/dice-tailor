pub fn measure(list_length: i32, sides: Option<i32>) {
    let mut lowest_loss: i32 = i32::MAX;
    let mut best_sides_to_use = 0;

    match sides {
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
            "Recommend cutting list length to {}. Can then use {} {}-sided dice.",
            list_length - lowest_loss,
            optimal_number_of_dice,
            best_sides_to_use
        );
    }
}

use plotters::prelude::*;
use std::fs;
pub fn make_plots(sides: Option<i32>) {
    fs::create_dir_all("images/").unwrap();
    if let Some(sides) = sides {
        let file_name = format!("images/{}-sided-die.png", sides);
        let specified_drawing_area =
            BitMapBackend::new(&file_name, (2048, 1536)).into_drawing_area();
        specified_drawing_area.fill(&WHITE).unwrap();

        let max_number_of_words = 27000;
        let mut specified_chart = ChartBuilder::on(&specified_drawing_area)
            .margin(5)
            .set_all_label_area_size(100)
            .build_cartesian_2d(0..max_number_of_words, 0..max_number_of_words)
            .unwrap();
        specified_chart
            .configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("Word list length")
            .y_desc("Words needed to cut to fit dice sides")
            .draw()
            .unwrap();

        specified_chart
            .draw_series(LineSeries::new(
                (5..max_number_of_words).map(|x| (x, get_loss(sides, x))),
                &RED,
            ))
            .unwrap()
            .label(sides.to_string());
        println!("Chart printed to {}", file_name);
    } else {
        eprintln!("No number of sides specified.\nPrinting a sample of common dice sides charts to ./images directory.");
        let common_drawing_area =
            BitMapBackend::new("images/common_dice.png", (2048, 1536)).into_drawing_area();
        common_drawing_area.fill(&WHITE).unwrap();
        let common_drawing_area = common_drawing_area
            .titled(
                "Word list 'fit' for 6-, 8-, and 12-sides dice",
                ("sans-serif", 60),
            )
            .unwrap();

        let mut common_chart = ChartBuilder::on(&common_drawing_area)
            .margin(5)
            .set_all_label_area_size(100)
            .build_cartesian_2d(0..20000, 0..20000)
            .unwrap();
        common_chart
            .configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("Word list length")
            .y_desc("Words needed to cut to fit dice sides")
            .draw()
            .unwrap();

        common_chart
            .draw_series(LineSeries::new(
                (5..20000).map(|x| (x, get_loss(6, x))),
                &RED,
            ))
            .unwrap()
            .label("6-sided")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        common_chart
            .draw_series(LineSeries::new(
                (5..20000).map(|x| (x, get_loss(8, x))),
                &BLUE,
            ))
            .unwrap()
            .label("8-sided")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        common_chart
            .draw_series(LineSeries::new(
                (5..20000).map(|x| (x, get_loss(12, x))),
                &GREEN,
            ))
            .unwrap()
            .label("12-sided")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        common_chart
            .configure_series_labels()
            .background_style(&RGBColor(128, 128, 128))
            .draw()
            .unwrap();

        let all_dice_drawing_area =
            BitMapBackend::new("images/all_dice.png", (2048, 1536)).into_drawing_area();
        all_dice_drawing_area.fill(&WHITE).unwrap();

        let all_dice_drawing_area = all_dice_drawing_area
            .titled(
                "Word list 'fit' for dice of sides 2 to 36",
                ("sans-serif", 60),
            )
            .unwrap();

        let mut all_chart = ChartBuilder::on(&all_dice_drawing_area)
            .margin(5)
            .set_all_label_area_size(100)
            .build_cartesian_2d(0..20000, 0..20000)
            .unwrap();

        all_chart
            .configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("Word list length")
            .y_desc("Words needed to cut to fit dice sides")
            .draw()
            .unwrap();
        for sides in 2..36 {
            all_chart
                .draw_series(LineSeries::new(
                    (5..20000).map(|x| (x, get_loss(sides, x))),
                    &BLACK,
                ))
                .unwrap();
        }
    }
}

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
