use crate::constants::{
    BLS_MODULUS_BYTES, INVERSE_WIDTH_BYTES, PRIMITIVE_ROOTS_OF_UNITY, ROOTS_OF_UNITY_BRP_BYTES,
};
use bls12_381::Scalar;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::error::Error;

fn compute_powers(x: &Scalar, n: u64) -> Vec<Scalar> {
    let mut current_power = Scalar::one();
    let mut powers = Vec::with_capacity(n.try_into().unwrap());

    for _ in 0..n {
        powers.push(current_power);
        current_power = current_power * x;
    }

    powers
}

fn compute_roots_of_unity(order: u64) -> Vec<Scalar> {
    let bls_modulus: BigUint = BigUint::parse_bytes(BLS_MODULUS_BYTES, 10).unwrap();
    let bls_modulus_minus_one = &bls_modulus - BigUint::one();
    let order_biguint = BigUint::from(order);
    assert!(&bls_modulus_minus_one % &order_biguint == BigUint::zero());

    let primitive = BigUint::from(PRIMITIVE_ROOTS_OF_UNITY);
    let exponent = &bls_modulus_minus_one.div_floor(&order_biguint);

    let roots_of_unity_biguint = primitive.modpow(&exponent, &bls_modulus);

    let roots_of_unity_bytes: [u8; 32] = roots_of_unity_biguint.to_bytes_le().try_into().unwrap();
    let roots_of_unity = Scalar::from_bytes(&roots_of_unity_bytes).unwrap();
    compute_powers(&roots_of_unity, order)
}

fn calculate_inverted_width(width: usize) -> Scalar {
    let mut width_bytes = [0u8; 32];
    width_bytes[..std::mem::size_of::<usize>()].copy_from_slice(&width.to_le_bytes());
    let scalar_bytes = Scalar::from_bytes(&width_bytes).unwrap();
    scalar_bytes.invert().unwrap()
}

fn bit_length(n: usize) -> u32 {
    usize::BITS - n.leading_zeros() - 1
}

fn reverse_bits(n: usize, order: usize) -> usize {
    let order_bit_length = bit_length(order);

    let bin_str = format!(
        "{:0width$b}",
        n,
        width = order_bit_length.try_into().unwrap()
    );
    let reversed_bin_str: String = bin_str.chars().rev().collect();
    usize::from_str_radix(&reversed_bin_str, 2).unwrap()
}

fn bit_reversal_permutation(sequence: [Scalar; 4096]) -> [Scalar; 4096] {
    let n = sequence.len();
    let new_sequence = sequence
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let reversed_index = reverse_bits(i, n);
            sequence[reversed_index].clone()
        })
        .collect::<Vec<_>>();

    new_sequence.try_into().unwrap()
}

fn get_width(width: usize, use_precomputed_values: bool) -> (usize, Scalar) {
    if use_precomputed_values == true {
        let inverse_width_scalar = Scalar::from_bytes(&INVERSE_WIDTH_BYTES).unwrap();
        (width, inverse_width_scalar)
    } else {
        let inverse_width = calculate_inverted_width(width);
        (width, inverse_width)
    }
}

fn get_roots_of_unity_brp(width: usize, use_precomputed_values: bool) -> [Scalar; 4096] {
    if use_precomputed_values == true {
        let mut precalculated: [Scalar; 4096] = std::array::from_fn(|_| Scalar::zero());
        for (i, single_root) in ROOTS_OF_UNITY_BRP_BYTES.iter().enumerate() {
            precalculated[i] = Scalar::from_bytes(&single_root).unwrap();
        }

        precalculated
    } else {
        let roots_of_unity = compute_roots_of_unity(width.try_into().unwrap());
        let roots_of_unity_brp: [Scalar; 4096] =
            bit_reversal_permutation(roots_of_unity.try_into().unwrap());

        roots_of_unity_brp
    }
}

pub fn evaluate(
    blob_bytes: [u8; 131072],
    mut z_bytes: [u8; 32],
    use_precomputed_values: bool,
) -> Result<Scalar, Box<dyn Error>> {
    let mut blob_scalars: [Scalar; 4096] = std::array::from_fn(|_| Scalar::zero());

    for (i, chunk) in blob_bytes.chunks(32).into_iter().enumerate() {
        let mut chunk_array: [u8; 32] = chunk.try_into().expect("Chunk size is not 32 bytes");
        chunk_array.reverse();
        blob_scalars[i] = Scalar::from_bytes(&chunk_array).unwrap();
    }

    let (width, inverse_width) = get_width(blob_scalars.len(), use_precomputed_values);
    let roots_of_unity_brp = get_roots_of_unity_brp(width, use_precomputed_values);

    z_bytes.reverse();
    let z_scalar = Scalar::from_bytes(&z_bytes).unwrap();

    if let Some(eval_index) = roots_of_unity_brp.iter().position(|&root| root == z_scalar) {
        Ok(blob_scalars[eval_index])
    } else {
        let mut result = Scalar::zero();
        for i in 0..width {
            let a = blob_scalars[i] * roots_of_unity_brp[i];
            let b = z_scalar - roots_of_unity_brp[i];
            result += a * b.invert().unwrap();
        }

        let r = z_scalar.pow_vartime(&[width as u64, 0, 0, 0]) - Scalar::one();
        result = result * r * inverse_width;
        Ok(result)
    }
}
