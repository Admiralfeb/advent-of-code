use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;

use common::impl_day;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(4, YEAR, "day04.txt", {
    puzzle1: |_day: &Day, _path: &Path| {
        let string = "yzbqklnj";
        let start = Instant::now();
        let result = Day::calc_hash(&string, 5);
        println!("Day 4 Puzzle 1 took: {:?}", start.elapsed());
        Ok(result)
    },
    puzzle2: |_day: &Day, _path: &Path| {
        let string = "yzbqklnj";
        let start = Instant::now();
        let result = Day::calc_hash(&string, 6);
        println!("Day 4 Puzzle 2 took: {:?}", start.elapsed());
        Ok(result)
    },
});

impl Day {
    fn calc_hash(input: &str, num_zeros: usize) -> i64 {
        // Determine how many leading bytes need to be zero
        // 5 zeros = 2.5 bytes (first 2 bytes must be 0, third byte < 16)
        // 6 zeros = 3 bytes (first 3 bytes must be 0)
        let full_zero_bytes = num_zeros / 2;
        let half_byte = num_zeros % 2 == 1;

        // Use parallel search with rayon
        // Search in chunks to avoid too many threads
        const CHUNK_SIZE: i64 = 100_000;

        (0..)
            .step_by(CHUNK_SIZE as usize)
            .find_map(|start| {
                (start..start + CHUNK_SIZE)
                    .into_par_iter()
                    .find_first(|&i| {
                        let digest = md5::compute(format!("{}{}", input, i));
                        let bytes = digest.0;

                        // Check full zero bytes
                        for &byte in bytes.iter().take(full_zero_bytes) {
                            if byte != 0 {
                                return false;
                            }
                        }

                        // Check half byte if needed (must be < 16, i.e., high nibble is 0)
                        if half_byte {
                            if bytes[full_zero_bytes] >= 16 {
                                return false;
                            }
                        }

                        true
                    })
            })
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdef" => 609043)]
    #[test_case("pqrstuv" => 1048970)]
    fn test_calc_hash(input: &str) -> i64 {
        Day::calc_hash(input, 5)
    }
}
