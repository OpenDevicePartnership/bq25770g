use bitfield_struct::bitfield;

/// Charge Option 0 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeOption4 {
    /// PTM operation status bit monitor.
    ///
    /// 0b = Not Active
    ///
    /// 1b = Active
    #[bits(1, default = StatPtm::Inactive)]
    pub stat_ptm: StatPtm,

    /// The status is latched until a read from host.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatIdchg2::NotTriggered)]
    pub stat_idchg2: StatIdchg2,

    /// Enable IDCHG_TH2 PROCHOT Profile.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPIdchg2::Disable)]
    pub pp_idchg2: PPIdchg2,

    /// Battery discharge current limit2 based on percentage of
    /// IDCHG_TH1. Note IDCHG_TH2 setting higher than 40A should lose
    /// accuracy derating between target value and 40A.
    ///
    /// 000b = 125%*IDCHG_TH1
    ///
    /// 001b = 150%*IDCHG_TH1
    ///
    /// 010b = 175%*IDCHG_TH1
    ///
    /// 011b = 200%*IDCHG_TH1
    ///
    /// 100b = 250%*IDCHG_TH1
    ///
    /// 101b = 300%*IDCHG_TH1
    ///
    /// 110b = 350%*IDCHG_TH1
    ///
    /// 111b = 400%*IDCHG_TH1
    #[bits(3, default = Idchg2Threshold::Threshold150)]
    pub idchg_th2: Idchg2Threshold,

    /// Battery discharge current limit 2 deglitch time.
    ///
    /// 00b = 81us(min)/98us(Typ.)/115us(max)
    ///
    /// 01b = 1.3ms(min)/1.55ms(Typ.)/1.8ms(max)
    ///
    /// 10b = 5.2ms(min)/6.25ms(Typ.)/7.3ms(max)
    ///
    /// 11b = 10.4ms(min)/12.5ms(Typ.)/14.6ms(max)
    #[bits(2, default = IdchgDeg2::Time1550us)]
    pub idchg_deg2: IdchgDeg2,

    /// STAT_VBUS_VAP.
    ///
    /// 0b = Not Triggered
    ///
    /// 1b = Triggered
    #[bits(1, default = StatVbusVap::NotTriggered)]
    pub stat_vbus_vap: StatVbusVap,

    /// Enable VBUS_VAP PROCHOT Profile.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = PPVbusVap::Disable)]
    pub pp_vbus_vap: PPVbusVap,

    /// Disable VSYS_UVP Hiccup mode operation.
    ///
    /// 0b = Hiccup Mode Enabled
    ///
    /// 1b = Hiccup Mode Disabled
    #[bits(1, default = Hiccup::Disable)]
    pub vsys_uvp_no_hiccup: Hiccup,

    /// Frequency Dither configuration.
    ///
    /// 00b = Disable
    ///
    /// 01b = 1X
    ///
    /// 10b = 2X
    ///
    /// 11b = 3X
    #[bits(2, default = Dither::Disable)]
    pub en_dither: Dither,

    /// VSYS Under Voltage Lock Out After UVP is triggered the charger
    /// enters hiccup mode, and then the charger is latched off if the
    /// restart fails 7 times in 90s The hiccup mode during the UVP
    /// can be disabled by setting VSYS_UVP_NO_HICCUP=1b.
    ///
    /// 000b = 2.4V
    ///
    /// 001b = 3.2V
    ///
    /// 010b = 4.0V
    ///
    /// 011b = 4.8V
    ///
    /// 100b = 5.6V
    ///
    /// 101b = 6.4V
    ///
    /// 110b = 7.2V
    ///
    /// 111b = 8.0V
    #[bits(3, default = VsysUvp::MilliVolt2400)]
    pub vsys_uvp: VsysUvp,
}

impl ChargeOption4 {
    pub(crate) const fn addr() -> u8 {
        0x36
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatPtm {
    Inactive = 0,
    Active = 1,
}

impl StatPtm {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Inactive,
            _ => Self::Active,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatIdchg2 {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatIdchg2 {
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
pub enum PPIdchg2 {
    Disable = 0,
    Enable = 1,
}

impl PPIdchg2 {
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
pub enum Idchg2Threshold {
    Threshold125 = 0,
    Threshold150 = 1,
    Threshold175 = 2,
    Threshold200 = 3,
    Threshold250 = 4,
    Threshold300 = 5,
    Threshold350 = 6,
    Threshold400 = 7,
}

impl Idchg2Threshold {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Threshold125,
            1 => Self::Threshold150,
            2 => Self::Threshold175,
            3 => Self::Threshold200,
            4 => Self::Threshold250,
            5 => Self::Threshold300,
            6 => Self::Threshold350,
            _ => Self::Threshold400,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum IdchgDeg2 {
    Time98us = 0,
    Time1550us = 1,
    Time6250us = 2,
    Time12500us = 3,
}

impl IdchgDeg2 {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time98us,
            1 => Self::Time1550us,
            2 => Self::Time6250us,
            _ => Self::Time12500us,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatVbusVap {
    NotTriggered = 0,
    Triggered = 1,
}

impl StatVbusVap {
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
pub enum PPVbusVap {
    Disable = 0,
    Enable = 1,
}

impl PPVbusVap {
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
pub enum Hiccup {
    Disable = 0,
    Enable = 1,
}

impl Hiccup {
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
pub enum Dither {
    Disable = 0,
    Dither1x = 1,
    Dither2x = 2,
    Dither3x = 3,
}

impl Dither {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Disable,
            1 => Self::Dither1x,
            2 => Self::Dither2x,
            _ => Self::Dither3x,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum VsysUvp {
    MilliVolt2400,
    MilliVolt3200,
    MilliVolt4000,
    MilliVolt4800,
    MilliVolt5600,
    MilliVolt6400,
    MilliVolt7200,
    MilliVolt8000,
}

impl VsysUvp {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::MilliVolt2400,
            1 => Self::MilliVolt3200,
            2 => Self::MilliVolt4000,
            3 => Self::MilliVolt4800,
            4 => Self::MilliVolt5600,
            5 => Self::MilliVolt6400,
            6 => Self::MilliVolt7200,
            _ => Self::MilliVolt8000,
        }
    }
}
