//! Java modified UTF-8 over UTF-16 code units, without a length prefix.

use alloc::vec::Vec;

/// Encodes UTF-16 code units, preserving unpaired surrogates.
pub fn encode(units: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(units.len());
    for &value in units {
        if (0x0001..=0x007f).contains(&value) {
            bytes.push(value as u8);
        } else if value <= 0x07ff {
            bytes.push((0xc0 | (value >> 6)) as u8);
            bytes.push((0x80 | (value & 0x3f)) as u8);
        } else {
            bytes.push((0xe0 | (value >> 12)) as u8);
            bytes.push((0x80 | ((value >> 6) & 0x3f)) as u8);
            bytes.push((0x80 | (value & 0x3f)) as u8);
        }
    }
    bytes
}

/// Decodes using Java DataInput's one-, two-, and three-byte rules.
/// Returns `None` for malformed input; unpaired surrogates are preserved.
pub fn decode(bytes: &[u8]) -> Option<Vec<u16>> {
    let continuation = |i: usize| match bytes.get(i) {
        Some(&b) if b & 0xc0 == 0x80 => Some((b & 0x3f) as u16),
        _ => None,
    };

    let mut units = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i] as u16;
        let (unit, length) = match bytes[i] {
            0x00..=0x7f => (b, 1),
            0xc0..=0xdf => (((b & 0x1f) << 6) | continuation(i + 1)?, 2),
            0xe0..=0xef => (((b & 0x0f) << 12) | (continuation(i + 1)? << 6) | continuation(i + 2)?, 3),
            _ => return None,
        };
        units.push(unit);
        i += length;
    }
    Some(units)
}
