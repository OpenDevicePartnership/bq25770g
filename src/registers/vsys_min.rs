use bitfield_struct::bitfield;

/// Vsys Min Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct VsysMin {
    /// Minimum system voltage configuration register
    ///
    /// Note: Writing value beyond clamp high/low will actually set
    /// register to the clamp high/low value.
    ///
    /// POR: 6600mV (528h)
    ///
    /// Range: 5000mV-21000mV (3E8h-1068h)
    ///
    /// Clamped Low
    ///
    /// Clamped High
    ///
    /// Bit Step: 5mV
    ///
    /// Mode: 2s
    ///
    /// 6600mV Mode: 3s
    ///
    /// 9200mV
    ///
    /// POR: 9200mV (730h) Mode: 4s
    ///
    /// 12300mV
    ///
    /// POR: 12300mV (99Ch) Mode: 5s
    ///
    /// 15400mV
    ///
    /// POR: 15400mV (C08h)
    #[bits(13, default = 0x528)]
    pub voltage: u16,

    #[bits(3, default = 0)]
    reserved15_13: u8,
}

impl VsysMin {
    pub(crate) const fn addr() -> u8 {
        0x3e
    }
}
