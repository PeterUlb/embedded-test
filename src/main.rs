#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::gpio::{Disconnected, Level, Output, Pin, PushPull};
use microbit::hal::prelude::*;
use microbit::hal::pwm::{Channel, Instance, Pwm};
use microbit::hal::Timer;
use microbit::*;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

struct Light {
    red: Pin<Output<PushPull>>,
    yellow: Pin<Output<PushPull>>,
    green: Pin<Output<PushPull>>,
}

impl Light {
    pub fn new(
        red: Pin<Disconnected>,
        yellow: Pin<Disconnected>,
        green: Pin<Disconnected>,
    ) -> Self {
        Self {
            red: red.into_push_pull_output(Level::Low),
            yellow: yellow.into_push_pull_output(Level::Low),
            green: green.into_push_pull_output(Level::Low),
        }
    }

    pub fn red(&mut self) {
        self.set_off();
        self.red.set_high().unwrap();
    }

    pub fn yellow(&mut self) {
        self.set_off();
        self.yellow.set_high().unwrap();
    }

    pub fn green(&mut self) {
        self.set_off();
        self.green.set_high().unwrap();
    }

    pub fn set_off(&mut self) {
        self.red.set_low().unwrap();
        self.yellow.set_low().unwrap();
        self.green.set_low().unwrap();
    }
}

struct Fan<T: Instance> {
    pwm: Pwm<T>,
}

impl<T: Instance> Fan<T> {
    pub fn new(forward: Pin<Disconnected>, backward: Pin<Disconnected>, pwm: T) -> Self {
        let forward = forward.into_push_pull_output(Level::Low);
        let backward = backward.into_push_pull_output(Level::Low);

        let pwm = Pwm::new(pwm);
        pwm.set_output_pin(Channel::C0, forward)
            .set_output_pin(Channel::C1, backward);

        rprintln!("Max Duty: {}", pwm.max_duty());
        rprintln!("Period: {}", pwm.period().0);
        // pwm.set_period(800u32.hz());

        Self { pwm }
    }

    fn forward(&self, speed: u16) {
        self.pwm.set_duty_off(Channel::C1, 0);
        self.pwm.set_duty_on(Channel::C0, speed);
    }

    fn backwards(&self, speed: u16) {
        self.pwm.set_duty_off(Channel::C0, 0);
        self.pwm.set_duty_on(Channel::C1, speed);
    }

    fn stop(&self) {
        self.pwm.disable();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let fan = Fan::new(
        board.pins.p0_02.degrade(),
        board.pins.p0_03.degrade(),
        board.PWM0,
    );
    let mut light = Light::new(
        board.pins.p0_04.degrade(),
        board.display_pins.col3.into_disconnected().degrade(),
        board.display_pins.col1.into_disconnected().degrade(),
    );
    light.set_off();
    light.red();
    timer.delay_ms(2000_u32);
    light.yellow();
    timer.delay_ms(2000_u32);
    light.green();

    const DELAY: u32 = 5000_u32;
    loop {
        fan.forward(800);
        timer.delay_ms(DELAY);
        fan.stop();
        timer.delay_ms(DELAY);
        fan.backwards(800);
        timer.delay_ms(DELAY);
        fan.stop();
        light.red();
        loop {
            timer.delay_ms(500000_u32);
        }
    }
}
