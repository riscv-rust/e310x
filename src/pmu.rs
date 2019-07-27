//! PMU Extension
#![allow(missing_docs)]
use e310x::{PMU, RTC};

/// value required written to pmukey register before writing to other PMU registers
pub const PMU_KEY_VAL: u32 = 0x51F15E;

const DEFAULT_SLEEP_PROGRAM: [u32; 8] = [
    0x2F0, // assert corerst
    0x3F0, // assert hfclkrst
    0x3D0, // deassert pmu_out_1
    0x3C0, // deassert pmu_out_0
    0x3C0, // repeats
    0x3C0,
    0x3C0,
    0x3C0,
];

const DEFAULT_WAKE_PROGRAM: [u32; 8] = [
    0x3F0, // assert all resets and enable all power supplies
    0x2F8, // idle 2^8 cycles, then deassert hfclkrst
    0x030, // deassert corerst and padrst
    0x030, // repeats
    0x030, 
    0x030,
    0x030,
    0x030,
];

pub trait PMUExt {
    fn configure(&self) -> PMUCfg;
}

impl PMUExt for PMU {
    fn configure(&self) -> PMUCfg {
        PMUCfg {
            _0: ()
        }
    }
}

pub struct PMUCfg {
    _0: (),
}

impl PMUCfg {
    ///
    /// Resets SLEEP and WAKE programs on the PMU to defaults
    /// 
    /// *Registers*
    /// - pmukey
    /// - pmusleeppm
    /// - pmuwakepm
    /// 
    pub fn load_default_programs(&self) {
        unsafe {
            for i in 0..8 {
                (*PMU::ptr()).pmukey.write(|w| w.bits(PMU_KEY_VAL));
                (*PMU::ptr()).pmusleeppm[i].write(|w| w.bits(DEFAULT_SLEEP_PROGRAM[i]));

                (*PMU::ptr()).pmukey.write(|w| w.bits(PMU_KEY_VAL));
                (*PMU::ptr()).pmuwakepm[i].write(|w| w.bits(DEFAULT_WAKE_PROGRAM[i]));
            }
        }
    }

    ///
    /// Puts device to sleep for N seconds, allowing wake-up button to wake it up as well
    /// 
    /// *Registers*
    /// - pmukey
    /// - pmuie
    /// - pmusleep
    /// - rtccfg
    /// - rtccmp
    /// 
    pub fn sleep(&self, sleep_time: u32) {
        unsafe {
            // set interrupt source to RTC enabled, each pmu register needs key set before write
            (*PMU::ptr()).pmukey.write(|w| w.bits(PMU_KEY_VAL));
            (*PMU::ptr()).pmuie.write(|w| w.rtc().set_bit().dwakeup().set_bit());
            // set RTC clock scale to once per second for easy calculation
            (*RTC::ptr()).rtccfg.write(|w| w.enalways().set_bit().scale().bits(15));
            // get current RTC clock value scaled
            let rtc_now = (*RTC::ptr()).rtcs.read().bits();
            // set RTC clock comparator
            (*RTC::ptr()).rtccmp.write(|w| w.bits(rtc_now + sleep_time));
            // go to sleep for sleep_time seconds, need to set pmukey here as well
            (*PMU::ptr()).pmukey.write(|w| w.bits(PMU_KEY_VAL));
            (*PMU::ptr()).pmusleep.write(|w| w.sleep().set_bit());
        }
    }
}
