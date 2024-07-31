use bitfield_struct::bitfield;

/// Autotune Force Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AutotuneForce {
    /// Force value for phase B inductor time constant L(uH)/DCR(mΩ).
    ///
    /// FORCE_AUTOTUNE_B = 256-265*L(uH)/DCR(mΩ).
    ///
    /// Default 0xA8 refers to 0.211 uH/mΩ
    #[bits(8, default = 0xa8)]
    pub force_autotune_b: u8,

    /// Force value for phase A inductor time constant L(uH)/DCR(mΩ).
    ///
    /// FORCE_AUTOTUNE_A = 256-265*L(uH)/DCR(mΩ).
    ///
    /// Default 0xA8 refers to 0.211 uH/mΩ
    #[bits(8, default = 0xa8)]
    pub force_autotune_a: u8,
}

impl AutotuneForce {
    pub(crate) const fn addr() -> u8 {
        0x61
    }
}
