pub enum CpuMode {
    Stop,
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
    Stop,
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
