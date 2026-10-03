use rustfft::num_complex::Complex;

pub fn get_buffer(win_signal: &[f32]) -> Vec<Complex<f32>> {
    let mut buffer: Vec<Complex<f32>> = Vec::with_capacity(win_signal.len());
    for n in 0..win_signal.len() {
        buffer.push(
            Complex { 
                re: win_signal[n], 
                im: 0f32 
            }
        )
    }
    buffer
}

pub fn get_spectrum(bins: &[Complex<f32>], sample_rate: u32) 
    -> Vec<(f32, f32)> 
{
    let mut magnitude_spectrum: Vec<(f32, f32)> = Vec::with_capacity(bins.len());
    for n in 0..bins.len() / 2 {
        // Insert frequency into index 0
        let frequency = n as f32 * sample_rate as f32 / bins.len() as f32;
        // Insert magnitude into index 1
        let magnitude = 
            f32::sqrt( 
                f32::powi(bins[n].re, 2) 
                + f32::powi(bins[n].im, 2) 
            );
        
        magnitude_spectrum.push((frequency, magnitude));
    }   
    magnitude_spectrum
}