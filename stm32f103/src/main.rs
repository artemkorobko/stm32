#![no_main]
#![no_std]

use panic_halt as _;

mod blink;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [TAMPER])]
mod app {
    use stm32f1xx_hal::{gpio, pac, prelude::*, timer};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
        timer: timer::CountDownTimer<pac::TIM1>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Setup MCU
        let mut cp = cx.core;
        cp.DWT.enable_cycle_counter();

        // Configure peripherals
        let pac = cx.device;
        let mut flash = pac.FLASH.constrain();
        let mut afio = pac.AFIO.constrain();
        let rcc = pac.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // Disable JTAG
        let gpioa = pac.GPIOA.split();
        let gpiob = pac.GPIOB.split();
        let (_pa15, _pb3, _pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Find LED pin
        let mut gpioc = pac.GPIOC.split();
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, gpio::PinState::High);

        // Start timer
        let mut timer = timer::Timer::tim1(pac.TIM1, &clocks).start_count_down(1.hz());
        timer.listen(timer::Event::Update);

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    use crate::blink::blink;

    extern "Rust" {
        #[task(binds = TIM1_UP, local = [led, timer])]
        fn blink(context: blink::Context);
    }
}
