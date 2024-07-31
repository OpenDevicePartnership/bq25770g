use bitfield_struct::bitfield;

/// ADC Vsys Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcVsys {
    /// VSYS ADC reading.
    ///
    /// POR: 0mV (0h)
    ///
    /// Format: 2s Complement
    ///
    /// Range: 0mV-65534mV (0h-7FFFh)
    ///
    /// Clamped Low
    ///
    /// Bit Step: 2mV
    #[bits(16, default = 0)]
    pub voltage: i16,
}

impl AdcVsys {
    pub(crate) const fn addr() -> u8 {
        0x26
    }
}
