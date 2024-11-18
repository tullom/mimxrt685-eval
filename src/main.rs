#![no_std]
#![no_main]

use core::arch::asm;

use defmt::*;
use embassy_executor::Spawner;
pub use cortex_m_rt::interrupt;
use cortex_m_rt::entry;
use mimxrt685s_pac as pac;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::assert!(true);
    let p = pac::Peripherals::take().unwrap();

    loop {
        unsafe { asm!("nop") };
        
    }
}
