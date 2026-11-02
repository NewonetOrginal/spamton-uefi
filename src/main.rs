#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::time::Duration;
use uefi::boot;
use uefi::prelude::*;

// line printing with delay
fn type_writer(text: &str, delay_ms: u64) {
    for c in text.chars() {
        uefi::print!("{}", c);
        boot::stall(Duration::from_millis(delay_ms));
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    type_writer("HOLY ", 70);
    type_writer("[[Cungadero]] ", 50);
    type_writer("DO I FEEL GOOD ", 80);
    type_writer("...\n", 200);

    type_writer("HERE I AM!! ", 70);
    boot::stall(Duration::from_millis(250));
    type_writer("KRIS!!\n", 70);

    type_writer("BIG\n", 100);
    boot::stall(Duration::from_millis(250));
    type_writer("BIG,\n", 150);
    boot::stall(Duration::from_millis(250));
    type_writer("[[BIGGER AND BETTER THAN EVER]]\n", 90);

    type_writer("HA ", 50);
    boot::stall(Duration::from_millis(100));
    type_writer("HA ", 50);
    boot::stall(Duration::from_millis(100));
    type_writer("HA ", 50);
    boot::stall(Duration::from_millis(100));
    type_writer("... ", 250);
    type_writer("THIS POWER IS\n", 70);
    boot::stall(Duration::from_millis(500));
    type_writer("FREEDOM\n", 250);
    boot::stall(Duration::from_millis(500));

    type_writer("I WON'T HAVE TO BE JUST A PUPPET ANY MORE!!!!\n", 60);
    boot::stall(Duration::from_millis(1000));
    type_writer("...\n", 1000);
    boot::stall(Duration::from_millis(100));

    type_writer("OR...", 120);
    type_writer("so...", 110);
    type_writer("I...", 100);
    type_writer("thought.\n", 90);
    boot::stall(Duration::from_millis(100));

    type_writer("WHAT ARE THESE STRINGS!? ", 120);
    boot::stall(Duration::from_millis(1000));
    type_writer("WHY AM I NOT ", 500);
    type_writer("[BIG] ", 60);
    type_writer("ENOUGH!? ", 120);
    boot::stall(Duration::from_millis(1000));
    type_writer("It's still DARK", 120);
    type_writer("... ", 350);
    type_writer("SO DARK!\n", 100);
    boot::stall(Duration::from_millis(1000));

    //HOT SPAMTONS IN YOUR NEIGHBOURHOOD

    type_writer("KRIS.\n", 250);
    boot::stall(Duration::from_millis(400));

    type_writer("KRIS. ", 150);
    type_writer("KRIS. ", 150);
    type_writer("KRIS.\n", 150);
    boot::stall(Duration::from_millis(500));

    type_writer("THAT'S RIGHT. ", 70);
    type_writer("YOU. ", 150);
    type_writer("I NEED YOU.\n", 90);
    boot::stall(Duration::from_millis(400));

    type_writer("TO BE BIG. ", 80);
    type_writer("WITH ME.\n", 80);
    boot::stall(Duration::from_millis(500));

    type_writer("VERY    VERY    BIG\n", 180);
    boot::stall(Duration::from_millis(600));

    type_writer("SO BIG WE'LL STAND UP TALL AND SEE PAST THE DARK\n", 50);
    boot::stall(Duration::from_millis(300));
    type_writer("STAND UP WITH OUR HEADS IN THE CLOUDS AND LOOK INTO\n", 50);
    boot::stall(Duration::from_millis(500));

    type_writer("H E A V E N\n", 250);
    boot::stall(Duration::from_millis(1200));

    type_writer("I JUST NEED THAT LITTLE,\n", 70);
    boot::stall(Duration::from_millis(300));
    type_writer("[[SOUL]] ", 40);
    type_writer("Y O U HAVE\n", 180);
    boot::stall(Duration::from_millis(1200));

    //usless chuds interfering with the glorius SPAMTON

    type_writer("[Susie] Kris!\n", 40);
    boot::stall(Duration::from_millis(300));
    type_writer("[Susie] The hell was that weirdo!?\n", 40);
    boot::stall(Duration::from_millis(500));

    type_writer("[Ralsei] Kris, we were worried about you, so...\n", 40);
    boot::stall(Duration::from_millis(800));

    // SPAMTON gets back his freedom of speech

    type_writer("KRIS!!! ", 70);
    type_writer("YOU HAVE ", 60);
    type_writer("[Friends]!?\n", 40);
    boot::stall(Duration::from_millis(400));
    type_writer("WHY DON'T YOU TELL THEM ABOUT MY ", 70);
    type_writer("[3 for One Specil]!\n", 40);
    boot::stall(Duration::from_millis(400));
    type_writer("TODAY, THE WHOLE FAMILY CAN TAKE A LITTLE ", 70);
    type_writer("[Ride Around Town]...\n", 40);
    boot::stall(Duration::from_millis(600));

    type_writer("[Attention Customers! Clean up on Aisle 3!]\n", 30);
    boot::stall(Duration::from_millis(500));

    type_writer("SOMEONE LEFT ", 80);
    type_writer("[There] ", 60);
    type_writer("SOULS, ", 100);
    type_writer("[Lyeing Around......]\n", 150);
    boot::stall(Duration::from_millis(1000));

    type_writer("Kris!?!? ", 80);
    type_writer("WAS THAT A ", 150);
    type_writer("[BIG SHOT] ", 50);
    type_writer("JUST NOW!?\n", 100);
    boot::stall(Duration::from_millis(500));
    type_writer("WOW!!! ", 100);
    type_writer("I'M SO ", 80);
    type_writer("[Proud] ", 50);
    type_writer("OF YOU, ", 80);
    type_writer("I COULD ", 60);
    type_writer("[Killed] ", 80);
    type_writer("YOU!\n", 60);
    boot::stall(Duration::from_millis(800));

    type_writer("[Heaven], ", 100);
    type_writer("are you ", 80);
    type_writer("WATCHING?\n", 500);
    boot::stall(Duration::from_millis(1000));

    type_writer("IT'S TIME TO MAKE A VERY ", 110);
    type_writer("[Specil] ", 60);
    type_writer("DEAL...\n", 90);
    boot::stall(Duration::from_millis(2000));

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
