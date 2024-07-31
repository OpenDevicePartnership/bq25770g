use bitfield_struct::bitfield;

/// Iin Dpm Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct IinDpm {
    #[bits(2, default = 0)]
    reserved1_0: u16,

    /// Input current setting with 10mΩ sense resistor.
    ///
    /// POR: 5000mA (C8h)
    ///
    /// Range: 400mA-8200mA (10h-148h)
    ///
    /// Clamped Low
    ///
    /// Clamped High
    ///
    /// Bit Step: 25mA
    #[bits(9, default = 0xc8)]
    pub current: u16,

    #[bits(5, default = 0)]
    reserved15_11: u16,
}
impl IinDpm {
    pub(crate) const fn addr() -> u8 {
        0x22
    }
}
