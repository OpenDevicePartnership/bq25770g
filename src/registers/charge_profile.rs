use bitfield_struct::bitfield;

/// Charge Profile Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeProfile {
    /// Termination current setting with 5mΩ sense resistor.
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value. POR: 256mA (20h)
    ///
    /// Range: 128mA-2016mA (10h-FCh)
    /// Clamped Low
    /// Clamped High
    /// Bit Step: 8mA
    #[bits(8, default = 0x20)]
    pub iterm: u8,

    /// Maximum precharge current clamp setting with 5mΩ sense
    /// resistor(The lower setting of CHARGE_CURRENT() and IPRECHG
    /// determine the practical precharge current when VBAT<
    /// VSYS_MIN()): Note when 2mΩ sense resistor is chosen
    /// RSNS_RSR=1b, then the IPRECHG() upper clamp should be 66H to
    /// limit BATFET thermal dissipation.
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value . POR: 384mA (30h)
    ///
    /// Range: 128mA-2016mA (10h-FCh)
    /// Clamped Low
    /// Clamped High
    /// Bit Step: 8mA
    #[bits(8, default = 0x30)]
    pub iprechg: u8,
}

impl ChargeProfile {
    pub(crate) const fn addr() -> u8 {
        0x17
    }
}
