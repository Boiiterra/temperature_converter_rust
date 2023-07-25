mod lib;

fn main() {
    println!("{}", lib::temperature("f", "c", 100));
    println!("{}", lib::temperature("c", "f", 38));
}