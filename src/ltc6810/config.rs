use crate::config::{Cell, DischargeTimeout, VoltageOutOfRangeError, GPIO};

/// Abstracted configuration of configuration register(s)
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Computed value of register A
    pub(crate) register_a: [u8; 6],
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            register_a: [
                0b1111_1000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ],
        }
    }
}

impl Configuration {
    /// Enables pull-down of the given GPIO pin
    pub fn enable_gpio_pull_down(&mut self, pin: GPIO) {
        match pin {
            GPIO::GPIO1 => self.register_a[0] &= 0b1111_0111,
            GPIO::GPIO2 => self.register_a[0] &= 0b1110_1111,
            GPIO::GPIO3 => self.register_a[0] &= 0b1101_1111,
						_ => unimplemented!("unsupported GPIO")
        }
    }

    /// Enables pull-down of the given GPIO pin
    pub fn disable_gpio_pull_down(&mut self, pin: GPIO) {
        match pin {
            GPIO::GPIO1 => self.register_a[0] |= 0b0000_1000,
            GPIO::GPIO2 => self.register_a[0] |= 0b0001_0000,
            GPIO::GPIO3 => self.register_a[0] |= 0b0010_0000,
						_ => unimplemented!("unsupported GPIO")
        }
    }

    /// References remain powered up until watchdog timeout
    pub fn enable_reference_power(&mut self) {
        self.register_a[0] |= 0b0000_0100
    }

    /// References shut down after conversions (Default)
    pub fn disable_reference_power(&mut self) {
        self.register_a[0] &= 0b1111_1011
    }

    /// Enables the discharge timer for discharge switches
    pub fn enable_discharge_timer(&mut self) {
        self.register_a[0] |= 0b0000_0010
    }

    /// Disables the discharge timer
    pub fn disable_discharge_timer(&mut self) {
        self.register_a[0] &= 0b1111_1101
    }

    /// Sets the under-voltage comparison voltage in uV
    pub fn set_uv_comp_voltage(&mut self, voltage: u32) -> Result<(), VoltageOutOfRangeError> {
        if voltage == 0 {
            self.register_a[1] = 0x0;
            self.register_a[2] &= 0b1111_0000;
            return Ok(());
        }

        if !(3200..=6553600).contains(&voltage) {
            return Err(VoltageOutOfRangeError {});
        }

        let value = ((voltage / 1600) - 1) as u16;

        self.register_a[1] = value as u8;
        self.register_a[2] &= 0b1111_0000;
        self.register_a[2] |= (value >> 8) as u8;

        Ok(())
    }

    /// Sets the over-voltage comparison voltage in uV
    pub fn set_ov_comp_voltage(&mut self, voltage: u32) -> Result<(), VoltageOutOfRangeError> {
        if voltage == 0 {
            self.register_a[2] &= 0b0000_1111;
            self.register_a[3] = 0x0;
            return Ok(());
        }

        if !(1600..=6552000).contains(&voltage) {
            return Err(VoltageOutOfRangeError {});
        }

        let value = (voltage / 1600) as u16;

        self.register_a[3] = (value >> 4) as u8;
        self.register_a[2] &= 0b0000_1111;
        self.register_a[2] |= (value << 4) as u8;

        Ok(())
    }

    /// Turn ON Shorting Switch for Cell x
    pub fn discharge_cell(&mut self, cell: Cell) {
        match cell {
            Cell::Cell1 => self.register_a[4] |= 0b0000_0001,
            Cell::Cell2 => self.register_a[4] |= 0b0000_0010,
            Cell::Cell3 => self.register_a[4] |= 0b0000_0100,
            Cell::Cell4 => self.register_a[4] |= 0b0000_1000,
            Cell::Cell5 => self.register_a[4] |= 0b0001_0000,
            Cell::Cell6 => self.register_a[4] |= 0b0010_0000,
						_ => unimplemented!("Unsupported cell")

        }
    }

    /// Sets the discharge timeout
    pub fn set_discharge_timeout(&mut self, timeout: DischargeTimeout) {
        self.register_a[5] &= 0b0000_1111;
        self.register_a[5] |= (timeout as u8) << 4;
    }

    /// Alternative ADC modes 14kHz, 3kHz, 1kHz or 2kHz
    pub fn set_alternative_adc_modes(&mut self) {
        self.register_a[0] |= 0b0000_0001
    }

    /// Default ADC modes 27kHz, 7kHz, 422Hz or 26Hz
    pub fn set_default_adc_modes(&mut self) {
        self.register_a[0] &= 0b1111_1110
    }

    /// Forces the digital redundancy comparison for ADC Conversions to fail
    pub fn force_digital_redundancy_fail(&mut self) {
        self.register_a[5] |= 0b0000_0100;
    }

    /// Enables the discharge timer monitor function if the DTEN Pin is Asserted
    /// Otherwise (default) the discharge dimer monitor function is disabled. The normal discharge
    /// timer function will be enabled if the DTEN pin is asserted
    pub fn enable_discharge_monitor(&mut self) {
        self.register_a[5] |= 0b0000_0001;
    }
}

impl PartialEq<Self> for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.register_a == other.register_a
    }
}

impl Eq for Configuration {}
