/// Precomputed read command for cell voltage register A
pub static CMD_R_CELL_V_REG_A: [u8; 4] = [0x00, 0x04, 0x07, 0xC2];

/// Precomputed read command for cell voltage register B
pub static CMD_R_CELL_V_REG_B: [u8; 4] = [0x00, 0x06, 0x9A, 0x94];

/// Precomputed read command for cell voltage register C
pub static CMD_R_CELL_V_REG_C: [u8; 4] = [0x00, 0x08, 0x5E, 0x52];

/// Precomputed read command for cell voltage register D
pub static CMD_R_CELL_V_REG_D: [u8; 4] = [0x00, 0x0A, 0xC3, 0x04];

/// Precomputed read command for cell voltage register E
pub static CMD_R_CELL_V_REG_E: [u8; 4] = [0x00, 0x09, 0xD5, 0x60];

/// Precomputed read command for cell voltage register F
pub static CMD_R_CELL_V_REG_F: [u8; 4] = [0x00, 0x0B, 0x48, 0x36];

/// Precomputed read command for auxiliary voltage register A
pub static CMD_R_AUX_V_REG_A: [u8; 4] = [0x00, 0xC, 0xEF, 0xCC];

/// Precomputed read command for auxiliary voltage register B
pub static CMD_R_AUX_V_REG_B: [u8; 4] = [0x00, 0xE, 0x72, 0x9A];

/// Precomputed read command for auxiliary voltage register C
pub static CMD_R_AUX_V_REG_C: [u8; 4] = [0x00, 0xD, 0x64, 0xFE];

/// Precomputed read command for auxiliary voltage register D
pub static CMD_R_AUX_V_REG_D: [u8; 4] = [0x00, 0xF, 0xF9, 0xA8];

/// Precomputed read command for status register group A
pub static CMD_R_STATUS_A: [u8; 4] = [0x00, 0x10, 0xED, 0x72];

/// Precomputed read command for status register group B
pub static CMD_R_STATUS_B: [u8; 4] = [0x00, 0x12, 0x70, 0x24];

/// Precomputed read command for configuration register group A
pub static CMD_R_CONF_A: [u8; 4] = [0x00, 0x2, 0x2B, 0xA];

/// Precomputed read command for configuration register group B
pub static CMD_R_CONF_B: [u8; 4] = [0x00, 0x26, 0x2C, 0xC8];

/// Precomputed write command for configuration register group A
pub static CMD_W_CONF_A: [u8; 4] = [0x00, 0x1, 0x3D, 0x6E];

/// Precomputed write command for configuration register group B
pub static CMD_W_CONF_B: [u8; 4] = [0x00, 0x24, 0xB1, 0x9E];

/// Precomputed write command for configuration register group B
pub static CMD_W_PWM: [u8; 4] = [0x00, 0x20, 0x00, 0x00];

/// Precomputed read command for configuration register group B
pub static CMD_R_PWM: [u8; 4] = [0x00, 0x22, 0x9D, 0x56];
