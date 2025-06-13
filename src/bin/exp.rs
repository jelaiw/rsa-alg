use rsa_alg::{fast_exp, get_i64};

fn main() {
    let num = 8;
    let pow = 6;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));

    let num = 7;
    let pow = 10;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));

    loop {
        let num = get_i64("num = ");
        let pow = get_i64("pow = ");
        println!("{num} ^ {pow} = {}", fast_exp(num, pow));
    }
}
