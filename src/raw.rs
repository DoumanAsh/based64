//! Low level functions

use core::ptr::NonNull;
use super::{PAD, encode_len, decode_len};

#[cold]
#[inline(never)]
fn unlikely_false() -> bool {
    false
}

///Raw encoding function.
///
///`src` - Input to encode;
///`dst` - Output to write;
///`len` - Output length, modified with required size regardless of outcome, unless calculation wrapping happens.
///
///Returns `true` on success.
///Returns `false` if buffer overflow would to happen or required_len is too big.
pub fn encode(table: &[u8; 64], src: &[u8], dst: NonNull<u8>, len: &mut usize) -> bool {
    let required_len = encode_len(src.len());
    if required_len < src.len() {
        //bro, how likely is overflow?
        return unlikely_false();
    } else if required_len > *len {
        *len = required_len;
        return false;
    }

    let mut it = src.as_ptr();
    let it_end = unsafe {
        it.add(src.len())
    };
    let mut cursor = dst.as_ptr();
    while (it_end as usize) - (it as usize) >= 3 {
        unsafe {
            *cursor = *table.get_unchecked(
                (*it).wrapping_shr(2) as usize
            );
            cursor = cursor.add(1);

            *cursor = *table.get_unchecked(
                (((*it) & 0x03).wrapping_shl(4) | (*it.add(1)).wrapping_shr(4)) as usize
            );
            cursor = cursor.add(1);

            *cursor = *table.get_unchecked(
                (((*it.add(1)) & 0x0f).wrapping_shl(2) | (*it.add(2)).wrapping_shr(6)) as usize
            );
            cursor = cursor.add(1);

            *cursor = *table.get_unchecked(
                ((*it.add(2)) & 0x3f) as usize
            );
            cursor = cursor.add(1);

            it = it.add(3);
        }
    }

    let remain_len = (it_end as usize) - (it as usize);
    if remain_len > 0 {
        unsafe {
            *cursor = *table.get_unchecked(
                (*it).wrapping_shr(2) as usize
            );
            cursor = cursor.add(1);

            if remain_len == 1 {
                *cursor = *table.get_unchecked(
                    ((*it) & 0x03).wrapping_shl(4) as usize
                );
                cursor = cursor.add(1);

                *cursor = PAD;
                cursor = cursor.add(1);
            } else {
                *cursor = *table.get_unchecked(
                    (((*it) & 0x03).wrapping_shl(4) | (*it.add(1)).wrapping_shr(4)) as usize
                );
                cursor = cursor.add(1);

                *cursor = *table.get_unchecked(
                    ((*it.add(1)) & 0x0f).wrapping_shl(2) as usize
                );
                cursor = cursor.add(1);
            }

            *cursor = PAD;
            cursor = cursor.add(1);
        }
    }

    *len = cursor as usize - dst.as_ptr() as usize;
    true
}

///Raw decoding function.
///
///`src` - Input to decode;
///`dst` - Output to write;
///`len` - Output length, modified with required size regardless of outcome.
///
///Returns `true` on success.
///Returns `false` if buffer overflow would to happen or `src` is empty or invalid base64.
pub fn decode(table: &[u8; 64], mut src: &[u8], dst: NonNull<u8>, len: &mut usize) -> bool {
    let required_len = decode_len(src);

    if required_len == 0 {
        if src.is_empty() {
            *len = 0;
            return true;
        } else {
            return unlikely_false();
        }
    }

    let mut cursor = dst.as_ptr();
    let mut chunk = [0u8; 4];
    let mut chunk_len = 0;

    macro_rules! get_base64_byte {
        ($src:ident[$idx:literal]) => {{
            let ch = match $src.get($idx) {
                Some(ch) if *ch != PAD => *ch,
                _ => {
                    chunk_len = $idx - 1;
                    break;
                },
            };

            match table.iter().position(|&b| b == ch) {
                Some(pos) => pos as u8,
                None => return unlikely_false(),
            }
        }}
    }

    loop {
        chunk[0] = get_base64_byte!(src[0]);
        chunk[1] = get_base64_byte!(src[1]);
        chunk[2] = get_base64_byte!(src[2]);
        chunk[3] = get_base64_byte!(src[3]);

        unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
            *cursor = (chunk[1] & 0xf).wrapping_shl(4).wrapping_add((chunk[2] & 0x3c).wrapping_shr(2));
            cursor = cursor.add(1);
            *cursor = (chunk[2] & 0x3).wrapping_shl(6).wrapping_add(chunk[3]);
            cursor = cursor.add(1);
        }

        src = &src[4..];

        if src.is_empty() {
            break;
        }
    }

    match chunk_len {
        3 => unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
            *cursor = (chunk[1] & 0xf).wrapping_shl(4).wrapping_add((chunk[2] & 0x3c).wrapping_shr(2));
            cursor = cursor.add(1);
            *cursor = (chunk[2] & 0x3).wrapping_shl(6).wrapping_add(chunk[3]);
            cursor = cursor.add(1);
        },
        2 => unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
            *cursor = (chunk[1] & 0xf).wrapping_shl(4).wrapping_add((chunk[2] & 0x3c).wrapping_shr(2));
            cursor = cursor.add(1);
        },
        1 => unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
        },
        _ => (),
    }

    *len = cursor as usize - dst.as_ptr() as usize;
    true
}
