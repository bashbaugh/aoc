use std::fs;

fn main () {
    let mut ep = "";
    let mut gam = "";

    let dat = fs::read_to_string("./intest1.txt").unwrap();
    // let commands = dat.split("\n");

    for c in dat.lines().nth(1).unwrap().split("") {
        println!("{}", c)
    }
}
