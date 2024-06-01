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

    fn encrypt_block(&self, input: &mut [[u8; 4]; 4]) {
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

    fn decrypt_block(&self, input: &mut [[u8; 4]; 4]) {
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

    pub fn encrypt(&self, input: &Vec<u8>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut prev = self.iv;
        for chunk in input.chunks(16) {
            let mut matrix_input: [[u8; 4]; 4] = [[0; 4]; 4];
            for (i, c) in chunk.iter().enumerate() {
                matrix_input[i / 4][i % 4] = *c;
            }

            for i in 0..4 {
                for j in 0..4 {
                    matrix_input[i][j] ^= prev[i][j];
                }
            }
            self.encrypt_block(&mut matrix_input);
            prev = matrix_input.clone();

            for i in 0..4 {
                for j in 0..4 {
                    result.push(matrix_input[i][j]);
                }
            }
        }
        result
    }

    pub fn decrypt(&self, input: &Vec<u8>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut prev = self.iv;
        for chunk in input.chunks(16) {
            let mut matrix_input: [[u8; 4]; 4] = [[0; 4]; 4];
            for (i, c) in chunk.iter().enumerate() {
                matrix_input[i / 4][i % 4] = *c;
            }

            let original_matrix_input = matrix_input.clone();

            self.decrypt_block(&mut matrix_input);
            for i in 0..4 {
                for j in 0..4 {
                    matrix_input[i][j] ^= prev[i][j];
                }
            }
            prev = original_matrix_input;

            for i in 0..4 {
                for j in 0..4 {
                    result.push(matrix_input[i][j]);
                }
            }
        }
        result
    }
}
