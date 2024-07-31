use bitfield_struct::bitfield;

/// ADC CMPIN TR Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcCmpinTr {
    /// CMPIN_TR pin voltage ADC reading.
    ///
    /// POR: 0mV (0h)
    ///
    /// Range: 0mV-8191mV (0h-1FFFh)
    ///
    /// Clamped High
    ///
    /// Bit Step: 1mV
    #[bits(16, default = 0)]
    pub voltage: u16,
}

impl AdcCmpinTr {
    pub(crate) const fn addr() -> u8 {
        0x29
    }
}
