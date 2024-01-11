pub enum CpuMode {
    Normal = 2,
    Callback = 3,
    Sequence = 4,
    Debug = 5,
}

pub enum OpeMode {
    Inst,
    Addr,
}

pub enum AddrMode {
    Acc,
    Imm,
    Abs,
    AbsX,
    AbsY,
    Zp,
    ZpX,
    ZpY,
    Impl,
    Rel,
    IndX,
    IndY,
    Ind,
    Nop,
}
