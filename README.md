# Audio Visualizer made in Rust
Project started out as an Audio Analyzer in order to practice theory learned in Digital Signal Processing course, ended up turning it into a visualizer too.\
<img width="796" height="599" alt="image" src="https://github.com/user-attachments/assets/03b1d348-848b-42c5-82ba-ac3c614c247f" />\

## Dependencies
cpal - Record output/input audio\
rustfft - Fast Fourier Transform algorithm\
eframe - For visualizer program\
egui_plot - Histogram plot

## How to change from visualizing output (desktop audio) to input (microphone audio):
Change these lines:

### main.rs, line 14:
`let device = host.default_output_device().expect("Coldnt find Input device");`\
to\
`let device = host.default_input_device().expect("Coldnt find Input device");`\

### main.rs, line 20:
`let supported_configs = device.default_output_config().expect("Couldnt get default input confih");`\
to\
`let supported_configs = device.default_input_config().expect("Couldnt get default input confih");`
