#![allow(unused)]
#![allow(non_snake_case)]
use tfhe::array::Slicing;
//use tfhe::shortint::parameters::v1_0::V1_0_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64;
use tfhe::shortint::prelude::*;
use tfhe::{prelude::*, CpuFheUint8Array, FheUint, FheUint8Id};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8, FheInt8, FheInt8Id};
use tfhe::shortint::parameters::DynamicDistribution;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();
    //println!("{:?}", config);

    // Key generation
    //let params = ClassicPBSParameters {
        //lwe_dimension: LweDimension(879),
        //glwe_dimension: GlweDimension(1),
        //polynomial_size: PolynomialSize(2048),
        //lwe_noise_distribution: DynamicDistribution::new_t_uniform(46),
        //glwe_noise_distribution: DynamicDistribution::new_t_uniform(17),
        //pbs_base_log: DecompositionBaseLog(23),
        //pbs_level: DecompositionLevelCount(1),
        //ks_base_log: DecompositionBaseLog(3),
        //ks_level: DecompositionLevelCount(5),
        //message_modulus: MessageModulus(4),
        //carry_modulus: CarryModulus(4),
        //max_noise_level: MaxNoiseLevel::new(5),
        //log2_p_fail: -71.625,
        //ciphertext_modulus: CiphertextModulus::new_native(),
        //encryption_key_choice: EncryptionKeyChoice::Big,
        //modulus_switch_noise_reduction_params: None,
    //};
    let (client_key, server_key) = generate_keys(config);//gen_keys(params);
    //println!("{}", client_key.parameters.message_modulus().0);
    set_server_key(server_key);
    let x = 10u8;
    let y = 4u8;
    let mut x_enc = FheUint8::encrypt(x, &client_key);
    //let x1_enc = x_enc.scalar_add(1u8);
    x_enc += 2u8;
    let x_dec: u8 = x_enc.decrypt(&client_key);
    println!("{}",x_dec);
    //let x_add_y = server_key.scalar_add(&x_enc, y);
    //let xaddy = client_key.decrypt(&x_add_y);
    //println!("10 + 4 = {}", xaddy);
    // On the server side:
    //set_server_key(server_keys);

    // problem parameter
    //const n = 7;
    //const m = 9;
    //const u = 1;
    //const v = 9;
    //const sO = 5;
    //const sE = 3;
    Ok(())
}
