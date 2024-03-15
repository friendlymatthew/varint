mod tests;

pub const MAX_LEN_64: usize = 10;

pub struct Rizz64;
impl Rizz64 {

    /// `write_u64` encodes an u64 into a buffer and returns the number of bytes written.
    /// If the buffer is too small, it will return an `Err`.
    #[inline]
    pub fn write_u64(buf: &mut [u8], mut x: u64) -> Result<usize, &'static str> {
        if Self::size_u64(x) > buf.len() {
            return Err("buffer overflow");
        }

        let mut i = 0;
        while {
            let byte = (x & 0x7F) as u8;
            x >>= 7;
            let more = x != 0;

            unsafe {
                *buf.get_unchecked_mut(i) = byte | if more { 0x80 } else { 0 };
            }

            i += 1;
            more
        } {}

        Ok(i)
    }

    /// `read_u64` reads from the buffer and decodes the `u64`. It returns the value and the number of bytes read.
    /// If an `Err` is returned, the value is larger than u64 and a buffer overflow occurred.
    #[inline]
    pub fn read_u64(buf: &[u8]) -> Result<(u64, usize), &'static str> {
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

                return Ok((x | (b as u64) << s, i + 1));
            }

            x |= ((b & 0x7f) as u64) << s;
            s += 7
        }

        return Ok((0, 0));
    }

    /// `size_u64` returns the encoded size of the `u64` where size = [1, `MAX_LEN_64`].
    #[inline]
    pub fn size_u64(x: u64) -> usize {
        return if x == 0 { 1 } else {
            ((64 - x.leading_zeros() as usize) + 6) / 7
        }
    }


    /// `write_i64` encodes a i64 into a buffer and returns the number of bytes written. If the buffer is too small, it will return an `Err`
    /// Implements a form of zig-zag encoding for `i64`.
    #[inline]
    pub fn write_i64(buf: &mut [u8], x: i64) -> Result<usize, &'static str> {
        if buf.len() < Self::size_i64(x) {
            return Err("Buffer overflow");
        }

        let mut ux = (x as u64) << 1;
        if x < 0 {
            ux = !ux;
        }

        Self::write_u64(buf, ux)
    }

    /// `read_i64` reads from the buffer and decodes the `i64`. It returns the value and the number of bytes read.
    /// If an `Err` is returned, the value is larger than `i64` and a buffer overflow occured.
    #[inline]
    pub fn read_i64(buf: &[u8]) -> Result<(i64, usize), &'static str> {
        let (ux, n) = Self::read_u64(buf)?;

        let mut x = (ux >> 1) as i64;
        if ux & 1 != 0 {
            x = !x;
        }

        Ok((x, n))
    }

    /// `size_i64` returns the encoded size of the `i64` where size = [1, `MAX_LEN_64`].
    #[inline]
    pub fn size_i64(x: i64) -> usize {
        let ux = ((x as u64) << 1) ^ ((x).wrapping_shr(63) as u64);
        Self::size_u64(ux)
    }
}
