use bitfield_struct::bitfield;

/// ADC Option Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct AdcOption {
    /// Enable SRN pin Voltage ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcVbat::Disable)]
    pub en_adc_vbat: EnAdcVbat,

    /// Enable VSYS pin Voltage ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcVsys::Disable)]
    pub en_adc_vsys: EnAdcVsys,

    /// Enable ICHG ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcIbat::Disable)]
    pub en_adc_ibat: EnAdcIbat,

    #[bits(1, default = 0)]
    reserved3: u16,

    /// Enable IIN ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcIin::Disable)]
    pub en_adc_iin: EnAdcIin,

    /// Enable PSYS pin Voltage ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcPsys::Disable)]
    pub en_adc_psys: EnAdcPsys,

    /// Enable VBUS pin Voltage ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcVbus::Disable)]
    pub en_adc_vbus: EnAdcVbus,

    /// Enable CMPIN_TR pin Voltage ADC Channel.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAdcCmpin::Disable)]
    pub en_adc_cmpin: EnAdcCmpin,

    #[bits(2, default = 0)]
    reserved9_8: u16,

    /// ADC average initial value control.
    ///
    /// 0b = Start average using existing register value
    ///
    /// 1b = Start average using new ADC conversion
    #[bits(1, default = AdcAvgInit::UseExisting)]
    pub adc_avg_init: AdcAvgInit,

    /// ADC average control.
    ///
    /// 0b = Single Value
    ///
    /// 1b = Running average
    #[bits(1, default = AdcAvg::SingleValue)]
    pub adc_avg: AdcAvg,

    /// ADC sample resolution selection, each channel conversion time
    /// is also determined based on resolution.
    ///
    /// 00b = 15 bits effective resolution(24ms conversion time per
    /// channel)
    ///
    /// 01b = 14 bits effective resolution(12ms conversion time per
    /// channel)
    ///
    /// 10b = 13 bits effective resolution(6ms conversion time per
    /// channel)
    ///
    /// 11b = Reserved
    #[bits(2, default = AdcSample::Bits14)]
    pub adc_sample: AdcSample,

    /// ADC conversion enable command.
    ///
    /// Under one-shot ADC configuration ADC_RATE=0b, After the
    /// one-shot update is complete, this bit automatically resets to
    /// zero.
    ///
    /// 0b = Idle
    ///
    /// 1b = Start
    #[bits(1, default = AdcEn::Disable)]
    pub adc_en: AdcEn,

    /// ADC conversion type selection.
    ///
    /// Typical conversion time is determined by resolution accuracy.
    ///
    /// 0b = Continuous update. Cycling set of conversion updates to
    /// ADC registers without break. The total period of whole set is
    /// determined by the ADC channel enabled count times conversion
    /// time for each channel determined by ADC_SAMPLE setting.
    ///
    /// 1b = One-shot update. Do one set of conversion updates to ADC
    /// registers after ADC_START =1. The total period of whole set is
    /// determined by the ADC channel enabled count times conversion
    /// time for each channel determined by ADC_SAMPLE setting.
    #[bits(1, default = AdcRate::OneShot)]
    pub adc_rate: AdcRate,
}

impl AdcOption {
    pub(crate) const fn addr() -> u8 {
        0x35
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnAdcVbat {
    Disable = 0,
    Enable = 1,
}

impl EnAdcVbat {
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
pub enum EnAdcVsys {
    Disable = 0,
    Enable = 1,
}

impl EnAdcVsys {
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
pub enum EnAdcIbat {
    Disable = 0,
    Enable = 1,
}

impl EnAdcIbat {
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
pub enum EnAdcIin {
    Disable = 0,
    Enable = 1,
}

impl EnAdcIin {
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
pub enum EnAdcPsys {
    Disable = 0,
    Enable = 1,
}

impl EnAdcPsys {
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
pub enum EnAdcVbus {
    Disable = 0,
    Enable = 1,
}

impl EnAdcVbus {
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
pub enum EnAdcCmpin {
    Disable = 0,
    Enable = 1,
}

impl EnAdcCmpin {
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
pub enum AdcAvgInit {
    UseExisting = 0,
    StartNewAverage = 1,
}

impl AdcAvgInit {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::UseExisting,
            _ => Self::StartNewAverage,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AdcAvg {
    SingleValue = 0,
    RunningAvg = 1,
}

impl AdcAvg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::SingleValue,
            _ => Self::RunningAvg,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AdcSample {
    Bits15 = 0,
    Bits14 = 1,
    Bits13 = 2,
}

impl AdcSample {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Bits15,
            1 => Self::Bits14,
            _ => Self::Bits13,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AdcEn {
    Disable = 0,
    Enable = 1,
}

impl AdcEn {
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
pub enum AdcRate {
    Continuous = 0,
    OneShot = 1,
}

impl AdcRate {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Continuous,
            _ => Self::OneShot,
        }
    }
}
