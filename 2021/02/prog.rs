use std::fs;

fn main () {
    let mut h: u32 = 0;
    let mut d: u32 = 0;

    let dat = fs::read_to_string("./in1.txt").unwrap();
    // let commands = dat.split("\n");

    for c in dat.lines() {
        if c.starts_with("forward") {
            h += c.chars().last().unwrap().to_digit(10).unwrap()
        }
        if c.starts_with("down") {
            d += c.chars().last().unwrap().to_digit(10).unwrap()
        }
        if c.starts_with("up") {
            d -= c.chars().last().unwrap().to_digit(10).unwrap()
        }
    }

    println!("h: {}, d: {}, multiplied: {}", h, d, h*d)
}
