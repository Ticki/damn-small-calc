use std::io;
use std::io::*;


fn main() {
    println!("Welcome to damn small calculator! Note that it uses reverse polish notation. Syntax for (1 + 1) * 2: 1 1 + 2 *");
    loop {
        print!("     > ");

        if let Err(x) = io::stdout().flush() {
            println!("could't flush: {}", x);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()
            .expect("Couldn't read.");

        let mut register  = [0i64, 0i64];
        let mut first     = true; // Is it pointing on register 0?
        let mut error     = false;

        for i in input.chars() {
            match i {
                '0' => {
                    register[to_reg(first)] *= 10;
                },
                '1' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 1;
                },
                '2' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 2;
                },
                '3' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 3;
                },
                '4' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 4;
                },
                '5' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 5;
                },
                '6' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 6;
                },
                '7' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 7;
                },
                '8' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 8;
                },
                '9' => {
                    register[to_reg(first)] *= 10;
                    register[to_reg(first)] += 9;
                },
                ' ' => {
                    first = !first;
                },
                '+' => {
                    register[0] = register[0] + register[1];
                    register[1] = 0;
                    first = true;
                },
                '-' => {
                    register[0] = register[0] - register[1];
                    register[1] = 0;
                    first = true;
                },
                '*' => {
                    register[0] = register[0] * register[1];
                    register[1] = 0;
                    first = true;
                },
                '^' => {
                    if register[1] < 0 {
                        println!("Floats not supported");
                        error = true;
                    }
                    register[0] = register[0].pow(register[1] as u32);
                    register[1] = 0;
                    first = true;
                },
                '\n' => {},
                _ => {
                    println!("Unknown symbol: {}", i);
                    error = true;
                    break;
                },
            }
        }
        if !error {
            println!("{}", register[0]);
        }
    }
}

fn to_reg(ptr: bool) -> usize {
    match ptr {
        true => 0,
        false => 1,
    }
}
