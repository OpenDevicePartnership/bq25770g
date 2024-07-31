use bitfield_struct::bitfield;

/// ADC Psys Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcPsys {
    /// System Power PSYS ADC reading.
    ///
    /// POR: 0mV (0h)
    ///
    /// Range: 0mV-8191mV (0h-1FFFh)
    ///
    /// Clamped High
    ///
    /// Bit Step: 1mV
    #[bits(16, default = 0)]
    pub power: u16,
}

impl AdcPsys {
    pub(crate) const fn addr() -> u8 {
        0x28
    }
}
