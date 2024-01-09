#### sequence mode
In sequence mode, one operation can be executed. See 6502 instruction for more information.

#### debug mode

In debug mode, above subcommands prepared.

- 0x00: test addressing mode. Sending specific mode returns referencing address through uart. Above is the addressing mode and the code.
```
0x01: Acc,
0x02: Imm,
0x03: Abs,
0x04: AbsX,
0x05: AbsY,
0x06: Zp,
0x07: ZpX,
0x08: ZpY,
0x09: Impl,
0x10: Rel,
0x11: IndX,
0x12: IndY,
0x13: Ind,
0x14: Nop,
```
