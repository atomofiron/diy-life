# DIY Conway's Game of Life
ProMicro + SSD1306 + TTP223

<img width="640" src="https://github.com/user-attachments/assets/4da68e10-da22-4d59-9024-edf3626d64e0" />

[\[Play\]](https://youtu.be/DmD4GHbSqxI)

## Development

### Setup
```
cargo install cargo-generate
cargo install ravedude
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```
https://github.com/Rahix/avr-hal/tree/main/examples \
[avrdude.conf v7.3](https://github.com/avrdudes/avrdude/releases/tag/v7.3) (is 8.0 compatible?)

### Build
```
cargo build --release
avr-objcopy -O ihex -R .eeprom target/avr-none/release/life.elf firmware.hex
```
`/dev/cu.usbmodem101` – MacOS

### Check
```
avr-size -C --mcu=atmega32u4 target/avr-none/release/life.elf
```

### Flash
```
avrdude -C avrdude7.3.conf -p atmega32u4 -c avr109 -P /dev/cu.usbmodem101 -b 57600 -U flash:w:firmware.hex
```

### Other
```
stty -f /dev/cu.usbmodem101 1200
```

## Recovering via USBASP (jumpers 3.3V, JP3)

<img width="640" src="https://github.com/user-attachments/assets/bd8b131e-b2c7-4d43-a348-fb1aeb4e46b7" />

### Configuration (MacOS)
```
brew install avrdude
system_profiler SPUSBDataType
```

### Connection
| USBASP | ProMicro |
|--------|----------|
| VCC    | VCC      |
| GND    | GND      |
| RST    | RST      |
| MISO   | 14       |
| SCK    | 15       |
| MOSI   | 16       |

### Check
```
avrdude -C avrdude7.3.conf -c usbasp -p m32u4 -v
```

### Erase (optional)
```
avrdude -C avrdude7.3.conf -p m32u4 -c usbasp -e
```

### Recovery
[Caterina-promicro16.hex](https://github.com/sparkfun/Arduino_Boards/blob/main/sparkfun/avr/bootloaders/caterina/Caterina-promicro16.hex) \
connect second usb to ProMicro
```
avrdude -C avrdude7.3.conf -p m32u4 -c usbasp -U flash:w:Caterina-Promicro16.hex:i -U lfuse:w:0xff:m -U hfuse:w:0xd8:m -U efuse:w:0xcb:m
```