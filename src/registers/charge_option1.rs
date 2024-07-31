use bitfield_struct::bitfield;

/// Charge Option 1 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeOption1 {
    /// SC_VBUSACP protection enable register bit.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = VbusAcp::Enable)]
    pub en_sc_vbusacp: VbusAcp,

    /// Discharge SRN for Shipping Mode
    ///
    /// Used to discharge SRN pin capacitor voltage which is necessary
    /// for battery gauge device shipping mode. When this bit is 1,
    /// discharge SRN pin down in 340 ms with around 20mA current
    /// flowing through VSYS pin. When 340 ms is over, this bit is
    /// reset to 0 automatically. If this bit is written to 0b by host
    /// before 340ms expires, VSYS pin should stop discharging
    /// immediately. After SRN is discharged to 0V the discharge
    /// current will shut off automatically in order to get rid of any
    /// negative voltage on SRN pin.
    ///
    /// Note if after 340ms SRN voltage is still not low enough for
    /// battery gauge device entering ship mode, the host may need to
    /// write this bit to 1b again to start a new 340ms discharge
    /// cycle.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = ShipDchg::Disable)]
    pub en_ship_dchg: ShipDchg,

    /// PTM enable register bit, it will automatically reset to zero.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = Ptm::Disable)]
    pub en_ptm: Ptm,

    /// Force Power Path Off
    ///
    /// When independent comparator triggers, charger turns off Q1 and
    /// Q4 (same as disable converter) so that the system is
    /// disconnected from the input source. At the same time, CHRG_OK
    /// signal goes to LOW to notify the system. It should be
    /// effective during forward mode with AC plugged in or battery
    /// only performance mode. Both FRC_CONV_OFF and CMP_EN should be
    /// 1b to enable this feature. No need for EN_LWPWR, EN_LWPWR_CMP
    /// to be high which are employed under battery only low power
    /// mode.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = FrcConvOff::Disable)]
    pub frc_conv_off: FrcConvOff,

    /// Independent comparator deglitch time, only applied to the
    /// falling edge of CMPOUT (HIGH to LOW).
    ///
    /// 00b = 1us (Not in battery only low power mode)/ 40us (Battery
    /// only low power mode).
    ///
    /// 01b = 2.05ms~2.73ms
    ///
    /// 10b = 20.85ms~27.31ms
    ///
    /// 11b = 5.34s~6.99s
    #[bits(2, default = CmpDeg::Time1us)]
    pub cmp_deg: CmpDeg,

    /// Independent Comparator output Polarity.
    ///
    /// 0b = When CMPIN_TR is above internal threshold, CMPOUT is LOW
    /// (internal hysteresis)
    ///
    /// 1b = When CMPIN_TR is below internal threshold, CMPOUT is LOW
    /// (external hysteresis)
    #[bits(1, default = CmpPol::ActiveLow)]
    pub cmp_pol: CmpPol,

    /// Force SYSOVP protection threshold to 27V neglecting
    /// CELL_BATPRES pin configuration.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = SysOvpMax::Disable)]
    pub sysovp_max: SysOvpMax,

    /// Enable OTG compensation for VBUS effective capacitance larger
    /// than 60uF.
    ///
    /// 0b = Disable OTG large VBUS capacitance compensation
    /// (Recommended for VBUS effective capacitance smaller than 60uF
    /// effective capacitance)
    ///
    /// 1b = Enable OTG large VBUS capacitance compensation
    /// (Recommended for VBUS effective capacitance larger than 60uF
    /// effective capacitance)
    #[bits(1, default = EnOtgBigCap::Disable)]
    pub en_otg_big_cap: EnOtgBigCap,

    /// PSYS Gain
    ///
    /// Ratio of PSYS output current vs total input and battery power.
    ///
    /// 0b = 0.25 uA/W
    ///
    /// 1b = 1.00 uA/W
    #[bits(1, default = PsysGain::Gain1_00)]
    pub psys_ratio: PsysGain,

    /// Charge sense resistor RSR. Not recommend change this value
    /// during ICHG/IPRECHG/BATFET_CLAMP1/BATFET_CLAMP2/BAT_SHORT
    /// regulation.
    ///
    /// Under adapter plugged in: make changes right after converter
    /// starts up with light loading and before charge is enabled.
    ///
    /// With battery only : make changes before EN_OTG pin is pulled up
    ///
    /// 0b = 5 mOhms
    ///
    /// 1b = 2 mOhms
    #[bits(1, default = RsnsRsr::MilliOhms5)]
    pub rsns_rsr: RsnsRsr,

    /// Input sense resistor RAC. Not recommend change this value
    /// during IINDPM/IOTG regulation: Under adapter plugged in: make
    /// changes right after converter starts up with light loading and
    /// before charge is enabled.
    ///
    /// With battery only: make changes before EN_OTG pin is pulled up.
    ///
    /// 0b = 10 mOhms
    ///
    /// 1b = 5 mOhms
    #[bits(1, default = RsnsRac::MilliOhms10)]
    pub rsns_rac: RsnsRac,

    /// PSYS Enable and Definition Register.
    ///
    /// Enable PSYS sensing circuit and output buffer (whole PSYS
    /// circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and
    /// buffer are always disabled regardless of this bit value.
    ///
    /// 00b = PBUS+PBAT
    ///
    /// 01b = PBUS
    ///
    /// 10b = RESERVED
    ///
    /// 11b = OFF
    #[bits(2, default = PsysConfig::Off)]
    pub psys_config: PsysConfig,

    /// Independent Comparator Enable
    ///
    /// Enable independent comparator under battery only low power
    /// mode (EN_LWPWR=1b).
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnLwPwrCmp::Disable)]
    pub en_lwpwr_cmp: EnLwPwrCmp,

    /// IBAT Enable
    ///
    /// Enable the IBAT output buffer. In low power mode
    /// (EN_LWPWR=1b), IBAT buffer is always disabled regardless of
    /// this bit value.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnIbat::Disable)]
    pub en_ibat: EnIbat,
}

impl ChargeOption1 {
    pub(crate) const fn addr() -> u8 {
        0x30
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum VbusAcp {
    Disable = 0,
    Enable = 1,
}

impl VbusAcp {
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
pub enum ShipDchg {
    Disable = 0,
    Enable = 1,
}

impl ShipDchg {
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
pub enum Ptm {
    Disable = 0,
    Enable = 1,
}

impl Ptm {
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
pub enum FrcConvOff {
    Disable = 0,
    Enable = 1,
}

impl FrcConvOff {
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
pub enum CmpDeg {
    Time1us = 0,
    Time20500us = 1,
    Time20850us = 2,
    Time5340000us = 3,
}

impl CmpDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time1us,
            1 => Self::Time20500us,
            2 => Self::Time20850us,
            _ => Self::Time5340000us,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum CmpPol {
    ActiveLow = 0,
    ActiveHigh = 1,
}

impl CmpPol {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::ActiveLow,
            _ => Self::ActiveHigh,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum SysOvpMax {
    Disable = 0,
    Enable = 1,
}

impl SysOvpMax {
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
pub enum EnOtgBigCap {
    Disable = 0,
    Enable = 1,
}

impl EnOtgBigCap {
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
pub enum PsysGain {
    Gain0_25 = 0,
    Gain1_00 = 1,
}

impl PsysGain {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Gain0_25,
            _ => Self::Gain1_00,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum RsnsRsr {
    MilliOhms5 = 0,
    MilliOhms2 = 1,
}

impl RsnsRsr {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::MilliOhms5,
            _ => Self::MilliOhms2,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum RsnsRac {
    MilliOhms10 = 0,
    MilliOhms5 = 1,
}

impl RsnsRac {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::MilliOhms10,
            _ => Self::MilliOhms5,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PsysConfig {
    PbusPbat = 0,
    Pbus = 1,
    Off = 3,
}

impl PsysConfig {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::PbusPbat,
            1 => Self::Pbus,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnLwPwrCmp {
    Disable = 0,
    Enable = 1,
}

impl EnLwPwrCmp {
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
pub enum EnIbat {
    Disable = 0,
    Enable = 1,
}

impl EnIbat {
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
