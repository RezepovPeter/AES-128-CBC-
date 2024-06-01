use crate::utils::*;

pub struct AES {
    pub key: [[u8; 4]; 4],
    pub round_keys: [[[u8; 4]; 4]; 10],
    pub iv: [[u8; 4]; 4],
}

impl AES {
    pub fn new(key: [[u8; 4]; 4], iv: [[u8; 4]; 4]) -> Self {
        let mut init_key = key.clone();
        transpose_matrix(&mut init_key);

        let mut init_round_keys = generate_expanded_keys(key);
        for i in 0..init_round_keys.len() {
            transpose_matrix(&mut init_round_keys[i]);
        }
        Self {
            key: init_key,
            round_keys: init_round_keys,
            iv,
        }
    }

    pub fn encrypt_block(&self, input: &mut [[u8; 4]; 4]) {
        transpose_matrix(input);
        add_round_key(input, self.key);
        for i in 0..9 {
            sub_bytes(input);
            shift_rows(input);
            mix_columns(input);
            add_round_key(input, self.round_keys[i]);
        }
        sub_bytes(input);
        shift_rows(input);
        add_round_key(input, self.round_keys[9]);
    }

    pub fn decrypt_block(&self, input: &mut [[u8; 4]; 4]) {
        add_round_key(input, self.round_keys[9]);
        for i in (0..9).rev() {
            shift_rows_inv(input);
            sub_bytes_inv(input);
            add_round_key(input, self.round_keys[i]);
            mix_columns_inv(input);
        }
        shift_rows_inv(input);
        sub_bytes_inv(input);
        add_round_key(input, self.key);
        transpose_matrix(input);
    }
}
