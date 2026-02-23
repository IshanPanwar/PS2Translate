use crate::eetran::cpu::*;

pub trait Gen {
    fn generate(&self) -> String;
}

impl Gen for EE {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Special {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Regimm {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Mmi {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Mmi0 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Mmi1 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Mmi2 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Mmi3 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Cop0 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Bc0 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Tlb {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Cop1 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Bc1 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Fpus {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Fpuw {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Cop2 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Bc2 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Special1 {
    fn generate(&self) -> String {
        todo!();
    }
}

impl Gen for Special2 {
    fn generate(&self) -> String {
        todo!();
    }
}
