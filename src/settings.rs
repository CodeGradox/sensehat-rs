/// Settings for the LSM9DS1 sensor
#[derive(Debug, Default)]
pub struct Settings {
    gyro_sample_rate: GyroSampleRate,
    gyro_bandwidth: GyroBandwidth,
    gyro_fsr: GyroFullScaleRange,
    gyro_hpf: GyroHighPassFilter,
    accel_sample_rate: AccelSampleRate,
    accel_fsr: AccelFullScaleRange,
    accel_lpf: AccelLowPassFilter,
    compass_sample_rate: CompassSampleRate,
    compass_fsr: CompassFullScaleRange,
}

/// Samplingrate of the gyroscope.
///
/// Represents sample rate in Hz.
#[derive(Debug)]
pub enum GyroSampleRate {
    Hz_14_9,
    Hz_59_5,
    Hz_119,
    Hz_238,
    Hz_476,
    Hz_952,
}

#[derive(Debug)]
/// Gyro bandwidth.
///
/// 0 - 3, see the LSM9DS1 manual for details.
pub enum GyroBandwidth {
    Bw0,
    Bw1,
    Bw2,
    Bw3,
}

/// Gyro full scale range.
///
/// Represents degrees per second.
#[derive(Debug)]
pub enum GyroFullScaleRange {
    Dps250,
    Dps500,
    Dps2000,
}

/// Gyro high pass filter.
///
/// 0 - 9, see the LSM9DS1 manual for details.
#[derive(Debug)]
pub enum GyroHighPassFilter {
    Hpf0,
    Hpf1,
    Hpf2,
    Hpf3,
    Hpf4,
    Hpf5,
    Hpf6,
    Hpf7,
    Hpf8,
    Hpf9,
}

/// Accelerometer sample rate.
///
/// Represents sample rate in Hz.
#[derive(Debug)]
pub enum AccelSampleRate {
    Hz_14_9,
    Hz_59_5,
    Hz_119,
    Hz_238,
    Hz_476,
    Hz_952,
}

/// Accelerometer full scale range.
///
/// ± x gauss, where x is either 4, 8, 12, 16
#[derive(Debug)]
pub enum AccelFullScaleRange {
    G4,
    G8,
    G12,
    G16,
}

/// Accelerometer low pass filter.
#[derive(Debug)]
pub enum AccelLowPassFilter {
    Hz_408,
    Hz_211,
    Hz_105,
    Hz_50,
}

/// Compass sample rate.
///
/// Represents sample rate in Hz.
#[derive(Debug)]
pub enum CompassSampleRate {
    Hz_0_625,
    Hz_1_25,
    Hz_2_5,
    Hz_5,
    Hz_10,
    Hz_20,
    Hz_40,
    Hz_80,
}

/// Compass full scale range.
///
/// ± x uT, where x is either 400, 800, 1200 or 1600
#[derive(Debug)]
pub enum CompassFullScaleRange {
    uT_4,
    uT_8,
    uT_12,
    uT_16,
}

impl Default for GyroSampleRate {
    fn default() -> Self { GyroSampleRate::Hz_119 }
}

impl Default for GyroBandwidth {
    fn default() -> Self { GyroBandwidth::Bw1 }
}

impl Default for GyroFullScaleRange {
    fn default() -> Self { GyroFullScaleRange::Dps500 }
}

impl Default for GyroHighPassFilter{
    fn default() -> Self { GyroHighPassFilter::Hpf4 }
}

impl Default for AccelSampleRate {
    fn default() -> Self { AccelSampleRate::Hz_119 }
}

impl Default for AccelFullScaleRange {
    fn default() -> Self { AccelFullScaleRange::G8 }
}

impl Default for AccelLowPassFilter {
    fn default() -> Self { AccelLowPassFilter::Hz_50 }
}

impl Default for CompassSampleRate {
    fn default() -> Self { CompassSampleRate::Hz_20 }
}

impl Default for CompassFullScaleRange {
    fn default() -> Self { CompassFullScaleRange::uT_4 }
}