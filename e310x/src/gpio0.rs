#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    input_val: InputVal,
    input_en: InputEn,
    output_en: OutputEn,
    output_val: OutputVal,
    pullup: Pullup,
    drive: Drive,
    rise_ie: RiseIe,
    rise_ip: RiseIp,
    fall_ie: FallIe,
    fall_ip: FallIp,
    high_ie: HighIe,
    high_ip: HighIp,
    low_ie: LowIe,
    low_ip: LowIp,
    iof_en: IofEn,
    iof_sel: IofSel,
    out_xor: OutXor,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin value."]
    #[inline(always)]
    pub const fn input_val(&self) -> &InputVal {
        &self.input_val
    }
    #[doc = "0x04 - Pin Input Enable Register"]
    #[inline(always)]
    pub const fn input_en(&self) -> &InputEn {
        &self.input_en
    }
    #[doc = "0x08 - Pin Output Enable Register"]
    #[inline(always)]
    pub const fn output_en(&self) -> &OutputEn {
        &self.output_en
    }
    #[doc = "0x0c - Output Port Value Register"]
    #[inline(always)]
    pub const fn output_val(&self) -> &OutputVal {
        &self.output_val
    }
    #[doc = "0x10 - Internal Pull-Up Enable Register"]
    #[inline(always)]
    pub const fn pullup(&self) -> &Pullup {
        &self.pullup
    }
    #[doc = "0x14 - Drive Strength Register"]
    #[inline(always)]
    pub const fn drive(&self) -> &Drive {
        &self.drive
    }
    #[doc = "0x18 - Rise Interrupt Enable Register"]
    #[inline(always)]
    pub const fn rise_ie(&self) -> &RiseIe {
        &self.rise_ie
    }
    #[doc = "0x1c - Rise Interrupt Pending Register"]
    #[inline(always)]
    pub const fn rise_ip(&self) -> &RiseIp {
        &self.rise_ip
    }
    #[doc = "0x20 - Fall Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fall_ie(&self) -> &FallIe {
        &self.fall_ie
    }
    #[doc = "0x24 - Fall Interrupt Pending Register"]
    #[inline(always)]
    pub const fn fall_ip(&self) -> &FallIp {
        &self.fall_ip
    }
    #[doc = "0x28 - High Interrupt Enable Register"]
    #[inline(always)]
    pub const fn high_ie(&self) -> &HighIe {
        &self.high_ie
    }
    #[doc = "0x2c - High Interrupt Pending Register"]
    #[inline(always)]
    pub const fn high_ip(&self) -> &HighIp {
        &self.high_ip
    }
    #[doc = "0x30 - Low Interrupt Enable Register"]
    #[inline(always)]
    pub const fn low_ie(&self) -> &LowIe {
        &self.low_ie
    }
    #[doc = "0x34 - Low Interrupt Pending Register"]
    #[inline(always)]
    pub const fn low_ip(&self) -> &LowIp {
        &self.low_ip
    }
    #[doc = "0x38 - HW I/O Function Enable Register"]
    #[inline(always)]
    pub const fn iof_en(&self) -> &IofEn {
        &self.iof_en
    }
    #[doc = "0x3c - HW I/O Function Select Register"]
    #[inline(always)]
    pub const fn iof_sel(&self) -> &IofSel {
        &self.iof_sel
    }
    #[doc = "0x40 - Output XOR (invert) Register"]
    #[inline(always)]
    pub const fn out_xor(&self) -> &OutXor {
        &self.out_xor
    }
}
#[doc = "input_val (rw) register accessor: Pin value.\n\nYou can [`read`](crate::Reg::read) this register and get [`input_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input_val`]
module"]
#[doc(alias = "input_val")]
pub type InputVal = crate::Reg<input_val::InputValSpec>;
#[doc = "Pin value."]
pub mod input_val;
#[doc = "input_en (rw) register accessor: Pin Input Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input_en`]
module"]
#[doc(alias = "input_en")]
pub type InputEn = crate::Reg<input_en::InputEnSpec>;
#[doc = "Pin Input Enable Register"]
pub mod input_en;
#[doc = "output_en (rw) register accessor: Pin Output Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`output_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_en`]
module"]
#[doc(alias = "output_en")]
pub type OutputEn = crate::Reg<output_en::OutputEnSpec>;
#[doc = "Pin Output Enable Register"]
pub mod output_en;
#[doc = "output_val (rw) register accessor: Output Port Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`output_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_val`]
module"]
#[doc(alias = "output_val")]
pub type OutputVal = crate::Reg<output_val::OutputValSpec>;
#[doc = "Output Port Value Register"]
pub mod output_val;
#[doc = "pullup (rw) register accessor: Internal Pull-Up Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pullup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pullup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pullup`]
module"]
#[doc(alias = "pullup")]
pub type Pullup = crate::Reg<pullup::PullupSpec>;
#[doc = "Internal Pull-Up Enable Register"]
pub mod pullup;
#[doc = "drive (rw) register accessor: Drive Strength Register\n\nYou can [`read`](crate::Reg::read) this register and get [`drive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drive`]
module"]
#[doc(alias = "drive")]
pub type Drive = crate::Reg<drive::DriveSpec>;
#[doc = "Drive Strength Register"]
pub mod drive;
#[doc = "rise_ie (rw) register accessor: Rise Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rise_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rise_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rise_ie`]
module"]
#[doc(alias = "rise_ie")]
pub type RiseIe = crate::Reg<rise_ie::RiseIeSpec>;
#[doc = "Rise Interrupt Enable Register"]
pub mod rise_ie;
#[doc = "rise_ip (rw) register accessor: Rise Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rise_ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rise_ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rise_ip`]
module"]
#[doc(alias = "rise_ip")]
pub type RiseIp = crate::Reg<rise_ip::RiseIpSpec>;
#[doc = "Rise Interrupt Pending Register"]
pub mod rise_ip;
#[doc = "fall_ie (rw) register accessor: Fall Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fall_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fall_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fall_ie`]
module"]
#[doc(alias = "fall_ie")]
pub type FallIe = crate::Reg<fall_ie::FallIeSpec>;
#[doc = "Fall Interrupt Enable Register"]
pub mod fall_ie;
#[doc = "fall_ip (rw) register accessor: Fall Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fall_ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fall_ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fall_ip`]
module"]
#[doc(alias = "fall_ip")]
pub type FallIp = crate::Reg<fall_ip::FallIpSpec>;
#[doc = "Fall Interrupt Pending Register"]
pub mod fall_ip;
#[doc = "high_ie (rw) register accessor: High Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`high_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_ie`]
module"]
#[doc(alias = "high_ie")]
pub type HighIe = crate::Reg<high_ie::HighIeSpec>;
#[doc = "High Interrupt Enable Register"]
pub mod high_ie;
#[doc = "high_ip (rw) register accessor: High Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`high_ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high_ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_ip`]
module"]
#[doc(alias = "high_ip")]
pub type HighIp = crate::Reg<high_ip::HighIpSpec>;
#[doc = "High Interrupt Pending Register"]
pub mod high_ip;
#[doc = "low_ie (rw) register accessor: Low Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`low_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_ie`]
module"]
#[doc(alias = "low_ie")]
pub type LowIe = crate::Reg<low_ie::LowIeSpec>;
#[doc = "Low Interrupt Enable Register"]
pub mod low_ie;
#[doc = "low_ip (rw) register accessor: Low Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`low_ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_ip`]
module"]
#[doc(alias = "low_ip")]
pub type LowIp = crate::Reg<low_ip::LowIpSpec>;
#[doc = "Low Interrupt Pending Register"]
pub mod low_ip;
#[doc = "iof_en (rw) register accessor: HW I/O Function Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iof_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iof_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iof_en`]
module"]
#[doc(alias = "iof_en")]
pub type IofEn = crate::Reg<iof_en::IofEnSpec>;
#[doc = "HW I/O Function Enable Register"]
pub mod iof_en;
#[doc = "iof_sel (rw) register accessor: HW I/O Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iof_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iof_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iof_sel`]
module"]
#[doc(alias = "iof_sel")]
pub type IofSel = crate::Reg<iof_sel::IofSelSpec>;
#[doc = "HW I/O Function Select Register"]
pub mod iof_sel;
#[doc = "out_xor (rw) register accessor: Output XOR (invert) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_xor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_xor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xor`]
module"]
#[doc(alias = "out_xor")]
pub type OutXor = crate::Reg<out_xor::OutXorSpec>;
#[doc = "Output XOR (invert) Register"]
pub mod out_xor;
