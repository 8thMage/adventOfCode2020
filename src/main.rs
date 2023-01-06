use std::arch::x86_64::{__m128i, _mm_add_epi64};
use std::{
    arch::x86_64::{_mm_clmulepi64_si128, _mm_extract_epi64, _mm_set1_epi64x, _mm_set_epi64x},
    collections::{HashMap, HashSet},
    thread::current,
};

fn main() {
    let time = std::time::Instant::now();
    let seive_size = 1 << 27;
    let mut seive = vec![false; seive_size];
    seive[0] = true;
    seive[1] = true;
    let mut count = 0;
    for i in 2..seive_size {
        if seive[i] == false {
            count += 1;
            // println!("{:b}", i);
            if count == 5_000_000 {
                let duration = time.elapsed().as_millis();
                println!("5_000_000th prime {}", i);
                println!("time {}", duration);
            }
            let mut j = 2;
            unsafe {
                let i_vec = _mm_set1_epi64x(i as i64);
                let mut j_vec = _mm_set_epi64x(2, 3);
                let one_vec = _mm_set1_epi64x(2);

                loop {
                    let index = unsafe {
                        _mm_extract_epi64(_mm_clmulepi64_si128::<0>(i_vec, j_vec), 0) as u64
                    };
                    if index as usize >= seive.len() {
                        break;
                    }
                    seive[index as usize] = true;

                    let index = unsafe {
                        _mm_extract_epi64(_mm_clmulepi64_si128::<1>(i_vec, j_vec), 0) as u64
                    };
                    if index as usize >= seive.len() {
                        break;
                    }
                    seive[index as usize] = true;
                    j_vec = _mm_add_epi64(j_vec, one_vec);
                }
            }
        }
    }
    println!("count {}", count);
    let x = 0;
}

fn carry_less_mul_clmul(mut a: __m128i, mut b: __m128i) -> u64 {
    unsafe { _mm_extract_epi64(_mm_clmulepi64_si128::<0>(a, b), 0) as u64 }
}

fn carry_less_mul(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        return carry_less_mul(b, a);
    }
    let mut sum = 0;
    while b != 0 {
        if b % 2 != 0 {
            sum ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    sum
}
