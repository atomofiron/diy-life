use crate::types::TerminalDisplay;
use crate::values::{SCREEN_HEIGHT, SCREEN_WIDTH};
use arduino_hal::hal::Adc;
use arduino_hal::port::mode::Analog;
use arduino_hal::port::{Pin, A0};
use arduino_hal::DefaultClock;
use core::ops::Shr;

pub const WIDTH: usize = SCREEN_WIDTH as usize;
pub const WIDTH_MASK: usize = WIDTH - 1;
pub const HEIGHT: usize = SCREEN_HEIGHT as usize;
const BUF_SIZE: usize = WIDTH;

pub struct Universe {
    generation: [u64; 128],
    unused: Pin<Analog, A0>,
    splash: bool,
}

impl Universe {

    pub fn new(unused: Pin<Analog, A0>) -> Universe {
        Universe {
            generation: [0; BUF_SIZE],
            unused,
            splash: false,
        }
    }

    pub fn evolve(
        &mut self,
        display: &mut TerminalDisplay,
        adc: &mut Adc<DefaultClock>,
        splash: bool,
    ) {
        self.splash = true;
        self.sow(adc);
        self.evolution(splash);
        let bytes: &[u8] = bytemuck::cast_slice(&self.generation);
        display.draw(bytes).unwrap();
    }

    pub fn armageddon(&mut self) {
        self.generation.fill(0);
    }

    #[inline]
    fn sow(&mut self, adc: &mut Adc<DefaultClock>) {
        let a = self.random(adc);
        let b = self.random(adc);
        let x = (a & 0b1111111) as usize; // 0-127
        let y = (b & 0b111111) as u32; // 0-63
        self.generation[x] |= ((a & 0b1111) as u64).rotate_left(y);
        self.generation[(x + 1) & WIDTH_MASK] |= (a.shr(4) as u64).rotate_left(y);
        self.generation[(x + 2) & WIDTH_MASK] |= ((b & 0b1111) as u64).rotate_left(y);
        self.generation[(x + 3) & WIDTH_MASK] |= (b.shr(4) as u64).rotate_left(y);
    }

    fn evolution(&mut self, splash: bool) {
        let first = self.generation[0];
        let mut left;
        let mut center = self.generation[BUF_SIZE - 1];
        let mut right = first;
        let mut neighbors = [0u64; 8];
        neighbors[3] = center.rotate_left(1);
        neighbors[4] = center.rotate_right(1);
        neighbors[5] = right.rotate_left(1);
        neighbors[7] = right.rotate_right(1);
        for cursor in 0..WIDTH {
            left = center;
            center = right;
            right = match cursor + 1 {
                WIDTH => first,
                next => self.generation[next],
            };
            neighbors[0] = neighbors[3];
            neighbors[1] = left;
            neighbors[2] = neighbors[4];
            neighbors[3] = neighbors[5];
            neighbors[4] = neighbors[7];
            neighbors[5] = right.rotate_left(1);
            neighbors[6] = right;
            neighbors[7] = right.rotate_right(1);
            self.generation[cursor] = calculate_column(center, neighbors, splash);
        }
        self.splash = false;
    }

    fn random(&mut self, adc: &mut Adc<DefaultClock>) -> u8 {
        let mut rnd: u8 = 0;
        for _ in 0..8 {
            let v = adc.read_blocking(&self.unused);
            rnd = (rnd << 1) | ((v & 1) as u8);
            core::hint::spin_loop();
        }
        return rnd
    }
}

fn calculate_column(current: u64, neighbors: [u64; 8], splash: bool) -> u64 {
    let mut box_0 = 0u64;
    let mut box_1 = 0u64;
    let mut box_2 = 0u64;
    for n in neighbors {
        let second = box_0 & n;
        let third = box_1 & second;
        box_0 ^= n;
        box_1 ^= second;
        box_2 ^= third;
    }
    return if splash {
        let can_birth = box_0 & !box_2;
        let can_survive = !box_2;
        (!current & can_birth) | (current & can_survive)
    } else {
        let sum_is_3 = box_0 & box_1 & !box_2;
        let sum_is_2 = !box_0 & box_1 & !box_2;
        sum_is_3 | (sum_is_2 & current)
    }
}
