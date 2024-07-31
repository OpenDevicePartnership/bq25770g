use bitfield_struct::bitfield;

/// VMin Active Protection Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct VminActiveProtection {
    /// Fast Role Swap Feature Enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = FastRoleSwap::Disable)]
    pub en_frs: FastRoleSwap,

    /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting
    /// register VSYS_TH2 setting.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = Follow::Disable)]
    pub en_vsysth2_follow_vsysth1: Follow,

    /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
    ///
    /// POR: 5900mV (9h)
    ///
    /// Range: 5000mV-11300mV (0h-3Fh)
    ///
    /// Bit Step: 100mV
    ///
    /// Offset: 5000mV
    #[bits(6, default = 9)]
    pub vsys_th2: u16,

    /// Disable BATOVP 20mA discharge current through VSYS pin.
    ///
    /// 0b = Discharge 20mA under BATOVP
    ///
    /// 1b = Not discharge 20mA under BATOVP
    #[bits(1, default = BatOvpDchrg::Dchrg20mA)]
    pub dis_batovp_20ma: BatOvpDchrg,

    /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
    ///
    /// POR: 3200mV (0h)
    ///
    /// Range: 3200mV-15900mV (0h-7Fh)
    ///
    /// Bit Step: 100mV
    ///
    /// Offset: 3200mV
    #[bits(7, default = 0)]
    pub vbus_vap_th: u16,
}

impl VminActiveProtection {
    pub(crate) const fn addr() -> u8 {
        0x37
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum FastRoleSwap {
    Disable,
    Enable,
}

impl FastRoleSwap {
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
pub enum Follow {
    Disable,
    Enable,
}

impl Follow {
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
pub enum BatOvpDchrg {
    Dchrg20mA,
    NoDchrg20mA,
}

impl BatOvpDchrg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Dchrg20mA,
            _ => Self::NoDchrg20mA,
        }
    }
}
