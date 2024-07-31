use bitfield_struct::bitfield;

/// ADC Vbat Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcVbat {
    /// VBAT ADC reading: POR: 0mV (0h)
    ///
    /// Format: 2s Complement
    ///
    /// Range: 0mV-32767mV (0h-7FFFh)
    ///
    /// Clamped Low
    ///
    /// Bit Step: 1mV
    #[bits(16, default = 0)]
    pub voltage: i16,
}

impl AdcVbat {
    pub(crate) const fn addr() -> u8 {
        0x27
    }
}
