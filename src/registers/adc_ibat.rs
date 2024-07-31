use bitfield_struct::bitfield;

/// ADC Ibat Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcIbat {
    /// IBAT ADC reading with 5mΩ sense resistor: Note the charger
    /// only measures discharging current (negative voltage) under
    /// battery only or OTG modes, and only measure charging
    /// current(positive voltage) when valid adapter is plugged in
    /// POR.
    ///
    /// 0mA (0h)
    ///
    /// Format: 2s Complement
    ///
    /// Range: -32768mA-32767mA (8000h-7FFFh)
    ///
    /// Bit Step: 1mA
    #[bits(16, default = 0)]
    pub current: i16,
}

impl AdcIbat {
    pub(crate) const fn addr() -> u8 {
        0x24
    }
}
