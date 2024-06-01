mod aes_struct;
mod utils;
mod consts;

use aes_struct::AES;

fn main() {
    let key = String::from("Thats my KuTTTTT");
    let iv = String::from("This is KungGGG");
    let input = String::from("Лол это очен круто! One Nine TwoTwo One Nine TwoTwo One Nine Two");
    let mut matrix_key: [[u8; 4]; 4] = [[0; 4]; 4];
    let mut matrix_iv: [[u8; 4]; 4] = [[0; 4]; 4];

    for (i, c) in key.chars().enumerate() {
        matrix_key[i / 4][i % 4] = c as u8;
    }
    for (i, c) in iv.chars().enumerate() {
        matrix_iv[i / 4][i % 4] = c as u8;
    }

    let aes = AES::new(matrix_key, matrix_iv);
    let input_bytes = input.into_bytes();

    let encrypted = aes.encrypt(&input_bytes);

    println!("{:?}", encrypted);

    let decrypted_bytes = aes.decrypt(&encrypted);

    let decrypted_string = String::from_utf8(decrypted_bytes).expect("Invalid UTF-8 sequence");

    println!("{}", decrypted_string);
}
