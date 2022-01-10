pub type Frame = [f32; NUM_CHANNELS];

pub const SAMPLE_RATE: usize = 41000;
pub const MAX_DELAY_TIME_SECONDS: usize = 10;
pub const NUM_CHANNELS: usize = 2;
pub const GRAIN_AMPLITUDE: f32 = 0.7;
pub const MAX_GRAINS: usize = 100;
pub const DEFAULT_DELAY_FEEDBACK: f32 = 0.6;
pub const DEFAULT_VOLUME: f32 = 0.5;
// 1 - wet, 0 - dry
pub const DEFAULT_WET_DRY: f32 = 1.0;
pub const SILENT_FRAME: Frame = [0.0, 0.0];
