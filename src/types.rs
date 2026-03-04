use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::I2c;
use ssd1306::mode::{BufferedGraphicsMode, TerminalMode};
use ssd1306::prelude::{DisplaySize128x64, I2CInterface};
use ssd1306::Ssd1306;

pub type POD = Pin<Output, Dynamic>;
pub type TerminalDisplay = Ssd1306<I2CInterface<I2c>, DisplaySize128x64, TerminalMode>;
pub type GraphicDisplay = Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;
