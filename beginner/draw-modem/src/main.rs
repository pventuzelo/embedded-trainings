#![no_main]
#![no_std]

// Built in dependencies
use core::fmt::Write;

// Crates.io dependencies
use dw1000::{DW1000 as DW};
use dwm1001::{
    self,
    nrf52832_hal::{
        delay::Delay,
        prelude::*,
        timer::Timer,
        gpio::{Pin, Output, PushPull, Level, p0::P0_17},
        rng::Rng,
        spim::{Spim},
        nrf52832_pac::{
            TIMER0,
            SPIM2,
        },
    },
    new_dw1000,
    new_usb_uarte,
    UsbUarteConfig,
    DW_RST,
};
use heapless::{String, consts::*};
use rtfm::app;

// NOTE: Panic Provider
use panic_ramdump as _;

// Workspace dependencies
use protocol::{ModemUartMessages};
use nrf52_bin_logger::Logger;


#[app(device = dwm1001::nrf52832_hal::nrf52832_pac)]
const APP: () = {
    static mut LED_RED_1: Pin<Output<PushPull>>     = ();
    static mut TIMER:     Timer<TIMER0>             = ();
    static mut LOGGER:    Logger<U1024, ModemUartMessages> = ();
    static mut DW1000:    DW<
                            Spim<SPIM2>,
                            P0_17<Output<PushPull>>,
                            dw1000::Ready,
                          > = ();
    static mut DW_RST_PIN: DW_RST                   = ();
    static mut RANDOM:     Rng                      = ();

    #[init]
    fn init() {
        let timer = device.TIMER0.constrain();
        let pins = device.P0.split();
        let uarte0 = new_usb_uarte(
            device.UARTE0,
            pins.p0_05,
            pins.p0_11,
            UsbUarteConfig::default(),
        );

        let rng = device.RNG.constrain();

        let dw1000 = new_dw1000(
            device.SPIM2,
            pins.p0_16,
            pins.p0_20,
            pins.p0_18,
            pins.p0_17,
        );

        let mut rst_pin = DW_RST::new(pins.p0_24.into_floating_input());

        let clocks = device.CLOCK.constrain().freeze();

        let mut delay = Delay::new(core.SYST, clocks);

        rst_pin.reset_dw1000(&mut delay);

        let dw1000 = dw1000.init().unwrap();

        RANDOM = rng;
        DW_RST_PIN = rst_pin;
        DW1000 = dw1000;
        LOGGER = Logger::new(uarte0);
        TIMER = timer;
        LED_RED_1 = pins.p0_14.degrade().into_push_pull_output(Level::High);
    }

    #[idle(resources = [TIMER, LED_RED_1, LOGGER, RANDOM, DW1000])]
    fn idle() -> ! {
        loop {
            // let mut out: String<U256> = String::new();
            // write!(&mut out, "got message! \r\n").unwrap();
            // write!(&mut out, "small: {:016X}\r\n", 10).unwrap();
            // write!(&mut out, "med:   {:016X}\r\n", 20).unwrap();
            // write!(&mut out, "large  {:016X}\r\n", 30).unwrap();
            // write!(&mut out, "text: {}\r\n", "Hi there!").unwrap();
            // resources.LOGGER.log(out.as_str()).unwrap();
            resources.LOGGER.log("hello!").unwrap();
            delay(resources.TIMER, 1_000_000);
        }
    }
};

use nb::{
    block,
};


pub fn delay<T>(timer: &mut Timer<T>, cycles: u32) where T: TimerExt {
    timer.start(cycles);
    block!(timer.wait()).expect("wait fail");
}