use crate::eetran::cpu::*;

pub trait Trans<T> {
    fn translate(inst: u32) -> T;
}

impl Trans<EE> for EE {
    fn translate(inst: u32) -> Self {
        match (inst >> 26) ^ 0x3F {
            0x00 => match Special::translate(inst) {
                Special::ILLEGAL => return Self::ILLEGAL,
                i => return Self::SPECIAL(i),
            },
            0x01 => match Regimm::translate(inst) {
                Regimm::ILLEGAL => return Self::ILLEGAL,
                i => return Self::REGIMM(i),
            },
            0x02 => return Self::J(inst),
            0x03 => return Self::JAL(inst),
            0x04 => return Self::BEQ(inst),
            0x05 => return Self::BNE(inst),
            0x06 => return Self::BLEZ(inst),
            0x07 => return Self::BGTZ(inst),
            0x08 => return Self::ADDI(inst),
            0x09 => return Self::ADDIU(inst),
            0x0A => return Self::SLTI(inst),
            0x0B => return Self::SLTIU(inst),
            0x0C => return Self::ANDI(inst),
            0x0D => return Self::ORI(inst),
            0x0E => return Self::XORI(inst),
            0x0F => return Self::LUI(inst),
            0x10 => match Cop0::translate(inst) {
                Cop0::ILLEGAL => return Self::ILLEGAL,
                i => return Self::COP0(i),
            },
            0x11 => match Cop1::translate(inst) {
                Cop1::ILLEGAL => return Self::ILLEGAL,
                i => return Self::COP1(i),
            },
            0x12 => match Cop2::translate(inst) {
                Cop2::ILLEGAL => return Self::ILLEGAL,
                i => return Self::COP2(i),
            },
            0x14 => return Self::BEQL(inst),
            0x15 => return Self::BNEL(inst),
            0x16 => return Self::BLEZL(inst),
            0x17 => return Self::BGTZL(inst),
            0x18 => return Self::DADDI(inst),
            0x19 => return Self::DADDIU(inst),
            0x1A => return Self::LDL(inst),
            0x1B => return Self::LDR(inst),
            0x1C => match Mmi::translate(inst) {
                Mmi::ILLEGAL => return Self::ILLEGAL,
                i => return Self::MMI(i),
            },
            0x1E => return Self::LQ(inst),
            0x1F => return Self::SQ(inst),
            0x20 => return Self::LB(inst),
            0x21 => return Self::LH(inst),
            0x22 => return Self::LWL(inst),
            0x23 => return Self::LW(inst),
            0x24 => return Self::LBU(inst),
            0x25 => return Self::LHU(inst),
            0x26 => return Self::LWR(inst),
            0x27 => return Self::LWU(inst),
            0x28 => return Self::SB(inst),
            0x29 => return Self::SH(inst),
            0x2A => return Self::SWL(inst),
            0x2B => return Self::SW(inst),
            0x2C => return Self::SDL(inst),
            0x2D => return Self::SDR(inst),
            0x2E => return Self::SWR(inst),
            0x2F => return Self::CACHE(inst),
            0x31 => return Self::LWC1(inst),
            0x33 => return Self::PREF(inst),
            0x36 => return Self::LQC2(inst),
            0x37 => return Self::LD(inst),
            0x39 => return Self::SWC1(inst),
            0x3E => return Self::SQC2(inst),
            0x3F => return Self::SD(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Special> for Special {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x3F {
            0x00 => return Self::SLL(inst),
            0x02 => return Self::SRL(inst),
            0x03 => return Self::SRA(inst),
            0x04 => return Self::SLLV(inst),
            0x06 => return Self::SRLV(inst),
            0x07 => return Self::SRAV(inst),
            0x08 => return Self::JR(inst),
            0x09 => return Self::JALR(inst),
            0x0A => return Self::MOVZ(inst),
            0x0B => return Self::MOVN(inst),
            0x0C => return Self::SYSCALL(inst),
            0x0D => return Self::BREAK(inst),
            0x0F => return Self::SYNC(inst),

            // Row 010 (0x10 - 0x17)
            0x10 => return Self::MFHI(inst),
            0x11 => return Self::MTHI(inst),
            0x12 => return Self::MFLO(inst),
            0x13 => return Self::MTLO(inst),
            0x14 => return Self::DSLLV(inst),
            0x16 => return Self::DSRLV(inst),
            0x17 => return Self::DSRAV(inst),

            // Row 011 (0x18 - 0x1F)
            0x18 => return Self::MULT(inst),
            0x19 => return Self::MULTU(inst),
            0x1A => return Self::DIV(inst),
            0x1B => return Self::DIVU(inst),

            // Row 100 (0x20 - 0x27)
            0x20 => return Self::ADD(inst),
            0x21 => return Self::ADDU(inst),
            0x22 => return Self::SUB(inst),
            0x23 => return Self::SUBU(inst),
            0x24 => return Self::AND(inst),
            0x25 => return Self::OR(inst),
            0x26 => return Self::XOR(inst),
            0x27 => return Self::NOR(inst),

            // Row 101 (0x28 - 0x2F)
            0x28 => return Self::MFSA(inst),
            0x29 => return Self::MTSA(inst),
            0x2A => return Self::SLT(inst),
            0x2B => return Self::SLTU(inst),
            0x2C => return Self::DADD(inst),
            0x2D => return Self::DADDU(inst),
            0x2E => return Self::DSUB(inst),
            0x2F => return Self::DSUBU(inst),

            // Row 110 (0x30 - 0x37)
            0x30 => return Self::TGE(inst),
            0x31 => return Self::TGEU(inst),
            0x32 => return Self::TLT(inst),
            0x33 => return Self::TLTU(inst),
            0x34 => return Self::TEQ(inst),
            0x36 => return Self::TNE(inst),

            // Row 111 (0x38 - 0x3F)
            0x38 => return Self::DSLL(inst),
            0x3A => return Self::DSRL(inst),
            0x3B => return Self::DSRA(inst),
            0x3C => return Self::DSLL32(inst),
            0x3E => return Self::DSRL32(inst),
            0x3F => return Self::DSRA32(inst),

            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Regimm> for Regimm {
    fn translate(inst: u32) -> Self {
        match (inst >> 16) ^ 0x1F {
            0x00 => return Self::BLTZ(inst),
            0x01 => return Self::BGEZ(inst),
            0x02 => return Self::BLTZL(inst),
            0x03 => return Self::BGEZL(inst),
            0x08 => return Self::TGEI(inst),
            0x09 => return Self::TGEIU(inst),
            0x0A => return Self::TLTI(inst),
            0x0B => return Self::TLTIU(inst),
            0x0C => return Self::TEQI(inst),
            0x0E => return Self::TNEI(inst),
            0x10 => return Self::BLTZAL(inst),
            0x11 => return Self::BGEZAL(inst),
            0x12 => return Self::BLTZALL(inst),
            0x13 => return Self::BGEZAL(inst),
            0x18 => return Self::MTSAB(inst),
            0x19 => return Self::MTSAH(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Mmi> for Mmi {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x3F {
            0x00 => return Self::MADD(inst),
            0x01 => return Self::MADDU(inst),
            0x04 => return Self::PLZCW(inst),
            0x08 => match Mmi0::translate(inst) {
                Mmi0::ILLEGAL => return Self::ILLEGAL,
                i => return Self::MMI0(i),
            },
            0x09 => match Mmi2::translate(inst) {
                Mmi2::ILLEGAL => return Self::ILLEGAL,
                i => return Self::MMI2(i),
            },
            0x10 => return Self::MFHI1(inst),
            0x11 => return Self::MTHI1(inst),
            0x12 => return Self::MFLO1(inst),
            0x13 => return Self::MTLO1(inst),
            0x18 => return Self::MULT1(inst),
            0x19 => return Self::MULTU1(inst),
            0x1A => return Self::DIV1(inst),
            0x1B => return Self::DIVU1(inst),
            0x20 => return Self::MADD1(inst),
            0x21 => return Self::MADDU1(inst),
            0x28 => match Mmi1::translate(inst) {
                Mmi1::ILLEGAL => return Self::ILLEGAL,
                i => return Self::MMI1(i),
            },
            0x29 => match Mmi3::translate(inst) {
                Mmi3::ILLEGAL => return Self::ILLEGAL,
                i => return Self::MMI3(i),
            },
            0x30 => return Self::PMFHL(inst),
            0x31 => return Self::PMTHL(inst),
            0x34 => return Self::PSLLH(inst),
            0x36 => return Self::PSRLH(inst),
            0x37 => return Self::PSRAH(inst),
            0x3C => return Self::PSLLW(inst),
            0x3E => return Self::PSRLW(inst),
            0x3F => return Self::PSRAW(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Mmi0> for Mmi0 {
    fn translate(inst: u32) -> Self {
        match (inst >> 6) ^ 0x1F {
            0x00 => return Self::PADDW(inst),
            0x01 => return Self::PSUBW(inst),
            0x02 => return Self::PCGTW(inst),
            0x03 => return Self::PMAXW(inst),
            0x04 => return Self::PADDH(inst),
            0x05 => return Self::PSUBH(inst),
            0x06 => return Self::PCGTH(inst),
            0x07 => return Self::PMAXH(inst),
            0x08 => return Self::PADDB(inst),
            0x09 => return Self::PSUBB(inst),
            0x0A => return Self::PCGTB(inst),
            0x10 => return Self::PADDSW(inst),
            0x11 => return Self::PSUBSW(inst),
            0x12 => return Self::PEXTLW(inst),
            0x13 => return Self::PPACW(inst),
            0x14 => return Self::PADDSH(inst),
            0x15 => return Self::PSUBSH(inst),
            0x16 => return Self::PEXTLH(inst),
            0x17 => return Self::PPACH(inst),
            0x18 => return Self::PADDSB(inst),
            0x19 => return Self::PSUBSB(inst),
            0x1A => return Self::PEXTLB(inst),
            0x1B => return Self::PPACB(inst),
            0x1E => return Self::PEXT5(inst),
            0x1F => return Self::PPAC5(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Mmi1> for Mmi1 {
    fn translate(inst: u32) -> Self {
        match (inst >> 6) ^ 0x000007C0 {
            0x01 => return Self::PABSW(inst),
            0x02 => return Self::PCEQW(inst),
            0x03 => return Self::PMINW(inst),
            0x04 => return Self::PADSBH(inst),
            0x05 => return Self::PABSH(inst),
            0x06 => return Self::PCEQH(inst),
            0x07 => return Self::PMINH(inst),
            0x0A => return Self::PCEQB(inst),
            0x10 => return Self::PADDUW(inst),
            0x11 => return Self::PSUBUW(inst),
            0x12 => return Self::PEXTUW(inst),
            0x14 => return Self::PADDUH(inst),
            0x15 => return Self::PSUBUH(inst),
            0x16 => return Self::PEXTUH(inst),
            0x18 => return Self::PADDUB(inst),
            0x19 => return Self::PSUBUB(inst),
            0x1A => return Self::PEXTUB(inst),
            0x1B => return Self::QFSRV(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Mmi2> for Mmi2 {
    fn translate(inst: u32) -> Self {
        match (inst >> 6) ^ 0x1F {
            0x00 => return Self::PMADDW(inst),
            0x02 => return Self::PSLLVW(inst),
            0x03 => return Self::PSRLVW(inst),
            0x04 => return Self::PMSUBW(inst),
            0x08 => return Self::PMFHI(inst),
            0x09 => return Self::PMFLO(inst),
            0x0A => return Self::PINTH(inst),
            0x0C => return Self::PMULTW(inst),
            0x0D => return Self::PDIVW(inst),
            0x0E => return Self::PCPYLD(inst),
            0x10 => return Self::PMADDH(inst),
            0x11 => return Self::PHMADH(inst),
            0x12 => return Self::PAND(inst),
            0x13 => return Self::PXOR(inst),
            0x14 => return Self::PMSUBH(inst),
            0x15 => return Self::PHMSBH(inst),
            0x1A => return Self::PEXEH(inst),
            0x1B => return Self::PREVH(inst),
            0x1C => return Self::PMULTH(inst),
            0x1D => return Self::PDIVBW(inst),
            0x1E => return Self::PEXEW(inst),
            0x1F => return Self::PROT3W(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Mmi3> for Mmi3 {
    fn translate(inst: u32) -> Self {
        match (inst >> 6) ^ 0x1F {
            0x00 => return Self::PMADDUW(inst),
            0x03 => return Self::PSRAVW(inst),
            0x08 => return Self::PMTHI(inst),
            0x09 => return Self::PMTLO(inst),
            0x0A => return Self::PINTEH(inst),
            0x0C => return Self::PMULTUW(inst),
            0x0D => return Self::PDIVUW(inst),
            0x0E => return Self::PCPYUD(inst),
            0x12 => return Self::POR(inst),
            0x13 => return Self::PNOR(inst),
            0x1A => return Self::PEXCH(inst),
            0x1B => return Self::PCPYH(inst),
            0x1E => return Self::PEXCW(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Cop0> for Cop0 {
    fn translate(inst: u32) -> Self {
        match (inst >> 21) ^ 0x1F {
            0x00 => return Self::MFC0(inst),
            0x04 => return Self::MTC0(inst),
            0x08 => match Bc0::translate(inst) {
                Bc0::ILLEGAL => return Self::ILLEGAL,
                i => return Self::BC0(i),
            },
            0x10 => match Tlb::translate(inst) {
                Tlb::ILLEGAL => return Self::ILLEGAL,
                i => return Self::TLB(i),
            },
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Bc0> for Bc0 {
    fn translate(inst: u32) -> Self {
        match (inst >> 16) ^ 0x1F {
            0x00 => return Self::BC0F(inst),
            0x01 => return Self::BC0T(inst),
            0x02 => return Self::BC0FL(inst),
            0x03 => return Self::BC0TL(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Tlb> for Tlb {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x3F {
            0x01 => return Self::TLBR(inst),
            0x02 => return Self::TLBWI(inst),
            0x06 => return Self::TLBWR(inst),
            0x08 => return Self::TLBP(inst),
            0x18 => return Self::ERET(inst),
            0x38 => return Self::EI(inst),
            0x39 => return Self::DI(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Cop1> for Cop1 {
    fn translate(inst: u32) -> Self {
        match (inst >> 21) ^ 0x1F {
            0x00 => return Self::MFC1(inst),
            0x02 => return Self::CFC1(inst),
            0x04 => return Self::MTC1(inst),
            0x06 => return Self::CTC1(inst),
            0x08 => match Bc1::translate(inst) {
                Bc1::ILLEGAL => return Self::ILLEGAL,
                i => return Self::BC1(i),
            },
            0x10 => match Fpus::translate(inst) {
                Fpus::ILLEGAL => return Self::ILLEGAL,
                i => return Self::FPUS(i),
            },
            0x14 => match Fpuw::translate(inst) {
                Fpuw::ILLEGAL => return Self::ILLEGAL,
                i => return Self::FPUW(i),
            },
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Bc1> for Bc1 {
    fn translate(inst: u32) -> Self {
        match (inst >> 16) ^ 0x1F {
            0x00 => return Self::BC1F(inst),
            0x01 => return Self::BC1T(inst),
            0x02 => return Self::BC1FL(inst),
            0x03 => return Self::BC1TL(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Fpus> for Fpus {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x3F {
            0x00 => return Self::ADD_S(inst),
            0x01 => return Self::SUB_S(inst),
            0x02 => return Self::MUL_S(inst),
            0x03 => return Self::DIV_S(inst),
            0x04 => return Self::SQRT_S(inst),
            0x05 => return Self::ABS_S(inst),
            0x06 => return Self::MOV_S(inst),
            0x07 => return Self::NEG_S(inst),
            0x16 => return Self::RSQRT_S(inst),
            0x18 => return Self::ADDA_S(inst),
            0x19 => return Self::SUBA_S(inst),
            0x1A => return Self::MULA_S(inst),
            0x1C => return Self::MADD_S(inst),
            0x1D => return Self::MSUB_S(inst),
            0x1E => return Self::MADDA_S(inst),
            0x1F => return Self::MSUBA_S(inst),
            0x24 => return Self::CVT_W(inst),
            0x28 => return Self::MAX_S(inst),
            0x29 => return Self::MIN_S(inst),
            0x30 => return Self::C_F(inst),
            0x32 => return Self::C_EQ(inst),
            0x34 => return Self::C_LT(inst),
            0x36 => return Self::C_LE(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Fpuw> for Fpuw {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x3F {
            0x20 => return Self::CVT_S(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Cop2> for Cop2 {
    fn translate(inst: u32) -> Self {
        match (inst >> 21) ^ 0x1F {
            0x01 => return Self::QMFC2(inst),
            0x02 => return Self::CFC2(inst),
            0x05 => return Self::QMTC2(inst),
            0x06 => return Self::CTC2(inst),
            0x08 => match Bc2::translate(inst) {
                Bc2::ILLEGAL => return Self::ILLEGAL,
                i => return Self::BC2(i),
            },
            0x10..=0x1F => match Special1::translate(inst) {
                Special1::ILLEGAL => return Self::ILLEGAL,
                i => return Self::SPECIAL1(i),
            },
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Bc2> for Bc2 {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x001F0000 {
            0x00 => return Self::BC2F(inst),
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Special1> for Special1 {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x3F {
            0x00 => return Self::VADDx(inst),
            0x01 => return Self::VADDy(inst),
            0x02 => return Self::VADDz(inst),
            0x03 => return Self::VADDw(inst),
            0x04 => return Self::VSUBx(inst),
            0x05 => return Self::VSUBy(inst),
            0x06 => return Self::VSUBz(inst),
            0x07 => return Self::VSUBw(inst),
            0x08 => return Self::VMADDx(inst),
            0x09 => return Self::VMADDy(inst),
            0x0A => return Self::VMADDz(inst),
            0x0B => return Self::VMADDw(inst),
            0x0C => return Self::VMSUBx(inst),
            0x0D => return Self::VMSUBy(inst),
            0x0E => return Self::VMSUBz(inst),
            0x0F => return Self::VMSUBw(inst),
            0x10 => return Self::VMAXx(inst),
            0x11 => return Self::VMAXy(inst),
            0x12 => return Self::VMAXz(inst),
            0x13 => return Self::VMAXw(inst),
            0x14 => return Self::VMINIx(inst),
            0x15 => return Self::VMINIy(inst),
            0x16 => return Self::VMINIz(inst),
            0x17 => return Self::VMINIw(inst),
            0x18 => return Self::VMULx(inst),
            0x19 => return Self::VMULy(inst),
            0x1A => return Self::VMULz(inst),
            0x1B => return Self::VMULw(inst),
            0x1C => return Self::VMULq(inst),
            0x1D => return Self::VMAXi(inst),
            0x1E => return Self::VMULi(inst),
            0x1F => return Self::VMINIi(inst),
            0x20 => return Self::VADDq(inst),
            0x21 => return Self::VMADDq(inst),
            0x22 => return Self::VADDi(inst),
            0x23 => return Self::VMADDi(inst),
            0x24 => return Self::VSUBq(inst),
            0x25 => return Self::VMSUBq(inst),
            0x26 => return Self::VSUbi(inst),
            0x27 => return Self::VMSUBi(inst),
            0x28 => return Self::VADD(inst),
            0x29 => return Self::VMADD(inst),
            0x2A => return Self::VMUL(inst),
            0x2B => return Self::VMAX(inst),
            0x2C => return Self::VSUB(inst),
            0x2D => return Self::VMSUB(inst),
            0x2E => return Self::VOPMSUB(inst),
            0x2F => return Self::VMINI(inst),
            0x30 => return Self::VIADD(inst),
            0x31 => return Self::VISUB(inst),
            0x32 => return Self::VIADDI(inst),
            0x34 => return Self::VIAND(inst),
            0x35 => return Self::VIOR(inst),
            0x38 => return Self::VCALLMS(inst),
            0x39 => return Self::CALLMSR(inst),
            0x3C..=0x3F => match Special2::translate(inst) {
                Special2::ILLEGAL => return Self::ILLEGAL,
                i => return Self::SPECIAL2(i),
            },
            _ => return Self::ILLEGAL,
        }
    }
}

impl Trans<Special2> for Special2 {
    fn translate(inst: u32) -> Self {
        match inst ^ 0x07FF {
            0x078 => Self::VADDAx(inst),
            0x079 => Self::VADDAy(inst),
            0x07A => Self::VADDAz(inst),
            0x07B => Self::VADDAw(inst),
            0x07C => Self::VSUBAx(inst),
            0x07D => Self::VSUBAy(inst),
            0x07E => Self::VSUBAz(inst),
            0x07F => Self::VSUBAw(inst),
            0x0F8 => Self::VMADDAx(inst),
            0x0F9 => Self::VMADDAy(inst),
            0x0FA => Self::VMADDAz(inst),
            0x0FB => Self::VMADDAw(inst),
            0x0FC => Self::VMSUBAx(inst),
            0x0FD => Self::VMSUBAy(inst),
            0x0FE => Self::VMSUBAz(inst),
            0x0FF => Self::VMSUBAw(inst),
            0x178 => Self::VITOF0(inst),
            0x179 => Self::VITOF4(inst),
            0x17A => Self::VITOF12(inst),
            0x17B => Self::VITOF15(inst),
            0x17C => Self::VFTOI0(inst),
            0x17D => Self::VFTOI4(inst),
            0x17E => Self::VFTOI12(inst),
            0x17F => Self::VFTOI15(inst),
            0x1F8 => Self::VMULAx(inst),
            0x1F9 => Self::VMULAy(inst),
            0x1FA => Self::VMULAz(inst),
            0x1FB => Self::VMULAw(inst),
            0x1FC => Self::VMULAq(inst),
            0x1FD => Self::VABS(inst),
            0x1FE => Self::VMULAi(inst),
            0x1FF => Self::VCLIPw(inst),
            0x278 => Self::VADDAq(inst),
            0x279 => Self::VMADDAq(inst),
            0x27A => Self::VADDAi(inst),
            0x27B => Self::VMADDAi(inst),
            0x27C => Self::VSUBAq(inst),
            0x27D => Self::VMSUBAq(inst),
            0x27E => Self::VSUBAi(inst),
            0x27F => Self::VMSUBAi(inst),
            0x2F8 => Self::VADDA(inst),
            0x2F9 => Self::VMADDA(inst),
            0x2FA => Self::VMULA(inst),
            0x2FC => Self::VSUBA(inst),
            0x2FD => Self::VMSUBA(inst),
            0x2FE => Self::VOPMULA(inst),
            0x2FF => Self::VNOP(inst),
            0x378 => Self::VMOVE(inst),
            0x379 => Self::VMR32(inst),
            0x37C => Self::VLQI(inst),
            0x37D => Self::VSQI(inst),
            0x37E => Self::VLQD(inst),
            0x37F => Self::VSQD(inst),
            0x3F8 => Self::VDIV(inst),
            0x3F9 => Self::VSQRT(inst),
            0x3FA => Self::VRSQRT(inst),
            0x3FB => Self::VWAITQ(inst),
            0x3FC => Self::VMTIR(inst),
            0x3FD => Self::VMFIR(inst),
            0x3FE => Self::VILWR(inst),
            0x3FF => Self::VISWR(inst),
            0x478 => Self::VRNEXT(inst),
            0x479 => Self::VRGET(inst),
            0x47A => Self::VRINIT(inst),
            0x47B => Self::VRXOR(inst),
            _ => Self::ILLEGAL,
        }
    }
}
