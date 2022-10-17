#![no_std]
#![no_main]

#[cfg(feature="esp32")]
use esp32_hal as hal;
#[cfg(feature="esp32s2")]
use esp32s2_hal as hal;
#[cfg(feature="esp32s3")]
use esp32s3_hal as hal;
#[cfg(feature="esp32c3")]
use esp32c3_hal as hal;

use hal::{
    clock::ClockControl,
    pac::{self, Peripherals},
    gpio_types::{Event, Input, Pin, PullDown},
    interrupt,
    gpio::{Gpio7},
    prelude::*,
    spi,
    timer::TimerGroup,    
    systimer::SystemTimer,
    Rtc,
    IO,
    Delay,
};

use display_interface_spi::SPIInterfaceNoCS;
use pcd8544::PCD8544;
//use hc_sr04::{HcSr04, Unit};

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_graphics::image::Image;
use embedded_graphics::geometry::*;
use embedded_graphics::draw_target::DrawTarget;

use core::fmt::Write;
use core::cell::RefCell;
use critical_section::Mutex;

use profont::{PROFONT_24_POINT, PROFONT_18_POINT};

const SOUND_SPEED_0C: f32 = 331.3;

const SOUND_SPEED_INC_OVER_TEMP: f32 = 0.606;

#[cfg(feature="xtensa-lx-rt")]
use xtensa_lx_rt::entry;
#[cfg(feature="riscv-rt")]
use riscv_rt::entry;

use esp_println::println;
use esp_backtrace as _;

//static BUTTON: Mutex<RefCell<Option<Gpio7<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    #[cfg(any(feature = "esp32"))]
    let mut system = peripherals.DPORT.split();
    #[cfg(any(feature = "esp32s2", feature = "esp32s3", feature = "esp32c3"))]
    let mut system = peripherals.SYSTEM.split();

    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();


    

    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    
    let mut trigger = io.pins.gpio6.into_push_pull_output();
    trigger.set_low();

    
    let mut echo = io.pins.gpio7.into_pull_down_input();
    //echo.listen(Event::AnyEdge);

    // critical_section::with(|cs| BUTTON.borrow_ref_mut(cs).replace(echo));

    // interrupt::enable(pac::Interrupt::GPIO, interrupt::Priority::Priority1).unwrap();

    // unsafe {
    //     riscv::interrupt::enable();
    // }

    let sound_speed = SOUND_SPEED_0C + (SOUND_SPEED_INC_OVER_TEMP * 23.5);


    
    loop {
        // println!("Cooldown...");
        // trigger.set_low();
        // delay.delay_ms(2000 as u32);
        //println!("Trigger is high now!");
        trigger.set_high();
        delay.delay_us(10 as u32);
        trigger.set_low();
        //println!("Trigger is low again");

        while echo.is_low().unwrap() {}
        let start_timestamp = SystemTimer::now();
        while echo.is_high().unwrap() {}
        let end_timestamp = SystemTimer::now();

        println!("distance is : {}", sound_speed * ((end_timestamp as f32 - start_timestamp as f32) / 10000.0) / 2.0);

    }
        
    }