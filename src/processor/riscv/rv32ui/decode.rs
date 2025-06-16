use std::fmt::Display;

use bitmatch::bitmatch;
use bitvec::{bitvec, field::BitField, prelude::Lsb0, view::BitView};

use crate::processor::ProcessorError;
use crate::processor::ProcessorErrorTrait;

use super::x_register::XRegisters;

pub enum DecodeErrorType {
    NotMatchOpcode,
}

pub struct DecodeError {
    error_type: DecodeErrorType,
}

impl DecodeError {
    fn new(error_type: DecodeErrorType) -> Box<Self> {
        Box::new(DecodeError { error_type })
    }
}

impl ProcessorErrorTrait for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            DecodeErrorType::NotMatchOpcode => write!(f, "Not match opcode."),
        }
    }
}

#[deny(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    LB,  // todo
    LH,  // todo
    LW,  // Ok
    LBU, // todo
    LHU, // todo

    SB, // todo
    SH, // todo
    SW, // Ok

    ADD,  // Ok
    ADDI, // Ok

    SUB, // Ok

    AND,  // Ok
    OR,   // Ok
    XOR,  // Ok
    ANDI, // Ok
    ORI,  // Ok
    XORI, // Ok

    SLL,  // Ok
    SRL,  // Ok
    SRA,  // Ok
    SLLI, // Ok
    SRLI, // Ok
    SRAI, // Ok

    SLT,   // Ok
    SLTU,  // Ok
    SLTI,  // Ok
    SLTIU, // Ok

    BEQ,  // Ok
    BNE,  // Ok
    BLT,  // Ok
    BGE,  // Ok
    BLTU, // Ok
    BGEU, // Ok

    JAL,  // Ok
    JALR, // Ok

    LUI,   // Ok
    AUIPC, // Ok

    CSRRW,
    CSRRWI,
    CSRRS,
    CSRRSI,
    CSRRC,
    CSRRCI,

    URET, // todo
    SRET, // todo
    MRET, // todo

    WFI, // todo

    ECALL,
    EBREAK, // todo

    FENCE,     // todo
    FENCEI,    // todo
    SFENCEVMA, //todo
}

#[derive(Debug, Clone, Copy)]
pub struct DecodeResult {
    pub opcode: Opcode,

    pub rs1_data: u32,
    pub rs2_data: u32,
    pub rd: u32,

    pub imm_i: u32,
    pub imm_i_sext: i32,

    pub imm_s: u32,
    pub imm_s_sext: i32,

    pub imm_b: u32,
    pub imm_b_sext: i32,

    pub imm_j: u32,
    pub imm_j_sext: i32,

    pub imm_u: u32,
    pub imm_u_sext_shifted: i32,

    pub imm_z: u32,

    pub csr: u32,
}

pub struct Decode();

impl Decode {
    pub fn decode(&self, inst: u32, xregs: &XRegisters) -> Result<DecodeResult, ProcessorError> {
        let inst_slice = inst.view_bits::<Lsb0>();

        let rs1_addr = inst_slice[15..=19].load::<u32>(); // R, I, S, B type
        let rs2_addr = inst_slice[20..=24].load::<u32>(); // R, S, B type
        let rd = inst_slice[7..=11].load::<u32>(); // rd

        let rs1_data = xregs.read(rs1_addr);
        let rs2_data = xregs.read(rs2_addr);

        let imm_i = inst_slice[20..=31].load::<u32>();
        let imm_i_sext = inst_slice[20..=31].load::<i32>();

        let mut imm_s_vec = bitvec![u32, Lsb0;];
        imm_s_vec.append(&mut inst_slice[7..=11].to_bitvec());
        imm_s_vec.append(&mut inst_slice[25..=31].to_bitvec());
        let imm_s = imm_s_vec.load::<u32>();
        let imm_s_sext = imm_s_vec.load::<i32>();

        let mut imm_b_vec = bitvec![u32, Lsb0;];
        imm_b_vec.push(false);
        imm_b_vec.append(&mut inst_slice[8..=11].to_bitvec());
        imm_b_vec.append(&mut inst_slice[25..=30].to_bitvec());
        imm_b_vec.push(inst_slice[7]);
        imm_b_vec.push(inst_slice[31]);
        let imm_b = imm_b_vec.load::<u32>();
        let imm_b_sext = imm_b_vec.load::<i32>();

        let mut imm_j_vec = bitvec![u32, Lsb0;];
        imm_j_vec.push(false);
        imm_j_vec.append(&mut inst_slice[21..=30].to_bitvec());
        imm_j_vec.push(inst_slice[20]);
        imm_j_vec.append(&mut inst_slice[12..=19].to_bitvec());
        imm_j_vec.push(inst_slice[31]);
        let imm_j = imm_j_vec.load::<u32>();

        let imm_j_sext = imm_j_vec.load::<i32>();

        let imm_u = inst_slice[12..=31].load::<u32>();
        let imm_u_sext_shifted = (imm_u << 12) as i32;

        let imm_z = inst_slice[15..=19].load::<u32>();

        let csr = inst_slice[20..=31].load::<u32>();

        if let Some(opcode) = self.match_opcode(inst) {
            println!("Decode: opcode \x1b[38;5;2m{:?}\x1b[m", opcode);
            println!(
                "        rs1_addr: 0b{:0>5b}({}),    rs2_addr: 0b{:0>5b}({}), rd(wb_addr): 0b{:0>5b}({})",
                rs1_addr, rs1_addr, rs2_addr, rs2_addr, rd, rd
            );

            println!(
                "        rs1_data: 0x{:0>8x}({}), rs2_data: 0x{:0>8x}({})",
                rs1_data, rs1_data, rs2_data, rs2_data
            );

            println!(
                "        imm_i: 0x{:0>8x}({}), imm_s: 0x{:0>8x}({}), imm_b: 0x{:0>8x}({}), imm_j: 0x{:0>8x}({}), imm_u: 0x{:0>8x}({}), imm_z: 0x{:0>8x}({}),",
                imm_i, imm_i, imm_s, imm_s, imm_b, imm_b, imm_j, imm_j, imm_u, imm_u, imm_z, imm_z,
            );

            Ok(DecodeResult {
                opcode,
                rs1_data,
                rs2_data,
                rd,
                imm_i,
                imm_i_sext,
                imm_s,
                imm_s_sext,
                imm_b,
                imm_b_sext,
                imm_j,
                imm_j_sext,
                imm_u,
                imm_u_sext_shifted,
                imm_z,
                csr,
            })
        } else {
            Err(DecodeError::new(DecodeErrorType::NotMatchOpcode))
        }
    }

    // 0b1010100100010010010

    #[bitmatch]
    fn match_opcode(&self, inst: u32) -> Option<Opcode> {
        #[bitmatch]
        match inst {
            "?????????????????000?????0000011" => Some(Opcode::LB),
            "?????????????????001?????0000011" => Some(Opcode::LH),
            "?????????????????010?????0000011" => Some(Opcode::LW),
            "?????????????????100?????0000011" => Some(Opcode::LBU),
            "?????????????????101?????0000011" => Some(Opcode::LHU),

            "?????????????????000?????0100011" => Some(Opcode::SB),
            "?????????????????001?????0100011" => Some(Opcode::SH),
            "?????????????????010?????0100011" => Some(Opcode::SW),

            "0000000??????????000?????0110011" => Some(Opcode::ADD),
            "?????????????????000?????0010011" => Some(Opcode::ADDI),

            "0100000??????????000?????0110011" => Some(Opcode::SUB),

            "0000000??????????111?????0110011" => Some(Opcode::AND),
            "0000000??????????110?????0110011" => Some(Opcode::OR),
            "0000000??????????100?????0110011" => Some(Opcode::XOR),
            "?????????????????111?????0010011" => Some(Opcode::ANDI),
            "?????????????????110?????0010011" => Some(Opcode::ORI),
            "?????????????????100?????0010011" => Some(Opcode::XORI),

            "0000000??????????001?????0110011" => Some(Opcode::SLL),
            "0000000??????????101?????0110011" => Some(Opcode::SRL),
            "0100000??????????101?????0110011" => Some(Opcode::SRA),
            "0000000??????????001?????0010011" => Some(Opcode::SLLI),
            "0000000??????????101?????0010011" => Some(Opcode::SRLI),
            "0100000??????????101?????0010011" => Some(Opcode::SRAI),

            "0000000??????????010?????0110011" => Some(Opcode::SLT),
            "0000000??????????011?????0110011" => Some(Opcode::SLTU),
            "?????????????????010?????0010011" => Some(Opcode::SLTI),
            "?????????????????011?????0010011" => Some(Opcode::SLTIU),

            "?????????????????000?????1100011" => Some(Opcode::BEQ),
            "?????????????????001?????1100011" => Some(Opcode::BNE),
            "?????????????????100?????1100011" => Some(Opcode::BLT),
            "?????????????????101?????1100011" => Some(Opcode::BGE),
            "?????????????????110?????1100011" => Some(Opcode::BLTU),
            "?????????????????111?????1100011" => Some(Opcode::BGEU),

            "?????????????????????????1101111" => Some(Opcode::JAL),
            "?????????????????000?????1100111" => Some(Opcode::JALR),

            "?????????????????????????0110111" => Some(Opcode::LUI),
            "?????????????????????????0010111" => Some(Opcode::AUIPC),

            "?????????????????001?????1110011" => Some(Opcode::CSRRW),
            "?????????????????101?????1110011" => Some(Opcode::CSRRWI),
            "?????????????????010?????1110011" => Some(Opcode::CSRRS),
            "?????????????????110?????1110011" => Some(Opcode::CSRRSI),
            "?????????????????011?????1110011" => Some(Opcode::CSRRC),
            "?????????????????111?????1110011" => Some(Opcode::CSRRCI),

            "00000000000000000000000001110011" => Some(Opcode::ECALL),

            "00110000001000000000000001110011" => Some(Opcode::MRET),

            "0000????????00000000000000001111" => Some(Opcode::FENCE),

            _ => None,
        }
    }
}

/*
pub fn sign_extension(value: u32, n_top: u32) -> u32 {
    // 符号を取得
    let sign = (value >> n_top) % 2;

    // 符号ビットを除外する
    let mut value = value % 2u32.pow(n_top);

    // 符号がマイナス
    if sign == 1 {
        // 符号拡張
        value = u32::MAX - 2u32.pow(n_top) + value + 1
    }

    value
}
*/
