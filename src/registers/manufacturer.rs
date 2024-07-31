use bitfield_struct::bitfield;

/// Manufacturer ID Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct Manufacturer {
    #[bits(8, default = 0x0a)]
    pub id: u8,

    #[bits(8, default = 0)]
    reserved15_8: u8,
}

impl Manufacturer {
    pub(crate) const fn addr() -> u8 {
        0xfe
    }
}
