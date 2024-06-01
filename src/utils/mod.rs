use crate::consts::*;

pub fn generate_expanded_keys(key: [[u8; 4]; 4]) -> [[[u8; 4]; 4]; 10] {
    let mut state = key.clone();
    let mut key_exp = key.clone();
    let mut expanded_keys: [[[u8; 4]; 4]; 10] = [[[0; 4]; 4]; 10];

    for j in 0..10 {
        state[3].rotate_left(1);
        for i in 0..4 {
            state[3][i] = S_BOX[state[3][i] as usize];
            state[3][i] ^= ROUND_CONSTS[j][i];

            key_exp[0][i] = key_exp[0][i] ^ state[3][i];
            key_exp[1][i] = key_exp[0][i] ^ state[1][i];
            key_exp[2][i] = key_exp[1][i] ^ state[2][i];
            key_exp[3][i] = key_exp[2][i] ^ key_exp[3][i];
        }
        state = key_exp;
        expanded_keys[j] = key_exp;
    }
    expanded_keys
}

pub fn add_round_key(input: &mut [[u8; 4]; 4], round_key: [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            input[i][j] ^= round_key[i][j];
        }
    }
}

pub fn sub_bytes(input: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            input[i][j] = S_BOX[input[i][j] as usize];
        }
    }
}

pub fn sub_bytes_inv(input: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            input[i][j] = INV_S_BOX[input[i][j] as usize];
        }
    }
}

pub fn shift_rows(input: &mut [[u8; 4]; 4]) {
    for i in 1..4 {
        input[i].rotate_left(i);
    }
}

pub fn shift_rows_inv(input: &mut [[u8; 4]; 4]) {
    for i in 1..4 {
        input[i].rotate_right(i);
    }
}

pub fn mix_columns(input: &mut [[u8; 4]; 4]) {
    let mut result: [[u8; 4]; 4] = [[0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result[i][j] =
                gf_mul(MIXX[i][0], input[0][j]) ^
                gf_mul(MIXX[i][1], input[1][j]) ^
                gf_mul(MIXX[i][2], input[2][j]) ^
                gf_mul(MIXX[i][3], input[3][j]);
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            input[i][j] = result[i][j];
        }
    }
}

pub fn mix_columns_inv(input: &mut [[u8; 4]; 4]) {
    let mut result: [[u8; 4]; 4] = [[0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result[i][j] =
                gf_mul(MIXX_INV[i][0], input[0][j]) ^
                gf_mul(MIXX_INV[i][1], input[1][j]) ^
                gf_mul(MIXX_INV[i][2], input[2][j]) ^
                gf_mul(MIXX_INV[i][3], input[3][j]);
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            input[i][j] = result[i][j];
        }
    }
}

pub fn transpose_matrix(matrix: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in i + 1..4 {
            matrix[i][j] ^= matrix[j][i];
            matrix[j][i] ^= matrix[i][j];
            matrix[i][j] ^= matrix[j][i];
        }
    }
}

fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0;
    let mut hi_bit_set;

    for _ in 0..8 {
        if (b & 1) == 1 {
            p ^= a;
        }
        hi_bit_set = a & 0x80;
        a <<= 1;
        if hi_bit_set == 0x80 {
            a ^= 0x1b; // Irreducible polynomial for AES
        }
        b >>= 1;
    }
    p
}
