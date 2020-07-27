//! interrupt descriptor table
#![allow(unused)]
/// type of DPL
pub enum DPL {
    Machine,
    Supervisor,
    User,
}
/// type of gate
pub enum GATE {
    Interrupt,
    Trap,
    System,
}
/// save address of interrupt handle function
///
/// ### `#[repr(C)]`
/// arrange the memory like C
#[repr(C)]
//#[derive(Clone, Copy)]
pub struct Gate {
    pub base: usize,
    pub offset: usize,
    pub dpl: DPL,
    pub gtype: GATE,
}
impl Gate {
    /// creat initialized `gate`
    pub fn new(gate_base: usize, gate_offset: usize, gate_dpl: DPL, gate_type: GATE) -> Self {
        Gate {
            base: gate_base,
            offset: gate_offset,
            dpl: gate_dpl,
            gtype: gate_type,
        }
    }
}

/// struct IDT
///
/// arrange the memory like C
#[repr(C)]
pub struct IDT {
    pub length: usize,
    pub gates: [Gate; 10],
}

impl IDT {
    /// creat initialized `IDT`
    pub fn new() -> Self {
        IDT {
            length: 10,
            gates: [
                Gate::new(0, 0, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 1, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 2, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 3, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 4, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 5, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 6, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 7, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 8, DPL::Supervisor, GATE::Interrupt),
                Gate::new(0, 9, DPL::Supervisor, GATE::Interrupt),
            ],
        }
    }
}
