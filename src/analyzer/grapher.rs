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
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::Arc,
};

struct BasicBlk {
    pub list: Vec<EE>,
    pub next: Vec<Arc<BasicBlk>>,
    pub prev: Vec<Arc<BasicBlk>>,
}

struct SymbolTable {
    pub symbols: HashMap<u64, String>,
}

pub struct Program {
    blk_list: Vec<BasicBlk>,
    symbol_table: SymbolTable,
}

impl BasicBlk{
    pub fn new() -> Self {
        Self{
            list: Vec::new(),
            next: Vec::new(),
            prev: Vec::new()
        }
    }
}

impl Program {
    pub fn init() -> Self {
        return Self {
            blk_list: Vec::new(),
            symbol_table: SymbolTable {
                symbols: HashMap::new(),
            },
        };
    }
    pub fn begin(&mut self, path: &str) -> Result<()> {
        let buffer = match fs::read(path) {
            Ok(i) => i,
            Err(_) => return Err(anyhow!("Failed to parse given file")),
        };
        let buf = match Object::parse(&buffer) {
            Ok(i) => i,
            Err(_) => return Err(anyhow!("Failed to parse given exec file")),
        };
        match buf {
            Object::Elf(elf) => {
                if Program::contains_text(&elf) {
                    self.parse_text(elf, buffer)
                } else if Program::contains_pt_load(&elf) {
                    self.parse_pt_load(elf, buffer)
                }
            }
            _ => return Err(anyhow!("Provided file is not Elf file")),
        }
        Ok(())
    }
    fn contains_text(elf: &Elf) -> bool {
        for section in elf.section_headers.iter() {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                if name == ".text" {
                    return true;
                }
            }
        }
        return false;
    }
    fn contains_pt_load(elf: &Elf) -> bool {
        for ph in elf.program_headers.iter() {
            if ph.p_type == PT_LOAD && (ph.p_flags & PF_X) != 0 {
                return true;
            }
        }
        return false;
    }
    fn parse_text(&mut self, elf: Elf, buffer: Vec<u8>) {
        let pending: Vec<u32> = Vec::new();
        let processed: HashSet<u32> = HashSet::new();
        for sec in elf.section_headers.iter() {
            if let Some(name) = elf.shdr_strtab.get_at(sec.sh_name) && name == ".text" {
                let start  = sec.sh_offset as usize;
                let end = start + sec.sh_size as usize;
                let code_bytes = &buffer[start..end];

                let blk = BasicBlk {

                }
                for chunk in code_bytes.chunks_exact(4) {

                }
        }}
    }
    fn parse_pt_load(&mut self, elf: Elf) {
        todo!()
    }
}
