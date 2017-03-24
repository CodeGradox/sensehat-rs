use {SenseHatError, SenseHatResult};
use settings::Settings;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

/// I2C address to the accel and gyro sensor.
const ACCEL_GYRO_ADDR: u8 = 0x6a;
const MAG_ADDR: u8 = 0x00;

pub struct Imu {
    imu_dev: LinuxI2CDevice,
    // Settings file
    settings: Settings,
    /// true if cal mode, so don't use cal data!
    compass_calibration_mode: bool,
    /// true if cal mode, so don't use cal data!
    accel_calibration_mode: bool,
    /// samples per second
    sample_rate: i32,
    /// interval betwwen samples in microseconds
    sample_interval: u64,
    /// gyro bias rapid learning rate
    gyro_learning_alpha: f64,
    /// gyro bias continous (slow) learning rate
    gyro_continious_alpha: f64,
    /// number of gyro samples used
    gyro_sample_count: i32,
    compass_cal_offset: [f64; 3],
    compass_cal_scale: [f64; 3],
    /// array of rotation matrices
    axis_rotation: [[f64; 9]; 24],
    gyro_scale: f64,
    accel_scale: f64,
    compass_scale: f64,
}

impl Imu {
    pub fn new() -> SenseHatResult<Self> {
        let mut imu = Self {
            imu_dev: LinuxI2CDevice::new("/dev/i2c-1", 0x6a)?,
            settings: Settings::default(),
            compass_calibration_mode: false,
            accel_calibration_mode: false,
            sample_rate: 100,
            sample_interval: 0,
            gyro_learning_alpha: 0.0,
            gyro_continious_alpha: 0.0,
            gyro_sample_count: 0,
            compass_cal_offset: [0.0; 3],
            compass_cal_scale: [0.0; 3],
            axis_rotation: [[0.0; 9]; 24],
            gyro_scale: 0.0,
            accel_scale: 0.0,
            compass_scale: 0.0,
        };

        imu.imu_init()?;

        Ok(imu)
    }

    fn imu_init(&mut self) -> SenseHatResult<()> {
        Ok(())
    }

    pub fn imu_read(&mut self) -> bool {
        false
    }
}
