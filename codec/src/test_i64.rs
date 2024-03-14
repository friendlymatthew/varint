
#[cfg(test)]
mod tests {
    use crate::{MAX_LEN_64, Rizz64};

    #[test]
    fn test_basic() {
        let tests = vec![
            -1 << 63 -1,
            -257,
            -256,
            -129,
            -128,
            -127,
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
            if let Ok((buf, byte_len_1)) = Rizz64::put_i64(test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::i64(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, Rizz64::size_i64(expected_value));
                }
            }
        }
    }
}