#![no_main]
#![no_std]

use core::cell::RefCell;
use core::convert::TryInto;
use core::{mem, slice};
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use daisy_bsp as daisy;
use granulator::*;

use daisy_bsp::hal;
use hal::delay::Delay;
use hal::prelude::*;

use daisy_bsp::pac;
use pac::interrupt;

use daisy::sdram;
use daisy_bsp::audio;
use daisy_bsp::led::Led;
use daisy_bsp::logger;
use daisy_bsp::loggit;
use log::info;

// - static global state ------------------------------------------------------

static AUDIO_INTERFACE: Mutex<RefCell<Option<audio::Interface>>> = Mutex::new(RefCell::new(None));
const DELAY_BUFFFER_SIZE: usize = 48000;

// - entry point --------------------------------------------------------------

#[entry]
fn main() -> ! {
    fn clamp(range: (f32, f32), v: f32) -> f32 {
        if (v < range.0) {
            return range.0;
        }
        if (v > range.1) {
            return range.1;
        }
        return v;
    };

    fn mapRange(fromRange: (f32, f32), toRange: (f32, f32), number: f32) -> f32 {
        (clamp(fromRange, number) - fromRange.0) * (toRange.1 - toRange.0)
            / (fromRange.1 - fromRange.0)
            + toRange.0
    }

    logger::init();
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();

    let dp = pac::Peripherals::take().unwrap();

    let ccdr = board.freeze_clocks(dp.PWR.constrain(), dp.RCC.constrain(), &dp.SYSCFG);

    let pins = board.split_gpios(
        dp.GPIOA.split(ccdr.peripheral.GPIOA),
        dp.GPIOB.split(ccdr.peripheral.GPIOB),
        dp.GPIOC.split(ccdr.peripheral.GPIOC),
        dp.GPIOD.split(ccdr.peripheral.GPIOD),
        dp.GPIOE.split(ccdr.peripheral.GPIOE),
        dp.GPIOF.split(ccdr.peripheral.GPIOF),
        dp.GPIOG.split(ccdr.peripheral.GPIOG),
        dp.GPIOH.split(ccdr.peripheral.GPIOH),
        dp.GPIOI.split(ccdr.peripheral.GPIOI),
    );

    let mut led_user = daisy::led::LedUser::new(pins.LED_USER);

    let i2c2_pins = (
        pins.WM8731.SCL.into_alternate_af4(),
        pins.WM8731.SDA.into_alternate_af4(),
    );

    let sai1_pins = (
        pins.WM8731.MCLK_A.into_alternate_af6(),
        pins.WM8731.SCK_A.into_alternate_af6(),
        pins.WM8731.FS_A.into_alternate_af6(),
        pins.WM8731.SD_A.into_alternate_af6(),
        pins.WM8731.SD_B.into_alternate_af6(),
    );

    let sai1_prec = ccdr
        .peripheral
        .SAI1
        .kernel_clk_mux(hal::rcc::rec::Sai1ClkSel::PLL3_P);

    let i2c2_prec = ccdr.peripheral.I2C2;

    let audio_interface = audio::Interface::init(
        &ccdr.clocks,
        sai1_prec,
        sai1_pins,
        i2c2_prec, // added i2c init
        i2c2_pins,
        ccdr.peripheral.DMA1,
    )
    .unwrap();

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, ccdr.clocks);
    let sdram: &mut [f32] = sdram::Sdram::new(
        dp.FMC,
        ccdr.peripheral.FMC,
        &ccdr.clocks,
        &mut delay,
        &mut cp.SCB,
        &mut cp.MPU,
        pins.SDRAM,
    )
    .into();

    let sdram_size_bytes = sdram::Sdram::bytes();
    let u32_size = 4;
    let sdram_size = sdram_size_bytes / u32_size;

    // Make sure that we're not reading memory from a previous test run
    info!("Clear memory... ");
    // for item in sdram.iter_mut().take(sdram_size) {
    //     *item = 0.0;
    // }

    let delay_line_buffer: &'static mut [Frame; DELAY_BUFFFER_SIZE] = unsafe {
        slice::from_raw_parts_mut(core::mem::transmute(sdram.as_mut_ptr()), DELAY_BUFFFER_SIZE)
            .try_into()
            .unwrap()
    };

    for item in delay_line_buffer.iter_mut() {
        *item = (0.0, 0.0);
    }

    let options: GranulatorOptions<DELAY_BUFFFER_SIZE> = GranulatorOptions {
        delay_line_buffer: Some(delay_line_buffer),
        ..GranulatorOptions::default()
    };
    let mut granulator = Granulator::new(options);

    // - audio callback -------------------------------------------------------
    let audio_interface = {
        audio_interface.spawn(move |fs, block| {
            for frame in block {
                let (left, right) = *frame;
                let (processed_left, processed_right) = &granulator.process((left, right));
                *frame = (*processed_left, *processed_right);
            }
        })
    };

    let audio_interface = match audio_interface {
        Ok(audio_interface) => audio_interface,
        Err(e) => {
            loggit!("Failed to start audio interface: {:?}", e);
            loop {}
        }
    };

    cortex_m::interrupt::free(|cs| {
        AUDIO_INTERFACE.borrow(cs).replace(Some(audio_interface));
    });

    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().0;

    loop {
        led_user.on();
        asm::delay(one_second);
        led_user.off();
        asm::delay(one_second);
    }
}

// - interrupts ---------------------------------------------------------------

/// interrupt handler for: dma1, stream1
#[interrupt]
fn DMA1_STR1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(audio_interface) = AUDIO_INTERFACE.borrow(cs).borrow_mut().as_mut() {
            match audio_interface.handle_interrupt_dma1_str1() {
                Ok(()) => (),
                Err(e) => {
                    loggit!("Failed to handle interrupt: {:?}", e);
                }
            };
        }
    });
}
