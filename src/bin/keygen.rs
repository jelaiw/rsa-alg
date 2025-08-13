use rsa_alg::{find_prime, inverse_mod, random_exponent, totient};

fn main() {
    const NUM_TESTS: u8 = 20;
    let p = find_prime(1000, 10000, NUM_TESTS);
    let q = find_prime(1000, 10000, NUM_TESTS);

    let n = p * q;
    let λ_n = totient(p, q);
    let e = random_exponent(λ_n);
    let d = inverse_mod(e, λ_n);

    assert_eq!(1, e * d % λ_n);

    println!("public key");
    println!("==========");
    println!("n = {n}");
    println!("e = {e}");
    println!();

    println!("private key");
    println!("===========");
    println!("p = {p}, q = {q}");
    println!("λ(n) = {λ_n}");
    println!("d = {d}");
}
