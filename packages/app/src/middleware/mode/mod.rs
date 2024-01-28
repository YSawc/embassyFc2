pub enum CpuMode {
    Nop,
    Normal,
    Debug,
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
