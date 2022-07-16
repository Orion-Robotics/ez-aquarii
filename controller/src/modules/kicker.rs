use std::time::{Duration, Instant};

use anyhow::Result;
use rppal::gpio::{Gpio, OutputPin};

pub struct Kicker {
	kicker_pin: OutputPin,
	last_kick: std::time::Instant,
}

impl Kicker {
	pub fn new(pin: u8) -> Result<Self> {
		let gpio = Gpio::new()?;
		let kicker_pin = gpio.get(pin)?.into_output();
		Ok(Self {
			kicker_pin,
			last_kick: Instant::now(),
		})
	}

	pub fn kick(&mut self) {
		if Instant::now().duration_since(self.last_kick) > Duration::from_millis(2000) {
			self.kicker_pin.set_high();
			self.last_kick = Instant::now();
		} else if Instant::now().duration_since(self.last_kick) > Duration::from_millis(500) {
			self.kicker_pin.set_low();
		}
	}
}
