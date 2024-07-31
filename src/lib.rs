//! This is a platform-agnostic Rust driver for the BQ2577x series of Battery
//! Charger ICs based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//! For further details of the device architecture and operation, please refer
//! to the official [`Datasheet`].
//!
//! [`Datasheet`]: https://www.ti.com/lit/gpn/bq25770g

#![doc(html_root_url = "https://docs.rs/bq2577x/latest")]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

use embedded_hal_async::i2c::I2c;
use registers::*;

pub mod registers;

pub struct Bq2577x<I2C: I2c> {
    /// The concrete I2C bus instance
    i2c: I2C,

    /// This device's I2C address
    addr: u8,
}

impl<I2C: I2c> Bq2577x<I2C> {
    const ADDR: u8 = 0x09;

    pub fn new(i2c: I2C) -> Self {
        Self { i2c, addr: Self::ADDR }
    }

    async fn read(&mut self, reg: u8) -> Result<u16, I2C::Error> {
        let mut bytes = [0; 2];
        self.i2c.write_read(self.addr, &[reg], &mut bytes).await?;

        Ok(u16::from_be_bytes(bytes))
    }

    async fn write(&mut self, reg: u8, value: u16) -> Result<(), I2C::Error> {
        let mut bytes = [0u8; 3];
        let content = value.to_be_bytes();

        bytes[0] = reg;
        bytes[1] = content[0];
        bytes[2] = content[1];

        self.i2c.write(self.addr, &[reg]).await?;

        Ok(())
    }
}

macro_rules! impl_read {
    ($method:ident, $r:ty) => {
        impl<I2C: I2c> Bq2577x<I2C> {
            pub async fn $method(&mut self) -> Result<$r, I2C::Error> {
                Ok(<$r>::from_bits(self.read(<$r>::addr()).await?))
            }
        }
    };
}

macro_rules! impl_write {
    ($method:ident, $r:ty) => {
        impl<I2C: I2c> Bq2577x<I2C> {
            pub async fn $method(&mut self, reg: $r) -> Result<(), I2C::Error> {
                self.write(<$r>::addr(), reg.into_bits()).await
            }
        }
    };
}

impl_read!(charge_option0, ChargeOption0);
impl_write!(set_charge_option0, ChargeOption0);

impl_read!(charge_current, ChargeCurrent);
impl_write!(set_charge_current, ChargeCurrent);

impl_read!(charge_voltage, ChargeVoltage);
impl_write!(set_charge_voltage, ChargeVoltage);

impl_read!(charge_profile, ChargeProfile);
impl_write!(set_charge_profile, ChargeProfile);

impl_read!(gate_drive, GateDrive);
impl_write!(set_gate_drive, GateDrive);

impl_read!(charge_option5, ChargeOption5);
impl_write!(set_charge_option5, ChargeOption5);

impl_read!(auto_charge, AutoCharge);
impl_write!(set_auto_charge, AutoCharge);

impl_read!(charger_status0, ChargerStatus0);

impl_read!(charger_status1, ChargerStatus1);
impl_write!(set_charger_status1, ChargerStatus1);

impl_read!(prochot_status, ProchotStatus);
impl_write!(set_prochot_status, ProchotStatus);

impl_read!(iin_dpm, IinDpm);
impl_read!(adc_vbus, AdcVbus);
impl_read!(adc_ibat, AdcIbat);
impl_read!(adc_iin, AdcIin);
impl_read!(adc_vsys, AdcVsys);
impl_read!(adc_vbat, AdcVbat);
impl_read!(adc_psys, AdcPsys);
impl_read!(adc_cmpin_tr, AdcCmpinTr);

impl_read!(charge_option1, ChargeOption1);
impl_write!(set_charge_option1, ChargeOption1);

impl_read!(charge_option2, ChargeOption2);
impl_write!(set_charge_option2, ChargeOption2);

impl_read!(charge_option3, ChargeOption3);
impl_write!(set_charge_option3, ChargeOption3);

impl_read!(prochot_option0, ProchotOption0);
impl_write!(set_prochot_option0, ProchotOption0);

impl_read!(prochot_option1, ProchotOption1);
impl_write!(set_prochot_option1, ProchotOption1);

impl_read!(adc_option, AdcOption);
impl_write!(set_adc_option, AdcOption);

impl_read!(charge_option4, ChargeOption4);
impl_write!(set_charge_option4, ChargeOption4);

impl_read!(vmin_active_protection, VminActiveProtection);
impl_write!(set_vmin_active_protection, VminActiveProtection);

impl_read!(otg_voltage, OtgVoltage);
impl_write!(set_otg_voltage, OtgVoltage);

impl_read!(otg_current, OtgCurrent);
impl_write!(set_otg_current, OtgCurrent);

impl_read!(vin_dpm, VinDpm);
impl_write!(set_vin_dpm, VinDpm);

impl_read!(vsys_min, VsysMin);
impl_write!(set_vsys_min, VsysMin);

impl_read!(iin_host, IinHost);
impl_write!(set_iin_host, IinHost);

impl_read!(autotune_read, AutotuneRead);
impl_write!(set_autotune_read, AutotuneRead);

impl_read!(autotune_force, AutotuneForce);
impl_write!(set_autotune_force, AutotuneForce);

impl_read!(gm_adjust_force, GmAdjustForce);
impl_write!(set_gm_adjust_force, GmAdjustForce);

impl_read!(virtual_control, VirtualControl);
impl_write!(set_virtual_control, VirtualControl);

impl_read!(device, Device);
impl_read!(manufacturer, Manufacturer);
