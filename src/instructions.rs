type Addr = u16;
type Register = u8;
type Key = u8;

pub fn parse(upper: u8, lower: u8) -> Instruction {

    let start = upper >> 4;

    use Instruction::*;
    match start {
        0 => match lower {
            0xE0 => Cls,
            0xEE => Ret,
            _ => panic!("Invalid instruciton"),
        },

        1 => Jump(to_address(upper, lower)),
        2 => Call(to_address(upper, lower)),
        3 => SkipEqConst(to_reg_upper(upper), lower),
        4 => SkipNotEqConst(to_reg_upper(upper), lower),
        5 => SkipEqReg(to_reg_upper(upper), to_reg_lower(lower)),
        6 => LoadConst(to_reg_upper(upper), lower),
        7 => AddConst(to_reg_upper(upper), lower),
        8 => match lower & 0x0F {
            0 => LoadReg(to_reg_upper(upper), to_reg_lower(lower)),
            1 => Or(to_reg_upper(upper), to_reg_lower(lower)),
            2 => And(to_reg_upper(upper), to_reg_lower(lower)),
            3 => Xor(to_reg_upper(upper), to_reg_lower(lower)),
            4 => Add(to_reg_upper(upper), to_reg_lower(lower)),
            5 => Sub(to_reg_upper(upper), to_reg_lower(lower)),
            6 => ShiftRight(to_reg_upper(upper), to_reg_lower(lower)),
            7 => SubN(to_reg_upper(upper), to_reg_lower(lower)),
            0xE => ShiftLeft(to_reg_upper(upper), to_reg_lower(lower)),
            _ => panic!("Not a valid instruciton {:#01x}, {:#01x}", upper, lower)
        },

        9 => SkipNotEqReg(to_reg_upper(upper), to_reg_lower(lower)), // TODO: Not 100% corect we match 9XY_ and not only 9XY0, we should also check that the lower 4 bits of lower is
        0xA => LoadAddr(to_address(upper, lower)),
        0xB => JumpOffset(to_address(upper, lower)),
        0xC => Rand(to_reg_upper(upper), lower),
        0xD => Draw(to_reg_upper(upper), to_reg_lower(lower), lower & 0x0F),
        0xE => match lower & 0x0F {
            0x9E => SkipOnKeyPressed(to_reg_upper(upper)),
            0xA1 => SkipKeyNotPressed(to_reg_upper(upper)),
            _ => panic!("Not a valid instruciton {:#02x}, {:#02x}", upper, lower)
        },

        0xF => match lower {
            0x07 => LoadDelay(to_reg_upper(upper)),
            0x0A => WaitKeyPress(to_reg_upper(upper)),
            0x15 => SetDelay(to_reg_upper(upper)),
            0x18 => SetSound(to_reg_upper(upper)),
            0x1e => AddAddr(to_reg_upper(upper)),
            0x29 => SetSpriteAddr(to_reg_upper(upper)),
            0x33 => BCD(to_reg_upper(upper)),
            0x55 => Store(to_reg_upper(upper)),
            0x65 => Load(to_reg_upper(upper)),
            _ => panic!("Not a valid instruciton {:#02x}, {:#02x}", upper, lower)
        },
        _ => panic!("Not implemented"),
    }
}


fn to_reg_upper(upper: u8) -> Register {
    upper & 0x0F
}

fn to_reg_lower(lower: u8) -> Register {
    (lower & 0xF0) >> 4
}

fn to_address( upper: u8, lower: u8) -> Addr {
    // TODO: maybe add a check to see if address is over 4096

    (((upper & 0x0F) as u16) << 8) + (lower as u16)
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Cls,
    Ret,
    Jump(Addr),
    Call(Addr),
    SkipEqConst(Register, u8),
    SkipNotEqConst(Register, u8),
    SkipEqReg(Register, Register),
    SkipNotEqReg(Register, Register),
    LoadConst(Register, u8),
    AddConst(Register, u8),
    LoadReg(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    ShiftRight(Register, Register),
    SubN(Register, Register),
    ShiftLeft(Register, Register),
    LoadAddr(Addr),
    JumpOffset(Addr),
    Rand(Register, u8),
    Draw(Register, Register, u8),
    SkipOnKeyPressed(Register),
    SkipKeyNotPressed(Register),
    LoadDelay(Register),
    WaitKeyPress(Register),
    SetDelay(Register),
    SetSound(Register),
    AddAddr(Register),
    SetSpriteAddr(Register),
    BCD(Register),
    Store(Register),
    Load(Register)
}


#[cfg(test)]
mod tests {

    use rstest::*;
    use super::*;
    use super::Instruction::*;

    #[rstest]
    #[case(0x00E0, Cls)]
    #[case(0x00EE, Ret)]

    #[case(0x10EE, Jump(0x0EE))]
    #[case(0x23EE, Call(0x3EE))]
    #[case(0x33EE, SkipEqConst(3, 0xEE))]
    #[case(0x42EE, SkipNotEqConst(2, 0xEE))]
    #[case(0x5150, SkipEqReg(1, 5))]
    #[case(0x6150, LoadConst(1, 0x50))]
    #[case(0x7250, AddConst(2, 0x50))]
    #[case(0x80a0, LoadReg(0, 0xa))]

    fn parse_test(#[case] data: u16, #[case] expected: Instruction) {

        println!("{:#04x}", data);
        let instr = parse(((data & 0xFF00) >> 8) as u8, (data & 0x00FF) as u8);

        println!("{:?}", instr);
        assert_eq!(instr,expected);
    }

    #[rstest]
    #[case(0xF3, 3)]
    #[case(0xFa, 0xa)]
    #[case(0xF0, 0x0)]
    #[case(0xFd, 0xd)]
    fn reg_upper(#[case] upper: u8, #[case] expected: u8) {
        assert_eq!(to_reg_upper(upper), expected);

    }

    #[rstest]
    #[case(0x3F, 3)]
    #[case(0xa0, 0xa)]
    #[case(0x0F, 0x0)]
    #[case(0xd3, 0xd)]
    fn reg_lower(#[case] lower: u8,  #[case] expected: u8) {
        assert_eq!(to_reg_lower(lower), expected);

    }
}
