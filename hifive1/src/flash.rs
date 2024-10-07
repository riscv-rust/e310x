//! On-board SPI Flash

use e310x_hal::clock::Clocks;
use e310x_hal::e310x::Qspi0;

#[cfg(target_arch = "riscv32")]
core::arch::global_asm!(
    r#"
    .cfi_sections .debug_frame

    .section .data._setup_is25lp
    .global _setup_is25lp
    .cfi_startproc
    _setup_is25lp:
        li  a1, 0x10014000  // QSPI0 base address

        // Disable mapped region
        sw  zero,96(a1)  // fctrl.en = 0

        // Construct ffmt value for 4 dummy cycles
        li  a2, 0x00BB1447

        beqz a0, 2f

        // We need to set 8 dummy cycles instead of 4.
        // Issue a "Set Read Parameters" command.

        li  a0,2
        sw  a0,24(a1)  // csmode = HOLD
        li  a0,0xC0
        sw  a0,72(a1)  // txdata = 0xC0
        li  a0,0xF0
        sw  a0,72(a1)  // txdata = 0xF0
        sw  zero,24(a1)  // csmode = AUTO

        // Discard two response bytes
    1:  lw  a0,76(a1)
        bltz a0,1b
    1:  lw  a0,76(a1)
        bltz a0,1b

        addi a2,a2,0x40  // ffmt: 4 -> 8 dummy cycles
    2:
        sw  a2,100(a1)  // Write ffmt

        // Enable mapped region
        li  a0, 1
        sw	a0,96(a1)  // fctrl.en = 1
        ret

    .cfi_endproc
    .size _setup_is25lp, . - _setup_is25lp
    "#
);

/// Configure SPI Flash interface to maximum supported speed
#[inline(always)]
pub fn configure_spi_flash(qspi: &Qspi0, clocks: &Clocks) {
    unsafe {
        extern "C" {
            fn _setup_is25lp(dummy8: bool);
        }

        if clocks.coreclk().0 <= 208_000_000 {
            _setup_is25lp(false)
        } else {
            _setup_is25lp(true)
        }
    }
    qspi.sckdiv().modify(|_, w| unsafe { w.div().bits(0) });
}
