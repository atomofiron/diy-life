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
[avrdude.conf v7.3](https://github.com/avrdudes/avrdude/releases/tag/v7.3)

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
| USBASP  | ProMicro |
|---------|----------|
| 2  VCC  | VCC      |
| 10 GND  | GND      |
| 5  RST  | RST      |
| 9  MISO | 14       |
| 7 SCK   | 15       |
| 1 MOSI  | 16       |

### Check
```
avrdude -C avrdude7.3.conf -c usbasp -p m32u4 -v
```
<details>
<summary>output</summary>

```
Using port            : usb
Using programmer      : usbasp
AVR part              : ATmega32U4
Programming modes     : SPM, ISP, HVPP, JTAG
Programmer type       : usbasp
Description           : USBasp ISP and TPI programmer
Error: cannot set sck period; please check for usbasp firmware update

AVR device initialized and ready to accept instructions
Device signature = 1E 95 87 (ATmega32U4)
```
</details>

### Bootloader
get length
```
avrdude -C avrdude7.3.conf -c usbasp -p m32u4 -U hfuse:r:-:h
```

<details>
<summary>output</summary>

```
Reading hfuse memory ...
Writing 1 byte to output file <stdout>
0xd8
```
| hfuse | bootloader | flash space | bootloader start |
|-------|------------|-------------|------------------|
| 0xDE  | 512 байт   | 32256 байт  | 0x3F00           |
| 0xD6  | 1024 байта | 31744 байта | 0x3E00           |
| 0xD2  | 2048 байт  | 30720 байт  | 0x3C00           |
| 0xD8  | 4096 байт  | 28672 байта | 0x3800           |
</details>

current Lock Bits
```
avrdude -C avrdude7.3.conf -c usbasp -p m32u4 -U lock:r:-:h
```
<details>
<summary>output</summary>

```
Reading lock memory ...
Writing 1 byte to output file <stdout>
0xff
```
`0xff` = fully unlocked (bits: 0 = locked, 1 = unlocked)
</details>

lock the bootloader
```
avrdude -C avrdude7.3.conf -c usbasp -p m32u4 -U lock:w:0x2f:m
```
<details>
<summary>output</summary>

```
Reading 1 byte for lock from input file 0x2f
Writing 1 byte (0x2F) to lock, 1 byte written, 1 verified
```
</details>

### Recover
erase (optional)
```
avrdude -C avrdude7.3.conf -p m32u4 -c usbasp -e
```
[Caterina-promicro16.hex](https://github.com/sparkfun/Arduino_Boards/blob/main/sparkfun/avr/bootloaders/caterina/Caterina-promicro16.hex)
```
avrdude -C avrdude7.3.conf -p m32u4 -c usbasp -U flash:w:Caterina-Promicro16.hex:i -U lfuse:w:0xff:m -U hfuse:w:0xd8:m -U efuse:w:0xcb:m -U lock:w:0x2f:m
```