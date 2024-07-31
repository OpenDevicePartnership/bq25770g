mod adc_cmpin_tr;
mod adc_ibat;
mod adc_iin;
mod adc_option;
mod adc_psys;
mod adc_vbat;
mod adc_vbus;
mod adc_vsys;
mod auto_charge;
mod autotune_force;
mod autotune_read;
mod charge_current;
mod charge_option0;
mod charge_option1;
mod charge_option2;
mod charge_option3;
mod charge_option4;
mod charge_option5;
mod charge_profile;
mod charge_voltage;
mod charger_status0;
mod charger_status1;
mod device;
mod gate_drive;
mod gm_adjust_force;
mod iin_dpm;
mod iin_host;
mod manufacturer;
mod otg_current;
mod otg_voltage;
mod prochot_option0;
mod prochot_option1;
mod prochot_status;
mod vin_dpm;
mod virtual_control;
mod vmin_active_protection;
mod vsys_min;

pub use adc_cmpin_tr::*;
pub use adc_ibat::*;
pub use adc_iin::*;
pub use adc_option::*;
pub use adc_psys::*;
pub use adc_vbat::*;
pub use adc_vbus::*;
pub use adc_vsys::*;
pub use auto_charge::*;
pub use autotune_force::*;
pub use autotune_read::*;
pub use charge_current::*;
pub use charge_option0::*;
pub use charge_option1::*;
pub use charge_option2::*;
pub use charge_option3::*;
pub use charge_option4::*;
pub use charge_option5::*;
pub use charge_profile::*;
pub use charge_voltage::*;
pub use charger_status0::*;
pub use charger_status1::*;
pub use device::*;
pub use gate_drive::*;
pub use gm_adjust_force::*;
pub use iin_dpm::*;
pub use iin_host::*;
pub use manufacturer::*;
pub use otg_current::*;
pub use otg_voltage::*;
pub use prochot_option0::*;
pub use prochot_option1::*;
pub use prochot_status::*;
pub use vin_dpm::*;
pub use virtual_control::*;
pub use vmin_active_protection::*;
pub use vsys_min::*;

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum WdTmrAdj {
    Disable,
    Seconds5,
    Seconds88,
    Seconds175,
}

impl WdTmrAdj {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Disable,
            1 => Self::Seconds5,
            2 => Self::Seconds88,
            _ => Self::Seconds175,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum WdRst {
    Normal,
    Reset,
}

impl WdRst {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            _ => Self::Reset,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AutoChg {
    Disable,
    Enable,
}

impl AutoChg {
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
pub enum RegReset {
    Idle = 0,
    Reset = 1,
}

impl RegReset {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Idle,
            _ => Self::Reset,
        }
    }
}
