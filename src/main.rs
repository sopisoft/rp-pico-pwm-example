#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_halt as _;

use embedded_hal::pwm::SetDutyCycle;
use rp_pico::{
    hal::{self, prelude::*, pwm::Slices},
    pac,
};

const LOW: u16 = 0;
const HIGH: u16 = 25000;

#[hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio25);

    loop {
        info!("on");
        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        info!("off");
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        delay.delay_us(500);
    }
}
