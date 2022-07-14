use num_bigint::BigInt;

use secp256kfun::{marker::*, Scalar, Point,G,g};


// Rust Vec<u8> to [u8; 32] conversion
fn vec_to_u8_32(vec: &Vec<u8>) -> [u8; 32] {
    let mut array = [0u8; 32];
    array.copy_from_slice(&vec[..]);
    array
}


fn main() {
    let p1_x = BigInt::parse_bytes(b"36034668029310999675425029227919426304128362788024891102120850317866231552679", 10).unwrap();
    let p1_y = BigInt::parse_bytes(b"81120990977494636963407451456021843404486499021598452981689548730055179196713", 10).unwrap();

    let x_big_endian = p1_x.to_biguint().unwrap().to_bytes_be();
    let y_big_endian = p1_y.to_biguint().unwrap().to_bytes_be();




    // P1 P2 没有写出来具体的uncompressed point的encode格式，是因为感觉时间上来不及看文档和查资料了，随便查一下就浪费不少时间，但是0x04肯定是65个字节的前缀，
    // 后64个字节前32个字节和后32个字节分别编码x和y坐标

    // 最后还是做出来了。
    let mut buf: [u8; 65]  = [0x00; 65];
    buf[0] = 0x04;
    buf[1..33].copy_from_slice(&vec_to_u8_32(&x_big_endian));
    buf[33..65].copy_from_slice(&vec_to_u8_32(&y_big_endian));
    let p1 = Point::from_bytes_uncompressed(buf).unwrap();

     

    let neg_p1 = g!(-p1);
   
    
    
    println!("neg P1 is {}", neg_p1.to_string());

    let p2_x = BigInt::parse_bytes(b"17178020516540951919986460933710490672232047574774824837208169858689311129064", 10).unwrap();
    let p2_y = BigInt::parse_bytes(b"71957217096292920627957410906773462576199313707110833846387209016083557649656", 10).unwrap();

    let x_big_endian = p2_x.to_biguint().unwrap().to_bytes_be();
    let y_big_endian = p2_y.to_biguint().unwrap().to_bytes_be();

    let mut buf: [u8; 65]  = [0x00; 65];
    buf[0] = 0x04;
    buf[1..33].copy_from_slice(&vec_to_u8_32(&x_big_endian));
    buf[33..65].copy_from_slice(&vec_to_u8_32(&y_big_endian));
   
    
    let p2 = Point::from_bytes_uncompressed(buf).unwrap();

    let p1_add_p2 = g!(p1 + p2).normalize();
    
    // println!("P1 + P2 is {}", p1_add_p2);

    let scalar_bigint = BigInt::parse_bytes(b"112722522736802425171074620119739342837016662713926899217486478633056306669418", 10);
    let big_end = scalar_bigint.unwrap().to_biguint().unwrap().to_bytes_be();
    // P1 * 112722522736802425171074620119739342837016662713926899217486478633056306669418
    let mut buf: [u8; 32] = [0x00; 32];
    buf.copy_from_slice(&vec_to_u8_32(&big_end));
    let scalar =Scalar::from_bytes(buf).unwrap();
    let scalr_multi = g!(scalar * p1).normalize();
    // println!("P1 * scalar is: {}", scalr_multi.to_string());
    
    
}

