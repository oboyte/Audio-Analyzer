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