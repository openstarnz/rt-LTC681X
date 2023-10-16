use crate::pwm::{PwmDutyCycle, PwmRegisters};

#[derive(Default)]
pub struct Pwm {
    pub(crate) register_a: [u8; 6],
}

impl Pwm {
    pub fn set_duty_cycle(&mut self, pwm_duty_cycle: &PwmDutyCycle) {
        let duty_cycle_bits = *pwm_duty_cycle as u8;
        self.register_a[0] = (duty_cycle_bits << 4) | duty_cycle_bits;
        self.register_a[1] = (duty_cycle_bits << 4) | duty_cycle_bits;
        self.register_a[2] = (duty_cycle_bits << 4) | duty_cycle_bits;
    }
}

impl PwmRegisters for Pwm {
    fn register_a(&self) -> [u8; 6] {
        self.register_a
    }
}
