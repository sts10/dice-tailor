# Dice Tailor

Find out how many of what kinds of dice best suit your word list. 

## Installation

### Using Rust and cargo (suggested method)
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/dice-tailor --branch main`

Once installed, the executable will be `dicetailor`. You should then be able to run `dicetailor --help`.

## Usage

```text
Usage: dicetailor <COMMAND>

Commands:
  measure  Given list length, recommend a "fit"
  draw     Draw charts
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### Drawing 

Dice Tailor can draw a number of line charts related to how dice and word list fit together. 

The two charts that Dice Tailor is hard-coded to produce plot how word list lengths (X-axis) fit to word list cuts necessary to fit to a dice configuration. 

**Common die fits** (6,8,12)

![Common dice fits](images/common_dice.png)

**All dice 2 to 36 sides**
![All dice fits](images/all_dice.png)

To create these charts in an `./images/` directory, run `dicetailor draw`

If you want to investigate a specific die, say one with 11 sides, you can run `dicetailor draw -s 11`.

### Measuring 
Let's say you have a word list of 7,900 words, and you want to print a corresponding dice roll for each word. 

Running `dicetailor measure 7900` prints:
```text
Recommend cutting list length to 7776. Can use 5 6-sided dice.
```

But Dice Tailor won't always recommend using 6-sided dice. Let's say your word list is 8,003 words. Running `dicetailor measure 8003` prints

`Recommend cutting list length to 8000. Can use 3 20-sided dice.`

If you don't expect your users to have 20-sided dice available, you can "force" a 6-sided die by running `dicetailor measure -s 6 8003`. Dice Tailor will then print: 

`Recommend cutting list length to 7776. Can use 5 6-sided dice.`

## Actually printing the corresponding dice roll values 

Dice Tailor only gives you an efficient recommendation of how to map out your dice rolls. You'll need to use a different tool to actually print the dice rolls to your list file.

To do this, you can use a tool like [Tidy](https://github.com/sts10/tidy) to actually print the numbers to a new list. If you want to make the necessary cuts randomly, that might look something like this:

```bash
tidy --print-rand 7776 --dice 6 -o dice_list.txt my_input_list.txt
```
