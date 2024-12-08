#![no_main]
#![no_std]

extern crate panic_itm;

use cortex_m::iprintln;
use cortex_m_rt::entry;
use nb::block;
use stm32f3xx_hal::{gpio::GpioExt, pac, prelude::*, serial::Serial};

#[entry]
fn main() -> ! {
    /* Get device peripherals */
    let dp = pac::Peripherals::take().unwrap();

    /* Get ITM peripheral which is how we will write debug messages */
    let mut itm = cortex_m::Peripherals::take().unwrap().ITM;

    /*
     * Get peripherals required for serial communication, then set up serial communication
     *
     * The HM-10 module is attached to pins PA9/PA10
     */
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let tx_pin =
        gpioa
            .pa9
            .into_af_push_pull::<7>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let rx_pin =
        gpioa
            .pa10
            .into_af_push_pull::<7>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let mut serial = Serial::new(
        dp.USART1,
        (tx_pin, rx_pin),
        9600.Bd(),
        clocks,
        &mut rcc.apb2,
    );

    loop {
        match block!(serial.read()) {
            Ok(byte) => {
                iprintln!(
                    &mut itm.stim[0],
                    "Received byte {}, sending back {} >> 1 which is {}\n",
                    byte,
                    byte,
                    byte >> 1
                );

                block!(serial.write(byte >> 1)).ok();
            }
            Err(e) => {
                iprintln!(
                    &mut itm.stim[0],
                    "Encountered error while reading: {:?}\n",
                    e
                )
            }
        }
    }
}
