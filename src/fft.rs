// Hanning window function
fn get_han_window(n_samp: usize) -> Vec<f32> {
    let mut win_vec:Vec<f32> = Vec::with_capacity(n_samp);
    for n in 0..n_samp {
        let value = 0.5*( 1f32 - f32::cos( 2f32* std::f32::consts::PI * n as f32 / ((n_samp-1) as f32)));
        win_vec.push( value);
    }
    return win_vec;
}

// x[n]w[n]
pub fn get_windowed_signal(samples: &[f32]) -> Vec<f32> {
    let han_window = get_han_window(samples.len());

    let mut win_signal: Vec<f32> = Vec::with_capacity(samples.len());

    for n in 0..samples.len() {
        win_signal.push(samples[n] * han_window[n]);
    }

    return win_signal;
}

