use bitfield_struct::bitfield;

/// Device ID Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct Device {
    #[bits(8, default = 0x0a)]
    pub id: u8,

    #[bits(8, default = 0)]
    reserved15_8: u8,
}

impl Device {
    pub(crate) const fn addr() -> u8 {
        0xff
    }
}
