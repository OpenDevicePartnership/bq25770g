use bitfield_struct::bitfield;

use super::WdRst;

/// Charge Option 0 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeOption5 {
    /// Adjust dual phase to single phase (phase dropping transition)
    /// deglitch time.
    ///
    /// 00b = 70us (Min) / 93us (Typ) / 115us (Max)
    ///
    /// 01b = 1.12ms (Min) / 1.5ms (Typ) / 1.82ms (Max)
    ///
    /// 10b = 8.94ms (Min) / 11ms (Typ) / 1.46ms (Max)
    ///
    /// 11b = 71.5ms (Min) / 94ms (Typ) / 117ms (Max)
    #[bits(2, default = PhDropDeg::Time1500us)]
    pub ph_drop_deg: PhDropDeg,

    /// Adjust single phase to dual phase (phase adding transition)
    /// deglitch time.
    ///
    /// 00b = 0.727us(Min)/1.7us(Typ)/2.67us(Max)
    ///
    /// 01b = 2.91us(Min)/5.5us(Typ)/8us(Max)
    ///
    /// 10b = 11.6us(Min)/20us(Typ)/29.3us(Max)
    ///
    /// 11b = 46.6us(Min)/86us(Typ)/115us(Max)
    #[bits(2, default = PhAddDeg::Time5500ns)]
    pub ph_add_deg: PhAddDeg,

    /// Force single phase operation under buck mode when quasi dual
    /// phase is chosen through MODE pin programming.
    ///
    /// 0b = Automatically transit to dual phase based on
    /// SINGLE_DUAL_TRANS_TH threshold option
    ///
    /// 1b = Force Single Phase under buck mode
    #[bits(1, default = ForceSingle::Disable)]
    pub force_single: ForceSingle,

    /// Buck mode single to dual phase transition threshold adjustment
    /// based on output load current: (When Quasi dual phase is chosen
    /// at MODE pin programming).
    ///
    /// Note from dual phase to single phase transition the load
    /// current threshold is 1A lower than this configuration as
    /// hysteresis.
    ///
    /// 000b = Force Dual Phase Operation
    ///
    /// 001b = 3A
    ///
    /// 010b = 4A
    ///
    /// 011b = 5A
    ///
    /// 100b = 6A
    ///
    /// 101b = 7A
    ///
    /// 110b = 8A
    ///
    /// 111b = 9A
    #[bits(3, default = BuckTransitionThreshold::Current6A)]
    pub single_dual_trans_th: BuckTransitionThreshold,

    /// Enable high duty cycle buck mode to maximum buck mode
    /// operation range to keep Q4 constant on without boost
    /// switching.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = HighDutyBuck::Disable)]
    pub high_duty_buck: HighDutyBuck,

    /// Disable BATCOC and configure BATCOC thresholds across SRP-SRN.
    ///
    /// 00b = Disable
    ///
    /// 01b = 50mV
    ///
    /// 10b = 75mV
    ///
    /// 11b = 100mV
    #[bits(2, default = BatCocConfig::MiliVolts100)]
    pub batcoc_config: BatCocConfig,

    /// Enable REGN with scale down current 5mA capability under
    /// battery only and low power mode.
    ///
    /// 0b = Disabled REGN under battery only low power mode
    ///
    /// 1b = Enable REGN under battery only low power mode
    #[bits(1, default = RegnLwPwr::Disable)]
    pub en_reg_lwpwr: RegnLwPwr,

    /// Enable external 5V overdrive for REGN.
    ///
    /// 0b = Disabled external 5V over drive
    ///
    /// 1b = Enable external 5V over drive
    #[bits(1, default = RegnExt::Disable)]
    pub regn_ext: RegnExt,

    /// CPMIN_TS pin function selection.
    ///
    /// 0b = CPMIN function
    ///
    /// 1b = TREG function
    #[bits(1, default = Function::Cmpin)]
    pub cmpin_tr_select: Function,

    /// Reset watch dog timer control.
    ///
    /// 0b = Normal
    ///
    /// 1b = Reset (bit goes back to 0 after timer reset)
    #[bits(1, default = WdRst::Normal)]
    pub wd_rst: WdRst,

    /// Enable PTM auto exit under light load.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PtmAutoExit::Disable)]
    pub ptm_exit_light_load: PtmAutoExit,
}

impl ChargeOption5 {
    pub(crate) const fn addr() -> u8 {
        0x19
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PhDropDeg {
    Time93us,
    Time1500us,
    Time11000us,
    Time94000us,
}

impl PhDropDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time93us,
            1 => Self::Time1500us,
            2 => Self::Time11000us,
            _ => Self::Time94000us,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PhAddDeg {
    Time1700ns,
    Time5500ns,
    Time20000ns,
    Time86000ns,
}

impl PhAddDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time1700ns,
            1 => Self::Time5500ns,
            2 => Self::Time20000ns,
            _ => Self::Time86000ns,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ForceSingle {
    Disable,
    Enable,
}

impl ForceSingle {
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
pub enum BuckTransitionThreshold {
    ForceDual,
    Current3A,
    Current4A,
    Current5A,
    Current6A,
    Current7A,
    Current8A,
    Current9A,
}

impl BuckTransitionThreshold {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::ForceDual,
            1 => Self::Current3A,
            2 => Self::Current4A,
            3 => Self::Current5A,
            4 => Self::Current6A,
            5 => Self::Current7A,
            6 => Self::Current8A,
            _ => Self::Current9A,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum HighDutyBuck {
    Disable,
    Enable,
}

impl HighDutyBuck {
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
pub enum BatCocConfig {
    Disable,
    MiliVolts50,
    MiliVolts75,
    MiliVolts100,
}

impl BatCocConfig {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Disable,
            1 => Self::MiliVolts50,
            2 => Self::MiliVolts75,
            _ => Self::MiliVolts100,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum RegnLwPwr {
    Disable,
    Enable,
}

impl RegnLwPwr {
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
pub enum RegnExt {
    Disable,
    Enable,
}

impl RegnExt {
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
pub enum Function {
    Cmpin,
    Treg,
}

impl Function {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Cmpin,
            _ => Self::Treg,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PtmAutoExit {
    Disable,
    Enable,
}

impl PtmAutoExit {
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
