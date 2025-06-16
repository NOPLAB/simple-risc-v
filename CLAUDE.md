# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

Build the project:
```
cargo build
```

Run the emulator with a test file:
```
cargo run <test_file_path>
```

Example test execution:
```
cargo run test/rv32ui-p-add
```

## Code Architecture

This is a RISC-V (RV32I) processor emulator written in Rust with a hardware-oriented design inspired by Chisel implementations.

### Core Components

- **Computer** (`src/computer.rs`): Top-level emulator that orchestrates the processor and bus, handles program loading from files, and runs the main execution loop
- **Processor** (`src/processor.rs`): Abstract processor trait with the main `increment()` method that executes one instruction cycle
- **Bus** (`src/bus.rs`): Memory bus that routes memory operations to DRAM, handles address translation (DRAM base at 0x80000000)
- **DRAM** (`src/dram.rs`): Main memory implementation

### RISC-V Implementation

The RISC-V processor follows a classic 5-stage pipeline structure:

- **Fetch** (`src/processor/riscv/rv32ui/fetch.rs`): Instruction fetching from memory
- **Decode** (`src/processor/riscv/rv32ui/decode.rs`): Instruction decoding with bit pattern matching using the `bitmatch` crate
- **Execute** (`src/processor/riscv/rv32ui/execute.rs`): Arithmetic and logic operations
- **Memory** (integrated into execute stage): Load/store operations via the bus
- **Writeback** (`src/processor/riscv/rv32ui/writeback.rs`): Register file updates

### Register Management

- **X Registers** (`src/processor/riscv/rv32ui/x_register.rs`): 32 general-purpose registers (x0-x31)
- **CS Registers** (`src/processor/riscv/rv32ui/cs_register.rs`): Control and status registers

### Instruction Support

See `src/processor/riscv/rv32ui/decode.rs` for the complete list of implemented instructions. Key instruction types:
- Arithmetic: ADD, ADDI, SUB
- Logic: AND, OR, XOR and immediate variants
- Shifts: SLL, SRL, SRA and immediate variants
- Comparisons: SLT, SLTU and immediate variants
- Branches: BEQ, BNE, BLT, BGE, BLTU, BGEU
- Jumps: JAL, JALR
- Memory: LW, SW (LB, LH, SB, SH are marked as TODO)
- Upper immediates: LUI, AUIPC

### Test Files

The `test/` directory contains RISC-V test binaries:
- `rv32ui-p-*`: Physical memory tests
- `rv32ui-v-*`: Virtual memory tests
- Custom test cases in `testcase*` files

### Known Issues

Several memory instructions are not yet implemented (marked as TODO in decode.rs):
- LB, LH, LBU, LHU (load byte/halfword variants)
- SB, SH (store byte/halfword)

Error handling uses custom error types that implement the `ProcessorErrorTrait`.