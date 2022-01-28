use crate::app::blink;

pub fn blink(cx: blink::Context) {
    cx.local.led.toggle();
    cx.local.timer.clear_update_interrupt_flag();
}
