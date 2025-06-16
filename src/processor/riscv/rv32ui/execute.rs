use std::fmt::Display;

use super::decode::Opcode;
use super::decode::DecodeResult;
use crate::processor::ProcessorError;
use crate::processor::ProcessorErrorTrait;

pub enum ExecuteErrorType {}

pub struct ExecuteError {
    error_type: ExecuteErrorType,
}

impl ExecuteError {
    fn new(error_type: ExecuteErrorType) -> Box<Self> {
        Box::new(ExecuteError { error_type })
    }
}

impl ProcessorErrorTrait for ExecuteError {}

impl Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExecuteResult {
    pub alu_out: u32,
    pub br_target: Option<u32>,
    pub jmp_target: Option<u32>,
}

pub struct Execute();

impl Execute {
    pub fn execute(&self, decode: DecodeResult, pc: u32) -> Result<ExecuteResult, ProcessorError> {
        let alu_out: u32 = match decode.opcode {
            Opcode::LB => (decode.rs1_data as i32 + decode.imm_i_sext) as u32,
            Opcode::LBU => (decode.rs1_data as i32 + decode.imm_i_sext) as u32,
            Opcode::LH => (decode.rs1_data as i32 + decode.imm_i_sext) as u32,
            Opcode::LHU => (decode.rs1_data as i32 + decode.imm_i_sext) as u32,
            Opcode::LW => (decode.rs1_data as i32 + decode.imm_i_sext) as u32,

            Opcode::SB => (decode.rs1_data as i32 + decode.imm_s_sext) as u32,
            Opcode::SH => (decode.rs1_data as i32 + decode.imm_s_sext) as u32,
            Opcode::SW => (decode.rs1_data as i32 + decode.imm_s_sext) as u32,

            Opcode::ADD => decode.rs1_data.wrapping_add(decode.rs2_data),
            Opcode::ADDI => (decode.rs1_data as i32).wrapping_add(decode.imm_i_sext) as u32,

            Opcode::SUB => decode.rs1_data.wrapping_sub(decode.rs2_data),

            Opcode::AND => decode.rs1_data & decode.rs2_data,
            Opcode::OR => decode.rs1_data | decode.rs2_data,
            Opcode::XOR => decode.rs1_data ^ decode.rs2_data,
            Opcode::ANDI => (decode.rs1_data as i32 & decode.imm_i_sext) as u32,
            Opcode::ORI => (decode.rs1_data as i32 | decode.imm_i_sext) as u32,
            Opcode::XORI => (decode.rs1_data as i32 ^ decode.imm_i_sext) as u32,

            Opcode::SLL => decode.rs1_data.wrapping_shl(decode.rs2_data),
            Opcode::SRL => decode.rs1_data.wrapping_shr(decode.rs2_data),
            Opcode::SRA => (decode.rs1_data as i32).wrapping_shr(decode.rs2_data) as u32,
            Opcode::SLLI => decode.rs1_data << decode.imm_i_sext,
            Opcode::SRLI => decode.rs1_data >> decode.imm_i_sext,
            Opcode::SRAI => (decode.rs1_data as i32).wrapping_shr(decode.imm_i_sext as u32) as u32,

            Opcode::SLT => ((decode.rs1_data as i32) < (decode.rs2_data as i32)) as u32,
            Opcode::SLTU => (decode.rs1_data < decode.rs2_data) as u32,
            Opcode::SLTI => ((decode.rs1_data as i32) < decode.imm_i_sext) as u32,
            Opcode::SLTIU => (decode.rs1_data < (decode.imm_i_sext as u32)) as u32,
            // Opcode::BEQ => 0,
            // Opcode::BNE => 0,
            // Opcode::BLT => 0,
            // Opcode::BGE => 0,
            // Opcode::BLTU => 0,
            // Opcode::BGEU => 0,
            Opcode::JAL => pc + 4,
            Opcode::JALR => pc + 4,

            Opcode::LUI => decode.imm_u_sext_shifted as u32,
            Opcode::AUIPC => ((pc as i32).wrapping_add(decode.imm_u_sext_shifted)) as u32,
            // Opcode::CSRRW => 0,
            // Opcode::CSRRWI => 0,
            // Opcode::CSRRS => 0,
            // Opcode::CSRRSI => 0,
            // Opcode::CSRRC => 0,
            // Opcode::CSRRCI => 0,
            // Opcode::ECALL => 0,
            _ => 0,
        };

        let br_flg: bool = match decode.opcode {
            Opcode::BEQ => decode.rs1_data == decode.rs2_data,
            Opcode::BNE => decode.rs1_data != decode.rs2_data,
            Opcode::BLT => (decode.rs1_data as i32) < (decode.rs2_data as i32),
            Opcode::BGE => (decode.rs1_data as i32) >= (decode.rs2_data as i32),
            Opcode::BLTU => decode.rs1_data < decode.rs2_data,
            Opcode::BGEU => decode.rs1_data >= decode.rs2_data,
            _ => false,
        };

        let br_target: Option<u32> = if br_flg {
            Some(((pc as i32) + decode.imm_b_sext) as u32)
        } else {
            None
        };

        let jmp_target = match decode.opcode {
            Opcode::JAL => Some(((pc as i32) + decode.imm_j_sext) as u32),
            Opcode::JALR => Some((((decode.rs1_data as i32) + decode.imm_i_sext) & (!1)) as u32),
            _ => None,
        };

        print!("Execute: alu_out: {}", alu_out);
        if let Some(br) = br_target {
            print!(", br_target: 0x{:x}", br)
        }
        if let Some(jmp) = jmp_target {
            print!(", jmp_target: 0x{:x}", jmp)
        }
        println!();

        Ok(ExecuteResult {
            alu_out,
            br_target,
            jmp_target,
        })
    }
}
