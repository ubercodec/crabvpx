use crabvpx::vp8::decoder::dboolhuff::*;

#[test]
fn test_bool_decoder_init() {
    let data = [0u8; 10];
    let mut br = BOOL_DECODER::default();
    vp8dx_start_decode_safe(&mut br, &data, None, core::ptr::null_mut());
    assert_eq!(br.range, 255);
    assert_eq!(br.count, 56);
}

#[test]
fn test_safe_bool_decoder_init() {
    let data = [0u8; 10];
    let sbd = SafeBoolDecoder::new(&data, None, core::ptr::null_mut());
    assert_eq!(sbd.range, 255);
    assert_eq!(sbd.count, 56);
}

#[test]
fn test_safe_bool_decoder_read() {
    let data = [0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut sbd = SafeBoolDecoder::new(&data, None, core::ptr::null_mut());
    assert_eq!(sbd.read_bool(128), 1);
}
