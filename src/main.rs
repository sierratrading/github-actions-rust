use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

fn main() {
    println!("hello, Elizabeth");
    println!("2 + 2 = {}", add(2, 2));
    println!("2 - 2 = {}", sub(2, 2));
    println!("2 * 2 = {}", mul(2, 2));
    println!("2 / 2 = {}", div(2, 2));
}
