
#[cfg(test)]
mod tests {
    use crate::{MAX_LEN_64, Rizz64};

    #[test]
    fn test_basic() {
        let tests = vec![
            0,
            1,
            2,
            10,
            20,
            30,
            60,
            63,
            64,
            65,
            126,
            127,
            128,
            129,
            255,
            256,
            257,
            1 << 63 - 1,
        ];

        for test in tests {
            if let Ok((buf, byte_len_1)) = Rizz64::put_u64(test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, Rizz64::size_u64(expected_value));
                }
            }
        }


    }

    #[test]
    fn test_needs_one_byte() {
        let tests = vec![
            0,
            1,
            2,
            10,
            20,
            30,
            60,
            63,
            64,
            65,
            126,
            127,
        ];

        for test in tests {
            let mut buf: [u8; MAX_LEN_64] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            if let Ok(byte_len_1) = Rizz64::append_u64(&mut buf, test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, 1);
                    assert_eq!(byte_len_1, Rizz64::size_u64(expected_value));
                }
            }
        }
    }

    #[test]
    fn test_needs_two_bytes() {
        let tests = vec![
            128, 129, 16382, 16383
        ];

        for test in tests {
            if let Ok((buf, byte_len_1)) = Rizz64::put_u64(test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, 2);
                    assert_eq!(byte_len_1, Rizz64::size_u64(expected_value));
                }
            }
        }
    }

    #[test]
    fn test_needs_five_bytes() {
        let tests = vec![
            268435456, 34359738367
        ];

        for test in tests {
            if let Ok((buf, byte_len_1)) = Rizz64::put_u64(test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, 5);
                    assert_eq!(byte_len_1, Rizz64::size_u64(expected_value));
                }
            }
        }
    }
}