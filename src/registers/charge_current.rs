use bitfield_struct::bitfield;

/// Charge Current Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeCurrent {
    #[bits(3, default = 0)]
    reserved2_0: u8,

    /// Charge current setting with 5mΩ sense resistor (non-zero value
    /// lower than 128mA is treated as 128mA): Note when 2mΩ is chosen
    /// at RSNS_RSR=1b maximum charge current is clamped at 5DCh (30A
    /// with 20mA LSB). Under below scenarios CHARGE_CURRENT is reset
    /// to 0A:
    ///
    /// 1. BATCOC fault.
    /// 2. Charge Voltage() is written 0V
    /// 3. CELL_BATPRES going low (Battery removal)
    /// 4. STAT_AC is not valid (Adapter removal)
    /// 5. Watch dog event trigger
    /// 6. Autonomous charging get terminated (CHRG_STAT =111b)
    /// 7. Safety timer trigger
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value.
    ///
    /// POR: 0mA (0h)
    /// Range: 0mA-16320mA (0h-7F8h)
    /// Clamped High
    /// Bit Step: 8mA
    #[bits(11, default = 0)]
    pub current: u16,

    #[bits(2, default = 0)]
    reserved15_14: u8,
}

impl ChargeCurrent {
    pub(crate) const fn addr() -> u8 {
        0x14
    }
}
