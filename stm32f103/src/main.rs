#![no_main]
#![no_std]

use panic_halt as _;
use stm32f1xx_hal::prelude::*;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        // Setup MCU
        let mut cp = cx.core;
        cp.DWT.enable_cycle_counter();

        // Configure peripherals
        let pac = cx.device;
        let mut flash = pac.FLASH.constrain();
        let mut rcc = pac.RCC.constrain();
        let mut afio = pac.AFIO.constrain(&mut rcc.apb2);
        let _clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // Disable JTAG
        let gpioa = pac.GPIOA.split(&mut rcc.apb2);
        let gpiob = pac.GPIOB.split(&mut rcc.apb2);
        let (_pa15, _pb3, _pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    extern "C" {
        fn TAMPER();
    }
};
