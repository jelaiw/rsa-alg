## Overview
Repo for sharing solutions with fellow learners.

Third project in Rod Stephens "Algorithms in Rust" Manning LiveProject series. 

Have fun!!

## Quick Tour
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

### Fast Exponentiation
These examples show how to exit the program. Crash out on bad input. :-D

```sh
$ cargo run --bin fast_exp
   Compiling rsa-alg v0.8.0 (/workspaces/rsa-alg)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/fast_exp`
8 ^ 6 = 262144
8 ^ 6 % 10 = 4
num = 7
pow = 10
mod = 101
    fast_exp = 282475249
    7 ^ 10 = 282475249

    fast_exp_mod = 65
    7 ^ 10 % 101 = 65
---------
num = 9
pow = 13
mod = 283
    fast_exp = 2541865828329
    9 ^ 13 = 2541865828329

    fast_exp_mod = 179
    9 ^ 13 % 283 = 179
---------
num = 213
pow = 5
mod = 100
    fast_exp = 438427732293
    213 ^ 5 = 438427732293

    fast_exp_mod = 93
    213 ^ 5 % 100 = 93
---------
num = 

thread 'main' panicked at src/lib.rs:175:28:
Error parsing integer: ParseIntError { kind: Empty }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

### Sieve of Eratosthenes
```sh
$ cargo run --bin sieve
   Compiling rsa-alg v0.8.0 (/workspaces/rsa-alg)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/sieve`
max = 100
2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97
2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97 
There are 25 primes less than 100.
$
```

### Factoring Numbers
Benchmarks show speedup with a sieve.

```sh
$ cargo run --bin factoring
   Compiling rsa-alg v0.8.0 (/workspaces/rsa-alg)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/factoring`
Number to factor = 312680865509917
find_factors: 68.230606ms
find_factors_sieve: 5.059512ms

Number to factor = 1819448968910731
find_factors: 367.220113ms
find_factors_sieve: 6.605636ms

Number to factor = 12345678901234
find_factors: 2.513371ms
find_factors_sieve: 188.622µs

Number to factor = 6795742697625173
find_factors: 64.517246ms
find_factors_sieve: 4.506721ms

Number to factor = 64374108854777
find_factors: 72.704245ms
find_factors_sieve: 4.904232ms

m = 312680865509917
Factoring of 312680865509917: [40129483, 7791799]
m = 12345678901234
Factoring of 12345678901234: [12079920647, 73, 7, 2]
m = 64374108854777
Factoring of 64374108854777: [64374108854777]
m = 

thread 'main' panicked at src/lib.rs:175:28:
Error parsing integer: ParseIntError { kind: Empty }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

### Primality Testing
Find random primes for generating RSA keys.

```sh
$ cargo run --bin find_prime
   Compiling rsa-alg v0.8.0 (/workspaces/rsa-alg)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/find_prime`
Probability of prime = 0.9999990463256836
# digits (max 9): 9
Probable prime is 330841727.
# digits (max 9): 8
Probable prime is 49460321.
# digits (max 9): 7
Probable prime is 5133509.
# digits (max 9): 6
Probable prime is 348463.
# digits (max 9): 5
Probable prime is 93607.
# digits (max 9): 4
Probable prime is 7541.
# digits (max 9): 3
Probable prime is 199.
# digits (max 9): 2
Probable prime is 97.
# digits (max 9): 1
Probable prime is 3.
# digits (max 9): 

thread 'main' panicked at src/lib.rs:175:28:
Error parsing integer: ParseIntError { kind: Empty }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

## Tests
Unit tests for library crate.

```sh
$ cargo test --lib
   Compiling rsa-alg v0.8.0 (/workspaces/rsa-alg)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running unittests src/lib.rs (target/debug/deps/rsa_alg-45623a73567348f0)

running 18 tests
test tests::egcd_kulikov_lecture_examples ... ok
test tests::egcd_kulikov_lecture_examples_swap_argument_order ... ok
test tests::fast_exp_expected_values_from_given_validation_table ... ok
test tests::fast_exp_mod_expected_values_from_given_validation_table ... ok
test tests::gcd_expected_values_from_given_validation_table ... ok
test tests::gcd_kulikov_lecture_examples ... ok
test tests::gcd_swap_argument_order ... ok
test tests::inverse_mod_selected_examples_from_happy_path ... ok
test tests::is_probably_prime_returns_false_for_1 ... ignored
test tests::is_probably_prime_returns_true_for_2 ... ok
test tests::is_probably_prime_well_studied_fermat_numbers ... ok
test tests::is_probably_prime_returns_true_for_carmichael_numbers ... ok
test tests::lcm_expected_values_from_given_validation_table ... ok
test tests::sieve_of_eratosthenes_indexing_semantics ... ok
test tests::random_exponent_expected_postconditions_true_for_happy_path ... ok
test tests::sieve_to_primes_verify_max_100 ... ok
test tests::sieve_of_eratosthenes_max_semantics ... ok
test tests::totient_results_match_wikipedia_table ... ok

test result: ok. 17 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

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
