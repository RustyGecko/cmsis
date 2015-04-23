extern crate gcc;

use gcc::Config;

use std::env;

fn main() {

    println!("The ARM embedded toolchain must be available in the PATH");
    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");

    let mut config = Config::new();

    config
        .define("EFM32GG990F1024", None)

        .flag("-Wall")
        .flag("-mcpu=cortex-m3")
        .flag("-mthumb")

        .include("efm32-common/CMSIS/Include")
        .include("efm32-common/Device/EFM32GG/Include")

        .file("src/cmsis.c")

        .compile("libcmsis.a");

}
