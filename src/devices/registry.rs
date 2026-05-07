use crate::{
    devices::{uart::UartPl011, Driver},
    mutex::Mutex,
    util::dtb::DtbNode,
};

pub struct DeviceRegistry {
    pub uart: Option<Mutex<UartPl011>>,
    // TODO: more drivers
    // gic: Gic
    // timer: Timer
}

impl DeviceRegistry {
    pub const fn empty() -> Self {
        Self { uart: None }
    }

    pub fn try_bind(&mut self, node: &DtbNode) {
        if let Some(compat) = node.prop_compatible() {
            for value in compat {
                match value {
                    UartPl011::COMPATIBLE => {
                        if let Some(mut uart) = UartPl011::probe(node).ok() {
                            uart.init();
                            self.uart = Some(Mutex::new(uart));
                            return;
                        }
                    }

                    _ => continue, // check remaining compatible values
                }
            }
        }
    }
}
