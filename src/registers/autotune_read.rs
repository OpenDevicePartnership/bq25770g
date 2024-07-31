use bitfield_struct::bitfield;

/// Autotune Read Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AutotuneRead {
    /// Phase B inductor time constant L(uH)/DCR(mΩ) value.
    ///
    /// AUTOTUNE_A= 256-265*L(uH)/DCR(mΩ). When converter shuts off
    /// these bits are set back to 0.
    #[bits(8, default = 0)]
    pub autotune_b: u8,

    /// Phase A inductor time constant L(uH)/DCR(mΩ) value.
    ///
    /// AUTOTUNE_A= 256-265*L(uH)/DCR(mΩ). When converter shuts off
    /// these bits are set back to 0.
    #[bits(8, default = 0)]
    pub autotune_a: u8,
}

impl AutotuneRead {
    pub(crate) const fn addr() -> u8 {
        0x60
    }
}
