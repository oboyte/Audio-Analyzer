use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use rustfft::{FftPlanner};

mod window;
mod fft;


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

    // Want to collect 1024 samples from left microphone (monotone)
    let mut sample_buffer: Vec<f32> = Vec::new();

    let fft_size = 1024;
    // The planner in rustfft chooses which algorithm is best suited for the task
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size); // We want to get freq. components (bins), thus we use FFT forward algorithms
            

    let stream = device.build_input_stream(
        &config, 
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // React to stream events and read or write stream data inside this
            println!("Samples received: {}", data.len());

            for frame in data.chunks_exact(2) {
                sample_buffer.push(frame[0]); // Collect only left data
            }
            
            // rustfmt del
            while sample_buffer.len() >= fft_size {
                // Collect first 1024 samples into a block
                let fft_block = &sample_buffer[..fft_size];

                // Start with getting the windowed signal (x[n] * w[n]) to reduce leakage
                let windowed_signal = crate::window::get_windowed_signal(fft_block);
                
                // Get complex buffer {re:samp + im:j0}
                let mut buffer = crate::fft::get_buffer(&windowed_signal);

                fft.process(&mut buffer);

                // Remove first 1024 elements, let remaining carry on to the next time this if statement runs.
                sample_buffer.drain(..fft_size);

                // Get magnitude spectrum (frequency, magnitude)
                let spectrum = crate::fft::get_spectrum(&buffer, supported_configs.sample_rate().0);
            }
            
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
