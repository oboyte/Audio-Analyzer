use eframe::egui;
use egui_plot::{Bar, BarChart, Plot, PlotBounds};
use std::sync::{Arc, Mutex};

pub struct Visualizer {
    spectrum: Arc<Mutex<Vec<(f32, f32)>>>,
}

impl Visualizer {
    pub fn new(spectrum: Arc<Mutex<Vec<(f32, f32)>>>) -> Self {
        Self {
            spectrum
        }
    }
}

impl eframe::App for Visualizer {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        let spectrum = self.spectrum.lock().unwrap();
        ui.heading("Audio Spectrum");

        let floor = -150.0;

        let bars: Vec<Bar> = spectrum
            .iter()
            .map(|&(frequency, magnitude)| {
                Bar::new(
                    frequency as f64,
                    (magnitude as f64).max(floor) - floor,
                )
                .base_offset(floor)
                .width(60.0)
            })
            .collect();

        let chart = BarChart::new("Spectrum", bars)
            .element_formatter(Box::new(|bar, _chart| {
                format!("Frekvens: {:.1}, Magnitude: -{:.1} dB", bar.argument, bar.value)
            }));

        Plot::new("spectrum_plot")
            //.x_axis_label("Frequency [Hz]")
            //.y_axis_label("Magnitude")
            //.include_y(-0.2)
            //.include_y(80)
            //.include_x(0)
            //.include_x(24_000)
            .auto_bounds(false)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart);
                plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                    [0.0, floor], 
                    [24_000.0, 0.0]
                ));
                
            });
        ui.ctx().request_repaint();
    }
}