#[derive(Clone, Copy, Debug)]
pub enum CpuMode {
    Nop,
    Normal,
    Debug,
}

#[derive(Clone, Copy, Debug)]
pub enum OpeMode {
    Inst,
    RegisterTransfer,
}

#[derive(Clone, Copy, Debug)]
pub enum TxReg {
    A,
    X,
    Y,
    S,
    P,
    PC,
}

#[derive(Clone, Copy, Debug)]
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
