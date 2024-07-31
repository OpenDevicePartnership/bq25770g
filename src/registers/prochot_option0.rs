use bitfield_struct::bitfield;

/// Prochot Option 0
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ProchotOption0 {
    /// Enable lower threshold of PROCHOT_VINDPM comparator.
    ///
    /// 0b = PROCHOT_VINDPM follows VINDPM REG0x3D setting
    ///
    /// 1b = PROCHOT_VINDPM is lowered and determined by
    /// PROCHOT_VINDPM_80_90 bit setting
    #[bits(1, default = LowerProchotVinDpm::Enable)]
    pub lower_prohot_vindpm: LowerProchotVinDpm,

    /// INOM deglitch time.
    ///
    /// 0b = 0.84ms (min) / 0.988ms (typ.) / 1.14ms (max)
    ///
    /// 1b = 54ms (min) / 64ms (typ.) / 73ms (max)
    #[bits(1, default = InomDeg::Time9880us)]
    pub inom_deg: InomDeg,

    /// VSYS threshold to trigger discharging VBUS in VAP mode.
    ///
    /// POR: 6400mV (Eh)
    ///
    /// Range: 5000mV-11300mV (0h-3Fh)
    ///
    /// Bit Step: 100mV
    ///
    /// Offset: 5000mV
    #[bits(6, default = 0xe)]
    pub vsys_th1: u16,

    /// Lower threshold of the PROCHOT_VINDPM comparator.
    ///
    /// When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM
    /// is determined by this setting.
    ///
    /// 0b = 83% of VINDPM
    ///
    /// 1b = 91% of VINDPM
    #[bits(1, default = ProchotVinDpm::VinDpm83)]
    pub prochot_vindpm_80_90: ProchotVinDpm,

    /// ICRIT deglitch time to trigger PROCHOT.
    ///
    ///  00b = 12us(Min) / 14.5us(Typ.) / 17us(Max)
    ///
    ///  01b = 93us(Min) / 111us(Typ.) / 129us(Max)
    ///
    ///  10b = 372us(Min) / 443us(Typ.) / 513us(Max)
    ///
    ///  11b = 745us(Min) / 873us(Typ.) / 1000us(Max)
    #[bits(2, default = IcritDeg::Time111us)]
    pub icrit_deg: IcritDeg,

    /// ILIM2 Threshold.
    ///
    /// 00000b = OutOfRange_0x00
    ///
    /// 00001b = 110_percent
    ///
    /// 00010b = 115_percent
    ///
    /// 00011b = 120_percent
    ///
    /// 00100b = 125_percent
    ///
    /// 00101b = 130_percent
    ///
    /// 00110b = 135_percent
    ///
    /// 00111b = 140_percent
    ///
    /// 01000b = 145_percent
    ///
    /// 01001b = 150_percent
    ///
    /// 01010b = 155_percent
    ///
    /// 01011b = 160_percent
    ///
    /// 01100b = 165_percent
    ///
    /// 01101b = 170_percent
    ///
    /// 01110b = 175_percent
    ///
    /// 01111b = 180_percent
    ///
    /// 10000b = 185_percent
    ///
    /// 10001b = 190_percent
    ///
    /// 10010b = 195_percent
    ///
    /// 10011b = 200_percent
    ///
    /// 10100b = 205_percent
    ///
    /// 10101b = 210_percent
    ///
    /// 10110b = 215_percent
    ///
    /// 10111b = 220_percent
    ///
    /// 11000b = 225_percent
    ///
    /// 11001b = 230_percent
    ///
    /// 11010b = 250_percent
    ///
    /// 11011b = 300_percent
    ///
    /// 11100b = 350_percent
    ///
    /// 11101b = 400_percent
    ///
    /// 11110b = 450_percent
    ///
    /// 11111b = OutOfRange_0x1F
    #[bits(5, default = Ilim2Vth::Percent150)]
    pub ilim2_vth: Ilim2Vth,
}

impl ProchotOption0 {
    pub(crate) const fn addr() -> u8 {
        0x33
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LowerProchotVinDpm {
    Disable = 0,
    Enable = 1,
}

impl LowerProchotVinDpm {
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
pub enum InomDeg {
    Time9880us = 0,
    Time64000us = 1,
}

impl InomDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time9880us,
            _ => Self::Time64000us,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ProchotVinDpm {
    VinDpm83 = 0,
    VinDpm91 = 1,
}

impl ProchotVinDpm {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::VinDpm83,
            _ => Self::VinDpm91,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum IcritDeg {
    Time14_5us = 0,
    Time111us = 1,
    Time443us = 2,
    Time873us = 3,
}

impl IcritDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time14_5us,
            1 => Self::Time111us,
            2 => Self::Time443us,
            _ => Self::Time873us,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Ilim2Vth {
    Percent110 = 1,
    Percent115 = 2,
    Percent120 = 3,
    Percent125 = 4,
    Percent130 = 5,
    Percent135 = 6,
    Percent140 = 7,
    Percent145 = 8,
    Percent150 = 9,
    Percent155 = 10,
    Percent160 = 11,
    Percent165 = 12,
    Percent170 = 13,
    Percent175 = 14,
    Percent180 = 15,
    Percent185 = 16,
    Percent190 = 17,
    Percent195 = 18,
    Percent200 = 19,
    Percent205 = 20,
    Percent210 = 21,
    Percent215 = 22,
    Percent220 = 23,
    Percent225 = 24,
    Percent230 = 25,
    Percent250 = 26,
    Percent300 = 27,
    Percent350 = 28,
    Percent400 = 29,
    Percent450 = 30,
}

impl Ilim2Vth {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            1 => Self::Percent110,
            2 => Self::Percent115,
            3 => Self::Percent120,
            4 => Self::Percent125,
            5 => Self::Percent130,
            6 => Self::Percent135,
            7 => Self::Percent140,
            8 => Self::Percent145,
            9 => Self::Percent150,
            10 => Self::Percent155,
            11 => Self::Percent160,
            12 => Self::Percent165,
            13 => Self::Percent170,
            14 => Self::Percent175,
            15 => Self::Percent180,
            16 => Self::Percent185,
            17 => Self::Percent190,
            18 => Self::Percent195,
            19 => Self::Percent200,
            20 => Self::Percent205,
            21 => Self::Percent210,
            22 => Self::Percent215,
            23 => Self::Percent220,
            24 => Self::Percent225,
            25 => Self::Percent230,
            26 => Self::Percent250,
            27 => Self::Percent300,
            28 => Self::Percent350,
            29 => Self::Percent400,
            _ => Self::Percent450,
        }
    }
}
