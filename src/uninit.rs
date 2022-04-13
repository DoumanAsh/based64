//! Functions to work with uninit memory

use core::{ptr, mem};

///Encoding function writing to uninit slice.
///
///`src` - Input to encode;
///`dst` - Output to write;
///
///Returns `Some` if successful, containing number of bytes written.
///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8], dst: &mut [mem::MaybeUninit<u8>]) -> Option<usize> {
    let mut len = dst.len();
    let dst = unsafe {
        ptr::NonNull::new_unchecked(dst.as_ptr() as *mut u8)
    };
    match super::raw::encode(table, src, dst, &mut len) {
        true => Some(len),
        false => None,
    }
}

///Decoding function writing to uninit slice.
///
///`src` - Input to decode;
///`dst` - Output to write;
///
///Returns `Some` if successful, containing number of bytes written.
///Returns `None` if data cannot be encoded due to insufficient buffer size.
#[inline]
pub fn decode(table: &[u8; 64], src: &[u8], dst: &mut [mem::MaybeUninit<u8>]) -> Option<usize> {
    let mut len = dst.len();
    let dst = unsafe {
        ptr::NonNull::new_unchecked(dst.as_ptr() as *mut u8)
    };
    match super::raw::decode(table, src, dst, &mut len) {
        true => Some(len),
        false => None,
    }
}
