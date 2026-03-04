use crate::types::POD;

pub trait LedExt {
    fn on(&mut self);
    fn off(&mut self);
    fn blink(&mut self);
}

impl LedExt for POD {

    fn on(&mut self) {
        self.set_low();
    }

    fn off(&mut self) {
        self.set_high();
    }

    fn blink(&mut self) {
        self.on();
        arduino_hal::delay_ms(200);
        self.off();
        arduino_hal::delay_ms(200);
    }
}
