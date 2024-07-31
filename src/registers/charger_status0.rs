use bitfield_struct::bitfield;

/// Charger Status 0 Register
#[bitfield(u16)]
#[derive(PartialEq)]
pub struct ChargerStatus0 {
    #[bits(3, default = 0)]
    reserved2_0: u16,

    /// The status are latched until a read from host, if the fault
    /// still exist during host read this bit should be kept at
    /// 1b. However after host read fault status one time, this bit
    /// will be automatically reset when the original fault is
    /// cleared. In this way host doesn't need to read again to clear
    /// this fault bit after fault is removed.
    ///
    /// 0b = No Fault
    ///
    /// 1b = Fault
    #[bits(1, default = Fault::NoFault)]
    pub fault_regn: Fault,

    #[bits(1, default = 0)]
    reserved4: u16,

    /// The status are latched until a read from host, if the fault
    /// still exist during host read this bit should be kept at
    /// 1b. However after host read fault status one time, this bit
    /// will be automatically reset when the original fault is
    /// cleared.
    ///
    /// 0b = No Fault
    ///
    /// 1b = Fault
    #[bits(1, default = Fault::NoFault)]
    pub fault_ocp: Fault,

    #[bits(1, default = 0)]
    reserved6: u16,

    /// The status are latched until a read from host, if the fault
    /// still exist during host read this bit should be kept at
    /// 1b. However after host read fault status one time, this bit
    /// will be automatically reset when the original fault is
    /// cleared. In this way host doesn't need to read again to clear
    /// this fault bit after fault is removed.
    ///
    /// 0b = No Fault
    ///
    /// 1b = Fault
    #[bits(1, default = Fault::NoFault)]
    pub fault_batovp: Fault,

    /// MODE pin program status.
    ///
    /// 000b = Quasi Dual Phase/Normal Compensation/Fsw-600kHz
    ///
    /// 001b = Quasi Dual Phase/Normal Compensation/Fsw-800kHz
    ///
    /// 010b = Quasi Dual Phase/Slow Compensation/Fsw-600kHz
    ///
    /// 011b = Quasi Dual Phase/Slow Compensation/Fsw-800kHz
    ///
    /// 100b = NA/Normal Compensation/Fsw-600kHz
    ///
    /// 101b = NA/Normal Compensation/Fsw-800kHz
    ///
    /// 110b = NA/Slow Compensation/Fsw-600kHz
    ///
    /// 111b = NA/Slow Compensation/Fsw-800kHz
    #[bits(3, default = ModeStat::QuasiDualNormalCompFsw600kHz)]
    pub mode_stat: ModeStat,

    /// Temperature regulation status.
    ///
    /// 0b = Not in temperature regulation(TREG)
    ///
    /// 1b = In temperature regulation(TREG)
    #[bits(1, default = TRegStat::NotRegulating)]
    pub treg_stat: TRegStat,

    /// Charge safety timer status.
    ///
    /// 0b = Normal
    ///
    /// 1b = Charge safety timer expired
    #[bits(1, default = ChgTmrStat::Normal)]
    pub chg_tmr_stat: ChgTmrStat,

    /// Charge Cycle Status.
    ///
    /// 000b = Not Charging
    ///
    /// 001b = Trickle Charge (VBAT<VBAT_SHORT)
    ///
    /// 010b = Pre-Charge (VBAT<VSYS_MIN)
    ///
    /// 011b = Fast Charge(CC mode)
    ///
    /// 100b = Fast Charge(CV mode)
    ///
    /// 101b = Reserve1
    ///
    /// 110b = Reserve2
    ///
    /// 111b = Charge Termination Done
    #[bits(3, default = ChrgStat::NotCharging)]
    pub chrg_stat: ChrgStat,
}

impl ChargerStatus0 {
    pub(crate) const fn addr() -> u8 {
        0x1b
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Fault {
    NoFault,
    Fault,
}

impl Fault {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NoFault,
            _ => Self::Fault,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ModeStat {
    QuasiDualNormalCompFsw600kHz,
    QuasiDualNormalCompFsw800kHz,
    QuasiDualSlowCompFsw600kHz,
    QuasiDualSlowCompFsw800kHz,
    NormalCompFsw600kHz,
    NormalCompFsw800kHz,
    SlowCompFsw600kHz,
    SlowCompFsw800kHz,
}

impl ModeStat {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::QuasiDualNormalCompFsw600kHz,
            1 => Self::QuasiDualNormalCompFsw800kHz,
            2 => Self::QuasiDualNormalCompFsw600kHz,
            3 => Self::QuasiDualNormalCompFsw800kHz,
            4 => Self::NormalCompFsw600kHz,
            5 => Self::NormalCompFsw800kHz,
            6 => Self::SlowCompFsw600kHz,
            _ => Self::SlowCompFsw800kHz,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TRegStat {
    NotRegulating,
    Regulating,
}

impl TRegStat {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NotRegulating,
            _ => Self::Regulating,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChgTmrStat {
    Normal,
    Expired,
}

impl ChgTmrStat {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            _ => Self::Expired,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChrgStat {
    NotCharging = 0,
    TrickleCharge = 1,
    PreCharge = 2,
    FastChargeCC = 3,
    FastChargeCV = 4,
    ChargeTerminationDone = 7,
}

impl ChrgStat {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::NotCharging,
            1 => Self::TrickleCharge,
            2 => Self::PreCharge,
            3 => Self::FastChargeCC,
            4 => Self::FastChargeCV,
            _ => Self::ChargeTerminationDone,
        }
    }
}
