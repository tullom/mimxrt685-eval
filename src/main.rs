#![no_std]
#![no_main]

use core::arch::asm;

pub use cortex_m_rt::interrupt;
use defmt::*;
use embassy_executor::Spawner;
use mimxrt685s_pac as pac;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = pac::Peripherals::take().unwrap();

    info!("Hello world");
    loop {
        unsafe { asm!("nop") };
    }
}
