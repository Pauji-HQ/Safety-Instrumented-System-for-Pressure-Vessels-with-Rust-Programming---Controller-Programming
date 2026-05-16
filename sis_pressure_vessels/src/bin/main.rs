#![no_std]
#![no_main]

use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation, AdcCalCurve},
    gpio::{Input, Output, Level, Pull, InputConfig, OutputConfig}, 
    main,
    peripherals::ADC1,
};
use esp_println::println;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    let mut adc1_config = AdcConfig::new();
    let mut adc_pin = adc1_config.enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(peripherals.GPIO1, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let mut pin35 = Output::new(peripherals.GPIO35, Level::Low, OutputConfig::default());
    let mut pin37 = Output::new(peripherals.GPIO37, Level::Low, OutputConfig::default());
    
    let pin36 = Input::new(peripherals.GPIO36, InputConfig::default().with_pull(Pull::Up));

    let mut sample_count: u32 = 0;
    let mut accumulator: u32 = 0;
    let mut alarm_active = false;
    let mut led: u32 = 0;
    let mut solenoid: u32 = 0;


    loop {
        if pin36.is_low() {
            if alarm_active {
                alarm_active = false;
                pin37.set_low();
                led = 0;
            }
        }

        match nb::block!(adc1.read_oneshot(&mut adc_pin)) {
            Ok(mv) => {
                accumulator += mv as u32;
                sample_count += 1;
            }
            Err(_) => {}
        }

        if sample_count >= 1000 {
            let average_v = (accumulator as f32 / 1000.0) / 1000.0;
            let pressure = 3.0 * (average_v - 0.5);
            
            println!("{:.3},{:.3},{},{}", average_v, pressure, led, solenoid);

            if pressure >= 4.0 {
                pin37.set_high();
                led = 1;
            }
            if pressure >= 5.0 {
                if !alarm_active {
                    alarm_active = true;
                }
            }

            accumulator = 0;
            sample_count = 0;
        }

        if alarm_active {
            pin35.set_high();
            solenoid = 1;
        } else {
            pin35.set_low();   
            solenoid = 0;         
        }

    }
}