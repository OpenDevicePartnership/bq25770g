use bitfield_struct::bitfield;

/// Otg Current Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct OtgCurrent {
    #[bits(2, default = 0)]
    reserved1_0: u8,

    /// OTG output current limit with 10mΩ Rac current sense:
    //
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value . P
    ///
    /// OR: 3000mA (78h)
    ///
    /// Range: 100mA-3000mA (4h-78h)
    ///
    /// Clamped Low
    ///
    /// Clamped High
    ///
    /// Bit Step: 25mA
    #[bits(9, default = 0x78)]
    pub current: u16,

    #[bits(5, default = 0)]
    reserved15_11: u8,
}

impl OtgCurrent {
    pub(crate) const fn addr() -> u8 {
        0x3c
    }
}
