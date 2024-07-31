use bitfield_struct::bitfield;

/// Charge Option 2 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargeOption2 {
    /// Set battery discharge overcurrent threshold as percentage of
    /// PROCHOT battery discharge current limit.
    ///
    /// 0b = 2
    ///
    /// 1b = 3
    #[bits(1, default = BatDocVth::Threshold3)]
    pub batdoc_vth: BatDocVth,

    /// BATDOC Enable
    ///
    /// Battery discharge overcurrent (BATDOC) protection by sensing
    /// the voltage across SRN and SRP. Upon BATDOC, converter is
    /// disabled.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnBatDoc::Enable)]
    pub en_batdoc: EnBatDoc,

    /// ACOC Limit
    ///
    /// Set ACOC threshold as percentage of ILIM2_VTH with current
    /// sensed from RAC.
    ///
    /// 0b = 1.33
    ///
    /// 1b = 2
    #[bits(1, default = AcocVth::Threshold2)]
    pub acoc_vth: AcocVth,

    /// ACOC Enable
    ///
    /// Input overcurrent (ACOC) protection by sensing the voltage
    /// across ACP_A and ACN_A plus ACP_B and ACN_B. Upon ACOC (after
    /// 250-µs blank-out time), converter is disabled.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnAcoc::Enable)]
    pub en_acoc: EnAcoc,

    /// Over current protection threshold by sensing RAC resistor
    /// across voltage. When this fault is continuously triggered 1
    /// switching cycle, converter will be latched off. To re-enable
    /// converter, need to toggle EN_HIZ bit from 0 to 1 and back to
    /// 0.
    ///
    /// 0b = 300 mV (150mV under VSYS_UVP) for Q1_A and Q1_B
    ///
    /// 1b = 450 mV (300mV under VSYS_UVP) for Q1_A and Q1_B
    #[bits(1, default = OcpSw1x::Threshold450mV)]
    pub ocp_sw1x_high_range: OcpSw1x,

    /// Over current protection threshold by sensing Q4 Vds. When this
    /// fault is continuously triggered 1 switching cycle, converter
    /// will be latched off. To re-enable converter, need to toggle
    /// EN_HIZ bit from 0 to 1 and back to 0.
    ///
    /// 0b = 150mV
    ///
    /// 1b = 260mV
    #[bits(1, default = OcpSw2::Threshold260mV)]
    pub ocp_sw2_high_range: OcpSw2,

    /// IBAT pin monitor selection for discharge current and charge current.
    ///
    /// 0b = IBAT pin as Discharge Current
    ///
    /// 1b = IBAT pin as Charge Current
    #[bits(1, default = EnIchgIdchg::Discharge)]
    pub en_ichg_idchg: EnIchgIdchg,

    /// Enable ILIM_HIZ pin to set input current limit.
    ///
    /// 0b = Disable (Input current limit is set by IIN_HOST())
    ///
    /// 1b = Enable (Input current limit is set by the lower value of
    /// ILIM_HIZ pin and IIN_HOST())
    #[bits(1, default = EnExtIlim::Enable)]
    pub en_extilim: EnExtIlim,

    /// Peak power mode overload and relax cycle time.
    ///
    /// 00b = 20ms
    ///
    /// 01b = 40ms
    ///
    /// 10b = 80ms
    ///
    /// 11b = 1s
    #[bits(2, default = PkPwrTmax::Time20ms)]
    pub pkpwr_tmax: PkPwrTmax,

    /// Indicator that the device is in relaxation cycle. Write 0 to
    /// get out of relaxation cycle.
    ///
    /// 0b = Not In Relaxation
    ///
    /// 1b = In Relaxation
    #[bits(1, default = StatPkPwrRelax::NotInRelaxation)]
    pub stat_pkpwr_relax: StatPkPwrRelax,

    /// Indicator that the device is in overloading cycle. Write 0 to
    /// get out of overloading cycle.
    ///
    /// 0b = Not In Peak
    ///
    /// 1b = In Peak
    #[bits(1, default = StatPkPwrOvld::NotInPeak)]
    pub stat_pkpwr_ovld: StatPkPwrOvld,

    /// Enable Peak Power Mode triggered by system voltage
    /// under-shoot. If EN_PKPWR_IIN_DPM and EN_PKPWR_VSYS are 0b,
    /// peak power mode is disabled. Upon adapter removal, this bits
    /// is reset to 0b.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnPkPwrVsys::Disable)]
    pub en_pkpwr_vsys: EnPkPwrVsys,

    /// Enable Peak Power Mode triggered by input current
    /// overshoot. If EN_PKPWR_IIN_DPM and EN_PKPWR_VSYS are 0b, peak
    /// power mode is disabled. Upon adapter removal, this bits is
    /// reset to 0b.
    ///
    /// 0b = Disable
    ///
    /// 1b = Enable
    #[bits(1, default = EnPkPwrIinDpm::Disable)]
    pub en_pkpwr_iin_dpm: EnPkPwrIinDpm,

    /// Input Overload time in Peak Power Mode.
    ///
    /// 00b = 1ms
    ///
    /// 01b = 2ms
    ///
    /// 10b = 5ms
    ///
    /// 11b = 10ms
    #[bits(2, default = PkPwrTovldDeg::Time1ms)]
    pub pkpwr_tovld_deg: PkPwrTovldDeg,
}

impl ChargeOption2 {
    pub(crate) const fn addr() -> u8 {
        0x31
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BatDocVth {
    Threshold2 = 0,
    Threshold3 = 1,
}

impl BatDocVth {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Threshold2,
            _ => Self::Threshold3,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnBatDoc {
    Disable = 0,
    Enable = 1,
}

impl EnBatDoc {
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
pub enum AcocVth {
    Threshold1_33 = 0,
    Threshold2 = 1,
}

impl AcocVth {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Threshold1_33,
            _ => Self::Threshold2,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnAcoc {
    Disable = 0,
    Enable = 1,
}

impl EnAcoc {
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
pub enum OcpSw1x {
    Threshold300mV = 0,
    Threshold450mV = 1,
}

impl OcpSw1x {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Threshold300mV,
            _ => Self::Threshold450mV,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum OcpSw2 {
    Threshold150mV = 0,
    Threshold260mV = 1,
}

impl OcpSw2 {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Threshold150mV,
            _ => Self::Threshold260mV,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnIchgIdchg {
    Discharge = 0,
    Charge = 1,
}

impl EnIchgIdchg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Discharge,
            _ => Self::Charge,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnExtIlim {
    Disable = 0,
    Enable = 1,
}

impl EnExtIlim {
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
pub enum PkPwrTmax {
    Time20ms = 0,
    Time40ms = 1,
    Time80ms = 2,
    Time1000ms = 3,
}

impl PkPwrTmax {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time20ms,
            1 => Self::Time40ms,
            2 => Self::Time80ms,
            _ => Self::Time1000ms,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatPkPwrRelax {
    NotInRelaxation = 0,
    InRelaxation = 1,
}

impl StatPkPwrRelax {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NotInRelaxation,
            _ => Self::InRelaxation,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StatPkPwrOvld {
    NotInPeak = 0,
    InPeak = 1,
}

impl StatPkPwrOvld {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NotInPeak,
            _ => Self::InPeak,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnPkPwrVsys {
    Disable = 0,
    Enable = 1,
}

impl EnPkPwrVsys {
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
pub enum EnPkPwrIinDpm {
    Disable = 0,
    Enable = 1,
}

impl EnPkPwrIinDpm {
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
pub enum PkPwrTovldDeg {
    Time1ms = 0,
    Time2ms = 1,
    Time5ms = 2,
    Time10ms = 3,
}

impl PkPwrTovldDeg {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Time1ms,
            1 => Self::Time2ms,
            2 => Self::Time5ms,
            _ => Self::Time10ms,
        }
    }
}
