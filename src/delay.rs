use core::convert::TryInto;

use crate::rcc::Clocks;
use crate::time::duration::Microseconds;
pub use cortex_m::delay::*;
use cortex_m::{peripheral::SYST, prelude::_embedded_hal_blocking_delay_DelayUs};

pub trait SYSTDelayExt {
    fn delay(self, clocks: &Clocks) -> Delay;
}

impl SYSTDelayExt for SYST {
    fn delay(self, clocks: &Clocks) -> Delay {
        Delay::new(self, clocks.ahb_clk.0)
    }
}

pub trait DelayExt {
    fn delay<T: TryInto<Microseconds>>(&mut self, delay: T) -> Result<(), T::Error>;
}
impl DelayExt for Delay {
    fn delay<T: TryInto<Microseconds>>(&mut self, delay: T) -> Result<(), T::Error> {
        self.delay_us(delay.try_into()?.0);
        Ok(())
    }
}
