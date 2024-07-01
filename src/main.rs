#![no_std]
#![no_main]

//use core::fmt::Write;
use embassy_stm32::i2s::{Config, I2S};
//use heapless::String;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Error, I2c};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x1A; //WM8731 codec

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello world!");
    let p = embassy_stm32::init(Default::default());

    //let wm8731_reset: &[u8] = &[0b0001_1110, 0b0000_0000]; // [ B15-B8 , B7-B0 ]
    let wm8731_power1: &[u8] = &[0b0000_1100, 0b0001_0010]; // turn on everything except outputs.
    let wm8731_power2: &[u8] = &[0b0000_1100, 0b0000_0010]; // turn on outputs at the end to avoid pops.
    let wm8731_active: &[u8] = &[0b0001_0010, 0b0000_0001];
    
    let wm8731_left_linein: &[u8] = &[0b0000_0000, 0b0001_0111];
    let wm8731_right_linein: &[u8] = &[0b0000_0000, 0b0001_0111];
    
    let wm8731_analogue_path: &[u8] = &[0b0000_1000, 0b0001_0010];
    let wm8731_digital_path: &[u8] = &[0b0000_1010, 0b0000_0000];
    
    let wm8731_interface: &[u8] = &[0b0000_1110, 0b0100_0010]; // i2s master mode 24-bit

    //let wm8731_sampling: &[u8] = &[0b00011110, 0b00000000]; //defaults are fine

    let cmd_list = [//wm8731_reset,
                                wm8731_power1,
                                wm8731_left_linein,
                                wm8731_right_linein,
                                wm8731_analogue_path,
                                wm8731_digital_path,
                                wm8731_interface,
                                //wm8731_sampling,
                                wm8731_active,
                                wm8731_power2];

    let mut i2c = I2c::new_blocking(p.I2C2, p.PB10, p.PB11, Hertz(100_000), Default::default());

    for i in cmd_list {

        match i2c.blocking_write(ADDRESS, i) {
            Ok(()) => info!("Write OK"),
            Err(Error::Timeout) => error!("Operation timed out"),
            Err(e) => error!("I2c Error: {:?}", e),
        }
    }

    let mut i2s = I2S::new(
        p.SPI3,
        p.PB5,  // sd
        p.PA4, // ws
        p.PB3, // ck
        p.PC7,  // mck
        p.DMA1_CH5,
        p.DMA1_CH0,
        Hertz(1_000_000),
        Config::default(),
    );

    let write_word: &[u8; 1] = &[0xF0];
    loop {
        //let mut write: String<128> = String::new();
        //core::write!(&mut write, "A", n).unwrap();
        i2s.write(write_word).await.ok();
    }
}
