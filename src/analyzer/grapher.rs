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
    collections::{HashMap, HashSet, VecDeque},
    fs,
    iter::Enumerate,
    option::Iter,
    slice::ChunksExact,
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
                return Ok((
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
                        (idx as u64 + 1) * 4,
                    ));
                } else {
                    insts.push(trans);
                }
            }
        }
        return Err(anyhow!("Failed to create block"));
    }

    pub fn split_block(
        &mut self,
        target: u64, // target is the jump address
        buf: &[u8],
    ) -> (Result<(Self, u64)>, Result<(Self, u64)>) {
        let id = usize::from((target / 4) as u8);
        return (Self::new(&buf[..id]), Self::new(&buf[id..]));
    }
}

impl<'a> ProgAnalysis<'a> {
    pub fn new(path: &str) -> Self {
        Self {
            symbol_table: HashMap::new(),
            map: RangeMap::new(),
        }
    }
    pub fn graph(&mut self, path: &str) -> Self {
        let buf = match fs::read(path) {
            Ok(i) => i,
            Err(err) => {
                log::error!("Failed to translate due to error: {:?}", err);
                std::process::exit(-1);
            }
        };
        let (start_loc, instructions) = Self::load_instructions(&buf);

        //Actual graphing code starts here
        let mut processing: VecDeque<u64> = VecDeque::from([start_loc]);
        let mut processed: HashSet<u64> = HashSet::new();
        todo!()
    }
    fn load_instructions(buf: &[u8]) -> (u64, Enumerate<ChunksExact<'_, u8>>) {
        let obj = match Object::parse(buf) {
            Ok(i) => i,
            Err(err) => {
                log::error!("Failed to parse file due to error: {:?}", err);
                std::process::exit(-1);
            }
        };
        let elf = match obj {
            Object::Elf(elf) => elf,
            _ => {
                log::error!("Provided is not of type ELF, exiting");
                std::process::exit(-1);
            }
        };
        // Find the executable code section
        // For basic files
        let mut found_code = false;
        for section in elf.section_headers.iter() {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                if name == ".text" {
                    found_code = true;
                    let start = section.sh_offset as usize;
                    let end = start + section.sh_size as usize;
                    let code_bytes = &buf[start..end];
                    return (elf.entry, code_bytes.chunks_exact(4).enumerate());
                }
            }
        }

        // For stripped files
        if !found_code {
            for ph in elf.program_headers.iter() {
                if ph.p_type == PT_LOAD && (ph.p_flags & PF_X) != 0 {
                    found_code = true;
                    let start = ph.p_offset as usize;
                    let end = start + ph.p_filesz as usize;
                    let code_bytes = &buf[start..end];
                    return (elf.entry, code_bytes.chunks_exact(4).enumerate());
                }
            }
        } else {
            log::error!("Failed to find an executable section in the code, exiting...");
            std::process::exit(-1);
        }

        // This should be unreachable, no idea why compiler asks for this
        log::error!(
            "This should not be reachable, fatal error during finding executable section of file"
        );
        std::process::exit(-1);
    }
}
