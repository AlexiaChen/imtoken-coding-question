use num_bigint::BigInt;


fn main() {
    let p = BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();

    let element = BigInt::parse_bytes(b"37495995483093812530829120344068921073950778374277050857635845226183990889532", 10).unwrap();

    let pow = BigInt::parse_bytes(b"36273884976317350876892933450181613438664462160902682135941368945682163872771", 10).unwrap();

    let result = element.modpow(&pow, &p);
    println!("finite field compute result is: {}", result.to_str_radix(10));
}