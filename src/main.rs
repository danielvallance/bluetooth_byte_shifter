#![no_main]
#![no_std]

extern crate panic_itm;

use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut itm = Peripherals::take().unwrap().ITM;

    iprintln!(&mut itm.stim[0], "Hello world!\n");

    loop {}
}
