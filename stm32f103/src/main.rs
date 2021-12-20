#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {}

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
