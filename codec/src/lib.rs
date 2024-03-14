mod bits_len;
mod test_u64;
mod test_i64;

use crate::bits_len::len_u64;
pub const MAX_LEN_64: usize = 10;

pub struct Rizz64;
impl Rizz64 {

    /// `put_u64` encodes an u64 and returns the encoded buffer and the number of bytes written.
    /// Great for one-off encoding tasks.
    pub fn put_u64(x: u64) -> Result<([u8; MAX_LEN_64], usize), &'static str> {
        let mut bytes = [0u8; MAX_LEN_64];
        let n = Self::append_u64(&mut bytes, x)?;
        Ok((bytes, n))
    }

    /// `append_u64` encodes an u64 into a buffer and returns the number of bytes written.
    /// If the buffer is too small, it will return an `Err`.
    pub fn append_u64(buf: &mut [u8], x: u64) -> Result<usize, &'static str> {
        let mut i: usize = 0;
        let mut x = x;
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

    /// `u64` reads from the buffer and decodes the `u64`. It returns the value and the number of bytes read.
    /// If an `Err` is returned, the value is larger than u64 and a buffer overflow occurred.
    pub fn u64(buf: &[u8]) -> Result<(u64, usize), &'static str> {
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

    /// `size_u64` returns the encoded size of the `u64` where size = [1, `MAX_LEN_64`].
    pub fn size_u64(x: u64) -> usize {
        ((9 * len_u64(x) as u32 + 64) / 64) as usize
    }

    /// `put_i64` encodes an i64 into a buffer and returns the number of bytes written.
    /// Great for one-off encoding tasks.
    pub fn put_i64(x: i64) -> Result<([u8; MAX_LEN_64], usize), &'static str> {
        let mut buf= [0u8; MAX_LEN_64];
        let n = Self::append_i64(&mut buf, x)?;

        Ok((buf, n))
    }

    /// `append_i64` encodes a i64 into a buffer and returns the number of bytes written. If the buffer is too small, it will return an `Err`
    /// Implements a form of zig-zag encoding for `i64`.
    pub fn append_i64(buf: &mut [u8], x: i64) -> Result<usize, &'static str> {
        let mut ux= (x as u64) << 1;
        if x < 0 {
            ux = !ux;
        }

        Self::append_u64(buf, ux)
    }

    /// `i64` reads from the buffer and decodes the `i64`. It returns the value and the number of bytes read.
    /// If an `Err` is returned, the value is larger than `i64` and a buffer overflow occured.
    pub fn i64(buf: &[u8]) -> Result<(i64, usize), &'static str> {
        let (ux, n) = Self::u64(buf)?;

        let mut x = (ux >> 1) as i64;
        if ux&1 != 0 { // checks sign
            x = !x;
        }

        Ok((x, n))
    }

    /// `size_i64` returns the encoded size of the `i64` where size = [1, `MAX_LEN_64`].
    pub fn size_i64(x: i64) -> usize {
        let ux = if x < 0 {
            (!(x as u64)) << 1
        } else {
            (x as u64) << 1
        };

        Self::size_u64(ux)
    }
}
