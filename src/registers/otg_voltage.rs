use bitfield_struct::bitfield;

/// Otg Voltage Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct OtgVoltage {
    #[bits(2, default = 0)]
    reserved1_0: u8,

    /// OTG output voltage regulation:
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value.
    ///
    /// POR: 5000mV (FAh)
    ///
    /// Range: 3000mV-5000mV (96h-FAh)
    ///
    /// Clamped Low
    ///
    /// Clamped High
    ///
    /// Bit Step: 20mV
    #[bits(11, default = 0xfa)]
    pub voltage: u16,

    #[bits(3, default = 0)]
    reserved15_13: u8,
}

impl OtgVoltage {
    pub(crate) const fn addr() -> u8 {
        0x3b
    }
}
