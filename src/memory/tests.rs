#[test]
pub fn test_memory_rw_b() {
    use super::{writeb, readb};

    writeb(0xFFFFFF, 0x5A);
    writeb(0x1000000, 0x5A);

    assert_eq!(readb(0xFFFFFF), 0x5A);
    assert_eq!(readb(0x1000000), 0x5A);
}
