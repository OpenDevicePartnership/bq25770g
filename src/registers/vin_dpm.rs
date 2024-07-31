use bitfield_struct::bitfield;

/// Vin DPM Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct VinDpm {
    /// Input voltage limit:
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value.
    ///
    /// POR: 3200mV (A0h)
    ///
    /// Range: 3200mV-27000mV (A0h-546h)
    ///
    /// Clamped Low
    ///
    /// Clamped High
    ///
    /// Bit Step: 20mV
    #[bits(13, default = 0xa0)]
    pub voltage: u16,

    #[bits(3, default = 0)]
    reserved15_13: u8,
}

impl VinDpm {
    pub(crate) const fn addr() -> u8 {
        0x3d
    }
}
