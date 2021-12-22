#![no_main]
#![no_std]

use panic_halt as _;
use stm32f1xx_hal::{gpio, prelude::*, usb};

use crate::{
    drivers::cdc_acm,
    protocols::usb_device::{Inbound, Outbound, Reader, Writer},
};

mod device_id;
mod drivers;
mod protocols;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        usb: cdc_acm::Device,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Setup MCU
        let mut cp = cx.core;
        cp.DWT.enable_cycle_counter();

        // Configure peripherals
        let pac = cx.device;
        let mut flash = pac.FLASH.constrain();
        let mut rcc = pac.RCC.constrain();
        let mut afio = pac.AFIO.constrain(&mut rcc.apb2);
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        // Disable JTAG
        let mut gpioa = pac.GPIOA.split(&mut rcc.apb2);
        let gpiob = pac.GPIOB.split(&mut rcc.apb2);
        let (_pa15, _pb3, _pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure USB
        let usb_dp = gpioa
            .pa12
            .into_push_pull_output_with_state(&mut gpioa.crh, gpio::State::Low);
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

        init::LateResources { usb }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(resources = [usb])]
    fn handle_usb_inbound(cx: handle_usb_inbound::Context, inbound: Inbound) {
        let mut usb = cx.resources.usb;
        match inbound {
            Inbound::Version => {
                let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
                let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
                let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
                let outbound = Outbound::Version(major, minor, patch);
                usb.lock(|device| {
                    device.write_outbound(outbound).ok();
                });
            }
            Inbound::DeviceId => {
                let (id_0, id_1, id_2, id_3) = device_id::read();
                let outbound = Outbound::DeviceId(id_0, id_1, id_2, id_3);
                usb.lock(|device| {
                    device.write_outbound(outbound).ok();
                });
            }
            Inbound::Unknown => {}
        }
    }

    #[task(priority = 2, binds = USB_HP_CAN_TX, resources = [usb])]
    fn usb_tx(cx: usb_tx::Context) {
        cx.resources.usb.poll();
    }

    #[task(priority = 2, binds = USB_LP_CAN_RX0, spawn = [handle_usb_inbound], resources = [usb])]
    fn usb_rx0(cx: usb_rx0::Context) {
        let usb = cx.resources.usb;
        usb.poll_read_inbound(|inbound| {
            cx.spawn.handle_usb_inbound(inbound).ok();
        })
        .ok();
    }

    extern "C" {
        fn TAMPER();
    }
};
