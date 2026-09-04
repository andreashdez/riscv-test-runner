use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{LitInt, parse_macro_input};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn hash_values(values: &[u32]) -> u64 {
    values
        .iter()
        .enumerate()
        .fold(FNV_OFFSET, |hash, (index, value)| {
            let indexed_value = u64::from(*value) | ((index as u64) << 32);
            (hash ^ indexed_value).wrapping_mul(FNV_PRIME)
        })
}

fn generated_checksum(seed: u32, length: usize) -> u64 {
    let mut state = seed;
    let mut hash = FNV_OFFSET;

    for index in 0..length {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let indexed_value = u64::from(state) | ((index as u64) << 32);
        hash = (hash ^ indexed_value).wrapping_mul(FNV_PRIME);
    }

    hash
}

fn heapless_values(seed: u32, length: usize) -> Vec<u32> {
    let mut state = seed;
    (0..length)
        .map(|_| {
            state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            state
        })
        .collect()
}

fn heap_values(seed: u32, length: usize) -> Vec<u32> {
    let mut state = seed;
    (0..length)
        .map(|_| {
            state = state.wrapping_mul(22_695_477).wrapping_add(1);
            state
        })
        .collect()
}

fn sorted_values(seed: u32, length: usize) -> Vec<u32> {
    let mut state = seed;
    let mut values: Vec<_> = (0..length)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state
        })
        .collect();
    values.sort_unstable();
    values
}

pub(super) fn expand(input: TokenStream) -> TokenStream {
    let count_literal = parse_macro_input!(input as LitInt);
    let count = match count_literal.base10_parse::<usize>() {
        Ok(count @ 1..=1_000) => count,
        Ok(_) => {
            return syn::Error::new(
                count_literal.span(),
                "case count must be between 1 and 1000",
            )
            .to_compile_error()
            .into();
        }
        Err(error) => {
            return syn::Error::new(count_literal.span(), error)
                .to_compile_error()
                .into();
        }
    };

    let mut tests = Vec::with_capacity(count * 6);

    for index in 0..count {
        let ordinal = (index + 1) as u32;

        let arithmetic_name = format_ident!("test_arithmetic_{:04}", ordinal);
        let a = ordinal.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let b = ordinal.rotate_left((ordinal % 31) + 1) ^ 0xa5a5_5a5a;
        let rotation = ordinal.wrapping_mul(7) % 32;
        let mask = 0x9e37_79b9_u32.wrapping_mul(ordinal);
        let arithmetic_expected = a.wrapping_mul(3).wrapping_add(b).rotate_left(rotation) ^ mask;
        tests.push(quote! {
            #[riscv_test_macros::riscv_test]
            fn #arithmetic_name() -> crate::testing::TestResult {
                run_arithmetic_case(#a, #b, #rotation, #mask, #arithmetic_expected)
            }
        });

        let bitwise_name = format_ident!("test_bitwise_{:04}", ordinal);
        let bitwise_value = ordinal
            .wrapping_mul(747_796_405)
            .wrapping_add(2_891_336_453);
        let bitwise_shift = ordinal.wrapping_mul(11) % 32;
        let bitwise_expected = bitwise_value.rotate_right(bitwise_shift)
            ^ bitwise_value.reverse_bits()
            ^ bitwise_value.count_ones().wrapping_mul(0x0101_0101);
        tests.push(quote! {
            #[riscv_test_macros::riscv_test]
            fn #bitwise_name() -> crate::testing::TestResult {
                run_bitwise_case(#bitwise_value, #bitwise_shift, #bitwise_expected)
            }
        });

        let checksum_name = format_ident!("test_checksum_{:04}", ordinal);
        let checksum_seed = ordinal ^ 0xc001_d00d;
        let checksum_length = 256 + (index * 251 % 3_841);
        let checksum_expected = generated_checksum(checksum_seed, checksum_length);
        tests.push(quote! {
            #[riscv_test_macros::riscv_test]
            fn #checksum_name() -> crate::testing::TestResult {
                run_checksum_case(#checksum_seed, #checksum_length, #checksum_expected)
            }
        });

        let heapless_name = format_ident!("test_heapless_{:04}", ordinal);
        let heapless_seed = ordinal ^ 0x1357_9bdf;
        let heapless_length = 8 + (index * 17 % 57);
        let heapless_expected = hash_values(&heapless_values(heapless_seed, heapless_length));
        tests.push(quote! {
            #[riscv_test_macros::riscv_test]
            fn #heapless_name() -> crate::testing::TestResult {
                run_heapless_case(#heapless_seed, #heapless_length, #heapless_expected)
            }
        });

        let heap_name = format_ident!("test_heap_{:04}", ordinal);
        let heap_seed = ordinal ^ 0x2468_ace0;
        let heap_length = 64 + (index * 257 % 4_033);
        let heap_expected = hash_values(&heap_values(heap_seed, heap_length));
        tests.push(quote! {
            #[riscv_test_macros::riscv_test]
            fn #heap_name() -> crate::testing::TestResult {
                run_heap_case(#heap_seed, #heap_length, #heap_expected)
            }
        });

        let sorting_name = format_ident!("test_sorting_{:04}", ordinal);
        let sorting_seed = ordinal ^ 0xdead_beef;
        let sorting_length = 32 + (index * 53 % 481);
        let sorting_expected = hash_values(&sorted_values(sorting_seed, sorting_length));
        tests.push(quote! {
            #[riscv_test_macros::riscv_test]
            fn #sorting_name() -> crate::testing::TestResult {
                run_sorting_case(#sorting_seed, #sorting_length, #sorting_expected)
            }
        });
    }

    quote!(#(#tests)*).into()
}
