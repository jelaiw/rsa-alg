## Overview
Repo for sharing solutions with fellow learners.

Third project in Rod Stephens "Algorithms in Rust" Manning LiveProject series. 

Have fun!!

## Guided Tour
A quick run of each of the submitted programs.

### RSA demo
Textbook RSA proof-of-concept from final milestone.
```sh
$ cargo run --bin demo
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/demo`
public key
==========
n = 49032317
e = 11079847

private key
===========
p = 5209, q = 9413
λ(n) = 12254424
d = 2563495

m = 1234567
ciphertext = 14506555
plaintext = 1234567

m = 1337
ciphertext = 37620445
plaintext = 1337

m = 8675309
ciphertext = 25810469
plaintext = 8675309

m = q

thread 'main' panicked at src/lib.rs:175:28:
Error parsing integer: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

### GCD and LCM
Reproduce expected results from validation table provided in first milestone.

```sh
$ cargo run --bin gcd
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/gcd`
gcd(12, 18) = 6
lcm(12, 18) = 36
a = 270
b = 192
gcd(270, 192) = 6
lcm(270, 192) = 8640
a = 7469
b = 2464
gcd(7469, 2464) = 77
lcm(7469, 2464) = 239008
a = 55290
b = 115430
gcd(55290, 115430) = 970
lcm(55290, 115430) = 6579510
a = 

thread 'main' panicked at src/lib.rs:175:28:
Error parsing integer: ParseIntError { kind: Empty }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

## References
1. "Algorithms in Rust: Public Key Cryptography". Stephens, R. Manning LiveProject, https://www.manning.com/liveproject/public-key-cryptography-rust.
2. OnlineGDB: online compiler and debugger for C/C++, https://www.onlinegdb.com/.
2. "The Euclidean Algorithm". Khan Academy, https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/the-euclidean-algorithm.
2. Number Theory and Cryptography. Levin M, Kulikov A. University of California San Diego, https://www.coursera.org/learn/number-theory-cryptography.
2. "Sieve of Eratosthenes". Cruise, B. Khan Academy, https://www.khanacademy.org/computing/computer-science/cryptography/comp-number-theory/v/sieve-of-eratosthenes-prime-adventure-part-4.
2. "Probabilistically determine whether a number is prime in C#". Stephens, R. C# Helper, http://csharphelper.com/howtos/howto_check_primality.html.
2. "Fermat number". Wikipedia, https://en.wikipedia.org/wiki/Fermat_number.
2. Composite witness defined. Cruise, B. Khan Academy, "Randomized algorithms", https://www.khanacademy.org/computing/computer-science/cryptography/random-algorithms-probability/v/randomized-algorithms-prime-adventure-part-8.
2. Rust rand crate random_range. https://docs.rs/rand/latest/rand/trait.Rng.html#method.random_range.
2. "Cryptographically secure pseudo-random number generators (CSPRNGs)". The Rust Rand Book, https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs.
2. "Find random prime numbers in C#". Stephens, R. C# Helper, http://csharphelper.com/howtos/howto_find_primes.html.
2. "Fermat primality test". Cruise, B. Khan Academy, https://www.khanacademy.org/computing/computer-science/cryptography/random-algorithms-probability/v/fermat-primality-test-prime-adventure-part-10.
2. "Carmichael number". Wikipedia, https://en.wikipedia.org/wiki/Carmichael_number.
2. "RSA cryptosystem". Wikipedia, https://en.wikipedia.org/wiki/RSA_cryptosystem.
2. "Carmichael's totient function". Wikipedia, https://en.wikipedia.org/wiki/Carmichael_function.
2. "Euler's totient function". Cruise, B. Khan Academy, https://www.khanacademy.org/computing/computer-science/cryptography/modern-crypt/v/euler-s-totient-function-phi-function.
2. "Computing multiplicative inverses in modular structures". Wikipedia, https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures.
2. "Is there a modulus (not remainder) function". Kalbertodt, L. Stack Overflow, https://stackoverflow.com/a/57342011.
2. "modulo". r/rust, https://www.reddit.com/r/rust/comments/eh1f19/modulo/.
2. Rust i64 rem_euclid. https://doc.rust-lang.org/stable/std/primitive.i64.html#method.rem_euclid.
