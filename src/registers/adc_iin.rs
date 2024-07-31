use bitfield_struct::bitfield;

/// ADC Iin Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcIin {
    /// IIN ADC reading with 10mÎ© sense resistor: current flowing from
    /// the adapter to the co nverter (like in forward mode) is
    /// represented as positive and current flowing to the adapter
    /// (like in OTG mode) is negative.
    ///
    /// POR: 0mA(0h)
    ///
    /// Format: 2s Complement
    ///
    /// Range: -16384mA - 16383.5mA (8000h-7FFFh)
    ///
    /// Bit Step: 0.5mA
    #[bits(16, default = 0)]
    pub current: i16,
}

impl AdcIin {
    pub(crate) const fn addr() -> u8 {
        0x25
    }
}
