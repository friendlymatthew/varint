use crate::{bits_len::len_u64, Rizz64, Rizzler, MAX_LEN_64};

impl Rizzler for Rizz64 {
    /// `put_u64rz` encodes a u64 into a buffer and returns the number of bytes written.
    /// If the buffer is too small, it will throw an `Err`.
    fn put_u64rz(buf: &mut [u8], mut x: u64) -> Result<usize, &'static str> {
        let mut i: usize = 0;
        while x >= 0x80 {
            if i >= buf.len() {
                return Err("buffer overflow");
            }

            buf[i] = (x as u8) | 0x80;
            x >>= 7;
            i += 1;
        }

        if i >= buf.len() {
            return Err("buffer overflow");
        }

        buf[i] = x as u8;
        Ok(i + 1)
    }

    /// `u64rz` reads from the buffer and decodes the u64. It returns the value and the number of bytes read.
    /// If an `Err` is thrown, the value is larger than u64 and a buffer overflow occured.
    fn u64rz(buf: &[u8]) -> Result<(u64, usize), &'static str> {
        let mut x = 0u64;
        let mut s: usize = 0;

        for (i, &b) in buf.iter().enumerate() {
            if i == MAX_LEN_64 {
                return Err("buffer overflow");
            }

            if b < 0x80 {
                if i == MAX_LEN_64 - 1 && b > 1 {
                    return Err("buffer overflow");
                }

                return Ok(((x | (b as u64) << s), i + 1));
            }

            x |= ((b & 0x7f) as u64) << s;
            s += 7
        }

        return Ok((0, 0));
    }

    /// `size_u64rz` returns the encoded size of the `u64_rz` where size = [1, 10].
    fn size_u64rz(x: u64) -> usize {
        ((9 * len_u64(x) as u32 + 64) / 64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let mut buf: [u8; MAX_LEN_64] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            if let Ok(byte_len_1) = Rizz64::put_u64rz(&mut buf, test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64rz(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, Rizz64::size_u64rz(expected_value));
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
            if let Ok(byte_len_1) = Rizz64::put_u64rz(&mut buf, test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64rz(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, 1);
                    assert_eq!(byte_len_1, Rizz64::size_u64rz(expected_value));
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
            let mut buf: [u8; MAX_LEN_64] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            if let Ok(byte_len_1) = Rizz64::put_u64rz(&mut buf, test) {
                if let Ok((expected_value, byte_len_2)) = Rizz64::u64rz(&buf[0..byte_len_1]) {
                    assert_eq!(expected_value, test);
                    assert_eq!(byte_len_1, byte_len_2);
                    assert_eq!(byte_len_1, 2);
                    assert_eq!(byte_len_1, Rizz64::size_u64rz(expected_value));
                }
            }
        }
    }
}
