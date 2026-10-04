# Audio Spectrum Visualizer
Project started out as an Audio Analyzer in order to practice theory learned in Digital Signal Processing course, ended up turning it into a visualizer too.\
<img width="797" height="596" alt="image" src="https://github.com/user-attachments/assets/7375a848-aee7-44ab-9f69-8f8046c349e2" />


## Dependencies
[cpal](https://github.com/RustAudio/cpal) - Record output/input audio\
[rustfft](https://github.com/ejmahler/rustfft) - Fast Fourier Transform algorithm\
[egui_plot](https://github.com/emilk/egui_plot) - Histogram plot\
eframe - For visualizer program

## How to change from visualizing output (desktop audio) to input (microphone audio):
Change these lines:

### main.rs, line 14:
`let device = host.default_output_device().expect("Coldnt find Input device");`\
to\
`let device = host.default_input_device().expect("Coldnt find Input device");`

### main.rs, line 20:
`let supported_configs = device.default_output_config().expect("Couldnt get default input confih");`\
to\
`let supported_configs = device.default_input_config().expect("Couldnt get default input confih");`
