use bitfield_struct::bitfield;

/// Charger Status 1 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargerStatus1 {
    #[bits(16, default = 0)]
    val: u16,
}

impl ChargerStatus1 {
    pub(crate) const fn addr() -> u8 {
        0x20
    }
}
