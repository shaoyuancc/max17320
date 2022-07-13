//! An embedded hal driver for the MAX17320 (2S-4S ModelGauge m5 Fuel Gauge with Protector, Internal Self-Discharge Detection and SHA-256 Authentication)
#![cfg_attr(not(test), no_std)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![allow(dead_code)]

mod config;
mod error;
mod i2c_interface;
mod register;

pub use config::*;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use error::Error;
use register::*;

/// MAX17320 interface
#[derive(Debug, Clone, Copy)]
pub struct MAX17320<I2C: Write + WriteRead> {
    com: I2C,
    address: u8,
    address_nvm: u8,
    r_sense: f32,
}

impl<I2C, E> MAX17320<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
{
    /// Create new driver interface. r_sense is in mΩ.
    pub fn new(i2c: I2C, r_sense_mohm: f32) -> Result<Self, Error<E>> {
        MAX17320::with_addresses(i2c, 0x36, 0x0B, r_sense_mohm)
    }

    /// Create new driver interface with specific I2C address. r_sense is in mΩ.
    pub fn with_addresses(
        i2c: I2C,
        address: u8,
        address_nvm: u8,
        r_sense_mohm: f32,
    ) -> Result<Self, Error<E>> {
        let chip = Self {
            com: i2c,
            address,
            address_nvm,
            r_sense: r_sense_mohm,
        };
        Ok(chip)
    }

    /// Read the device name
    pub fn read_device_name(&mut self) -> Result<u16, Error<E>> {
        let name = self.read_named_register(Register::DevName)?;
        Ok(name)
    }

    /// Read alert status and chip status
    pub fn read_status(&mut self) -> Result<u16, Error<E>> {
        let val = self.read_named_register(Register::Status)?;
        Ok(val)
    }
    /// Read reported remaining capacity (mAh)
    pub fn read_capacity(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::RepCap)?;
        Ok(convert_to_capacity(raw, self.r_sense))
    }
    /// Read reported state of charge (%)
    pub fn read_state_of_charge(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::RepSoc)?;
        Ok(convert_to_percentage(raw))
    }
    /// Read the cell voltage for a single cell (v)
    pub fn read_vcell(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::VCell)?;
        Ok(convert_to_voltage(raw))
    }
    /// Read temperature (°C)
    pub fn read_temp(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Temp)? as i16;
        Ok(convert_to_temperature(raw))
    }
    /// Read internal die temperature (°C)
    pub fn read_die_temp(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::DieTemp)? as i16;
        Ok(convert_to_temperature(raw))
    }

    /// Read battery current (mA)
    pub fn read_current(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Current)? as i16;
        Ok(convert_to_current(raw, self.r_sense))
    }
    /// Read time to empty (seconds)
    pub fn read_time_to_empty(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::TimeToEmpty)?;
        Ok(convert_to_time(raw))
    }
    /// Read time to full (seconds)
    pub fn read_time_to_full(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::TimeToFull)?;
        Ok(convert_to_time(raw))
    }
    /// Read fault status of the protection functionality
    pub fn read_protection_status(&mut self) -> Result<u16, Error<E>> {
        let val = self.read_named_register(Register::ProtStatus)?;
        Ok(val)
    }
    /// Read history of previous fault status of the protection functionality
    pub fn read_protection_alert(&mut self) -> Result<u16, Error<E>> {
        let val = self.read_named_register(Register::ProtAlrt)?;
        Ok(val)
    }

    /// Direct cell voltage measurement for Cell1 (in volts)
    pub fn read_cell1(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Cell1)?;
        Ok(convert_to_voltage(raw))
    }

    /// Read permanent battery status information
    pub fn read_batt_status(&mut self) -> Result<u16, Error<E>> {
        let val = self.read_named_register_nvm(RegisterNvm::NBattStatus)?;
        Ok(val)
    }

    /// Unlock write protection
    pub fn unlock_write_protection(&mut self) -> Result<(), Error<E>> {
        self.write_named_register(Register::CommStat, 0x0000)?;
        self.write_named_register(Register::CommStat, 0x0000)?;
        Ok(())
    }

    /// Set the pack configuration according to application schematic.
    ///
    /// n_cells: number of cells, min 2, max 4.
    ///
    /// n_therms: number of thermistor channels to enable (not including the die thermistor), min 0, max 4.
    ///
    ///
    pub fn set_pack_config(
        &mut self,
        n_cells: u8,
        n_therms: u8,
        therm_type: ThermistorType,
        charge_pump_voltage_config: ChargePumpVoltageConfiguration,
        always_on_regulator_config: AlwaysOnRegulatorConfiguration,
        battery_package_update: BatteryPackageUpdate,
    ) -> Result<(), Error<E>> {
        if n_cells < 2 || n_cells > 4 {
            return Err(Error::InvalidConfigurationValue(n_cells as u16));
        }
        let n_cells = n_cells - 2;
        if n_therms > 4 {
            return Err(Error::InvalidConfigurationValue(n_therms as u16));
        }
        let n_therms = n_therms << 2;

        let code = n_cells as u16
            | n_therms as u16
            | therm_type as u16
            | charge_pump_voltage_config as u16
            | always_on_regulator_config as u16
            | battery_package_update as u16;
        self.unlock_write_protection()?;
        self.write_named_register_nvm(RegisterNvm::NPackCfg, code)?;
        Ok(())
    }
}
fn convert_to_time(raw: u16) -> f32 {
    raw as f32 * 5.625
}
fn convert_to_voltage(raw: u16) -> f32 {
    raw as f32 * 0.078125 / 1000.0
}

fn convert_to_percentage(raw: u16) -> f32 {
    raw as f32 / 256.0
}

fn convert_to_temperature(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn convert_to_capacity(raw: u16, r_sense: f32) -> f32 {
    raw as f32 * 5.0 / r_sense
}

fn convert_to_current(raw: i16, r_sense: f32) -> f32 {
    raw as f32 * 1.5625 / r_sense
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;
    #[test]
    fn max_temp_conversion() {
        let max_temp_raw: u16 = 0b01111111_11111111;
        let temp = convert_to_temperature(max_temp_raw as i16);
        println!("temp {}", temp);
        assert_eq!(temp, 127.99609)
    }
    #[test]
    fn min_temp_conversion() {
        let min_temp_raw: u16 = 0b10000000_00000000;
        let temp = convert_to_temperature(min_temp_raw as i16);
        println!("temp {}", temp);
        assert_eq!(temp, -128.0)
    }
}
