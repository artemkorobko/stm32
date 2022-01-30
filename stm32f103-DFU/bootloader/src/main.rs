#![no_main]
#![no_std]

use panic_halt as _;

mod device_id;
mod dfu;
mod drivers;
mod protocols;
mod tasks;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [TAMPER])]
mod app {
    use stm32f1xx_hal::{flash, gpio, prelude::*, usb};

    use crate::{device_id, drivers::cdc_acm};

    pub const FLASH_SIZE: flash::FlashSize = flash::FlashSize::Sz64K;

    #[shared]
    struct Shared {
        usb: cdc_acm::Device,
        flash: flash::Parts,
    }

    #[local]
    struct Local {}

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

        assert!(clocks.usbclk_valid());

        // Disable JTAG
        let mut gpioa = pac.GPIOA.split();
        let gpiob = pac.GPIOB.split();
        let (_pa15, _pb3, _pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure USB
        let usb_dp = gpioa
            .pa12
            .into_push_pull_output_with_state(&mut gpioa.crh, gpio::PinState::Low);
        let cpu_cycles_hz = clocks.sysclk().0;
        cortex_m::asm::delay(cpu_cycles_hz / 100);
        let usb_peripheral = usb::Peripheral {
            usb: pac.USB,
            pin_dm: gpioa.pa11,
            pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
        };
        let usb_descriptor = cdc_acm::Descriptor {
            vendor_id: 0x0483,
            product_id: 0x5740,
            manufacturer: "STMicroelectronics",
            product: "STM32 Virtual ComPort",
            serial_number: device_id::read_str(),
        };
        let usb = cdc_acm::Device::new(usb_peripheral, usb_descriptor);

        (Shared { usb, flash }, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    use crate::tasks::*;

    extern "Rust" {
        #[task(binds = USB_HP_CAN_TX, shared = [usb])]
        fn usb_tx(cx: usb_tx::Context);
        #[task(binds = USB_LP_CAN_RX0, shared = [usb, flash])]
        fn usb_rx(cx: usb_rx::Context);
    }
}
