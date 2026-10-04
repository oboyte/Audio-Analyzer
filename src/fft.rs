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
        let mag =  f32::sqrt(f32::powi(bins[n].re, 2) + f32::powi(bins[n].im, 2) ) / bins.len() as f32;
        let magnitude = 20.0 * mag.max(1e-12).log10();

        magnitude_spectrum.push((frequency, magnitude));
    }   
    magnitude_spectrum
}

pub fn get_dominant_freq(mag_spectrum: &[(f32, f32)])  -> (f32, f32) {
    let mut max: (f32, f32) = (0.0, f32::NEG_INFINITY);
    for &(freq, mag) in mag_spectrum {
        if mag >= max.1 {
            max = (freq, mag);
        }
    }
    max
}