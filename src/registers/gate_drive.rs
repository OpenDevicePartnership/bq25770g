use bitfield_struct::bitfield;

/// Gate Drive Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct GateDrive {
    #[bits(1, default = false)]
    reserved0: bool,

    /// System regulation loop bandwidth slow down to reduce input
    /// current overshoot during load transient.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = VsysRegSlow::Disable)]
    pub vsys_reg_slow: VsysRegSlow,

    /// LODRV2 LS MOSFET gate drive strength adjustment for both turn
    /// on and turn off.
    ///
    /// 000b = Scale0 (Vgs=4.5V Qg range:0-5nC)
    ///
    /// 001b = Scale1(Vgs=4.5V Qg range:5-13nC )
    ///
    /// 010b = Scale2 (Vgs=4.5V Qg range:13-21nC )
    ///
    /// 011b = Scale3(Vgs=4.5V Qg range:21-29nC)
    ///
    /// 100b = Scale4 (Vgs=4.5V Qg range:29-37nC)
    ///
    /// 101b = Scale5(Vgs=4.5V Qg range:37-45nC)
    ///
    /// 110b = Scale6 (Vgs=4.5V Qg range:45-53nC)
    ///
    /// 111b = Scale7(Vgs=4.5V Qg range: >53nC)
    #[bits(3, default = DrvStat::Scale3)]
    pub lodrv2_stat: DrvStat,

    /// HIDRV2 HS MOSFET gate drive strength adjustment for both turn
    /// on and turn off.
    ///
    /// 000b = Scale0 (Vgs=4.5V Qg range:0-5nC)
    ///
    /// 001b = Scale1(Vgs=4.5V Qg range:5-13nC )
    ///
    /// 010b = Scale2 (Vgs=4.5V Qg range:13-21nC )
    ///
    /// 011b = Scale3(Vgs=4.5V Qg range:21-29nC)
    ///
    /// 100b = Scale4 (Vgs=4.5V Qg range:29-37nC)
    ///
    /// 101b = Scale5(Vgs=4.5V Qg range:37-45nC)
    ///
    /// 110b = Scale6 (Vgs=4.5V Qg range:45-53nC)
    ///
    /// 111b = Scale7(Vgs=4.5V Qg range: >53nC)
    #[bits(3, default = DrvStat::Scale3)]
    pub hidrv2_stat: DrvStat,

    /// Enable BATOVP for both charge enable and disable scenarios
    /// including AC+battery and battery only.
    ///
    /// 0b: BATOVP is only active when charge is enabled(BATFET is
    /// turned on) when EN_BATOVP=1b
    ///
    /// 1b: BATOVP is active as long as EN_BATOVP=1b, no matter charge
    /// is enabled or not(BATFET is on or off)
    //
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = BatOvpExtend::Disable)]
    pub batovp_extend: BatOvpExtend,

    #[bits(1, default = false)]
    reserved9: bool,

    /// LODRV1_A and LODRV1_B LS MOSFET gate drive strength adjustment
    /// for both turn on and turn off.
    ///
    /// 000b = Scale0 (Vgs=4.5V Qg range:0-5nC)
    ///
    /// 001b = Scale1(Vgs=4.5V Qg range:5-13nC )
    ///
    /// 010b = Scale2 (Vgs=4.5V Qg range:13-21nC )
    ///
    /// 011b = Scale3(Vgs=4.5V Qg range:21-29nC)
    ///
    /// 100b = Scale4 (Vgs=4.5V Qg range:29-37nC)
    ///
    /// 101b = Scale5(Vgs=4.5V Qg range:37-45nC)
    ///
    /// 110b = Scale6 (Vgs=4.5V Qg range:45-53nC)
    ///
    /// 111b = Scale7(Vgs=4.5V Qg range: >53nC)
    #[bits(3, default = DrvStat::Scale1)]
    pub lodrv1_stat: DrvStat,

    /// HIDRV1_A and HIDRV1_B HS MOSFET gate drive strength adjustment
    /// for both turn on and turn off.
    ///
    /// 000b = Scale0 (Vgs=4.5V Qg range:0-5nC)
    ///
    /// 001b = Scale1(Vgs=4.5V Qg range:5-13nC )
    ///
    /// 010b = Scale2 (Vgs=4.5V Qg range:13-21nC )
    ///
    /// 011b = Scale3(Vgs=4.5V Qg range:21-29nC)
    ///
    /// 100b = Scale4 (Vgs=4.5V Qg range:29-37nC)
    ///
    /// 101b = Scale5(Vgs=4.5V Qg range:37-45nC)
    ///
    /// 110b = Scale6 (Vgs=4.5V Qg range:45-53nC)
    ///
    /// 111b = Scale7(Vgs=4.5V Qg range: >53nC)
    #[bits(3, default = DrvStat::Scale1)]
    pub hidrv1_stat: DrvStat,
}

impl GateDrive {
    pub(crate) const fn addr() -> u8 {
        0x18
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum VsysRegSlow {
    Disable = 0,
    Enable = 1,
}

impl VsysRegSlow {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Disable,
            _ => Self::Enable,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum DrvStat {
    Scale0 = 0,
    Scale1 = 1,
    Scale2 = 2,
    Scale3 = 3,
    Scale4 = 4,
    Scale5 = 5,
    Scale6 = 6,
    Scale7 = 7,
}

impl DrvStat {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Scale0,
            1 => Self::Scale1,
            2 => Self::Scale2,
            3 => Self::Scale3,
            4 => Self::Scale4,
            5 => Self::Scale5,
            6 => Self::Scale6,
            _ => Self::Scale7,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BatOvpExtend {
    Disable = 0,
    Enable = 1,
}

impl BatOvpExtend {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Disable,
            _ => Self::Enable,
        }
    }
}
