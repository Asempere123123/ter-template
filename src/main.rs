#![no_std]
#![no_main]

mod fmt;

extern crate alloc;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embedded_alloc::LlffHeap as Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();

use alloc::string::String;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};

use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    unsafe {
        embedded_alloc::init!(HEAP, 1024);
    }
    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PB7, Level::High, Speed::Low);
    let mut msg = String::from("Hola");
    loop {
        info!("{}", &msg);
        msg.push('a');
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
