use anchor_lang::prelude::*;
use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY_PUBLIC_PROGRAM_TRANSACTION2_IN2_OUT_MAIN: Groth16Verifyingkey =
    Groth16Verifyingkey {
        nr_pubinputs: 14,
        vk_alpha_g1: [
            45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51,
            251, 177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60,
            55, 206, 176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240,
            118, 202, 255, 0, 77, 25, 38,
        ],

        vk_beta_g2: [
            9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132,
            128, 166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173,
            76, 121, 131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36,
            30, 2, 19, 188, 127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245,
            232, 71, 217, 63, 140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17,
            35, 70, 167, 23, 57, 193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146,
            248, 150, 183, 198, 62, 234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
        ],

        vk_gamme_g2: [
            25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73,
            51, 53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31,
            30, 118, 66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70,
            222, 189, 92, 217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173,
            105, 12, 51, 149, 188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151,
            91, 18, 200, 94, 165, 219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227,
            209, 231, 105, 12, 67, 211, 123, 76, 230, 204, 1, 102, 250, 125, 170,
        ],

        vk_delta_g2: [
            21, 140, 98, 10, 135, 137, 82, 49, 240, 178, 254, 18, 64, 213, 253, 43, 50, 7, 205,
            221, 35, 250, 156, 129, 197, 117, 108, 12, 150, 73, 208, 52, 35, 69, 193, 65, 248, 246,
            4, 106, 229, 231, 209, 227, 72, 254, 119, 14, 206, 55, 115, 163, 0, 252, 190, 151, 133,
            89, 108, 238, 212, 125, 243, 89, 28, 75, 51, 58, 54, 54, 103, 150, 118, 224, 251, 53,
            10, 119, 243, 75, 39, 109, 52, 54, 197, 133, 103, 82, 131, 239, 94, 110, 242, 211, 226,
            73, 15, 130, 152, 75, 228, 149, 10, 218, 95, 200, 25, 41, 174, 27, 87, 248, 71, 138,
            228, 159, 33, 81, 219, 101, 162, 251, 202, 192, 25, 100, 207, 177,
        ],

        vk_ic: &[
            [
                40, 64, 51, 17, 212, 168, 7, 208, 77, 57, 144, 115, 117, 67, 244, 111, 208, 149,
                22, 244, 211, 72, 237, 57, 56, 39, 123, 204, 55, 124, 194, 84, 2, 106, 231, 179,
                213, 218, 49, 100, 62, 149, 72, 127, 170, 154, 154, 219, 254, 79, 184, 110, 112,
                175, 145, 61, 64, 21, 75, 53, 43, 150, 18, 32,
            ],
            [
                38, 198, 240, 210, 162, 83, 203, 55, 13, 246, 207, 181, 248, 39, 122, 147, 18, 40,
                218, 56, 232, 18, 197, 61, 53, 125, 72, 32, 204, 187, 134, 210, 8, 59, 30, 225,
                166, 131, 132, 175, 124, 23, 226, 51, 141, 45, 49, 112, 62, 72, 201, 232, 145, 68,
                167, 239, 235, 69, 31, 27, 204, 55, 13, 41,
            ],
            [
                33, 186, 251, 12, 45, 163, 17, 95, 127, 25, 28, 89, 183, 95, 41, 76, 10, 203, 134,
                228, 225, 221, 28, 35, 134, 178, 121, 133, 195, 46, 195, 131, 26, 181, 192, 49,
                207, 24, 237, 122, 211, 117, 208, 36, 41, 161, 179, 102, 145, 161, 29, 16, 5, 226,
                68, 148, 48, 160, 129, 173, 94, 116, 108, 188,
            ],
            [
                37, 185, 179, 130, 218, 101, 120, 119, 234, 86, 196, 82, 216, 143, 168, 172, 59,
                250, 90, 92, 65, 33, 111, 15, 134, 34, 83, 57, 140, 236, 42, 222, 42, 94, 233, 47,
                206, 25, 100, 190, 245, 9, 139, 200, 214, 68, 118, 101, 30, 83, 255, 136, 70, 148,
                68, 140, 240, 212, 244, 155, 247, 191, 4, 113,
            ],
            [
                14, 102, 255, 73, 49, 182, 176, 17, 235, 84, 3, 22, 22, 155, 231, 154, 39, 135,
                151, 93, 154, 14, 132, 35, 109, 150, 247, 185, 183, 20, 77, 48, 30, 214, 169, 85,
                23, 198, 146, 23, 92, 181, 148, 101, 72, 186, 238, 26, 156, 147, 194, 164, 107, 80,
                148, 71, 190, 52, 246, 74, 78, 49, 138, 28,
            ],
            [
                34, 225, 156, 20, 153, 49, 41, 60, 29, 238, 83, 69, 103, 98, 154, 191, 60, 130,
                244, 164, 60, 158, 53, 177, 23, 146, 171, 122, 233, 54, 76, 202, 40, 133, 8, 28,
                238, 63, 30, 96, 121, 30, 203, 194, 39, 234, 64, 191, 176, 91, 225, 27, 103, 225,
                118, 106, 148, 107, 235, 184, 174, 244, 58, 0,
            ],
            [
                47, 224, 155, 3, 249, 111, 252, 70, 10, 1, 91, 136, 34, 173, 243, 112, 252, 70,
                136, 203, 49, 34, 106, 218, 13, 91, 129, 205, 186, 219, 39, 161, 33, 101, 102, 10,
                43, 148, 161, 225, 58, 157, 200, 169, 91, 159, 66, 124, 196, 104, 182, 29, 32, 182,
                238, 4, 120, 136, 148, 111, 117, 152, 167, 14,
            ],
            [
                1, 100, 40, 28, 148, 80, 115, 99, 0, 129, 196, 190, 41, 234, 224, 239, 187, 64, 33,
                195, 100, 245, 163, 221, 140, 23, 23, 46, 120, 251, 59, 17, 13, 161, 41, 227, 240,
                105, 142, 139, 45, 75, 16, 148, 156, 142, 61, 235, 63, 209, 235, 116, 40, 45, 4,
                159, 12, 233, 233, 249, 63, 94, 236, 33,
            ],
            [
                45, 209, 100, 30, 103, 0, 69, 15, 202, 255, 95, 154, 56, 172, 208, 170, 201, 199,
                29, 117, 162, 197, 197, 169, 103, 87, 133, 127, 156, 189, 179, 123, 45, 114, 183,
                56, 41, 16, 174, 74, 193, 136, 194, 216, 114, 127, 182, 179, 168, 206, 102, 202,
                245, 224, 178, 77, 224, 70, 248, 55, 206, 220, 201, 153,
            ],
            [
                20, 12, 70, 84, 141, 97, 170, 62, 155, 202, 70, 110, 30, 10, 38, 144, 127, 68, 59,
                212, 199, 241, 147, 105, 62, 93, 21, 219, 186, 55, 169, 85, 5, 43, 171, 155, 150,
                225, 233, 51, 80, 190, 196, 163, 150, 190, 77, 224, 77, 52, 86, 148, 2, 170, 12,
                215, 65, 236, 248, 142, 172, 146, 205, 226,
            ],
            [
                22, 230, 2, 209, 238, 243, 21, 67, 114, 150, 51, 168, 69, 52, 11, 58, 250, 6, 142,
                88, 196, 81, 208, 55, 142, 165, 38, 144, 240, 178, 39, 53, 35, 78, 49, 153, 194,
                52, 127, 186, 38, 197, 127, 230, 251, 7, 219, 57, 188, 207, 162, 108, 212, 7, 164,
                86, 79, 138, 121, 70, 114, 216, 139, 29,
            ],
            [
                31, 149, 203, 133, 161, 63, 142, 113, 106, 21, 33, 89, 36, 98, 63, 220, 81, 206,
                213, 9, 7, 250, 105, 77, 152, 37, 76, 102, 92, 194, 81, 27, 2, 93, 51, 68, 245,
                157, 13, 183, 180, 81, 32, 135, 108, 209, 230, 207, 180, 170, 100, 57, 247, 89, 79,
                61, 213, 235, 110, 225, 238, 227, 85, 227,
            ],
            [
                31, 121, 46, 91, 241, 7, 99, 155, 130, 5, 175, 92, 51, 103, 37, 227, 97, 34, 64,
                205, 41, 9, 59, 248, 198, 99, 199, 51, 77, 77, 249, 223, 38, 234, 242, 128, 114,
                14, 110, 179, 232, 48, 209, 180, 100, 239, 88, 41, 82, 18, 32, 105, 200, 88, 32,
                51, 205, 100, 139, 26, 148, 15, 219, 202,
            ],
            [
                26, 234, 68, 175, 170, 112, 13, 222, 143, 23, 227, 95, 104, 195, 24, 220, 142, 225,
                156, 30, 176, 19, 108, 164, 121, 115, 197, 39, 78, 176, 34, 192, 47, 147, 59, 254,
                231, 20, 35, 105, 191, 247, 18, 8, 11, 190, 103, 210, 212, 212, 238, 215, 176, 109,
                21, 93, 231, 32, 62, 128, 219, 195, 217, 6,
            ],
            [
                22, 199, 88, 102, 126, 229, 134, 2, 191, 190, 224, 73, 153, 65, 243, 74, 87, 155,
                50, 136, 69, 33, 37, 32, 100, 186, 194, 78, 55, 208, 210, 89, 13, 67, 26, 109, 50,
                127, 90, 228, 35, 110, 127, 121, 127, 125, 2, 190, 236, 90, 116, 168, 169, 17, 93,
                202, 1, 53, 67, 76, 220, 133, 42, 22,
            ],
        ],
    };
#[account]
pub struct ZKpublicProgramTransaction2In2OutMainProofInputs {
    public_state_root: [u8; 2],
    public_amount_spl: u8,
    public_data_hash: u8,
    public_amount_sol: u8,
    public_mint_public_key: u8,
    public_in_utxo_hash: [u8; 2],
    public_out_utxo_hash: [u8; 2],
    public_new_address: [u8; 2],
    public_in_utxo_data_hash: [u8; 2],
    public_program_id: u8,
    public_transaction_hash: u8,
    asset_public_keys: [u8; 3],
    private_public_data_hash: u8,
    is_in_program_utxo: [u8; 2],
    in_owner: [u8; 2],
    in_amount: [[u8; 2]; 2],
    in_private_key: [u8; 2],
    in_blinding: [u8; 2],
    in_data_hash: [u8; 2],
    leaf_index: [u8; 2],
    merkle_proof: [[u8; 22]; 2],
    in_indices: [[[u8; 3]; 2]; 2],
    nullifier_leaf_index: [u8; 2],
    nullifier_merkle_proof: [[u8; 22]; 2],
    out_amount: [[u8; 2]; 2],
    out_owner: [u8; 2],
    out_blinding: [u8; 2],
    out_data_hash: [u8; 2],
    out_indices: [[[u8; 3]; 2]; 2],
    meta_hash: [u8; 2],
    is_meta_hash_utxo: [u8; 2],
    in_address: [u8; 2],
    is_in_address: [u8; 2],
    is_new_address: [u8; 2],
    is_out_program_utxo: [u8; 2],
}
#[account]
pub struct ZKpublicProgramTransaction2In2OutMainPublicInputs {
    public_state_root: [u8; 2],
    public_amount_spl: u8,
    public_data_hash: u8,
    public_amount_sol: u8,
    public_mint_public_key: u8,
    public_in_utxo_hash: [u8; 2],
    public_out_utxo_hash: [u8; 2],
    public_new_address: [u8; 2],
    public_in_utxo_data_hash: [u8; 2],
}
#[account]
pub struct InstructionDataLightInstructionPublicProgramTransaction2In2OutMainSecond {
    public_state_root: [[u8; 32]; 2],
    public_amount_spl: [u8; 32],
    public_data_hash: [u8; 32],
    public_amount_sol: [u8; 32],
    public_mint_public_key: [u8; 32],
    public_in_utxo_hash: [[u8; 32]; 2],
    public_out_utxo_hash: [[u8; 32]; 2],
    public_new_address: [[u8; 32]; 2],
    public_in_utxo_data_hash: [[u8; 32]; 2],
}