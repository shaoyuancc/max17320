An embedded hal driver for the MAX17320 (2S-4S ModelGauge m5 Fuel Gauge with Protector, Internal Self-Discharge Detection and SHA-256 Authentication)
## Examples

for more examples please see [max17320_stm32f401_examples](https://github.com/shaoyuancc/max17320_stm32f401_examples)

```rust
#![no_std]
#![no_main]

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use hal::{pac, prelude::*};
use panic_semihosting as _;
use stm32f4xx_hal as hal;
use vl6180x;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpiob = dp.GPIOB.split();
        let scl = gpiob
            .pb8
            .into_alternate()
            .internal_pull_up(true)
            .set_open_drain();
        let sda = gpiob
            .pb9
            .into_alternate()
            .internal_pull_up(true)
            .set_open_drain();
        let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);

        let mut bat = max17320::MAX17320::new(i2c, 5.0).expect("mx");

        hprintln!("status: {}", bat.read_status().unwrap()).unwrap();
        hprintln!("capacity: {}mAh", bat.read_capacity().unwrap()).unwrap();
        hprintln!("device name: {}", bat.read_device_name().unwrap()).unwrap();
        hprintln!("state of charge: {}%", bat.read_state_of_charge().unwrap()).unwrap();
        hprintln!("vcell: {}v", bat.read_vcell().unwrap()).unwrap();
        hprintln!("cell1: {}v", bat.read_cell1().unwrap()).unwrap();
        hprintln!("temp: {}°C", bat.read_temperature().unwrap()).unwrap();
        hprintln!("die temp: {}°C", bat.read_die_temperature().unwrap()).unwrap();
        hprintln!("current: {}mA", bat.read_current().unwrap()).unwrap();
        hprintln!("tte: {}", bat.read_time_to_empty().unwrap()).unwrap();
        hprintln!("ttf: {}", bat.read_time_to_full().unwrap()).unwrap();
        hprintln!("prot_status: {}", bat.read_protection_status().unwrap()).unwrap();
        hprintln!("prot_alert: {}", bat.read_protection_alert().unwrap()).unwrap();
    }
    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
```

## References
[MAX17320 datasheet](https://datasheets.maximintegrated.com/en/ds/MAX17320.pdf)

## Additional Notes
- Only tested with STM32F401 microcontroller
- 1-Wire communication protocol not implemented yet. Drop me an email or submit a pull request to add support.