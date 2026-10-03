use cpal::Data;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

use crate::window::get_windowed_signal;

mod window;

fn main() {
    // Get access to local audio devices on computer
    let host = cpal::default_host();

    // Find default input devices
    let device = host.default_input_device().expect("Coldnt find Input device");

    println!("Device name: {}", device.name().expect("Couldnt find name"));

    // Find the supported configs range for the input device
    //let mut supported_configs_range = device.supported_input_configs().expect("error while querying configs");
    let supported_configs = device.default_input_config().expect("Couldnt get default input confih");

    println!("Sample format: {:?}", supported_configs.sample_format());
    println!("Sample rate: {}", supported_configs.sample_rate().0);
    println!("Channels: {}", supported_configs.channels());

    let config = supported_configs.config();

    let stream = device.build_input_stream(
        &config, 
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // React to stream events and read or write stream data inside this
            println!("Samples received: {}", data.len());

            // Data gets x[n] values, as in samples from microphone, we want to convert these to X[m] values, so into frequency components.
            
            // Start with getting the windowed signal (x[n] * w[n]) to reduce leakage
            let windowed_signal = get_windowed_signal(data);
            
        },
        move |err| {
            // error reactions go here
            eprintln!("Stream error: {}", err);
        }, 
        None
    ).expect("Couldnt unpack input stream");

    // Start stream
    stream.play().expect("Error, couldnt start stream");
    println!("Stream started.");

    std::thread::sleep(std::time::Duration::from_secs(10));

    
}
