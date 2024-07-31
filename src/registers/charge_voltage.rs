use bitfield_struct::bitfield;

/// Charge Voltage Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeVoltage {
    #[bits(2, default = 0)]
    reserved1_0: u8,

    /// Charge voltage setting
    ///
    /// Note: Writing non-zero value beyond clamp high/low will
    /// actually set register to the clamp high/low value. When 0V is
    /// written, it should not change CHARGE_VOLTAGE() but reset
    /// CHARGE_CURRENT() to 0A POR: 0mV (0h)
    ///
    /// Range: 5000mV-23000mV (4E2h-1676h)
    /// Clamped Low
    /// Clamped High
    /// Bit Step: 4mV
    #[bits(13, default = 0)]
    pub voltage: u16,

    #[bits(1, default = 0)]
    reserved15: u8,
}

impl ChargeVoltage {
    pub(crate) const fn addr() -> u8 {
        0x15
    }
}
