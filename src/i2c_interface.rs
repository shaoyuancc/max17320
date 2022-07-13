use super::*;
use crate::register::Register;

impl<I2C, E> MAX17320<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
{
    pub(crate) fn read_named_register(&mut self, reg: Register) -> Result<u16, E> {
        self.read_register(reg as u8, self.address)
    }

    pub(crate) fn read_named_register_nvm(&mut self, reg: RegisterNvm) -> Result<u16, E> {
        self.read_register(reg as u8, self.address_nvm)
    }

    fn read_register(&mut self, reg: u8, address: u8) -> Result<u16, E> {
        let mut data: [u8; 2] = [0, 0];
        self.com.write_read(address, &[reg], &mut data)?;
        Ok(u16::from_le_bytes(data))
    }

    pub(super) fn write_named_register(&mut self, reg: Register, code: u16) -> Result<(), E> {
        self.write_register(reg as u8, self.address, code)
    }

    pub(super) fn write_named_register_nvm(
        &mut self,
        reg: RegisterNvm,
        code: u16,
    ) -> Result<(), E> {
        self.write_register(reg as u8, self.address_nvm, code)
    }

    fn write_register(&mut self, reg: u8, address: u8, code: u16) -> Result<(), E> {
        let mut buffer = [0];
        let code = code.to_be_bytes();
        let bytes: [u8; 3] = [reg, code[0], code[1]];
        self.com.write_read(address, &bytes, &mut buffer)
    }
}
