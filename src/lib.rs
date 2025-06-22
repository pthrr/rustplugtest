use nih_plug::prelude::*;
use std::sync::Arc;

/// A simple amplifier plugin
pub struct SimpleAmplifier {
    params: Arc<SimpleAmplifierParams>,
}

/// The plugin's parameters
#[derive(Params)]
struct SimpleAmplifierParams {
    /// The gain parameter in decibels
    #[id = "gain"]
    pub gain: FloatParam,

    /// Whether the plugin is bypassed
    #[id = "bypass"]
    pub bypass: BoolParam,
}

impl Default for SimpleAmplifier {
    fn default() -> Self {
        Self {
            params: Arc::new(SimpleAmplifierParams::default()),
        }
    }
}

impl Default for SimpleAmplifierParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            bypass: BoolParam::new("Bypass", false),
        }
    }
}

impl Plugin for SimpleAmplifier {
    const NAME: &'static str = "Simple Amplifier";
    const VENDOR: &'static str = "Your Name";
    const URL: &'static str = "";
    const EMAIL: &'static str = "your.email@example.com";
    const VERSION: &'static str = "0.1.0";

    // Unique plugin ID (generate a new one for your plugin)
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {
        // Clear any internal state here if needed
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // If bypassed, don't process anything
        if self.params.bypass.value() {
            return ProcessStatus::Normal;
        }

        for channel_samples in buffer.iter_samples() {
            // Get the current gain value (smoothed)
            let gain = self.params.gain.smoothed.next();

            // Apply gain to each channel
            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SimpleAmplifier {
    const CLAP_ID: &'static str = "com.yourname.simple-amplifier";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple amplifier plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for SimpleAmplifier {
    const VST3_CLASS_ID: [u8; 16] = *b"SimpleAmplifier1";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Distortion];
}

// Export the plugin for both CLAP and VST3
nih_export_clap!(SimpleAmplifier);
nih_export_vst3!(SimpleAmplifier);
