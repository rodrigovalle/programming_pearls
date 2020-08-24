// From Programming Pearls:
// Input:
//   A file containing at most N positive integers, each less than 10^7. It is
//   a fatal error if an integer occurs twice in the input.
// Output:
//   A sorted list in increasing order of the input integers.
// Constraints:
//   Roughly 1MB of storage is available in main memory, ample disk space is
//   available. Can have a run time of up to several minutes; < 10s is optimal.
use std::io;
use std::io::prelude::*;

const MEM_SIZE: usize = 1_300_000;

struct BitVector {
    mem: Vec<u8>,
    n_bits: usize,
}

impl BitVector {
    fn new() -> BitVector {
        BitVector {
            mem: vec![0; MEM_SIZE],
            n_bits: MEM_SIZE * 8,
        }
    }

    fn set_bit(&mut self, i: usize) {
        let byte = i / 8;
        let bit = i % 8;

        // might be inefficient because byte-addressing in x86 makes us generate
        // more complicated instructions to pull bytes from unaligned addresses?
        self.mem[byte] |= 0x01 << bit;
    }

    fn get_bit(&mut self, i: usize) -> bool {
        let byte = i / 8;
        let bit = i % 8;

        (self.mem[byte] >> bit) & 0x01 == 0x01
    }

    fn into_iter(self) -> BitVectorIter {
        BitVectorIter {
            bvec: self,
            cur_bit: 0,
        }
    }
}

struct BitVectorIter {
    bvec: BitVector,
    cur_bit: usize,
}

impl Iterator for BitVectorIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_bit < self.bvec.n_bits {
            let ret = self.bvec.get_bit(self.cur_bit);
            self.cur_bit += 1;
            Some(ret)
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let mut bit_vector = BitVector::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // bad: two unwraps
        let num: usize = line.unwrap().parse().unwrap();
        bit_vector.set_bit(num);
    }

    for (i, bit) in bit_vector.into_iter().enumerate() {
        if bit {
            println!("{}", i);
        }
    }
    Ok(())
}
