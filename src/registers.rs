pub const MPU60X0_ADDRESS: u8 = 0x68;

// MPU60X0 Registers
pub const WHO_AM_I: u8 = 0x75;
pub const SMPLRT_DIV: u8 = 0x19;
pub const FIFO_EN: u8 = 0x23;
pub const I2C_MST_CTRL: u8 = 0x24;
pub const PWR_MGMT_1: u8 = 0x6b;
pub const FIFO_COUNT_H: u8 = 0x72;
pub const FIFO_COUNT_L: u8 = 0x73;
pub const FIFO_DATA: u8 = 0x74;
pub const USER_CTRL: u8 = 0x6a;
pub const CONFIG: u8 = 0x1a;
pub const GYRO_CONFIG: u8 = 0x1b;
pub const ACCEL_CONFIG: u8 = 0x1c;
