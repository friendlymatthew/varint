## *rizz*64
Efficient varint encoding, purely for fun and learning purposes. This crate deals with encoding unsigned and signed 64-bit integers.

After working with Golang's `encoding/binary` package, I wanted to rewrite some of the internals. Just because it's funny, I'll continue refer to variable integers as *rizz*64. 

### Why *rizz*64 is helpful
It's super efficient in representing smaller numbers with fewer bytes. Instead of a fixed-size solution like `u64`, this form of encoding can store values using less space or memory.

Basic metrics:

| Values     | # of Bytes Needed |
|------------|-------------------|
| 0..127     | 1                 |
| 128..16383 | 2                 |

and so on... where if `n` is the current number of bytes needed, values that need `n+1` bytes can be ~128x larger than what `n` could store.

### How encoding works
For a given `u64`, we process the number by groups of 7 bits. For every group, if there are more than 7 bits still to be encoded, we loop.

In the loop, the least significant 7 bits are extracted and packed into a buffer. The 8th bit is the most significant bit (MSB); we set the MSB to `1` to indicate there are more groups ahead. Then, we shift right (`>>=`) by 7 bits and we process the next group.

After looping, the remaining bits of the number is packed into the buffer. This is the last byte and since we're halting, we don't set the MSB to `1`.


### Todo
- [ ] Write `rizz_i64`
- [ ] Performance testing
- [ ] Go outside

### Sources 
[sqlite variable-length ints](https://www.sqlite.org/src4/doc/trunk/www/varint.wiki)<br>
[varint.go](https://go.dev/src/encoding/binary/varint.go)<br>