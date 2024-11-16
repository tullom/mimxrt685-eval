#![no_std]
#![no_main]

use core::arch::asm;

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::assert!(true);

    loop {
        unsafe { asm!("nop") };
    }
}
