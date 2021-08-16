//! Platform-Level Interrupt Controller
use core::marker::PhantomData;
use e310x::Interrupt;
use e310x::PLIC;
use riscv::register::{mie, mip};

/// Priority of a plic::Interrupt.
#[derive(Clone, Copy)]
pub enum Priority {
    /// Priority 0: Never interrupt
    P0,
    /// Priority 1: Lowest active priority
    P1,
    /// Priority 2
    P2,
    /// Priority 3
    P3,
    /// Priority 4
    P4,
    /// Priority 5
    P5,
    /// Priority 6
    P6,
    /// Priority 7: Highest priority
    P7,
}

impl Priority {
    /// Takes a read interrupt priority or plic threshold
    /// register value and returns a plic::Priority enum.
    fn from(prio: u32) -> Option<Priority> {
        match prio {
            0 => Some(Priority::P0),
            1 => Some(Priority::P1),
            2 => Some(Priority::P2),
            3 => Some(Priority::P3),
            4 => Some(Priority::P4),
            5 => Some(Priority::P5),
            6 => Some(Priority::P6),
            7 => Some(Priority::P7),
            _ => None,
        }
    }
}

impl Into<u32> for Priority {
    /// Returns the numeric priority for writing to a
    /// interrupt priority or the plic threshold register.
    fn into(self) -> u32 {
        match self {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
            Priority::P5 => 5,
            Priority::P6 => 6,
            Priority::P7 => 7,
        }
    }
}

/// Watchdog interrupt (type state)
pub struct IrqWatchdog;
/// Realtime clock interrupt (type state)
pub struct IrqRtc;
/// Uart0 interrupt (type state)
pub struct IrqUart0;

/// Parts of `PLIC` peripheral for fine grained permissions.
pub struct Plic {
    /// Opaque mext register
    pub mext: MEXT,
    /// Opaque threshold register
    pub threshold: THRESHOLD,
    /// Opaque claim register
    pub claim: CLAIM,
    /// Opaque watchdog register
    pub wdog: INTERRUPT<IrqWatchdog>,
    /// Opaque rtc register
    pub rtc: INTERRUPT<IrqRtc>,
    /// Opaque uart0 register
    pub uart0: INTERRUPT<IrqUart0>,
}

impl From<PLIC> for Plic {
    fn from(_: PLIC) -> Self {
        Plic {
            mext: MEXT { _0: () },
            threshold: THRESHOLD { _0: () },
            claim: CLAIM { _0: () },
            wdog: INTERRUPT {
                offset: 0,
                mask: 1 << (Interrupt::WATCHDOG as u8),
                priority_offset: Interrupt::WATCHDOG as usize,
                _marker: PhantomData,
            },
            rtc: INTERRUPT {
                offset: 0,
                mask: 1 << (Interrupt::RTC as u8),
                priority_offset: Interrupt::RTC as usize,
                _marker: PhantomData,
            },
            uart0: INTERRUPT {
                offset: 0,
                mask: 1 << (Interrupt::UART0 as u8),
                priority_offset: Interrupt::UART0 as usize,
                _marker: PhantomData,
            },
        }
    }
}

/// Opaque MEXT register.
pub struct MEXT {
    _0: (),
}

impl MEXT {
    /// Enable MachineExternal interrupt.
    #[inline]
    pub fn enable(&mut self) {
        unsafe { mie::set_mext() };
    }

    /// Disable MachineExternal interrupt.
    #[inline]
    pub fn disable(&mut self) {
        unsafe { mie::clear_mext() };
    }

    /// Returns true when MachineExternal interrupt is pending.
    #[inline]
    pub fn is_pending(&self) -> bool {
        mip::read().mext()
    }
}

/// Opaque THRESHOLD register.
pub struct THRESHOLD {
    _0: (),
}

impl THRESHOLD {
    /// Returns the current active priority threshold.
    pub fn get(&self) -> Priority {
        // NOTE: Atomic read with no side effects.
        let threshold = unsafe { (*PLIC::ptr()).threshold.read() };
        Priority::from(threshold.bits()).unwrap()
    }

    /// Sets the current active priority threshold. This
    /// deactivates all interrupts with a lower priority.
    pub fn set(&mut self, priority: Priority) {
        // NOTE: Atomic write with no side effects.
        unsafe {
            (*PLIC::ptr()).threshold.write(|w| w.bits(priority.into()));
        }
    }
}

/// Opaque CLAIM register.
pub struct CLAIM {
    _0: (),
}

impl CLAIM {
    /// Claims the interrupt with the highest priority.
    pub fn claim(&mut self) -> Option<Interrupt> {
        // NOTE: Atomic read with side effects.
        let intr = unsafe { (*PLIC::ptr()).claim.read().bits() };

        // If no interrupt is pending return None
        if intr == 0 {
            None
        } else {
            Some(Interrupt::try_from(intr as u8).unwrap())
        }
    }

    /// Notifies the PLIC that a claimed interrupt is complete.
    pub fn complete(&mut self, intr: Interrupt) {
        // NOTE: Atomic write with side effects.
        unsafe {
            (*PLIC::ptr()).claim.write(|w| w.bits(intr as u32));
        }
    }
}

/// Fine grained interrupt handling.
pub struct INTERRUPT<IRQ> {
    /// Offset in to enable and pending plic registers
    offset: usize,
    /// Bitmask for enable and pending plic registers
    mask: u32,
    /// Offset in to priority plic registers
    priority_offset: usize,
    _marker: PhantomData<IRQ>,
}

impl<IRQ> INTERRUPT<IRQ> {
    /// Enable IRQ interrupt.
    #[inline]
    pub fn enable(&mut self) {
        // NOTE: should use atomic operations
        unsafe {
            (*PLIC::ptr()).enable[self.offset].modify(|r, w| w.bits(r.bits() | self.mask));
        }
    }

    /// Disable IRQ interrupt.
    #[inline]
    pub fn disable(&mut self) {
        // NOTE: should use atomic operations
        unsafe {
            (*PLIC::ptr()).enable[self.offset].modify(|r, w| w.bits(r.bits() & !self.mask));
        }
    }

    /// Returns true when IRQ interrupt is pending.
    pub fn is_pending(&self) -> bool {
        // NOTE: Atomic write without side effects.
        let pending = unsafe { (*PLIC::ptr()).pending[self.offset].read() };
        pending.bits() & self.mask == self.mask
    }

    /// Returns true when WDOG interrupt is enabled.
    pub fn is_enabled(&self) -> bool {
        // NOTE: Atomic write without side effects.
        let enabled = unsafe { (*PLIC::ptr()).enable[self.offset].read() };
        enabled.bits() & self.mask == self.mask
    }

    /// Returns the priority of the IRQ interrupt.
    pub fn priority(&self) -> Priority {
        // NOTE: Atomic read without side effects.
        let priority = unsafe { (*PLIC::ptr()).priority[self.priority_offset].read() };
        Priority::from(priority.bits()).unwrap()
    }

    /// Sets the priority of the IRQ interrupt.
    pub fn set_priority(&mut self, priority: Priority) {
        // NOTE: Atomic write without side effects.
        unsafe {
            (*PLIC::ptr()).priority[self.priority_offset].write(|w| w.bits(priority as u32));
        }
    }
}
