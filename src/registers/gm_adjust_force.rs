use bitfield_struct::bitfield;

/// GM Adjust Force Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct GmAdjustForce {
    /// Enable FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B effective for
    /// inductor DCR current sense.
    ///
    /// Converter will automatically shuts off and restart when
    /// FORCE_UPDATE bit is written from 0b to 1b to update these
    /// force values. When converter restarts from other reasons, as
    /// long as this bit is 1b, converter will follow the
    /// FORCE_AUTO_TUNE_A/B value and there is no auto calibration at
    /// beginning anymore.
    ///
    /// 0b = Disable FORCE_AUTOTUNE_A,FORCE_AUTOTUNE_B
    ///
    /// 1b = Enable FORCE_AUTOTUNE_A,FORCE_AUTOTUNE_B
    #[bits(1, default = ForceAutotune::Enable)]
    pub force_autotune_en: ForceAutotune,

    /// Enable FORCE_GM_ADJUST effective for inductor DCR current
    /// sense.
    ///
    /// Converter will automatically shuts off and restart when
    /// FORCE_UPDATE bit is written from 0b to 1b to update these
    /// force values. When converter restarts from other reason, as
    /// long as this bit is 1b, converter will force GM_ADJUST =
    /// FORCE_GM_ADJUST+1 as fixed value.
    ///
    /// 0b = Disable FORCE_GM_ADJUST
    ///
    /// 1b = Enable FORCE_GM_ADJUST
    #[bits(1, default = ForceGmAdjust::Enable)]
    pub force_gm_adjust_en: ForceGmAdjust,

    /// Force GM adjustment value for inductor DCR.
    ///
    /// GM_ADJUST=71.25-272/DCR(mΩ)
    ///
    /// Default value 0x31 refers to 12.2mΩ
    #[bits(6, default = 0x31)]
    pub force_gm_adjust: u16,

    #[bits(1, default = 0)]
    reserved8: u16,

    /// Update FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B, FORCE_GM_ADJUST
    /// value to be effective for inductor DCR current sense.
    ///
    /// Converter will automatically shuts off and restart when this
    /// bit is written from 0b to 1b to login new force values. After
    /// one time update completes, the converter automatically
    /// recovers switching and reset this bit to 0b.
    ///
    /// 0b = Idle
    ///
    /// 1b = Update FORCE_AUTOTUNE_A,FORCE_AUTOTUNE_B, FORCE_GM_ADJUST
    /// values to converter
    #[bits(1, default = ForceUpdate::Idle)]
    pub force_update: ForceUpdate,

    /// Auto adaptive adjustment value for inductor DCR. This value is
    /// 0 when converter shuts off. If converter starts switching and
    /// FORCE_GM_ADJUST_EN=1b then GM_ADJUST=FORCE_GM_ADJUST+1 as
    /// fixed value.
    #[bits(6, default = 0)]
    pub gm_adjust: u16,
}

impl GmAdjustForce {
    pub(crate) const fn addr() -> u8 {
        0x62
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ForceAutotune {
    Disable,
    Enable,
}

impl ForceAutotune {
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
pub enum ForceGmAdjust {
    Disable,
    Enable,
}

impl ForceGmAdjust {
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
pub enum ForceUpdate {
    Idle,
    Update,
}

impl ForceUpdate {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Idle,
            _ => Self::Update,
        }
    }
}
