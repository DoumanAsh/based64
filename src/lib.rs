//!BASE64 library for chads

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod raw;
pub mod uninit;

use core::mem;

///Base64 padding character
pub const PAD: u8 = b'=';
///Default character table used by based64
pub static STANDARD_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
///Alternative table URL safe.
pub static URL_TABLE: &[u8; 64] = b"ABCDEFGHIJLKMNOPQRSTUVWXYZabcdefghijlkmnopqrstuvwxyz0123456789-_";

///Validates custom character table by requiring user to provide table of specific size
///containing only ASCII characters.
pub const fn assert_valid_character_table(table: &[u8; 64]) -> bool {
    let mut idx = 0;
    while idx < table.len() {
        if !table[idx].is_ascii() {
            return false
        }

        idx += 1;
    }

    true
}

#[inline(always)]
///Returns number of bytes necessary to encode input of provided size (including padding).
///
///On overflow returns wrapped value.
pub const fn encode_len(input: usize) -> usize {
    input.wrapping_mul(4).wrapping_div(3).wrapping_add(3) & !3
}

///Returns number of bytes necessary to decode provided input.
pub const fn decode_len(input: &[u8]) -> usize {
    let len = input.len();
    if len == 0 || (len & 3 != 0) {
        0
    } else {       //len / 4 * 3
        let result = len.wrapping_div(4).wrapping_mul(3);
        if input[len - 1] != PAD {
            result
        } else if input[len - 2] != PAD {
            result - 1
        } else if input[len - 3] != PAD {
            result - 2
        } else {
            result - 3
        }
    }
}

///Encoding function writing to slice.
///
///`src` - Input to encode;
///`dst` - Output to write;
///
///Returns `Some` if successful, containing number of bytes written.
///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8], dst: &mut [u8]) -> Option<usize> {
    unsafe {
        uninit::encode(table, src, mem::transmute(dst))
    }
}

///Decoding function writing to slice.
///
///`src` - Input to decode;
///`dst` - Output to write;
///
///Returns `Some` if successful, containing number of bytes written.
///Returns `None` if data cannot be encoded due to insufficient buffer size.
#[inline]
pub fn decode(table: &[u8; 64], src: &[u8], dst: &mut [u8]) -> Option<usize> {
    unsafe {
        uninit::decode(table, src, mem::transmute(dst))
    }
}
