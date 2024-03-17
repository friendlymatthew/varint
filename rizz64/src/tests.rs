#[cfg(test)]
mod tests {
    use crate::{Error, Rizz128, MAX_LEN_64};

    fn test_rizz64<T>(
        buf: &mut [u8],
        value: T,
        put: fn(&mut [u8], T) -> Result<usize, Error>,
        decode: fn(&[u8]) -> Result<(T, usize), Error>,
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

    fn test_pack_rizz64<T>(
        buf: &mut [u8],
        value: T,
        pack: fn(buf: &mut [u8], x: T) -> Result<usize, Error>,
        decode: fn(&[u8]) -> Result<(T, usize), Error>,
    ) where
        T: Eq + std::fmt::Debug + Copy,
    {
        if let Ok(len) = pack(buf, value) {
            assert_eq!(len, MAX_LEN_64);

            if let Ok((expected, _x)) = decode(buf) {
                assert_eq!(expected, value);
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
            test_rizz64::<u64>(
                &mut buf,
                test as u64,
                Rizz128::write_u64,
                Rizz128::read_u64,
                Rizz128::size_u64,
            );
        }
    }

    #[test]
    fn test_ibasic() {
        for test in TESTS {
            let mut buf = [0u8; MAX_LEN_64];
            test_rizz64::<i64>(
                &mut buf,
                test,
                Rizz128::write_i64,
                Rizz128::read_i64,
                Rizz128::size_i64,
            );
        }
    }

    #[test]
    fn test_upacker() {
        for test in TESTS {
            let mut test = test;
            if test < 0 {
                test = !test;
            }

            let mut buf = [0u8; MAX_LEN_64];
            test_pack_rizz64::<u64>(
                &mut buf,
                test as u64,
                Rizz128::write_max_u64,
                Rizz128::read_u64,
            )
        }
    }

    #[test]
    fn test_ipacker() {
        for test in TESTS {
            let mut buf = [0u8; MAX_LEN_64];
            test_pack_rizz64::<i64>(&mut buf, test, Rizz128::write_max_i64, Rizz128::read_i64)
        }
    }

    #[test]
    fn test_zig_zag() {
        for x in TESTS {
            let ux = Rizz128::zig_zag(x);
            assert_eq!(x, Rizz128::decode_zig_zag(ux));
        }
    }
}
