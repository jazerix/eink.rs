use std::time::Duration;

use esp_idf_hal::gpio::{PinDriver, Output, OutputMode};
use esp_idf_hal::spi::config::{Config, Mode, Phase};
use esp_idf_hal::sys::{vTaskDelay, configTICK_RATE_HZ};
use esp_idf_hal::timer::{TimerDriver, TimerConfig};
use esp_idf_hal::{spi::*, gpio};
use esp_idf_hal::prelude::*;
use esp_idf_hal::peripherals::Peripherals;

mod eink;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals= Peripherals::take().unwrap();

    let sclk = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio2;
    let chip_selection = peripherals.pins.gpio20;
    let data_cmd_toggle = peripherals.pins.gpio23;
    let busy_signal = peripherals.pins.gpio18;
    let reset = peripherals.pins.gpio14;

    let display = eink::Display::new(sclk, mosi);
    display.configure(peripherals.spi2);

    /*let mut busy_signal = PinDriver::input(busy_signal);

    let mut reset = PinDriver::output(reset)?;
    initialize_display(&mut reset);

    let spi_driver: SpiDriver<'_> = SpiDriver::new(
        peripherals.spi2,
        sclk,
        mosi,
        None::<gpio::AnyIOPin>,
        &SpiDriverConfig::new()
    ).unwrap();

&*    let config = Config::new().baudrate(2.MHz().into()).data_mode(Mode {
        polarity: config::Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition
    });

    let mut spi = SpiDeviceDriver::new(spi_driver, Some(chip_selection), &config).unwrap();

    let data: u8 = 0x1B;
    let addr: u8 = 0x10;

    let mut send_array: [u8; 2] = [addr, data];
    
    std::thread::sleep(Duration::from_millis(5000));
    
    loop {
        std::thread::sleep(Duration::from_millis(5000));
        spi.write(&send_array)?;

    }

    //send_array = [];
    */

    Ok(())

}


fn initialize_display<'d, T: gpio::OutputPin, Mode: OutputMode>(reset_pin : &mut esp_idf_hal::gpio::PinDriver<'d, T, Mode>) {
    reset_pin.set_low();
    std::thread::sleep(Duration::from_millis(200));
    reset_pin.set_low();
    std::thread::sleep(Duration::from_millis(1));
    reset_pin.set_high();
    std::thread::sleep(Duration::from_millis(200));
}

fn wait_until_idle() {
    loop {
        //send_comand(dc_pin)
    }
}

/*fn send_comand<'d, T: gpio::OutputPin, Mode: OutputMode>(dc_pin:  &mut esp_idf_hal::gpio::PinDriver<'d, T, Mode>, spi: &mut SpiDeviceDriver) {
    dc_pin.set_low();
    spi.write([0x71])
}*/
