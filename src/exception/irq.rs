pub enum IRQ {
    GenericPhysTimer,
}

impl IRQ {
    pub fn gic_int_id(&self) -> usize {
        match self {
            Self::GenericPhysTimer => 30,
        }
    }
}
