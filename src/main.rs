#![no_std]
#![no_main]

use arduino_hal::{Adc, Pins};
use life::ext::pin_ext::LedExt;
use life::ext::terminal::TerminalDisplayExt;
use life::life::universe::Universe;
use life::values::{SCREEN_400K, SCREEN_HEIGHT, SCREEN_WIDTH};
#[allow(unused_imports)]
use panic_halt as _;
use ssd1306::command::AddrMode;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();

    let mut blue = pins.led_rx.into_output_high().downgrade();
    let mut green = pins.led_tx.into_output_high().downgrade();

    green.blink();

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d2.into_pull_up_input(),
        pins.d3.into_pull_up_input(),
        SCREEN_400K,
    );
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_terminal_mode();

    if display.init().is_err() {
        green.on();
        blue.on();
        panic!();
    }
    display.clear()
        .unwrap();
    blue.blink();

    display.set_position(0, 3)
        .unwrap();
    display.write_text("Hello World!")
        .unwrap();
    green.blink();
    arduino_hal::delay_ms(1000);

    let mut adc = Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);
    let touch = pins.d4.into_floating_input();

    blue.blink();

    let mut universe = Universe::new(adc, a0);

    display.set_addr_mode(AddrMode::Vertical)
        .unwrap();
    display.set_draw_area((0, 0), (SCREEN_WIDTH as u8, SCREEN_HEIGHT as u8))
        .unwrap();
    let mut loop_flag = true;
    let mut was_touched = false;
    let mut touch_counter = 0;
    loop {
        match loop_flag {
            true => green.on(),
            false => green.off(),
        }
        loop_flag = !loop_flag;

        let touched = touch.is_high();
        if touched != was_touched {
            led.toggle();
        }
        let splash = touched && touched != was_touched;
        was_touched = touched;
        if touch_counter == 30 {
            universe.armageddon();
        }
        match touched {
            true => touch_counter += 1,
            false => touch_counter = 0,
        }

        universe.evolve(&mut display, splash);
    }
}
