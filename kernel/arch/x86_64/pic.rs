//! Some code was borrowed from [Redox OS Kernel](https://gitlab.redox-os.org/redox-os/kernel)

use x86_64::instructions::port::Port;

pub static mut MASTER: Pic = Pic::new(0x20);
pub static mut SLAVE: Pic = Pic::new(0xA0);

pub unsafe fn init() {
    MASTER.data.write(0xff);
    SLAVE.data.write(0xff);

   // Start initialization
    MASTER.cmd.write(0x11);
    SLAVE.cmd.write(0x11);

    // Set offsets
    MASTER.data.write(0x20);
    SLAVE.data.write(0x28);

    // Set up cascade
    MASTER.data.write(0x4);
    SLAVE.data.write(0x2);

    // Set up interrupt mode (1 is 8086/88 mode, 2 is auto EOI)
    MASTER.data.write(0x1);
    SLAVE.data.write(0x1);

    // Unmask interrupts
    MASTER.data.write(0x0);
    SLAVE.data.write(0x0);

    // Ack remaining interrupts
    MASTER.ack();
    SLAVE.ack();
}

pub struct Pic {
    pub cmd: Port<u8>,
    pub data: Port<u8>,
}

impl Pic {
    pub const fn new(port: u16) -> Self {
        Self {
            cmd: Port::new(port),
            data: Port::new(port + 1),
        }
    }

    pub unsafe fn ack(&mut self) {
        self.cmd.write(0x20);
    }

    pub unsafe fn mask_set(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask |= 1 << irq;
        self.data.write(mask);
    }

    pub unsafe fn mask_clear(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask &= !(1 << irq);
        self.data.write(mask);
    }
}
