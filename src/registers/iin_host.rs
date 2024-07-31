use bitfield_struct::bitfield;

/// Iin Host Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct IinHost {
    #[bits(2, default = 0)]
    reserved1_0: u16,

    /// Maximum input current limit with 10mΩ sense resistor:
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value.
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

impl IinHost {
    pub(crate) const fn addr() -> u8 {
        0x3f
    }
}
