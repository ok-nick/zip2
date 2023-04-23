use std::io::Cursor;
use zip_next::read::ZipArchive;

const BUF: &[u8] = &[
    0x50, 0x4b, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x00, 0x1c, 0x00, 0x69, 0x6e,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2f,
    0x55, 0x54, 0x09, 0x00, 0x03, 0xf4, 0x5c, 0x88, 0x5a, 0xf4, 0x5c, 0x88, 0x5a, 0x75, 0x78, 0x0b,
    0x00, 0x01, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x50, 0x4b, 0x01, 0x02,
    0x1e, 0x03, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // time part: 0 seconds, 0 minutes, 0 hours
    0x00, 0x00, // date part: day 0 (invalid), month 0 (invalid), year 0 (1980)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x00, 0x18, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0xed, 0x41, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6e,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2f,
    0x55, 0x54, 0x05, 0x00, 0x03, 0xf4, 0x5c, 0x88, 0x5a, 0x75, 0x78, 0x0b, 0x00, 0x01, 0x04, 0xe8,
    0x03, 0x00, 0x00, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x50, 0x4b, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x58, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[test]
fn invalid_date() {
    let _archive = ZipArchive::new(Cursor::new(BUF)).unwrap();
}
