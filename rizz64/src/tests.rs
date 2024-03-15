#[cfg(test)]
mod tests {
    use crate::{Rizz64, MAX_LEN_64};

    fn test_rizz64<T>(
        buf: &mut [u8],
        value: T,
        put: fn(&mut [u8], T) -> Result<usize, &str>,
        decode: fn(&[u8]) -> Result<(T, usize), &'static str>,
        size: fn(T) -> usize,
    ) where
        T: Eq + std::fmt::Debug + Copy,
    {
        if let Ok(n1) = put(buf, value) {
            if let Ok((expected, n2)) = decode(&buf[..n1]) {
                assert_eq!(value, expected);
                assert_eq!(n1, n2);
                assert_eq!(n1, size(value));
            }
        }
    }


    const TESTS: [i64; 18] = [
        -1 << 63,
        (-1 << 63) + 1,
        -1,
        0,
        1,
        2,
        10,
        20,
        63,
        64,
        65,
        127,
        128,
        129,
        255,
        256,
        257,
        1 << 63 - 1,
    ];

    #[test]
    fn test_ubasic() {
        for test in TESTS {
            let mut test = test;
            if test < 0 {
                test = !test;
            }

            let mut buf = [0u8; MAX_LEN_64];
            test_rizz64::<u64>(&mut buf, test as u64, Rizz64::write_u64, Rizz64::read_u64, Rizz64::size_u64);
        }
    }

    #[test]
    fn test_ibasic() {
        for test in TESTS {
            let mut buf = [0u8; MAX_LEN_64];
            test_rizz64::<i64>(&mut buf, test, Rizz64::write_i64, Rizz64::read_i64, Rizz64::size_i64);
        }
    }
}
