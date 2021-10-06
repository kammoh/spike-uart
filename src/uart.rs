use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::convert::*;
use std::io::prelude::*;

use crate::mmio_device::MmioDevice;

impl Drop for Uart {
    fn drop(&mut self) {
        println!("Dropping Uart");
    }
}

#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(u64)]
enum Address {
    Data = 0,  // Data RX/TX
    IntrEn,    // Interrupt enable
    IntrId,    // Interrupt identification
    LineCtl,   // Line control
    ModemCtl,  // Modem control
    LineStat,  // Line status
    ModemStat, // Modem status
    Scratch,   // Scratch register
}

#[derive(IntoPrimitive)]
#[repr(u8)]
enum LineStatValue {
    DataReady = 1 << 0,
    // Overrun = 1 << 1,
    // ParityError = 1 << 2,
    // FramingError = 1 << 3,
    // Break = 1 << 4,
    TxHoldingRegEmpty = 1 << 5,
    TxCompletelyEmpty = 1 << 6,
}

struct Config {
    flush: bool,
}
pub struct Uart {
    conf: Config,
    out: Box<dyn Write>,
}

impl MmioDevice for Uart {
    fn new(args: Vec<String>) -> Result<Self, &'static str> {
        let conf = Config { flush: !args.contains(&String::from("no-flush")) };
        Ok(Uart { conf, out: Box::new(std::io::stdout()) })
    }

    fn store(&mut self, addr: u64, data: &[u8]) -> Result<(), &str> {
        if data.len() != 1 || addr != Address::Data as u64 {
            println!("in store! bad data");
            return Err("Only single-byte stores are supported");
        }
        match Address::try_from(addr) {
            Ok(Address::Data) => {
                self.out.write(data).unwrap();
                if self.conf.flush {
                    self.out.flush().unwrap();
                }
                Ok(())
            }
            _ => Err("Only stores to `Address::Data` (i.e, 0) are supported"),
        }
    }

    fn load(&self, addr: u64, len: usize) -> Result<Vec<u8>, &str> {
        if len != 1 {
            return Err("Only single-byte loads are supported");
        }

        match Address::try_from(addr) {
            Ok(Address::LineStat) => Ok(vec![
                u8::from(LineStatValue::DataReady)
                    | u8::from(LineStatValue::TxHoldingRegEmpty)
                    | u8::from(LineStatValue::TxCompletelyEmpty),
            ]),
            // Ok(Address::Data) => &[self.out. read_byte().unwrap()],
            _ => Ok(vec![0]),
        }
    }
}
