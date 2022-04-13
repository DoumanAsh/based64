use based64::{STANDARD_TABLE};
use based64::{decode, encode, encode_len, decode_len};
use based64::raw::encode as raw_encode;
use based64::raw::decode as raw_decode;

const fn all_ascii_chars() -> [u8; 128] {
    let mut result = [0u8; 128];
    let mut idx = 1u8;
    while idx < 128 {
        result[idx as usize] = idx;
        idx += 1;
    }
    result
}

const SAMPLE_DATA: [(&str, &str); 9] = [
    ("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", "MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5MTIzNDU2Nzg5"),
    ("", ""),
    ("f", "Zg=="),
    ("f", "Zg=="),
    ("fo", "Zm8="),
    ("foo", "Zm9v"),
    ("foob", "Zm9vYg=="),
    ("fooba", "Zm9vYmE="),
    ("foobar", "Zm9vYmFy"),
];

const ALL_ASCII: [u8; 128] = all_ascii_chars();
const ALL_ASCII_EXPECTED: &[u8] = b"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn8=";

#[test]
fn should_encode_with_default_table() {
    let mut buffer = [0u8; 4096];
    let mut decoded = [0u8; 4096];
    for (idx, (input, output)) in SAMPLE_DATA.iter().enumerate() {
        let len = match encode(STANDARD_TABLE, input.as_bytes(), &mut buffer) {
            Some(len) => len,
            None => panic!("base64 encode fails for idx={}", idx),
        };
        assert_eq!(len, output.len());
        let base64 = &buffer[..len];
        assert_eq!(base64, output.as_bytes());

        assert_eq!(decode_len(base64), input.len(), "decode_len() fails for idx={}", idx);
        let len = match decode(STANDARD_TABLE, base64, &mut decoded) {
            Some(len) => len,
            None => panic!("base64 encode fails for idx={}", idx),
        };
        assert_eq!(len, input.len());
        let decoded = &decoded[..len];
        assert_eq!(decoded, input.as_bytes());
    }
}

#[test]
fn should_raw_encode_with_default_table() {
    let mut buffer = [0u8; 4096];
    let mut decoded = [0u8; 4096];
    for (idx, (input, output)) in SAMPLE_DATA.iter().enumerate() {
        let mut len = buffer.len();
        let dst = core::ptr::NonNull::new(buffer.as_mut_ptr()).unwrap();
        assert!(raw_encode(STANDARD_TABLE, input.as_bytes(), dst, &mut len), "base64 encode fails for idx={}", idx);
        assert_eq!(len, output.len());
        let base64 = &buffer[..len];
        assert_eq!(base64, output.as_bytes());

        let mut len = decoded.len();
        let dst = core::ptr::NonNull::new(decoded.as_mut_ptr()).unwrap();
        assert_eq!(decode_len(base64), input.len(), "decode_len() fails for idx={}", idx);
        assert!(raw_decode(STANDARD_TABLE, base64, dst, &mut len), "base64 reverse decode fails for idx={}", idx);
        assert_eq!(len, input.len(), "base64 reverse decode has invalid len for idx={}", idx);
        let decoded = &decoded[..len];
        assert_eq!(decoded, input.as_bytes(), "base64 reverse decode wrongly idx={}", idx);
    }
}

#[test]
fn should_raw_encode_all_ascii_with_default_table() {
    let mut buffer = [0u8; ALL_ASCII_EXPECTED.len()];
    let mut decoded = [0u8; ALL_ASCII.len()];
    let mut len = buffer.len();
    let dst = core::ptr::NonNull::new(buffer.as_mut_ptr()).unwrap();
    assert!(raw_encode(STANDARD_TABLE, &ALL_ASCII, dst, &mut len), "base64 encode fails for all ASCII. Required len={} but len={}", len, buffer.len());
    assert_eq!(len, ALL_ASCII_EXPECTED.len());
    let base64 = &buffer[..len];
    assert_eq!(base64, ALL_ASCII_EXPECTED);

    let mut len = decoded.len();
    let dst = core::ptr::NonNull::new(decoded.as_mut_ptr()).unwrap();
    assert!(raw_decode(STANDARD_TABLE, base64, dst, &mut len), "base64 decode fails for all ASCII. Required len={} but len={}", len, decoded.len());
    assert_eq!(len, ALL_ASCII.len());
    let decoded = &decoded[..len];
    assert_eq!(decoded, ALL_ASCII);
}

#[test]
fn should_raw_encode_fail_all_ascii_with_default_table_on_buffer_overflow() {
    let mut buffer = [0u8; ALL_ASCII_EXPECTED.len()];
    let dst = core::ptr::NonNull::new(buffer.as_mut_ptr()).unwrap();
    for idx in 1..ALL_ASCII_EXPECTED.len() {
        let mut len = buffer.len() - idx;
        assert!(!raw_encode(STANDARD_TABLE, &ALL_ASCII, dst, &mut len), "base64 encode should fail, but it is successful");
        assert_eq!(len, ALL_ASCII_EXPECTED.len());
    }
}

#[test]
fn should_encode_fail_all_ascii_with_default_table_on_buffer_overflow() {
    let mut buffer = [0u8; ALL_ASCII_EXPECTED.len()];
    for idx in 1..ALL_ASCII_EXPECTED.len() {
        let dst = &mut buffer[idx..];
        assert!(encode(STANDARD_TABLE, &ALL_ASCII, dst).is_none(), "base64 encode should fail, but it is successful");
    }
}

#[test]
fn verify_encode_safety() {
    let mut src_buffer = [0u8; 500];
    let mut out_buffer = [0u8; 1000];
    for idx in 1..src_buffer.len() {
        let src = &mut src_buffer[idx..];
        let dst = &mut out_buffer[..encode_len(src.len())];
        getrandom::getrandom(src).expect("Random should work for fuck sake");
        assert!(encode(STANDARD_TABLE, src, dst).is_some(), "ENCODE SHOULD NOT FAIL");
        assert!(decode(STANDARD_TABLE, dst, src).is_some(), "DECODE SHOULD NOT FAIL");
    }
}
