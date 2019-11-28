#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux16::{entry, iprint, iprintln, prelude::*, I16x3, Sensitivity};


extern crate panic_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use f3::{
    hal::{prelude::*, spi::Spi, stm32f30x},
    l3gd20, L3gd20,
};

#[entry]
fn main() -> ! {
    // let (mut lsm303dlhc, mut delay, _mono_timer, mut itm) = aux16::init();

    // // extend sensing range to `[-12g, +12g]`
    // lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();
    // loop {
    //     const SENSITIVITY: f32 = 12. / (1 << 14) as f32;

    //     let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

    //     let x = f32::from(x) * SENSITIVITY;
    //     let y = f32::from(y) * SENSITIVITY;
    //     let z = f32::from(z) * SENSITIVITY;

    //     iprintln!(&mut itm.stim[0], "{:?} 3D digital linear acceleration sensor", (x, y, z));
    //     iprintln!(&mut itm.stim[0], "{:?} 3D digital magnetic sensor",lsm303dlhc.mag().unwrap());
        


    //     delay.delay_ms(1_000_u16);
    // }


    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // TRY the other clock configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high();

    // The `L3gd20` abstraction exposed by the `f3` crate requires a specific pin configuration to
    // be used and won't accept any configuration other than the one used here. Trying to use a
    // different pin configuration will result in a compiler error.
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    // sanity check: the WHO_AM_I register always contains this value
    assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    let _m = l3gd20.all().unwrap();

    // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // gyroscope and the temperature sensor readings
    asm::bkpt();

    loop {}
}
}
