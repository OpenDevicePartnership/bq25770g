use bitfield_struct::bitfield;

use super::{AutoChg, RegReset, WdRst, WdTmrAdj};

/// Virtual Control Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct VirtualControl {
    /// WATCHDOG Timer Adjust
    ///
    /// Set maximum delay between consecutive EC host write of charge
    /// voltage or charge current command.
    ///
    /// If device does not receive a write on the CHARGE_VOLTAGE() or
    /// the CHARGE_CURRENT() within the watchdog time period, the
    /// charger will be suspended by setting the CHARGE_CURRENT() to 0
    /// mA. After expiration, the timer will resume upon the write of
    /// CHARGE_CURRENT(), CHARGE_VOLTAGE() ,WDTMR_ADJ or
    /// WD_RST=1b. The charger will resume if the values are valid.
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

    /// Reset watch dog timer control.
    ///
    /// 0b = Normal
    ///
    /// 1b = Reset (bit goes back to 0 after timer reset)
    #[bits(1, default = WdRst::Normal)]
    pub wd_rst: WdRst,

    #[bits(1, default = 0)]
    reserved3: u16,

    /// Enable ILIM_HIZ pin to set input current limit.
    ///
    /// 0b = Disable(Input current limit is set by IIN_HOST())
    ///
    /// 1b = Enable(Input current limit is set by the lower value of
    /// ILIM_HIZ pin and IIN_HOST())
    #[bits(1, default = IlimHiZ::Enable)]
    pub en_extilim: IlimHiZ,

    #[bits(2, default = 0)]
    reserved6_5: u16,

    /// Reset Registers
    ///
    /// All the R/W and R registers go back to the default setting
    /// except: CHRG_STAT, MODE_STAT, HIDRV1_STAT, LODRV1_STAT,
    /// HIDRV2_STAT, LODRV2_STAT
    ///
    /// 0b = Idle
    ///
    /// 1b = Reset
    #[bits(1, default = RegReset::Idle)]
    pub reg_reset: RegReset,

    /// OTG Mode Enable
    ///
    /// Enable device in OTG mode when EN_OTG pin is HIGH.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = Otg::Disable)]
    pub en_otg: Otg,

    #[bits(6, default = 0)]
    reserved14_9: u16,

    /// Automatic charge control
    ///
    /// Recharge and terminate battery charging automatically.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = AutoChg::Disable)]
    en_auto_chg: AutoChg,
}

impl VirtualControl {
    pub(crate) const fn addr() -> u8 {
        0xfd
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum IlimHiZ {
    Disable,
    Enable,
}

impl IlimHiZ {
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
pub enum Otg {
    Disable,
    Enable,
}

impl Otg {
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
