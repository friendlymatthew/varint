#[cfg(test)]
mod tests {
    use crate::{MAX_LEN_64, Rizz64};

    fn test_rizz64<T>(
        value: T,
        put: fn(T) -> Result<([u8; 10], usize), &'static str>,
        decode: fn(&[u8]) -> Result<(T, usize), &'static str>,
        size: fn(T) -> usize
    )
        where
            T: Eq + std::fmt::Debug + Copy,
    {
        if let Ok((buf, n1)) = put(value) {
            if let Ok((expected, n2)) = decode(&buf[..n1]) {
                assert_eq!(value, expected);
                assert_eq!(n1, n2);
                assert_eq!(n1, size(value));
            }
        }
    }


    fn test_append_rizz64<T>(
        value: T,
        append: fn(buf: &mut [u8], x: T) -> Result<usize, &'static str>,
        decode: fn(&[u8]) -> Result<(T, usize), &'static str>,
        size: fn(T) -> usize
    )
        where
            T: Eq + std::fmt::Debug + Copy,
    {
        let mut buf: [u8; MAX_LEN_64] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        if let Ok(n1) = append(&mut buf, value) {
            let trim_buf = &buf[..n1];
            if let Ok((expected, n2)) = decode(trim_buf) {
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
            test_rizz64::<u64>(
                test as u64,
                Rizz64::put_u64,
                Rizz64::u64,
                Rizz64::size_u64,
            );

            test_append_rizz64::<u64>(
                test as u64,
                Rizz64::append_u64,
                Rizz64::u64,
                Rizz64::size_u64
            );
        }
    }

    #[test]
    fn test_ibasic() {
        for test in TESTS {
            test_rizz64::<i64>(
                test,
                Rizz64::put_i64,
                Rizz64::i64,
                Rizz64::size_i64
            );

            test_append_rizz64::<i64>(
                test,
                Rizz64::append_i64,
                Rizz64::i64,
                Rizz64::size_i64
            );
        }
    }
}
