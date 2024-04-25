//! # Processing
//! This module contains a bunch of built-in processing algorithms
//! (extraction events from waveforms)
//! see [params](crate::process::ProcessParams) for details.
//! 

use std::collections::BTreeMap;

#[cfg(feature = "egui")]
use {
    egui_plot::{HLine, VLine, Line, PlotUi},
    egui::Color32,
    crate::utils::color_for_index
};

use numass::protos::rsb_event;
use serde::{Deserialize, Serialize};

use crate::{constants::{baseline_2024_03, KEV_COEFF_FIRST_PEAK, KEV_COEFF_LIKHOVID, KEV_COEFF_MAX, KEV_COEFF_TRAPEZIOD}, types::{NumassEvent, NumassEvents, NumassWaveforms, ProcessedWaveform, RawWaveform}};


#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub struct HWResetParams {
    pub window: usize,
    pub treshold: i16,
    pub size: usize,
}

/// Built-in algorithms params for processing the data.
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum Algorithm {
    Max,
    Likhovid { left: usize, right: usize },
    FirstPeak { threshold: i16, left: usize },
    Trapezoid { 
        left: usize, center: usize, right: usize,
        treshold: i16,
        min_length: usize,
        reset_detection: HWResetParams,
    }
}

/// Неизменяемые параметры, необходимые для обработки кадра
/// могут либо задаваться статично, либо на каждую точку
#[derive(Clone)]
pub struct StaticProcessParams {
    pub baseline: Option<Vec<f32>> // TODO: make more versatile
}

impl StaticProcessParams {
    pub fn from_point(point: &rsb_event::Point) -> Self {
        let time = point.channels[0].blocks[0].time;
        Self {
            baseline: Some(vec![
                0.0, // ch1
                baseline_2024_03(time, 1), // ch2
                baseline_2024_03(time, 2), // ch3
                0.0, // ch4
                baseline_2024_03(time, 4), // ch5
                baseline_2024_03(time, 5), // ch6
                baseline_2024_03(time, 6), // ch7
            ])
        }
    }
}

pub const LIKHOVID_DEFAULT: Algorithm = Algorithm::Likhovid { left: 15, right: 36 };
pub const FIRSTPEAK_DEFAULT: Algorithm = Algorithm::FirstPeak { threshold: 10, left: 8 };
pub const TRAPEZOID_DEFAULT: Algorithm = Algorithm::Trapezoid { 
    left: 6, center: 15, right: 6,
    treshold: 27,
    min_length: 10,
    reset_detection: HWResetParams {
        window: 10,
        treshold: 800,
        size: 110
    }
};

impl Default for Algorithm {
    fn default() -> Self {
        Self::FirstPeak { threshold: 10, left: 8 }
    }
}

/// Built-in processing params.
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Hash)]
pub struct ProcessParams {
    pub algorithm: Algorithm,
    pub convert_to_kev: bool,
}

impl Default for ProcessParams {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::default(),
            convert_to_kev: true,
        }
    }
}

pub fn extract_waveforms(point: &rsb_event::Point) -> NumassWaveforms {
    let mut waveforms = BTreeMap::new();

    for channel in &point.channels {
        for block in &channel.blocks {
            for frame in &block.frames {
                let entry = waveforms.entry(frame.time).or_insert(BTreeMap::new());
                let waveform = process_waveform(frame);
                entry.insert(channel.id as u8, waveform);
            }
        }
    }
    waveforms
}

/// Built-in processing algorithm.
/// Function will extract events point wafevorms and keeps its hierarchy.
/// Do not use this function directly without reason, use [process_point](crate::storage::process_point) instead.
pub fn extract_events(point: &rsb_event::Point, params: &ProcessParams) -> NumassEvents {

    // TODO: merge with extract_waveforms (will affects performance?)
    let mut amplitudes = BTreeMap::new();
    let static_params = StaticProcessParams::from_point(point);

    for channel in &point.channels {
        for block in &channel.blocks {
            for frame in &block.frames {
                let entry = amplitudes.entry(frame.time).or_insert(BTreeMap::new());

                let waveform = process_waveform(frame);

                for (time, amp) in waveform_to_events(
                    &waveform, 
                    channel.id as u8, 
                    &params.algorithm, 
                    &static_params,
                    #[cfg(feature = "egui")] None
                ) {
                    let amp: f32 = if params.convert_to_kev {
                        convert_to_kev(&amp, channel.id as u8, &params.algorithm)
                    } else {
                        amp
                    };
                    entry.entry(channel.id as usize).or_insert(Vec::new()).push((time, amp));
                }
            }
        }
    }

    amplitudes
}

/// Prepare raw waveform stored in protobuf message for processing.
/// This function will calculate baseline and subtract it from the waveform.
/// TODO: add static correction
/// TODO: transform to impl From<RawWaveform> for ProcessedWaveform and move to types.rs
pub fn process_waveform(waveform: impl Into<RawWaveform>) -> ProcessedWaveform {
    let waveform = waveform.into();
    ProcessedWaveform(waveform.0.iter().map(|bin| *bin as f32).collect::<Vec<_>>())
}

/// Built-in keV convertion (according to crate::constants).
/// TODO: make configurable
pub fn convert_to_kev(amplitude: &f32, ch_id: u8, algorithm: &Algorithm) -> f32 {
    match algorithm {
        Algorithm::Max => {
            let [a, b] = KEV_COEFF_MAX[ch_id as usize];
            a * *amplitude + b
        }
        Algorithm::Likhovid { .. } => {
            let [a, b] = KEV_COEFF_LIKHOVID[ch_id as usize];
            a * *amplitude + b
        }
        Algorithm::FirstPeak { .. } => {
            let [a, b] = KEV_COEFF_FIRST_PEAK[ch_id as usize];
            a * *amplitude + b
        }
        Algorithm::Trapezoid { .. } => {
            let [a, b] = KEV_COEFF_TRAPEZIOD[ch_id as usize];
            a * *amplitude + b
        }
    }
}

/// Extract events from single waveform.
/// Do not use this function directly without reason, use [extract_events](crate::process::extract_events) instead.
/// TODO: add ui argument description
pub fn waveform_to_events(
    waveform: &ProcessedWaveform, 
    _ch_id: u8, algorithm: &Algorithm,
    static_params: &StaticProcessParams,
    #[cfg(feature = "egui")] ui: Option<&mut PlotUi>
) -> Vec<NumassEvent> {
    

    match algorithm {
        Algorithm::Max => {
            let (x, y) = waveform.0
            .iter()
            .enumerate()
            .max_by(|first, second| {
                first.1.partial_cmp(second.1).unwrap()
            })
            .unwrap();
            vec![(x as u16 * 8, *y)]
        }
        Algorithm::Likhovid { left, right } => {

            let (x, _) = waveform.0
            .iter()
            .enumerate()
            .max_by(|first, second| {
                first.1.partial_cmp(second.1).unwrap()
            })
            .unwrap();

            let amplitude = {
                let left = if x >= *left { x - left } else { 0 };
                let right = std::cmp::min(waveform.0.len(), x + right);
                let crop = &waveform.0[left..right];
                crop.iter().sum::<f32>() / crop.len() as f32
            };

            vec![(x as u16 * 8, amplitude)]
        }
        Algorithm::FirstPeak { threshold, left } => {
            let pos = find_first_peak(waveform, *threshold as f32);
            if let Some(pos) = pos {
                let left = if pos < *left {
                    0
                } else {
                    pos - left
                };
                // let length = (waveform.0.len() - pos) as f32;
                let amplitude = waveform.0[left..waveform.0.len()].iter().sum::<f32>();
                vec![(pos as u16 * 8, amplitude / 50.0)]
            } else {
                vec![]
            }
        }
        Algorithm::Trapezoid { 
            left, center, right, 
            treshold,
            min_length, 
            reset_detection: HWResetParams { 
                window: r_window, treshold: r_treshold, size: r_size } } => {

            let baseline = if let StaticProcessParams { baseline: Some(baseline) } = static_params {
                baseline[_ch_id as usize]
            } else {
                0.0
            };

            let mut events = vec![];

            let offset = left + center + right;

            let filtered = waveform.0.windows(left + center + right).map(|window| {
                (window[left+center..].iter().sum::<f32>() - window[..*left].iter().sum::<f32>()) / (left + right) as f32 - baseline
            }).collect::<Vec<_>>();

            #[cfg(feature = "egui")]
            let mut resets = vec![];
            #[cfg(feature = "egui")]
            let mut event_ranges = vec![];

            let mut i = 0;
            while i < filtered.len() {
                if i < filtered.len() - r_window && filtered[i] - filtered[i + r_window] > *r_treshold as f32 {
                    #[cfg(feature = "egui")]
                    resets.push(i);
                    i += r_size;
                    continue;
                }

                if (i == 0 ||  filtered[i - 1] < *treshold as f32) && filtered[i] >= *treshold as f32 {
                    let mut energy = 0.0;
                    let mut event_end = i;

                    while event_end < filtered.len() && filtered[event_end] >= *treshold as f32   {
                        energy += filtered[event_end];
                        event_end += 1
                    }

                    if (event_end - i) >= *min_length {
                        events.push(((i + offset) as u16 * 8, energy / offset as f32));
                        #[cfg(feature = "egui")]
                        event_ranges.push((i, event_end));
                    }

                    i = event_end;
                    continue;
                }

                i += 1;
            }

            #[cfg(feature = "egui")]
            if let Some(ui) = ui {
                let line = Line::new(
                    filtered.clone().into_iter().enumerate().map(|(idx, amp)| [(idx + offset) as f64, amp as f64]).collect::<Vec<_>>())
                    .color(color_for_index(_ch_id as usize))
                    .name(format!("filtered + baseline ch# {}", _ch_id + 1))
                    .style(egui_plot::LineStyle::Dashed { length: 10.0 });
                ui.line(line);

                ui.hline(
                    HLine::new(*treshold as f64)
                    .color(Color32::WHITE)
                    .name(format!("TRIGGER"))
                );

                for (idx, event_range) in event_ranges.into_iter().enumerate() {
                    ui.vline(VLine::new((event_range.0 + offset) as f64)
                        .color(color_for_index(_ch_id as usize))
                        .name(format!("ev# {idx} ch# {}", _ch_id + 1))
                    );
                    ui.vline(VLine::new((event_range.1 + offset) as f64)
                        .color(color_for_index(_ch_id as usize))
                        .name(format!("ev# {idx} ch# {}", _ch_id + 1))
                    );
                }

                for reset in resets {
                    ui.vline(VLine::new((reset + offset) as f64).color(Color32::WHITE).name(format!("RESET")));
                    ui.vline(VLine::new((reset + r_size + offset) as f64).color(Color32::WHITE).name(format!("RESET")));
                }
            };

            events
        }
    }
}

pub fn find_first_peak(waveform: &ProcessedWaveform, threshold: f32) -> Option<usize> {
    waveform.0
        .iter()
        .enumerate()
        .find(|(idx, amp)| {
            let amp = **amp;
            amp > threshold
                && (*idx == 0 || waveform.0[idx - 1] <= amp)
                && (*idx == waveform.0.len() - 1 || waveform.0[idx + 1] <= amp)
        })
        .map(|(idx, _)| idx)
}
