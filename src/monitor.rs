use crate::monitor::Error::TransferError;
use crate::pec15::PEC15;
use core::fmt::{Debug, Formatter};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Poll Strategy
pub trait PollMethod<CS: OutputPin> {
    /// Handles the CS pin state after command has been sent
    fn end_command(&self, cs: &mut CS) -> Result<(), CS::Error>;
}

/// Leaves CS Low and waits until SDO goes high
pub struct SDOLinePolling {}

impl<CS: OutputPin> PollMethod<CS> for SDOLinePolling {
    fn end_command(&self, _cs: &mut CS) -> Result<(), CS::Error> {
        Ok(())
    }
}

/// No ADC polling is used
pub struct NoPolling {}

impl<CS: OutputPin> PollMethod<CS> for NoPolling {
    fn end_command(&self, cs: &mut CS) -> Result<(), CS::Error> {
        cs.set_high()
    }
}

/// ADC frequency and filtering settings
pub enum ADCMode {
    /// 27kHz or 14kHz in case of CFGAR0=1 configuration
    Fast = 0x1,
    /// 7kHz or 3kHz in case of CFGAR0=1 configuration
    Normal = 0x2,
    /// 26Hz or 2kHz in case of CFGAR0=1 configuration
    Filtered = 0x3,
    /// 422Hz or 1kHz in case of CFGAR0=1 configuration
    Other = 0x0,
}

/// Cell selection for ADC conversion, s. page 62 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6813-1.pdf)
/// for conversion times
pub enum CellSelection {
    /// All cells
    All = 0x0,
    /// Cells 1, 7, 13
    Group1 = 0x1,
    /// Cells 2, 8, 14
    Group2 = 0x2,
    /// Cells 3, 9, 15
    Group3 = 0x3,
    /// Cells 4, 10, 16
    Group4 = 0x4,
    /// Cells 5, 11, 17
    Group5 = 0x5,
    /// cells 6, 12, 18
    Group6 = 0x6,
}

/// Cell voltage registers
pub enum CellVoltageRegister {
    RegisterA = 0x4,
    RegisterB = 0x6,
    RegisterC = 0x8,
    RegisterD = 0xA,
    RegisterE = 0x9,
    RegisterF = 0xB,
}

/// Error enum of LTC681X
#[derive(PartialEq)]
pub enum Error<B: Transfer<u8>, CS: OutputPin> {
    /// SPI transfer error
    TransferError(B::Error),

    /// Error while changing state of CS pin
    CSPinError(CS::Error),

    /// PEC checksum of returned data was invalid
    ChecksumMismatch,
}

/// Client for LTC681X IC
pub struct LTC681X<B: Transfer<u8>, CS: OutputPin, P: PollMethod<CS>, const L: usize> {
    /// SPI bus
    bus: B,

    /// SPI CS pin
    cs: CS,

    /// Poll method used for type state
    poll_method: P,
}

impl<B: Transfer<u8>, CS: OutputPin, const L: usize> LTC681X<B, CS, NoPolling, L> {
    pub fn new(bus: B, cs: CS) -> Self {
        LTC681X {
            bus,
            cs,
            poll_method: NoPolling {},
        }
    }
}

impl<B: Transfer<u8>, CS: OutputPin, P: PollMethod<CS>, const L: usize> LTC681X<B, CS, P, L> {
    /// Starts ADC conversion of cell voltages
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `cells`: Measures the given cell gorup
    /// * `dcp`: True if discharge is permitted during conversion
    pub fn start_conv_cells(&mut self, mode: ADCMode, cells: CellSelection, dcp: bool) -> Result<(), Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0010_0110_0000;

        command |= (mode as u16) << 7;
        command |= cells as u16;

        if dcp {
            command |= 0b0001_0000;
        }

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)
    }

    /// Reads and returns the cell voltages of the given register
    /// Returns one array for each device in daisy chain
    pub fn read_cell_voltages(&mut self, register: CellVoltageRegister) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        self.send_command(register as u16).map_err(Error::TransferError)?;

        let mut result = [[0, 0, 0]; L];
        for i in 0..L {
            result[i] = self.read()?;
        }

        self.cs.set_high().map_err(Error::CSPinError)?;
        Ok(result)
    }

    /// Sends the given command. Calculates and attaches the PEC checksum
    fn send_command(&mut self, command: u16) -> Result<(), B::Error> {
        let mut data = [(command >> 8) as u8, command as u8, 0x0, 0x0];
        let pec = PEC15::calc(&data[0..2]);

        data[2] = pec[0];
        data[3] = pec[1];

        self.bus.transfer(&mut data)?;
        Ok(())
    }

    /// Reads a register
    fn read(&mut self) -> Result<[u16; 3], Error<B, CS>> {
        let mut command = [0xff as u8; 8];
        let result = self.bus.transfer(&mut command).map_err(TransferError)?;

        let pec = PEC15::calc(&result[0..6]);
        if pec[0] != result[6] || pec[1] != result[7] {
            return Err(Error::ChecksumMismatch);
        }

        let mut registers = [result[0] as u16, result[2] as u16, result[4] as u16];
        registers[0] |= (result[1] as u16) << 8;
        registers[1] |= (result[3] as u16) << 8;
        registers[2] |= (result[5] as u16) << 8;

        Ok(registers)
    }

    /// Enables SDO ADC polling
    ///
    /// After entering a conversion command, the SDO line is driven low when the device is busy
    /// performing conversions. SDO is pulled high when the device completes conversions.
    pub fn enable_sdo_polling(self) -> LTC681X<B, CS, SDOLinePolling, L> {
        LTC681X {
            bus: self.bus,
            cs: self.cs,
            poll_method: SDOLinePolling {},
        }
    }
}

impl<B: Transfer<u8>, CS: OutputPin, const L: usize> LTC681X<B, CS, SDOLinePolling, L> {
    /// Returns false if the ADC is busy
    /// If ADC is ready, CS line is pulled high
    pub fn adc_ready(&mut self) -> Result<bool, Error<B, CS>> {
        let mut command = [0xff];
        let result = self.bus.transfer(&mut command).map_err(Error::TransferError)?;

        if result[0] == 0xff {
            self.cs.set_high().map_err(Error::CSPinError)?;
            return Ok(true);
        }

        Ok(false)
    }
}

impl<B: Transfer<u8>, CS: OutputPin> Debug for Error<B, CS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TransferError(_) => f.debug_struct("TransferError").finish(),
            Error::CSPinError(_) => f.debug_struct("CSPinError").finish(),
            Error::ChecksumMismatch => f.debug_struct("ChecksumMismatch").finish(),
        }
    }
}