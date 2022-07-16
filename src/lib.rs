//! An embedded hal driver for the MAX17320 (2S-4S ModelGauge m5 Fuel Gauge with Protector, Internal Self-Discharge Detection and SHA-256 Authentication)
//! ## Examples
//!
//! for more examples please see [max17320_stm32f401_examples](https://github.com/shaoyuancc/max17320_stm32f401_examples)
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! use cortex_m_rt::ExceptionFrame;
//! use cortex_m_rt::{entry, exception};
//! use cortex_m_semihosting::hprintln;
//! use hal::{pac, prelude::*};
//! use panic_semihosting as _;
//! use stm32f4xx_hal as hal;
//! use vl6180x;
//!
//! #[entry]
//! fn main() -> ! {
//!     if let (Some(dp), Some(_cp)) = (
//!         pac::Peripherals::take(),
//!         cortex_m::peripheral::Peripherals::take(),
//!     ) {
//!         let rcc = dp.RCC.constrain();
//!         let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
//!
//!         let gpiob = dp.GPIOB.split();
//!         let scl = gpiob
//!             .pb8
//!             .into_alternate()
//!             .internal_pull_up(true)
//!             .set_open_drain();
//!         let sda = gpiob
//!             .pb9
//!             .into_alternate()
//!             .internal_pull_up(true)
//!             .set_open_drain();
//!         let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);
//!
//!         let mut bat = max17320::MAX17320::new(i2c, 5.0).expect("mx");
//!
//!         hprintln!("status: {}", bat.read_status().unwrap()).unwrap();
//!         hprintln!("capacity: {}mAh", bat.read_capacity().unwrap()).unwrap();
//!         hprintln!("device name: {}", bat.read_device_name().unwrap()).unwrap();
//!         hprintln!("state of charge: {}%", bat.read_state_of_charge().unwrap()).unwrap();
//!         hprintln!("vcell: {}v", bat.read_vcell().unwrap()).unwrap();
//!         hprintln!("cell1: {}v", bat.read_cell1().unwrap()).unwrap();
//!         hprintln!("temp: {}°C", bat.read_temperature().unwrap()).unwrap();
//!         hprintln!("die temp: {}°C", bat.read_die_temperature().unwrap()).unwrap();
//!         hprintln!("current: {}mA", bat.read_current().unwrap()).unwrap();
//!         hprintln!("tte: {}", bat.read_time_to_empty().unwrap()).unwrap();
//!         hprintln!("ttf: {}", bat.read_time_to_full().unwrap()).unwrap();
//!         hprintln!("prot_status: {}", bat.read_protection_status().unwrap()).unwrap();
//!         hprintln!("prot_alert: {}", bat.read_protection_alert().unwrap()).unwrap();
//!     }
//!     loop {}
//! }
//!
//! #[exception]
//! unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
//!     panic!("{:#?}", ef);
//! }
//! ```

//! ## References
//! [MAX17320 datasheet](https://datasheets.maximintegrated.com/en/ds/MAX17320.pdf)
//!
//! ## Additional Notes
//! - Only tested with STM32F401 microcontroller
//! - 1-Wire communication protocol not implemented yet. Drop me an email or submit a pull request to add support.

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
    pub fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Temp)? as i16;
        Ok(convert_to_temperature(raw))
    }

    /// Read internal die temperature (°C)
    pub fn read_die_temperature(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::DieTemp)? as i16;
        Ok(convert_to_temperature(raw))
    }

    /// Read battery current (A)
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

    /// Clear protection alert register
    pub fn clear_protection_alert(&mut self) -> Result<(), Error<E>> {
        self.write_named_register(Register::ProtAlrt, 0x0000)?;
        Ok(())
    }

    /// Direct cell voltage measurement for Cell1 (in volts)
    pub fn read_cell1(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Cell1)?;
        Ok(convert_to_voltage(raw))
    }

    /// Direct cell voltage measurement for Cell2 (in volts)
    pub fn read_cell2(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Cell2)?;
        Ok(convert_to_voltage(raw))
    }

    /// Direct cell voltage measurement for Cell3 (in volts)
    pub fn read_cell3(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Cell3)?;
        Ok(convert_to_voltage(raw))
    }

    /// Direct cell voltage measurement for Cell4 (in volts)
    pub fn read_cell4(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Cell4)?;
        Ok(convert_to_voltage(raw))
    }

    /// Read the total pack voltage measured inside the protector (V)
    pub fn read_batt(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Batt)?;
        Ok(convert_to_voltage(raw))
    }

    /// Read the voltage between PACK+ and GND (V)
    pub fn read_pckp(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_named_register(Register::Pckp)?;
        Ok(convert_to_voltage(raw))
    }

    /// Read permanent battery status information
    pub fn read_battery_status(&mut self) -> Result<u16, Error<E>> {
        let val = self.read_named_register_nvm(RegisterNvm::NBattStatus)?;
        Ok(val)
    }

    /// Unlock write protection
    fn unlock_write_protection(&mut self) -> Result<(), Error<E>> {
        self.write_named_register(Register::CommStat, 0x0000)?;
        self.write_named_register(Register::CommStat, 0x0000)?;
        Ok(())
    }

    fn lock_write_protection(&mut self) -> Result<(), Error<E>> {
        self.write_named_register(Register::CommStat, 0x00F9)?;
        self.write_named_register(Register::CommStat, 0x00F9)?;
        Ok(())
    }

    /// Read the pack configuration
    pub fn read_pack_config(&mut self) -> Result<u16, Error<E>> {
        let val = self.read_named_register_nvm(RegisterNvm::NPackCfg)?;
        Ok(val)
    }

    /// Set the pack configuration according to application schematic.
    ///
    /// n_cells: number of cells, min 2, max 4.
    ///
    /// n_therms: number of thermistor channels to enable (not including the die thermistor), min 0, max 4.
    ///
    /// therm_type: 10kΩ NTC thermistor or 100kΩ NTC thermistor.
    ///
    /// charge_pump_voltage_config: Set according to the desired gate drive.
    ///
    /// always_on_regulator_config: Disabled, Enabled3p4V or Enabled3p4V
    ///
    /// battery_pack_update: UpdateEvery22p4s or AfterMeasurementsCompleted
    pub fn set_pack_config(
        &mut self,
        n_cells: u8,
        n_therms: u8,
        therm_type: ThermistorType,
        charge_pump_voltage_config: ChargePumpVoltageConfiguration,
        always_on_regulator_config: AlwaysOnRegulatorConfiguration,
        battery_pack_update: BatteryPackUpdate,
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
            | battery_pack_update as u16;
        self.unlock_write_protection()?;
        self.write_named_register_nvm(RegisterNvm::NPackCfg, code)?;
        self.lock_write_protection()?;
        Ok(())
    }

    /// Enable Alert on Fuel-Gauge Outputs.
    ///
    /// Default = disabled
    ///
    /// When Aen = 1, violation of any of the
    /// alert threshold register values by temperature, voltage, or SOC triggers
    /// an alert. This bit affects the ALRT pin operation only. The Smx, Smn, Tmx,
    /// Tmn, Vmx, Vmn, Imx, and Imn bits of the Status register (000h) are not
    /// disabled. Note that if this bit is set to 1, the ALSH bit will be set to
    /// 0 to prevent an alert condition from causing the device to enter shutdown mode.
    /// If this bit is set to 0, the ALSH bit is not changed.
    pub fn set_alert_output_enable(&mut self, enable: bool) -> Result<(), Error<E>> {
        let current_config = self.read_named_register(Register::Config)?;
        let new_config: u16;
        if enable {
            new_config = set_bit(current_config, 2);
            self.set_alert_shutdown_enable(false)?;
        } else {
            new_config = clear_bit(current_config, 2);
        }
        self.write_named_register(Register::Config, new_config)?;
        Ok(())
    }

    /// Enable alert shutdown. When ALSH = 1, if the ALRT pin = 1, the device will
    /// enter shutdown mode. Default = disabled.
    pub fn set_alert_shutdown_enable(&mut self, enable: bool) -> Result<(), Error<E>> {
        let current_nconfig = self.read_named_register_nvm(RegisterNvm::NConfig)?;
        let new_nconfig = if enable {
            set_bit(current_nconfig, 5)
        } else {
            clear_bit(current_nconfig, 5)
        };
        self.write_named_register_nvm(RegisterNvm::NConfig, new_nconfig)
    }

    /// Set the upper and lower limits that generate an ALRT pin interrupt if exceeded
    /// by any of the cell voltage readings.
    ///
    /// For each value, min = 0.0V, max = 5.1V; value must be multiple of 0.02V
    /// Defaults: min_v = 0.0V, max_v = 5.1V
    pub fn set_voltage_alert_threshold(&mut self, min_v: f32, max_v: f32) -> Result<(), Error<E>> {
        if !is_valid_voltage_threshold(max_v) {
            return Err(Error::InvalidConfigurationValue(max_v as u16));
        }
        if !is_valid_voltage_threshold(min_v) {
            return Err(Error::InvalidConfigurationValue(min_v as u16));
        }
        let threshold_array = [
            (max_v / VALRTTH_LSB_RESOLUTION) as u8,
            (min_v / VALRTTH_LSB_RESOLUTION) as u8,
        ];
        let threshold_code = u16::from_be_bytes(threshold_array);
        self.write_named_register(Register::VAlrtTh, threshold_code)?;
        Ok(())
    }

    /// Read the voltage alert threshold, returns tuple of (min_v, max_v)
    pub fn read_volatage_alert_threshold(&mut self) -> Result<(f32, f32), Error<E>> {
        let code = self.read_named_register(Register::VAlrtTh)?;
        let raw = code.to_be_bytes();
        Ok((
            raw[0] as f32 * VALRTTH_LSB_RESOLUTION, // Min
            raw[1] as f32 * VALRTTH_LSB_RESOLUTION, // Max
        ))
    }

    /// Set the upper and lower limits that generate an ALRT pin interrupt if exceeded
    /// by any thermistor reading.
    ///
    /// For each value, min = -128°C, max = 127°C
    /// Defaults: min_t = -128°C, max_t = 127°C (disabled)
    pub fn set_temperature_alert_threshold(
        &mut self,
        min_t: i8,
        max_t: i8,
    ) -> Result<(), Error<E>> {
        let threshold_array = [max_t as u8, min_t as u8];
        let threshold_code = u16::from_be_bytes(threshold_array);
        self.write_named_register(Register::TAlrtTh, threshold_code)?;
        Ok(())
    }

    /// Read the temperature alert threshold, returns tuple of (min_t, max_t)
    pub fn read_temperature_alert_threshold(&mut self) -> Result<(i8, i8), Error<E>> {
        let code = self.read_named_register(Register::TAlrtTh)?;
        let raw = code.to_be_bytes();
        Ok((
            raw[0] as i8, // Min
            raw[1] as i8, // Max
        ))
    }

    /// Set the upper and lower limits that generate an ALRT pin interrupt if exceeded
    /// by the selected RepSOC, AvSOC, MixSOC, or VFSOC register values.
    /// See the MiscCFG.SACFG setting for details.
    ///
    /// For each value, min = 0%, max = 255%
    /// Defaults: min_soc = 0%, max_soc = 255% (disabled)
    pub fn set_state_of_charge_alert_threshold(
        &mut self,
        min_soc: u8,
        max_soc: u8,
    ) -> Result<(), Error<E>> {
        let threshold_array = [max_soc, min_soc];
        let threshold_code = u16::from_be_bytes(threshold_array);
        self.write_named_register(Register::SAlrtTh, threshold_code)?;
        Ok(())
    }

    /// Read the state of charge alert threshold, returns tuple of (min_soc, max_soc)
    pub fn read_state_of_charge_alert_threshold(&mut self) -> Result<(u8, u8), Error<E>> {
        let code = self.read_named_register(Register::SAlrtTh)?;
        let raw = code.to_be_bytes();
        Ok((
            raw[0] as u8, // Min
            raw[1] as u8, // Max
        ))
    }

    /// Set the upper and lower limits that generate an ALRT pin interrupt if exceeded
    /// by any of the current register value.
    ///
    /// For each value, min = -128, max = 127; in units of 400μV
    /// Defaults: min_i = -128, max_i = 127
    pub fn set_current_alert_threshold(&mut self, min_i: i8, max_i: i8) -> Result<(), Error<E>> {
        let threshold_array = [max_i as u8, min_i as u8];
        let threshold_code = u16::from_be_bytes(threshold_array);
        self.write_named_register(Register::IAlrtTh, threshold_code)?;
        Ok(())
    }

    /// Read the current alert threshold, returns tuple of (min_i, max_i) in units of 400μV
    pub fn read_current_alert_threshold(&mut self) -> Result<(i8, i8), Error<E>> {
        let code = self.read_named_register(Register::IAlrtTh)?;
        let raw = code.to_be_bytes();
        Ok((
            raw[0] as i8, // Min
            raw[1] as i8, // Max
        ))
    }
}

const VALRTTH_LSB_RESOLUTION: f32 = 0.02; // mV

fn is_valid_voltage_threshold(raw: f32) -> bool {
    raw % VALRTTH_LSB_RESOLUTION < 0.0001 && raw >= 0.0 && raw <= (255.0 * VALRTTH_LSB_RESOLUTION)
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
    raw as f32 * 1.5625 / (r_sense / 1000.0)
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

    #[test]
    fn valid_voltage_threshold() {
        assert!(is_valid_voltage_threshold(5.1))
    }
}
