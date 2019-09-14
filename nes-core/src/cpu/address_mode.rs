#[derive(Clone, Copy)]
pub enum AddressMode {
    /// Unkown addressing mode.
    Unknown,
    /// Operand is AC.
    Accumulator,
    /// Operand is address $HHLL.
    Absolute,
    /// Operand is address incremented by X with carry.
    AbsoluteX,
    /// Operand is address incremented by Y with carry.
    AbsoluteY,
    /// Operand is byte (BB).
    Immediate,
    /// Operand implied.
    Implied,
    /// Operand is effective address; effective address is value of address.
    Indirect,
    /// Operand is effective zeropage address; effective address is byte (BB) incremented by X without carry.
    IndirectX,
    /// Operand is effective address incremented by Y with carry; effective address is word at zeropage address.
    IndirectY,
    /// Branch target is PC + offset (BB), bit 7 signifies negative offset.
    Relative,
    /// Operand is of address; address hibyte = zero ($00xx).
    Zeropage,
    /// Operand is address incremented by X; address hibyte = zero ($00xx); no page transition.
    ZeropageX,
    /// Operand is address incremented by Y; address hibyte = zero ($00xx); no page transition.
    ZeropageY,
}
