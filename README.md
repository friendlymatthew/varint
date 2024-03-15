## *rizz*64
Efficient varint encoding, purely for fun and learning purposes. This crate deals with encoding unsigned and signed 64-bit integers.

After working with Golang's `encoding/binary` package, I wanted to rewrite some of the internals. Just because it's funny, I'll continue refer to variable integers as *rizz*64. 

### Why *rizz*64 is helpful
It's super efficient in representing smaller numbers with fewer bytes. Instead of a fixed-size solution like `u64`, this form of encoding can store values using less space or memory.

Basic metrics:

| Decimal            | Powers of 2      | # of Bytes Needed |
|:-------------------|------------------|-------------------|
| 0..127             | 0..`2^7-1`       | 1                 |
| 128..16383         | `2^7`..`2^14-1`  | 2                 |
| 16384..2097151     | `2^14`..`2^21-1` | 3                 |
| 2097152..268435455 | `2^21`..`2^28-1` | 4                 |


and so on... to compute the number of bytes required, you can use the following:

The upper bound of `log2(max(x, 1)) / 7`, where `x` is your number.
This is derived from the fact that each byte can represent 7 bits. *rizz*64 uses a base-2 logarithm to determine the magnitude in bits. We call `max(x, 1)` to ensure this value could never be `0`. We take the upper bound and round to the nearest whole number.

### How encoding works
For a given `u64`, we process the number by groups of 7 bits. For every group, if there are more than 7 bits still to be encoded, we loop.

In the loop, the least significant 7 bits are extracted and packed into a buffer. The 8th bit is the most significant bit (MSB); we set the MSB to `1` to indicate there are more groups ahead. Then, we shift right (`>>=`) by 7 bits and we process the next group.

After looping, the remaining bits of the number is packed into the buffer. This is the last byte and since we're halting, we don't set the MSB to `1`.


### Todo
- [ ] Performance testing
- [ ] Go outside

### Sources 
[LEB128](https://en.wikipedia.org/wiki/LEB128)<br>
[sqlite variable-length ints](https://www.sqlite.org/src4/doc/trunk/www/varint.wiki)<br>
[varint.go](https://go.dev/src/encoding/binary/varint.go)<br>
