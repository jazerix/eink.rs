use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::{Peripherals, self};
use esp_idf_hal::gpio::{AnyIOPin, IOPin};
use esp_idf_hal::sys::spi_host_device_t;
use esp_idf_hal::{spi::*, gpio};

pub struct Display {
    sclk: AnyIOPin,
    mosi: AnyIOPin,
}

impl Display {

    pub fn new(sclk: impl Into<AnyIOPin>, mosi: impl Into<AnyIOPin>) -> Self {
        Display {
            sclk: sclk.into(),
            mosi: mosi.into(),
        }
    }

    pub fn configure<SPI: SpiAnyPins>(self, spi: impl Peripheral<P = SPI>) {
        let spi_driver = SpiDriver::new(
            spi,
            self.sclk,
            self.mosi,
            None::<gpio::AnyIOPin>,
            &SpiDriverConfig::new()
        );
    }
}