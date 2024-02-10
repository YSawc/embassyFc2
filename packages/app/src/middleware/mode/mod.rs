pub enum CpuMode {
    Nop,
    Normal,
    Debug,
}

pub enum OpeMode {
    Inst,
    RegisterTransfer,
    Reset,
}

pub enum TxReg {
    A,
    X,
    Y,
    S,
    P,
    PC,
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
