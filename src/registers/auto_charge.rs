use bitfield_struct::bitfield;

use super::AutoChg;

/// Auto Charge Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AutoCharge {
    /// ACOV protection threshold adjustment.
    ///
    /// 00b = 20V(15V SPR)
    ///
    /// 01b = 25V(20V SPR)
    ///
    /// 10b = 33V(28V EPR)
    ///
    /// 11b = 41V(36V EPR)
    #[bits(2, default = AcovProtection::Threshold25V)]
    pub acov_adj: AcovProtection,

    /// Adjust TREG thermal deglitch time to trigger prochot profile
    /// pull down pulse.
    ///
    /// 0b = 0.76sec (min) / 0.965sec (Typ.) / 1.17sec (max)
    ///
    /// 1b = 95.3ms (min) / 121ms (Typ.) / 146ms (max)
    #[bits(1, default = TRegDeg::Time965ms)]
    pub thermal_deg: TRegDeg,

    /// PROCHOT profile status bit for TREG thermal overheat
    /// (CMPIN_TR< 1.1V). The status is latched until a read from
    /// host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatThermal::NotTriggered)]
    pub stat_thermal: StatThermal,

    /// Enable temperature regulation(TREG) for PROCHOT profile.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPThermal::Disable)]
    pub pp_thermal: PPThermal,

    /// Enable temperature regulation function and pull down CMPOUT
    /// pin to GND if CMPIN_TR_SELECT=1b. If CMPIN_TR_SELECT=0b, then
    /// EN_TREG will not be effective.
    ///
    /// 0b = Disable temperature regulation function
    ///
    /// 1b = Enable temperature regulation function
    #[bits(1, default = EnTReg::Disable)]
    pub en_treg: EnTReg,

    /// Enable charge safety timer.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnChgTmr::Enable)]
    pub en_chg_tmr: EnChgTmr,

    /// Charge Safety Timer speed control.
    ///
    /// Note changing the state of EN_TMR2X only impacts the rate at
    /// which the counter is counting and has no effect on any
    /// existing accumulated count
    ///
    /// 0b = Timer always counts normally
    ///
    /// 1b = Timer slowed by 2x during VINDPM/IINDPM/TREG regulation
    #[bits(1, default = EnTmr2x::Slow)]
    pub en_tmr2x: EnTmr2x,

    /// Automatic Charge Safety Timer control.
    ///
    /// 00b = 5hr
    ///
    /// 01b = 8hr
    ///
    /// 10b = 12hr
    ///
    /// 11b = 24hr
    #[bits(2, default = ChgTmr::Timer8h)]
    pub chg_tmr: ChgTmr,

    /// Battery automatic recharge threshold below CHARGE_VOLTAGE():
    /// POR: 50mV (0h)
    ///
    /// Range: 50mV-800mV (0h-Fh)
    ///
    /// Bit Step: 50mV
    ///
    /// Offset: 50mV
    ///
    /// Mode: 2s
    ///
    /// 200mV
    ///
    /// POR: 200mV (3h) Mode: 3s
    ///
    /// 300mV
    ///
    /// POR: 300mV (5h) Mode: 4s
    ///
    /// 400mV
    ///
    /// POR: 400mV (7h) Mode: 5s
    ///
    /// 500mV
    ///
    /// POR: 500mV (9h)
    #[bits(4, default = 0)]
    pub vrechg: u16,

    /// Enable CHRG_OK pin for interrupt function.
    ///
    /// 0b = Disable (CHRG_OK pin is not pulled low when CHRG_STAT
    /// bits changes)
    ///
    /// 1b = Enable (CHRG_OK pin is pulled low for minimum 256us when
    /// CHRG_STAT bits changes)
    #[bits(1, default = ChrgOkInt::Disable)]
    pub chrg_ok_int: ChrgOkInt,

    /// Automatic charge control (recharge and terminate battery
    /// charging automatically).
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = AutoChg::Disable)]
    pub en_auto_chg: AutoChg,
}

impl AutoCharge {
    pub(crate) const fn addr() -> u8 {
        0x1a
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AcovProtection {
    Threshold20V,
    Threshold25V,
    Threshold33V,
    Threshold41V,
}

impl AcovProtection {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Threshold20V,
            1 => Self::Threshold25V,
            2 => Self::Threshold33V,
            _ => Self::Threshold41V,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TRegDeg {
    Time965ms,
    Time121ms,
}

impl TRegDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time965ms,
            _ => Self::Time121ms,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatThermal {
    NotTriggered,
    Triggered,
}

impl StatThermal {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NotTriggered,
            _ => Self::Triggered,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PPThermal {
    Disable,
    Enable,
}

impl PPThermal {
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
pub enum EnTReg {
    Disable,
    Enable,
}

impl EnTReg {
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
pub enum EnChgTmr {
    Disable,
    Enable,
}

impl EnChgTmr {
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
pub enum EnTmr2x {
    Normal,
    Slow,
}

impl EnTmr2x {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            _ => Self::Slow,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChgTmr {
    Timer5h,
    Timer8h,
    Timer12h,
    Timer24h,
}

impl ChgTmr {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Timer5h,
            1 => Self::Timer8h,
            2 => Self::Timer12h,
            _ => Self::Timer24h,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChrgOkInt {
    Disable,
    Enable,
}

impl ChrgOkInt {
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
