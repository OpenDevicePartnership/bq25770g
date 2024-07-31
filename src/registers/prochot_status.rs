use bitfield_struct::bitfield;

/// Prochot Status Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ProchotStatus {
    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatAdapterRemoval::NotTriggered)]
    pub stat_adapter_removal: StatAdapterRemoval,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatBatteryRemoval::NotTriggered)]
    pub stat_battery_removal: StatBatteryRemoval,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatVsys::NotTriggered)]
    pub stat_vsys_: StatVsys,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatIdchg1::NotTriggered)]
    pub stat_idchg1_: StatIdchg1,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatInom::NotTriggered)]
    pub stat_inom_: StatInom,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatIcrit::NotTriggered)]
    pub stat_icrit_: StatIcrit,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatComp::NotTriggered)]
    pub stat_comp_: StatComp,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatVinDpm::NotTriggered)]
    pub stat_vindpm_: StatVinDpm,

    /// When the charger is operated in VAP mode, it can exit VAP by
    /// either being disabled through host, or there is any charger
    /// faults.
    ///
    /// 0b = PROCHOT_EXIT_VAP is not active
    ///
    /// 1b = PROCHOT_EXIT_VAP is active, PROCHOT pin is low until host
    /// writes this status bit to 0
    #[bits(1, default = StatExitVap::NotActive)]
    pub stat_exit_vap: StatExitVap,

    /// This status bit reports a failure to load VBUS 7 consecutive
    /// times in VAP mode, which indicates the battery voltage might
    /// be not high enough to enter VAP mode, or the VAP loading
    /// current settings are too high.
    ///
    /// 0b = Not is VAP failure
    ///
    /// 1b = In VAP failure, the charger exits VAP mode, and latches
    /// off until the host writes this bit to 0.
    #[bits(1, default = StatVapFail::NoFailure)]
    pub stat_vap_fail: StatVapFail,

    /// TSHUT trigger.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = Tshut::NotTriggered)]
    pub tshut: Tshut,

    /// PROCHOT Pulse Clear.
    ///
    /// Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
    ///
    /// 0b = Clear PROCHOT pulse and drive /PROCHOT pin HIGH
    ///
    /// 1b = Idle
    #[bits(1, default = ProchotClear::Idle)]
    pub prochot_clear: ProchotClear,

    /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
    ///
    /// 00b = 83ms(min)/100ms(Typ.)/117ms(max)
    ///
    /// 01b = 42ms(min)/50ms(Typ.)/58ms(max)
    ///
    /// 10b = 5ms(min)/6.15ms(Typ.)/7.3ms(max)
    ///
    /// 11b = 10ms(min)/12.5ms(Typ.)/15ms(max)
    #[bits(2, default = ProchotWidth::Width12_5ms)]
    pub prochot_width: ProchotWidth,

    /// PROCHOT Pulse Extension Enable. When pulse extension is
    /// enabled, keep the PROCHOT pin voltage LOW until host writes
    /// PROCHOT_CLEAR= 0b.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnProchotExt::Disable)]
    pub en_prochot_ext: EnProchotExt,

    #[bits(1, default = 0)]
    reserved15: u16,
}

impl ProchotStatus {
    pub(crate) const fn addr() -> u8 {
        0x21
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatAdapterRemoval {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatAdapterRemoval {
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
pub enum StatBatteryRemoval {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatBatteryRemoval {
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
pub enum StatVsys {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatVsys {
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
pub enum StatIdchg1 {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatIdchg1 {
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
pub enum StatInom {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatInom {
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
pub enum StatIcrit {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatIcrit {
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
pub enum StatComp {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatComp {
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
pub enum StatVinDpm {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatVinDpm {
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
pub enum StatExitVap {
    NotActive = 0,
    Active = 1,
}

impl StatExitVap {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NotActive,
            _ => Self::Active,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatVapFail {
    NoFailure = 0,
    Fail = 1,
}

impl StatVapFail {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NoFailure,
            _ => Self::Fail,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Tshut {
    NotTriggered = 0,
    Triggered = 1,
}

impl Tshut {
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
pub enum ProchotClear {
    Clear = 0,
    Idle = 1,
}

impl ProchotClear {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Clear,
            _ => Self::Idle,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ProchotWidth {
    Width100ms = 0,
    Width58ms = 1,
    Width6_15ms = 2,
    Width12_5ms = 3,
}

impl ProchotWidth {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Width100ms,
            1 => Self::Width58ms,
            2 => Self::Width6_15ms,
            _ => Self::Width12_5ms,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnProchotExt {
    Disable = 0,
    Enable = 1,
}

impl EnProchotExt {
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
