use bitfield_struct::bitfield;

/// Prochot Option 1 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ProchotOption1 {
    /// Adapter removal PROCHOT profile enable.
    ///
    /// If PP_ACOK is enabled in PROCHOT after the adapter is removed,
    /// it will be pulled low.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPAcok::Disable)]
    pub pp_acock: PPAcok,

    /// Battery removal PROCHOT profile enable.
    ///
    /// If PP_BATPRES is enabled in PROCHOT after the battery is
    /// removed, it will immediately send out one-shot PROCHOT pulse.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPBatPres::Disable)]
    pub pp_batpres: PPBatPres,

    /// VSYS PROCHOT profile enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPVsys::Disable)]
    pub pp_vsys: PPVsys,

    /// IDCHG1 PROCHOT profile enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPIdchg1::Disable)]
    pub pp_idchg1: PPIdchg1,

    /// INOM PROCHOT profile enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPInom::Disable)]
    pub pp_inom: PPInom,

    /// ICRIT PROCHOT profile enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPIcrit::Disable)]
    pub pp_icrit: PPIcrit,

    /// COMP PROCHOT profile enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPCmp::Disable)]
    pub pp_cmp: PPCmp,

    /// VINDPM PROCHOT profile enable.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPVinDpm::Disable)]
    pub pp_vindpm: PPVinDpm,

    /// IDCHG Deglitch Time.
    ///
    /// 00b = 69ms(min) / 78ms(Typ.) / 93.6ms(max)
    ///
    /// 01b = 1.1sec(min) / 1.25sec(Typ.) / 1.4sec(max)
    ///
    /// 10b = 4.4sec(min) / 5sec(Typ.) / 5.6sec(max)
    ///
    /// 11b = 17.5sec(min) / 20sec(Typ.) / 22.3sec(max)
    #[bits(2, default = IdchgDeg1::Time1250ms)]
    pub idchg_deg1: IdchgDeg1,

    /// IDCHG level 1 Threshold
    ///
    /// 6 bit, range, range 1500A to 33A(5mΩ RSR), step 500 mA. There
    /// is a 1500 mA offset for all code
    ///
    /// Measure current between SRN and SRP.
    ///
    /// Trigger when the discharge current is above the threshold.
    ///
    /// If the value is programmed to 000000b PROCHOT is always
    /// triggered.
    ///
    /// Default: 9500 mA or 010000b POR: 9500mA (10h)
    ///
    /// Range: 1500mA-33000mA (0h-3Fh)
    ///
    /// Bit Step: 500mA
    ///
    /// Offset: 1500mA
    #[bits(6, default = 0x10)]
    pub idchg_th1: u16,
}

impl ProchotOption1 {
    pub(crate) const fn addr() -> u8 {
        0x34
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PPAcok {
    Disable = 0,
    Enable = 1,
}

impl PPAcok {
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
pub enum PPBatPres {
    Disable = 0,
    Enable = 1,
}

impl PPBatPres {
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
pub enum PPVsys {
    Disable = 0,
    Enable = 1,
}

impl PPVsys {
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
pub enum PPIdchg1 {
    Disable = 0,
    Enable = 1,
}

impl PPIdchg1 {
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
pub enum PPInom {
    Disable = 0,
    Enable = 1,
}

impl PPInom {
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
pub enum PPIcrit {
    Disable = 0,
    Enable = 1,
}

impl PPIcrit {
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
pub enum PPCmp {
    Disable = 0,
    Enable = 1,
}

impl PPCmp {
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
pub enum PPVinDpm {
    Disable = 0,
    Enable = 1,
}

impl PPVinDpm {
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
pub enum IdchgDeg1 {
    Time78ms = 0,
    Time1250ms = 1,
    Time5000ms = 2,
    Time20000ms = 3,
}

impl IdchgDeg1 {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time78ms,
            1 => Self::Time1250ms,
            2 => Self::Time5000ms,
            _ => Self::Time20000ms,
        }
    }
}
