use crate::eetran::cpu::*;
use crate::eetran::trans::*;
use anyhow::{Result, anyhow};
use goblin::{
    Object,
    elf::{
        Elf,
        program_header::{PF_X, PT_LOAD},
    },
};
use inkwell::{OptimizationLevel, builder::Builder, context::Context, module::Module};
use log;
use rangemap::map::RangeMap;
use std::{
    collections::{HashMap, HashSet},
    fs,
    option::Iter,
    sync::Arc,
};

pub struct Block {
    insts: Vec<EE>,
    next: HashSet<Arc<Block>>,
    prev: HashSet<Arc<Block>>,
}

pub struct ProgAnalysis<'a> {
    symbol_table: HashMap<u64, String>,
    map: RangeMap<Iter<'a, u64>, Block>,
}

impl Block {
    pub fn new(buf: &[u8]) -> Result<(Self, u64)> {
        let mut insts = Vec::new();
        let mut iterator = buf.chunks_exact(4).enumerate().peekable();
        while let Some((idx, inst)) = iterator.next() {
            if iterator.peek().is_none() {
                return OK((
                    Self {
                        insts: insts,
                        next: HashSet::new(),
                        prev: HashSet::new(),
                    },
                    idx as u64 * 4,
                ));
            } else {
                let trans = EE::translate(u32::from_le_bytes([inst[0], inst[1], inst[2], inst[3]]));
                if matches!(trans, EE::J(..))
                    || matches!(trans, EE::JAL(..))
                    || matches!(trans, EE::BEQ(..))
                    || matches!(trans, EE::BNE(..))
                    || matches!(trans, EE::BLEZ(..))
                    || matches!(trans, EE::BGTZ(..))
                    || matches!(trans, EE::BEQL(..))
                    || matches!(trans, EE::BNEL(..))
                    || matches!(trans, EE::BLEZL(..))
                    || matches!(trans, EE::BGTZL(..))
                    || matches!(trans, EE::SPECIAL(Special::JR(..)))
                    || matches!(trans, EE::SPECIAL(Special::JALR(..)))
                    || matches!(trans, EE::SPECIAL(Special::SYSCALL(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BLTZ(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BGEZ(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BLTZL(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BGEZL(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BLTZAL(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BGEZAL(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BLTZALL(..)))
                    || matches!(trans, EE::REGIMM(Regimm::BGEZALL(..)))
                {
                    return Ok((
                        Self {
                            insts: insts,
                            next: HashSet::new(),
                            prev: HashSet::new(),
                        },
                        idx as u64 * 4,
                    ));
                } else {
                    insts.push(trans);
                }
            }
        }
        return Err(anyhow!("Failed to create block"));
    }

    pub fn split_block(&mut Self, target: u64) -> (Self, Self) {
        let id = target/4 as u64;
        todo!()
    }
}
