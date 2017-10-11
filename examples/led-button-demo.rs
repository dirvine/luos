extern crate luos;

use luos::module::{Button, Led};

const BUTTON_PIN: u8 = 9;
const LED_PIN: u8 = 13;


fn main() {
    let core = luos::Core::new();

    let fire_button = Button::new("fire_button", BUTTON_PIN);
    core.register(&fire_button);

    let disco_led = Led::new("disco_led", LED_PIN);
    core.register(&disco_led);

    loop {
        if fire_button.pressed() {
            disco_led.on();
        } else {
            disco_led.off();
        }
    }
}
