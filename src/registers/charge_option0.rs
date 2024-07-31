use bitfield_struct::bitfield;

use super::WdTmrAdj;

/// Charge Option 0 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeOption0 {
    /// Charge Inhibit
    ///
    /// When this bit is 0, battery charging will start with valid
    /// values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
    #[bits(1, default = ChargeInhibit::Enable)]
    pub chrg_inhibit: ChargeInhibit,

    /// IIN_DPM Enable
    ///
    /// Host writes this bit to enable IIN_DPM regulation loop. When
    /// the IIN_DPM is disabled by the charger (refer to
    /// IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this
    /// bit is also used to enable/disable IOTG regulation
    #[bits(1, default = IinDpmEnable::Enable)]
    pub iin_dpm_enable: IinDpmEnable,

    /// LDO Mode Enable
    ///
    /// When battery voltage is below VSYS_MIN(), the charger is in
    /// pre-charge with LDO mode enabled.
    ///
    /// 0b = Disable LDO mode, BATFET fully ON when charge is enabled
    /// and VSYS_MIN() regulation is not effective unless VBAT<5V and
    /// system is regulated at 5V. When charge is disabled, BATFET is
    /// fully off and system is regulated at VBAT+160mV.
    ///
    /// 1b = Enable LDO mode, Precharge current is set by the lower
    /// setting of CHARGE_CURRENT() and IPRECHG(). The system is
    /// regulated by the VSYS_MIN() register.
    #[bits(1, default = LdoModeEnable::Enable)]
    pub en_ldo: LdoModeEnable,

    /// IBAT Amplifier Ratio
    ///
    /// The ratio of voltage on IBAT and voltage across SRP and SRN
    ///
    /// 0b = 8x
    ///
    /// 1b = 64x
    #[bits(1, default = IbatAmplifierRatio::Times64)]
    pub ibat_gain: IbatAmplifierRatio,

    /// IADPT Amplifier Ratio
    ///
    /// The ratio of voltage on IADPT and voltage across ACP and ACN.
    ///
    /// 0b = 20x
    ///
    /// 1b = 40x
    #[bits(1, default = IadptAmplifierRatio::Times20)]
    pub iadpt_gain: IadptAmplifierRatio,

    /// LEARN mode function enable
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = Learn::Disable)]
    pub en_learn: Learn,

    /// Enable system under voltage protection.
    ///
    /// 0b = Enable
    ///
    /// 1b = Disable
    #[bits(1, default = SystemUVP::Enable)]
    pub vsys_uvp_enz: SystemUVP,

    /// Enable Latch of Independent Comparator. Comparator output with
    /// effective low. If enabled in PROCHOT profile PP_CMP=1b,
    /// STAT_COMP bit keep 1b after triggered until read by host and
    /// clear. Host can clear CMPOUT pin by toggling this EN_CMP_LATCH
    /// bit
    ///
    /// 0b = No Latch
    ///
    /// 1b = Latch
    #[bits(1, default = CmpLatch::NoLatch)]
    pub en_cmp_latch: CmpLatch,

    /// Enable battery over voltage protection
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = BatteryOVP::Enable)]
    pub en_batovp: BatteryOVP,

    /// Switching Frequency Selection
    ///
    /// Recommend 600kHz with 2.2uH, 800 kHz with 1.5µH. After charger POR, the
    /// MODE pin programming process will make one time change on frequency
    /// selection.
    ///
    /// Note: Frequency is not allowed to change on the fly has to be changed
    /// when converter is HIZ.
    ///
    /// 0b = 800kHz
    ///
    /// 1b = 600kHz
    #[bits(1, default = PwmFrequency::KHz600)]
    pub pwm_freq: PwmFrequency,

    /// Out-of-Audio Enable
    ///
    /// 0b = No Limit
    ///
    /// 1b = Set minimum PFM frequency above 20 kHz to avoid audio noise
    #[bits(1, default = OutOfAudio::Limit)]
    pub en_ooa: OutOfAudio,

    /// Add OTG to CHRG_OK
    ///
    /// Drive CHRG_OK to HIGH when the device is in OTG mode.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = OtgChrgOk::Disable)]
    pub otg_on_chrgok: OtgChrgOk,

    /// IIN_DPM Auto Disable
    ///
    /// When CELL_BATPRES pin is LOW, the charger automatically disables the
    /// IIN_DPM function by setting EN_IIN_DPM to 0. The host can enable IIN_DPM
    /// function later by writing EN_IIN_DPM bit to 1.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = IinDpmAutoDisable::Disable)]
    pub iin_dpm_auto_disable: IinDpmAutoDisable,

    /// WATCHDOG Timer Adjust
    ///
    /// Set maximum delay between consecutive EC host write of charge voltage or
    /// charge current command. If device does not receive a write on the
    /// CHARGE_VOLTAGE() or the CHARGE_CURRENT() within the watchdog time
    /// period, the charger will be suspended by setting the CHARGE_CURRENT() to
    /// 0 mA. After expiration, the timer will resume upon the write of
    /// CHARGE_CURRENT(), CHARGE_VOLTAGE() ,WDTMR_ADJ or WD_RST=1b. The charger
    /// will resume if the values are valid.
    ///
    /// 00b = Disable
    ///
    /// 01b = 5 sec
    ///
    /// 10b = 88 sec
    ///
    /// 11b = 175 sec
    #[bits(2, default = WdTmrAdj::Seconds175)]
    pub wdtmr_adj: WdTmrAdj,

    /// Low Power Mode enable
    ///
    /// 0b = Disable Low Power Mode. Device in performance mode with battery
    /// only. The PROCHOT, IADPT/IBAT/PSYS and comparator follow corresponding
    /// register setting, REGN should be on with full capacity.
    ///
    /// 1b = Enable Low Power Mode. Device in low power mode with battery only
    /// for lowest quiescent current. The PROCHOT, discharge current monitor
    /// buffer, power monitor buffer and independent comparator are
    /// disabled. ADC is not available in Low Power Mode. Independent comparator
    /// can be enabled by setting EN_LWPWR_CMP to 1b. REGN can be enabled
    /// through EN_REGN_LWPWR=1b with 5mA current capability to save quiescent
    /// current.
    #[bits(1, default = LowPwrMode::Enable)]
    pub en_lwpwr: LowPwrMode,
}

impl ChargeOption0 {
    pub(crate) const fn addr() -> u8 {
        0x12
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChargeInhibit {
    Enable = 0,
    Disable = 1,
}

impl ChargeInhibit {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            1 => Self::Disable,
            _ => Self::Enable,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum IinDpmEnable {
    Disable = 0,
    Enable = 1,
}

impl IinDpmEnable {
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
pub enum LdoModeEnable {
    Disable = 0,
    Enable = 1,
}

impl LdoModeEnable {
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
pub enum IbatAmplifierRatio {
    Times8 = 0,
    Times64 = 1,
}

impl IbatAmplifierRatio {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Times8,
            _ => Self::Times64,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum IadptAmplifierRatio {
    Times20 = 0,
    Times40 = 1,
}

impl IadptAmplifierRatio {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Times20,
            _ => Self::Times40,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Learn {
    Disable = 0,
    Enable = 1,
}

impl Learn {
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
pub enum SystemUVP {
    Enable = 0,
    Disable = 1,
}

impl SystemUVP {
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
pub enum CmpLatch {
    NoLatch = 0,
    Latch = 1,
}

impl CmpLatch {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NoLatch,
            _ => Self::Latch,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BatteryOVP {
    Disable = 0,
    Enable = 1,
}

impl BatteryOVP {
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
pub enum PwmFrequency {
    KHz800 = 0,
    KHz600 = 1,
}

impl PwmFrequency {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::KHz800,
            _ => Self::KHz600,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum OutOfAudio {
    NoLimit = 0,
    Limit = 1,
}

impl OutOfAudio {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NoLimit,
            _ => Self::Limit,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum OtgChrgOk {
    Disable = 0,
    Enable = 1,
}

impl OtgChrgOk {
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
pub enum IinDpmAutoDisable {
    Disable = 0,
    Enable = 1,
}

impl IinDpmAutoDisable {
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
pub enum LowPwrMode {
    Disable,
    Enable,
}

impl LowPwrMode {
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
