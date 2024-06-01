mod aes_struct;
mod utils;
mod consts;

use aes_struct::AES;

fn main() {
    let key = String::from("Thats my Kung Fu");
    let iv = String::from("This is Kung Fu");
    let input = String::from("Two One Nine Two");
    let mut matrix_key: [[u8; 4]; 4] = [[0; 4]; 4];
    let mut matrix_iv: [[u8; 4]; 4] = [[0; 4]; 4];
    let mut matrix_input: [[u8; 4]; 4] = [[0; 4]; 4];

    for (i, c) in key.chars().enumerate() {
        matrix_key[i / 4][i % 4] = c as u8;
    }
    for (i, c) in iv.chars().enumerate() {
        matrix_iv[i / 4][i % 4] = c as u8;
    }
    for (i, c) in input.chars().enumerate() {
        matrix_input[i / 4][i % 4] = c as u8;
    }
    println!("{:?}", matrix_input);

    let aes = AES::new(matrix_key, matrix_iv);
    aes.encrypt_block(&mut matrix_input);
    println!("{:?}", matrix_input);

    aes.decrypt_block(&mut matrix_input);
    println!("{:?}", matrix_input);
}
