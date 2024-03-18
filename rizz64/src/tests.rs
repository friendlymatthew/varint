#[cfg(test)]
mod tests {
    use crate::{Error, Rizz64, MAX_LEN_64};

    fn test_rizz64_roundtrip<T>(
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

    fn test_pack_rizz64_roundtrip<T>(
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
            test_rizz64_roundtrip::<u64>(
                &mut buf,
                test as u64,
                Rizz64::write_u64,
                Rizz64::read_u64,
                Rizz64::size_u64,
            );
        }

        for i in 0..1024 {
            let mut buf = [0u8; MAX_LEN_64];
            test_rizz64_roundtrip::<u64>(
                &mut buf,
                i as u64,
                Rizz64::write_u64,
                Rizz64::read_u64,
                Rizz64::size_u64,
            );
        }
    }

    #[test]
    fn test_ibasic() {
        for test in TESTS {
            let mut buf = [0u8; MAX_LEN_64];
            test_rizz64_roundtrip::<i64>(
                &mut buf,
                test,
                Rizz64::write_i64,
                Rizz64::read_i64,
                Rizz64::size_i64,
            );
        }

        for i in -1024..1024 {
            let mut buf = [0u8; MAX_LEN_64];
            test_rizz64_roundtrip::<i64>(
                &mut buf,
                i as i64,
                Rizz64::write_i64,
                Rizz64::read_i64,
                Rizz64::size_i64,
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
            test_pack_rizz64_roundtrip::<u64>(
                &mut buf,
                test as u64,
                Rizz64::write_max_u64,
                Rizz64::read_u64,
            )
        }
    }

    #[test]
    fn test_ipacker() {
        for test in TESTS {
            let mut buf = [0u8; MAX_LEN_64];
            test_pack_rizz64_roundtrip::<i64>(
                &mut buf,
                test,
                Rizz64::write_max_i64,
                Rizz64::read_i64,
            )
        }
    }

    #[test]
    fn test_zig_zag() {
        for x in TESTS {
            let ux = Rizz64::zig_zag(x);
            assert_eq!(x, Rizz64::decode_zig_zag(ux));
        }
    }
}
