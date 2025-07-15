use rsa_alg::{fast_exp, fast_exp_mod, get_i64};

fn main() {
    let num = 8;
    let pow = 6;
    let modulus = 10;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));
    println!("{num} ^ {pow} % {modulus} = {}", fast_exp_mod(num, pow, modulus));

    loop {
        let num = get_i64("num = ");
        let pow = get_i64("pow = ");
        let modulus = get_i64("mod = ");
        println!("    fast_exp = {}", fast_exp(num, pow));
        println!("    {num} ^ {pow} = {}", num.pow(pow as u32));
        println!();
        println!("    fast_exp_mod = {}", fast_exp_mod(num, pow, modulus));
        println!("    {num} ^ {pow} % {modulus} = {}", num.pow(pow as u32) % modulus);
        println!("---------");
    }
}
