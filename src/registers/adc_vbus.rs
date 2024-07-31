use bitfield_struct::bitfield;

/// ADC Vbus Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcVbus {
    /// VBUS ADC reading:
    ///
    /// Note: When VBUS plugged in before converter starts up, VBUS
    /// ADC channel should execute one time to read the no-load VBUS
    /// voltage and save the value into ADC_VBUS().
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

impl AdcVbus {
    pub(crate) const fn addr() -> u8 {
        0x23
    }
}
