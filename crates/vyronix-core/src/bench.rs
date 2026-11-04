//! Deterministic benchmark kernels and memory-test pattern logic.
//!
//! These perform REAL work (integer/floating-point arithmetic, memory
//! verification). The backend times them to produce reproducible scores.
//! `black_box` prevents the optimizer from eliminating the work. No random
//! or fabricated results are produced here.

use std::hint::black_box;

/// Integer workload: a mixing hash over `iterations` steps. Returns a checksum
/// so the work cannot be optimized away. Deterministic for a given input.
pub fn integer_workload(iterations: u64, seed: u64) -> u64 {
    let mut acc = seed ^ 0x9E37_79B9_7F4A_7C15;
    for i in 0..iterations {
        let x = black_box(i).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        acc = acc.rotate_left(13) ^ x;
        acc = acc.wrapping_mul(0x94D0_49BB_1331_11EB);
        acc ^= acc >> 29;
        black_box(acc);
    }
    acc
}

/// Floating-point workload: a numerically stable series. Returns the sum.
pub fn float_workload(iterations: u64, seed: f64) -> f64 {
    let mut acc = seed;
    for i in 1..=iterations {
        let x = black_box(i as f64);
        acc += (x.sqrt() * (x * 0.000_001).sin()).cos() / (x + 1.0);
        black_box(acc);
    }
    acc
}

/// Memory test patterns for write/read verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Zero,
    Ones,
    Alternating55,
    AlternatingAA,
    WalkingOnes(u32),
    AddressBased,
}

impl Pattern {
    /// Byte value for this pattern at a given 8-byte word index and byte lane.
    pub fn byte_at(self, word_index: u64, lane: usize) -> u8 {
        match self {
            Pattern::Zero => 0x00,
            Pattern::Ones => 0xFF,
            Pattern::Alternating55 => 0x55,
            Pattern::AlternatingAA => 0xAA,
            Pattern::WalkingOnes(bit) => {
                let word = 1u64 << (bit % 64);
                ((word >> (lane * 8)) & 0xFF) as u8
            }
            Pattern::AddressBased => {
                let word = word_index.wrapping_mul(0x9E37_79B9_7F4A_7C15);
                ((word >> (lane * 8)) & 0xFF) as u8
            }
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Pattern::Zero => "All zeros (0x00)",
            Pattern::Ones => "All ones (0xFF)",
            Pattern::Alternating55 => "Alternating 0x55",
            Pattern::AlternatingAA => "Alternating 0xAA",
            Pattern::WalkingOnes(b) => match b % 4 {
                0 => "Walking ones (low)",
                1 => "Walking ones (mid-low)",
                2 => "Walking ones (mid-high)",
                _ => "Walking ones (high)",
            },
            Pattern::AddressBased => "Address-based",
        }
    }
}

/// Fill a buffer with a pattern. Buffer length should be a multiple of 8 for
/// full-word patterns; trailing bytes are filled lane-by-lane.
pub fn fill_pattern(buf: &mut [u8], pattern: Pattern) {
    let mut word_index = 0u64;
    let mut i = 0;
    while i + 8 <= buf.len() {
        for lane in 0..8 {
            buf[i + lane] = pattern.byte_at(word_index, lane);
        }
        word_index = word_index.wrapping_add(1);
        i += 8;
    }
    while i < buf.len() {
        buf[i] = pattern.byte_at(word_index, i % 8);
        i += 1;
    }
}

/// Verify a buffer against a pattern. Returns the number of mismatched bytes
/// and the index of the first mismatch (if any). This is real error detection.
pub fn verify_pattern(buf: &[u8], pattern: Pattern) -> (u64, Option<usize>) {
    let mut errors = 0u64;
    let mut first = None;
    let mut word_index = 0u64;
    let mut i = 0;
    while i + 8 <= buf.len() {
        for lane in 0..8 {
            let expected = pattern.byte_at(word_index, lane);
            if buf[i + lane] != expected {
                errors += 1;
                if first.is_none() {
                    first = Some(i + lane);
                }
            }
        }
        word_index = word_index.wrapping_add(1);
        i += 8;
    }
    while i < buf.len() {
        let expected = pattern.byte_at(word_index, i % 8);
        if buf[i] != expected {
            errors += 1;
            if first.is_none() {
                first = Some(i);
            }
        }
        i += 1;
    }
    (errors, first)
}

/// Compute a simple checksum over a buffer (for read-verification).
pub fn checksum(buf: &[u8]) -> u64 {
    let mut acc = 0xcbf2_9ce4_8422_2325u64;
    for &b in buf {
        acc ^= b as u64;
        acc = acc.wrapping_mul(0x100_0000_01b3);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_workload_deterministic() {
        assert_eq!(integer_workload(1000, 42), integer_workload(1000, 42));
        assert_ne!(integer_workload(1000, 42), integer_workload(1000, 43));
    }

    #[test]
    fn integer_workload_zero_iterations() {
        // With zero iterations the accumulator is just the seeded constant.
        let a = integer_workload(0, 42);
        let b = integer_workload(0, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn float_workload_deterministic_and_finite() {
        let a = float_workload(1000, 1.0);
        let b = float_workload(1000, 1.0);
        assert_eq!(a, b);
        assert!(a.is_finite());
    }

    #[test]
    fn pattern_fill_verify_roundtrip_all_patterns() {
        let patterns = [
            Pattern::Zero, Pattern::Ones, Pattern::Alternating55, Pattern::AlternatingAA,
            Pattern::WalkingOnes(0), Pattern::WalkingOnes(17), Pattern::AddressBased,
        ];
        for p in patterns {
            let mut buf = vec![0u8; 4096 + 3]; // non-multiple of 8 to test tail
            fill_pattern(&mut buf, p);
            let (errors, first) = verify_pattern(&buf, p);
            assert_eq!(errors, 0, "pattern {:?} should verify clean", p);
            assert!(first.is_none());
        }
    }

    #[test]
    fn verify_detects_corruption() {
        let mut buf = vec![0u8; 1024];
        fill_pattern(&mut buf, Pattern::AddressBased);
        buf[100] ^= 0xFF; // introduce a real error
        buf[500] ^= 0x01;
        let (errors, first) = verify_pattern(&buf, Pattern::AddressBased);
        assert_eq!(errors, 2);
        assert_eq!(first, Some(100));
    }

    #[test]
    fn walking_ones_varies() {
        let mut a = vec![0u8; 8];
        let mut b = vec![0u8; 8];
        fill_pattern(&mut a, Pattern::WalkingOnes(0));
        fill_pattern(&mut b, Pattern::WalkingOnes(1));
        assert_ne!(a, b);
    }

    #[test]
    fn checksum_stable_and_sensitive() {
        let data = b"vyronix memory verification";
        assert_eq!(checksum(data), checksum(data));
        let mut altered = data.to_vec();
        altered[0] ^= 1;
        assert_ne!(checksum(data), checksum(&altered));
    }

    #[test]
    fn empty_buffer_verify() {
        let buf: [u8; 0] = [];
        assert_eq!(verify_pattern(&buf, Pattern::Zero), (0, None));
    }
}
