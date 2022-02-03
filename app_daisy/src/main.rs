#![no_main]
#![no_std]
use core::convert::TryInto;
use core::{mem, slice};
use granulator::*;
use log::info;

use stm32h7xx_hal::stm32;
use stm32h7xx_hal::timer::Timer;

use libdaisy::audio;
use libdaisy::logger;
use libdaisy::prelude::*;
use libdaisy::system;

// const DELAY_BUFFFER_SIZE: usize = 64 * 1024 * 1024 / 2 / mem::size_of::<u32>();
const DELAY_BUFFFER_SIZE: usize = libdaisy::sdram::Sdram::bytes() / 2 / mem::size_of::<f32>();

#[rtic::app(
    device = stm32h7xx_hal::stm32,
    peripherals = true,
    monotonic = rtic::cyccnt::CYCCNT,
)]
const APP: () = {
    struct Resources {
        audio: audio::Audio,
        buffer: audio::AudioBuffer,
        timer2: Timer<stm32::TIM2>,
        granulator: Granulator<'static, DELAY_BUFFFER_SIZE>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        logger::init();
        let mut system = system::System::init(ctx.core, ctx.device);
        let buffer = [(0.0, 0.0); audio::BLOCK_SIZE_MAX];
        system.timer2.set_freq(1.ms());

        let delay_line_buffer: &'static mut [Frame; DELAY_BUFFFER_SIZE] = unsafe {
            slice::from_raw_parts_mut(
                core::mem::transmute(system.sdram.as_mut_ptr()),
                DELAY_BUFFFER_SIZE,
            )
            .try_into()
            .unwrap()
        };

        let options = GranulatorOptions {
            delay_line_buffer: Some(delay_line_buffer),
            ..GranulatorOptions::default()
        };
        let granulator = Granulator::new(options);

        info!("Startup done!");

        init::LateResources {
            audio: system.audio,
            buffer,
            timer2: system.timer2,
            granulator,
        }
    }

    // Interrupt handler for audio
    #[task( binds = DMA1_STR1, resources = [audio, buffer, granulator], priority = 8 )]
    fn audio_handler(ctx: audio_handler::Context) {
        let audio = ctx.resources.audio;
        let buffer = ctx.resources.buffer;
        let granulator = ctx.resources.granulator;

        if audio.get_stereo(buffer) {
            for (left, right) in buffer {
                let frame = granulator.process((*left, *right));
                audio.push_stereo(frame).unwrap();
            }
        } else {
            info!("Error reading data!");
        }
    }

    // Non-default idle ensures chip doesn't go to sleep which causes issues for
    // probe.rs currently
    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    // Knobs, buttons, switches handling.
    #[task( binds = TIM2, resources = [timer2, granulator] )]
    fn interface_handler(ctx: interface_handler::Context) {
        ctx.resources.timer2.clear_irq();
        cortex_m::asm::nop();
    }
};
