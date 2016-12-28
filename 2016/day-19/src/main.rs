// Part 1 is the josephus function
fn josephus(input: u32)
{
    let input_2 = 1 << (31 - input.leading_zeros());

    let remainder = input - input_2;

    println!("Position for {} is {}", input, remainder * 2 + 1);
}

fn power_3(input: u32) -> u32
{
    let mut base = 1;
    while 3 * base < input {
        base = base * 3;
    }

    return base;
}

// Part two is similar, but in base 3.
fn alternative(input: u32)
{
    let log = power_3(input);
    let remainder = input - log;

    let pos;

    if remainder <= log {
        pos = remainder;
    } else {
        pos = 2 * remainder - log;
    }

    println!("Alternative for {} is: {}", input, pos);
}

fn main() {
    josephus(3014603);
    alternative(3014603);
}
