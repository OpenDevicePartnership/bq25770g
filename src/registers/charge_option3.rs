use bitfield_struct::bitfield;

use super::RegReset;

/// Charge Option 3 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeOption3 {
    /// PSYS definition during OTG mode.
    //
    /// 0b = PSYS as battery discharge power minus OTG output power
    ///
    /// 1b = PSYS as battery discharge power only
    #[bits(1, default = PsysOtgIdchg::BatteryMinusOtg)]
    pub psys_otg_idchg: PsysOtgIdchg,

    /// Turn off BATFET during HIZ mode.
    ///
    /// 0b = On
    ///
    /// 1b = Off
    #[bits(1, default = BatFetOffHiZ::Off)]
    pub batfetoff_hiz: BatFetOffHiZ,

    /// Enable Independent Comparator with effective low.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = CmpEn::Enable)]
    pub cmp_en: CmpEn,

    /// 4 levels inductor average current clamp.
    ///
    /// 00b = 10A
    ///
    /// 01b = 18A
    ///
    /// 10b = 24A
    ///
    /// 11b = Disable (internal 30A limit)
    #[bits(2, default = IlAvg::Disable)]
    pub il_avg: IlAvg,

    /// The selection of the external EN_OTG pin control.
    ///
    /// 0b = VAP Mode
    ///
    /// 1b = OTG Mode
    #[bits(1, default = OtgVapMode::OtgMode)]
    pub otg_vap_mode: OtgVapMode,

    #[bits(1, default = 0)]
    reserved6: u16,

    /// Turn off BATFET under battery only low power mode. When not in
    /// low power mode like OTG or with AC plugged in, the bit
    /// configuration is neglected and not effective.
    ///
    /// 0b = Not force turn off BATFET
    ///
    /// 1b = Force turn off BATFET
    #[bits(1, default = BatFetEnZ::NoForceOff)]
    pub batfet_enz: BatFetEnZ,

    /// VSYS_MIN soft slew rate control for VSYS_MIN step up
    /// transition. Note for step down doesn't need the soft
    /// transition.
    ///
    /// 00b = Disable
    ///
    /// 01b = 6.25mV/us
    ///
    /// 10b = 3.125mV/us
    ///
    /// 11b = 1.5625mV/us
    #[bits(2, default = EnVsysMinSoftSr::SlewRate6_25)]
    pub en_vsys_min_soft_sr: EnVsysMinSoftSr,

    /// Enable BATFET control for dual port application.
    ///
    /// 0b = Disable BATFET control by HIZ BATDRV pin
    ///
    /// 1b = Enable BATFET control by active BATDRV pin
    #[bits(1, default = EnPortCtrl::Enable)]
    pub en_port_ctrl: EnPortCtrl,

    /// Enable ICO Algorithm.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnIcoMode::Disable)]
    pub en_ico_mode: EnIcoMode,

    /// OTG Mode Enable
    ///
    /// Enable device in OTG mode when EN_OTG pin is HIGH.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnOtg::Disable)]
    pub en_otg: EnOtg,

    /// Set VINDPM threshold based on VBUS measurement result minus
    /// 1.28V, Converter is disabled to measure VBUS. After VBUS
    /// measurement is done, VINDPM() is written with value
    /// VBUS-1.28V. Then this bit goes back to 0 and converter starts.
    ///
    /// 0b = Idle
    ///
    /// 1b = Measure VIN, write VIN-1.28V to VINDPM
    #[bits(1, default = DetectVinDpm::Idle)]
    pub detect_vindpm: DetectVinDpm,

    /// Reset Registers
    ///
    /// All the R/W and R registers go back to the default setting
    /// except: CHRG_STAT, MODE_STAT, HIDRV1_STAT, LODRV1_STAT,
    /// HIDRV2_STAT, LODRV2_STAT, PWM_FREQ.
    ///
    /// 0b = Idle
    ///
    /// 1b = Reset
    #[bits(1, default = RegReset::Idle)]
    pub reg_reset: RegReset,

    /// Device Hi-Z Mode Enable
    ///
    /// When the charger is in Hi-Z mode, the device draws minimal
    /// quiescent current. With VBUS above UVLO. REGN LDO stays on,
    /// and system powers from battery.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnHiZ::Disable)]
    pub en_hiz: EnHiZ,
}

impl ChargeOption3 {
    pub(crate) const fn addr() -> u8 {
        0x32
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PsysOtgIdchg {
    BatteryMinusOtg = 0,
    BatteryOnly = 1,
}

impl PsysOtgIdchg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::BatteryMinusOtg,
            _ => Self::BatteryOnly,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BatFetOffHiZ {
    Off = 0,
    On = 1,
}

impl BatFetOffHiZ {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Off,
            _ => Self::On,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum CmpEn {
    Disable = 0,
    Enable = 1,
}

impl CmpEn {
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
pub enum IlAvg {
    Limit10A = 0,
    Limit18A = 1,
    Limit24A = 2,
    Disable = 3,
}

impl IlAvg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Limit10A,
            1 => Self::Limit18A,
            2 => Self::Limit24A,
            _ => Self::Disable,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum OtgVapMode {
    VapMode = 0,
    OtgMode = 1,
}

impl OtgVapMode {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::VapMode,
            _ => Self::OtgMode,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BatFetEnZ {
    NoForceOff = 0,
    ForceOff = 1,
}

impl BatFetEnZ {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NoForceOff,
            _ => Self::ForceOff,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnVsysMinSoftSr {
    Disable = 0,
    SlewRate6_25 = 1,
    SlewRate3_125 = 2,
    SlewRate1_5625 = 3,
}

impl EnVsysMinSoftSr {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Disable,
            1 => Self::SlewRate6_25,
            2 => Self::SlewRate3_125,
            _ => Self::SlewRate1_5625,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnPortCtrl {
    Disable = 0,
    Enable = 1,
}

impl EnPortCtrl {
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
pub enum EnIcoMode {
    Disable = 0,
    Enable = 1,
}

impl EnIcoMode {
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
pub enum EnOtg {
    Disable = 0,
    Enable = 1,
}

impl EnOtg {
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
pub enum DetectVinDpm {
    Idle = 0,
    Measure = 1,
}

impl DetectVinDpm {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Idle,
            _ => Self::Measure,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnHiZ {
    Disable = 0,
    Enable = 1,
}

impl EnHiZ {
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
