# Dice Tailor

Find out how many of what kinds of dice best suit your word list. 

By default searches through die of sides numbering 2 through 36. Optionally, you can fix the number of sides with option `-s`.

## Installation

### Using Rust and cargo (suggested method)
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/dice-tailor --branch main`

Once installed, the executable will be `dice-tailor`. You should then be able to run `dice-tailor --help`.

## Usage

```text
USAGE:
    dice-tailor [OPTIONS] <Length of Initial List>

ARGS:
    <Length of Initial List>    Length of initial list

OPTIONS:
    -h, --help             Print help information
    -s, --sides <SIDES>    Set as a constant the number of dice sides (Optional)
    -V, --version          Print version information
```

## Examples

Let's say you have a word list of 7,900 words. And you want to print a corresponding dice roll for each word. 

Running `dice-tailor 7900` prints:
```text
Recommend cutting list length to 7776. Can use 5 6-sided dice.
```

But Dice Tailor won't always recommend using 6-sided dice. Let's say your word list is 8,003 words. Running `dice-tailor 8003` prints

`Recommend cutting list length to 8000. Can use 3 20-sided dice.`

If you don't expect your users to have 20-sided dice available, you can "force" a 6-sided die by running `dice-tailor -s 6 8003`. Dice Tailor will then print: 

`Recommend cutting list length to 7776. Can use 5 6-sided dice.`

## Actually printing the corresponding dice roll values 

Dice Tailor only gives you an efficient recommendation of how to map out your dice rolls. You'll need to use a different tool to actually print the dice rolls to your list file.

To do this, you can use a tool like [Tidy](https://github.com/sts10/tidy) to actually print the numbers to a new list. If you want to make the necessary cuts randomly, that might look something like this:

```bash
tidy --print-rand 7776 --dice 6 -o dice_list.txt my_input_list.txt
```
