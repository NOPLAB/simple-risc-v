# simple-riscv

## What is this

RISC-V(rv32i)をRust言語を用いてエミュレーターとして実装しました。

`simple-riscv` というリポジトリ名にある通り、シンプルで読みやすく理解しやすいように実装しました。

プログラムは別の方が書かれたChisel言語のエミュレーターを参考に作成しているので、ハードウェア寄りの実装になっています。

まだ未完成のため、一部命令は使用できません。`src/processor/decode.rs` に実装されている命令一覧があります。

## Run

```shell
cargo run filepath
```

## Example

```shell
cargo run test/rv32ui-p-add
```

- [ ] Error test/rv32ui-p-lb
- [ ] Error test/rv32ui-p-lbu
- [ ] Error test/rv32ui-p-lh
- [ ] Error test/rv32ui-p-lhu
- [ ] Error test/rv32ui-p-sb
- [ ] Error test/rv32ui-p-sh
