use arduino_hal::hal::Adc;
use arduino_hal::port::mode::Analog;
use arduino_hal::port::{Pin, A0};
use arduino_hal::DefaultClock;

pub struct Random {
    adc: Adc<DefaultClock>,
    pin: Pin<Analog, A0>,
}

impl Random {

    pub fn new(
        adc: Adc<DefaultClock>,
        pin: Pin<Analog, A0>,
    ) -> Random {
        Random { adc, pin }
    }

    pub fn next(&mut self) -> u8 {
        let mut rnd: u8 = 0;
        for _ in 0..8 {
            let v = self.adc.read_blocking(&self.pin);
            rnd = (rnd << 1) | ((v & 1) as u8);
            core::hint::spin_loop();
        }
        return rnd
    }
}
