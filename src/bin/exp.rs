use rsa_alg::fast_exp;

fn main() {
    let num = 8;
    let pow = 6;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));

    let num = 7;
    let pow = 10;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));
}
