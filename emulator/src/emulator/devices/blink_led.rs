use std::io::Write;

// Global variable for the LED strip
static mut LED_STRIP: [bool; 8] = [false; 8];

pub fn blink_led(address: u16, data: u8) {
    // If the address is 0x6002 and the data is 0xFF, then we want to turn on the LED
    if address == 0x6002 && data == 0xFF {
        unsafe {
            LED_STRIP[0] = true;
        }
    }

    // If the address is 0x6000, we want to enable the LED bits according to the data
    if address == 0x6000 {
        unsafe {
            for i in 0..8 {
                LED_STRIP[i] = (data & (1 << i)) != 0;
            }
        }
    }

    // Clear the line and print the LED strip
    print!("\x1B[K");
    print!("\rLED STRIP: ");
    for i in 0..8 {
        if unsafe { LED_STRIP[i] } {
            print!("█");
        } else {
            print!("░");
        }
    }

    // Flush stdout
    std::io::stdout().flush().unwrap();
}