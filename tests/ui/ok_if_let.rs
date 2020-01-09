// run-rustfix

#![warn(clippy::if_let_some_result)]

fn str_to_int(x: &str) -> i32 {
    if let Some(y) = x.parse().ok() {
        y
    } else {
        0
    }
}

fn str_to_int_ok(x: &str) -> i32 {
    if let Ok(y) = x.parse() {
        y
    } else {
        0
    }
}

fn nested_some_no_else(x: &str) -> i32 {
    {
        if let Some(y) = x.parse().ok() {
            return y;
        };
        0
    }
}

fn main() {
    let x = str_to_int("1");
    let y = str_to_int_ok("2");
    let z = nested_some_no_else("3");
    println!("{}{}{}", x, y, z);
}
