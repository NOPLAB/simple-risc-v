use std::fmt::Display;

use super::cs_register::ControlAndStatusRegister;
use super::decode::DecodeResult;
use super::decode::Opcode;
use super::execute::ExecuteResult;
use super::x_register::XRegisters;

use crate::bus::Bus;
use crate::processor::ProcessorError;
use crate::processor::ProcessorErrorTrait;

pub enum WritebackErrorType {}

pub struct WritebackError {
    error_type: WritebackErrorType,
}

impl WritebackError {
    fn new(error_type: WritebackErrorType) -> Box<Self> {
        Box::new(WritebackError { error_type })
    }
}

impl ProcessorErrorTrait for WritebackError {}

impl Display for WritebackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Writeback();

impl Writeback {
    pub fn writeback(
        &self,
        decode: DecodeResult,
        execute: ExecuteResult,
        xregs: &mut XRegisters,
        csr: &mut ControlAndStatusRegister,
        bus: &mut Bus,
    ) -> Result<(), ProcessorError> {
        if let Ok(rd_data) = bus.read32(execute.alu_out) {
            println!("Writeback: rd(wb_data) 0x{:0>8x}({})", rd_data, rd_data);
        };
        let crs_data = csr.read(decode.csr);

        match decode.opcode {
            Opcode::CSRRW => csr.write(decode.csr, decode.rs1_data),
            Opcode::CSRRWI => csr.write(decode.csr, decode.imm_z),
            Opcode::CSRRS => csr.write(decode.csr, csr.read(decode.csr) | decode.rs1_data),
            Opcode::CSRRSI => csr.write(decode.csr, csr.read(decode.csr) | decode.imm_z),
            Opcode::CSRRC => csr.write(decode.csr, csr.read(decode.csr) & (!decode.rs1_data)),
            Opcode::CSRRCI => csr.write(decode.csr, csr.read(decode.csr) & (!decode.imm_z)),
            _ => (),
        }

        match decode.opcode {
            Opcode::LB => {
                let byte_val = bus.read8(execute.alu_out)? as i8 as i32 as u32;
                xregs.write(decode.rd, byte_val);
            },
            Opcode::LBU => {
                let byte_val = bus.read8(execute.alu_out)? as u32;
                xregs.write(decode.rd, byte_val);
            },
            Opcode::LH => {
                let half_val = bus.read16(execute.alu_out)? as i16 as i32 as u32;
                xregs.write(decode.rd, half_val);
            },
            Opcode::LHU => {
                let half_val = bus.read16(execute.alu_out)? as u32;
                xregs.write(decode.rd, half_val);
            },
            Opcode::LW => xregs.write(decode.rd, bus.read32(execute.alu_out)?),

            Opcode::SB => bus.write8(execute.alu_out, decode.rs2_data as u8)?,
            Opcode::SH => bus.write16(execute.alu_out, decode.rs2_data as u16)?,
            Opcode::SW => bus.write32(execute.alu_out, decode.rs2_data)?,

            Opcode::BEQ => (),
            Opcode::BNE => (),
            Opcode::BLT => (),
            Opcode::BGE => (),
            Opcode::BLTU => (),
            Opcode::BGEU => (),

            Opcode::CSRRW => xregs.write(decode.rd, crs_data),
            Opcode::CSRRWI => xregs.write(decode.rd, crs_data),
            Opcode::CSRRS => xregs.write(decode.rd, crs_data),
            Opcode::CSRRSI => xregs.write(decode.rd, crs_data),
            Opcode::CSRRC => xregs.write(decode.rd, crs_data),
            Opcode::CSRRCI => xregs.write(decode.rd, crs_data),

            Opcode::ECALL => csr.write(0x342, 11),

            Opcode::MRET => (), // todo

            Opcode::FENCE => (), // todo

            _ => xregs.write(decode.rd, execute.alu_out),
        }

        Ok(())
    }
}
