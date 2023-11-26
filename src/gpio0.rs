#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    input_val: INPUT_VAL,
    input_en: INPUT_EN,
    output_en: OUTPUT_EN,
    output_val: OUTPUT_VAL,
    pullup: PULLUP,
    drive: DRIVE,
    rise_ie: RISE_IE,
    rise_ip: RISE_IP,
    fall_ie: FALL_IE,
    fall_ip: FALL_IP,
    high_ie: HIGH_IE,
    high_ip: HIGH_IP,
    low_ie: LOW_IE,
    low_ip: LOW_IP,
    iof_en: IOF_EN,
    iof_sel: IOF_SEL,
    out_xor: OUT_XOR,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin value."]
    #[inline(always)]
    pub const fn input_val(&self) -> &INPUT_VAL {
        &self.input_val
    }
    #[doc = "0x04 - Pin Input Enable Register"]
    #[inline(always)]
    pub const fn input_en(&self) -> &INPUT_EN {
        &self.input_en
    }
    #[doc = "0x08 - Pin Output Enable Register"]
    #[inline(always)]
    pub const fn output_en(&self) -> &OUTPUT_EN {
        &self.output_en
    }
    #[doc = "0x0c - Output Port Value Register"]
    #[inline(always)]
    pub const fn output_val(&self) -> &OUTPUT_VAL {
        &self.output_val
    }
    #[doc = "0x10 - Internal Pull-Up Enable Register"]
    #[inline(always)]
    pub const fn pullup(&self) -> &PULLUP {
        &self.pullup
    }
    #[doc = "0x14 - Drive Strength Register"]
    #[inline(always)]
    pub const fn drive(&self) -> &DRIVE {
        &self.drive
    }
    #[doc = "0x18 - Rise Interrupt Enable Register"]
    #[inline(always)]
    pub const fn rise_ie(&self) -> &RISE_IE {
        &self.rise_ie
    }
    #[doc = "0x1c - Rise Interrupt Pending Register"]
    #[inline(always)]
    pub const fn rise_ip(&self) -> &RISE_IP {
        &self.rise_ip
    }
    #[doc = "0x20 - Fall Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fall_ie(&self) -> &FALL_IE {
        &self.fall_ie
    }
    #[doc = "0x24 - Fall Interrupt Pending Register"]
    #[inline(always)]
    pub const fn fall_ip(&self) -> &FALL_IP {
        &self.fall_ip
    }
    #[doc = "0x28 - High Interrupt Enable Register"]
    #[inline(always)]
    pub const fn high_ie(&self) -> &HIGH_IE {
        &self.high_ie
    }
    #[doc = "0x2c - High Interrupt Pending Register"]
    #[inline(always)]
    pub const fn high_ip(&self) -> &HIGH_IP {
        &self.high_ip
    }
    #[doc = "0x30 - Low Interrupt Enable Register"]
    #[inline(always)]
    pub const fn low_ie(&self) -> &LOW_IE {
        &self.low_ie
    }
    #[doc = "0x34 - Low Interrupt Pending Register"]
    #[inline(always)]
    pub const fn low_ip(&self) -> &LOW_IP {
        &self.low_ip
    }
    #[doc = "0x38 - HW I/O Function Enable Register"]
    #[inline(always)]
    pub const fn iof_en(&self) -> &IOF_EN {
        &self.iof_en
    }
    #[doc = "0x3c - HW I/O Function Select Register"]
    #[inline(always)]
    pub const fn iof_sel(&self) -> &IOF_SEL {
        &self.iof_sel
    }
    #[doc = "0x40 - Output XOR (invert) Register"]
    #[inline(always)]
    pub const fn out_xor(&self) -> &OUT_XOR {
        &self.out_xor
    }
}
#[doc = "input_val (rw) register accessor: Pin value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input_val`]
module"]
pub type INPUT_VAL = crate::Reg<input_val::INPUT_VAL_SPEC>;
#[doc = "Pin value."]
pub mod input_val;
#[doc = "input_en (rw) register accessor: Pin Input Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input_en`]
module"]
pub type INPUT_EN = crate::Reg<input_en::INPUT_EN_SPEC>;
#[doc = "Pin Input Enable Register"]
pub mod input_en;
#[doc = "output_en (rw) register accessor: Pin Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_en`]
module"]
pub type OUTPUT_EN = crate::Reg<output_en::OUTPUT_EN_SPEC>;
#[doc = "Pin Output Enable Register"]
pub mod output_en;
#[doc = "output_val (rw) register accessor: Output Port Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_val`]
module"]
pub type OUTPUT_VAL = crate::Reg<output_val::OUTPUT_VAL_SPEC>;
#[doc = "Output Port Value Register"]
pub mod output_val;
#[doc = "pullup (rw) register accessor: Internal Pull-Up Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pullup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pullup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pullup`]
module"]
pub type PULLUP = crate::Reg<pullup::PULLUP_SPEC>;
#[doc = "Internal Pull-Up Enable Register"]
pub mod pullup;
#[doc = "drive (rw) register accessor: Drive Strength Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drive::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drive::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drive`]
module"]
pub type DRIVE = crate::Reg<drive::DRIVE_SPEC>;
#[doc = "Drive Strength Register"]
pub mod drive;
#[doc = "rise_ie (rw) register accessor: Rise Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rise_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rise_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rise_ie`]
module"]
pub type RISE_IE = crate::Reg<rise_ie::RISE_IE_SPEC>;
#[doc = "Rise Interrupt Enable Register"]
pub mod rise_ie;
#[doc = "rise_ip (rw) register accessor: Rise Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rise_ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rise_ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rise_ip`]
module"]
pub type RISE_IP = crate::Reg<rise_ip::RISE_IP_SPEC>;
#[doc = "Rise Interrupt Pending Register"]
pub mod rise_ip;
#[doc = "fall_ie (rw) register accessor: Fall Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fall_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fall_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fall_ie`]
module"]
pub type FALL_IE = crate::Reg<fall_ie::FALL_IE_SPEC>;
#[doc = "Fall Interrupt Enable Register"]
pub mod fall_ie;
#[doc = "fall_ip (rw) register accessor: Fall Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fall_ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fall_ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fall_ip`]
module"]
pub type FALL_IP = crate::Reg<fall_ip::FALL_IP_SPEC>;
#[doc = "Fall Interrupt Pending Register"]
pub mod fall_ip;
#[doc = "high_ie (rw) register accessor: High Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_ie`]
module"]
pub type HIGH_IE = crate::Reg<high_ie::HIGH_IE_SPEC>;
#[doc = "High Interrupt Enable Register"]
pub mod high_ie;
#[doc = "high_ip (rw) register accessor: High Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_ip`]
module"]
pub type HIGH_IP = crate::Reg<high_ip::HIGH_IP_SPEC>;
#[doc = "High Interrupt Pending Register"]
pub mod high_ip;
#[doc = "low_ie (rw) register accessor: Low Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_ie`]
module"]
pub type LOW_IE = crate::Reg<low_ie::LOW_IE_SPEC>;
#[doc = "Low Interrupt Enable Register"]
pub mod low_ie;
#[doc = "low_ip (rw) register accessor: Low Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_ip`]
module"]
pub type LOW_IP = crate::Reg<low_ip::LOW_IP_SPEC>;
#[doc = "Low Interrupt Pending Register"]
pub mod low_ip;
#[doc = "iof_en (rw) register accessor: HW I/O Function Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iof_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iof_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iof_en`]
module"]
pub type IOF_EN = crate::Reg<iof_en::IOF_EN_SPEC>;
#[doc = "HW I/O Function Enable Register"]
pub mod iof_en;
#[doc = "iof_sel (rw) register accessor: HW I/O Function Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iof_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iof_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iof_sel`]
module"]
pub type IOF_SEL = crate::Reg<iof_sel::IOF_SEL_SPEC>;
#[doc = "HW I/O Function Select Register"]
pub mod iof_sel;
#[doc = "out_xor (rw) register accessor: Output XOR (invert) Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_xor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_xor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xor`]
module"]
pub type OUT_XOR = crate::Reg<out_xor::OUT_XOR_SPEC>;
#[doc = "Output XOR (invert) Register"]
pub mod out_xor;
