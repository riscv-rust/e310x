#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin value."]
    pub input_val: INPUT_VAL,
    #[doc = "0x04 - Pin Input Enable Register"]
    pub input_en: INPUT_EN,
    #[doc = "0x08 - Pin Output Enable Register"]
    pub output_en: OUTPUT_EN,
    #[doc = "0x0c - Output Port Value Register"]
    pub output_val: OUTPUT_VAL,
    #[doc = "0x10 - Internal Pull-Up Enable Register"]
    pub pullup: PULLUP,
    #[doc = "0x14 - Drive Strength Register"]
    pub drive: DRIVE,
    #[doc = "0x18 - Rise Interrupt Enable Register"]
    pub rise_ie: RISE_IE,
    #[doc = "0x1c - Rise Interrupt Pending Register"]
    pub rise_ip: RISE_IP,
    #[doc = "0x20 - Fall Interrupt Enable Register"]
    pub fall_ie: FALL_IE,
    #[doc = "0x24 - Fall Interrupt Pending Register"]
    pub fall_ip: FALL_IP,
    #[doc = "0x28 - High Interrupt Enable Register"]
    pub high_ie: HIGH_IE,
    #[doc = "0x2c - High Interrupt Pending Register"]
    pub high_ip: HIGH_IP,
    #[doc = "0x30 - Low Interrupt Enable Register"]
    pub low_ie: LOW_IE,
    #[doc = "0x34 - Low Interrupt Pending Register"]
    pub low_ip: LOW_IP,
    #[doc = "0x38 - HW I/O Function Enable Register"]
    pub iof_en: IOF_EN,
    #[doc = "0x3c - HW I/O Function Select Register"]
    pub iof_sel: IOF_SEL,
    #[doc = "0x40 - Output XOR (invert) Register"]
    pub out_xor: OUT_XOR,
}
#[doc = "input_val (rw) register accessor: an alias for `Reg<INPUT_VAL_SPEC>`"]
pub type INPUT_VAL = crate::Reg<input_val::INPUT_VAL_SPEC>;
#[doc = "Pin value."]
pub mod input_val;
#[doc = "input_en (rw) register accessor: an alias for `Reg<INPUT_EN_SPEC>`"]
pub type INPUT_EN = crate::Reg<input_en::INPUT_EN_SPEC>;
#[doc = "Pin Input Enable Register"]
pub mod input_en;
#[doc = "output_en (rw) register accessor: an alias for `Reg<OUTPUT_EN_SPEC>`"]
pub type OUTPUT_EN = crate::Reg<output_en::OUTPUT_EN_SPEC>;
#[doc = "Pin Output Enable Register"]
pub mod output_en;
#[doc = "output_val (rw) register accessor: an alias for `Reg<OUTPUT_VAL_SPEC>`"]
pub type OUTPUT_VAL = crate::Reg<output_val::OUTPUT_VAL_SPEC>;
#[doc = "Output Port Value Register"]
pub mod output_val;
#[doc = "pullup (rw) register accessor: an alias for `Reg<PULLUP_SPEC>`"]
pub type PULLUP = crate::Reg<pullup::PULLUP_SPEC>;
#[doc = "Internal Pull-Up Enable Register"]
pub mod pullup;
#[doc = "drive (rw) register accessor: an alias for `Reg<DRIVE_SPEC>`"]
pub type DRIVE = crate::Reg<drive::DRIVE_SPEC>;
#[doc = "Drive Strength Register"]
pub mod drive;
#[doc = "rise_ie (rw) register accessor: an alias for `Reg<RISE_IE_SPEC>`"]
pub type RISE_IE = crate::Reg<rise_ie::RISE_IE_SPEC>;
#[doc = "Rise Interrupt Enable Register"]
pub mod rise_ie;
#[doc = "rise_ip (rw) register accessor: an alias for `Reg<RISE_IP_SPEC>`"]
pub type RISE_IP = crate::Reg<rise_ip::RISE_IP_SPEC>;
#[doc = "Rise Interrupt Pending Register"]
pub mod rise_ip;
#[doc = "fall_ie (rw) register accessor: an alias for `Reg<FALL_IE_SPEC>`"]
pub type FALL_IE = crate::Reg<fall_ie::FALL_IE_SPEC>;
#[doc = "Fall Interrupt Enable Register"]
pub mod fall_ie;
#[doc = "fall_ip (rw) register accessor: an alias for `Reg<FALL_IP_SPEC>`"]
pub type FALL_IP = crate::Reg<fall_ip::FALL_IP_SPEC>;
#[doc = "Fall Interrupt Pending Register"]
pub mod fall_ip;
#[doc = "high_ie (rw) register accessor: an alias for `Reg<HIGH_IE_SPEC>`"]
pub type HIGH_IE = crate::Reg<high_ie::HIGH_IE_SPEC>;
#[doc = "High Interrupt Enable Register"]
pub mod high_ie;
#[doc = "high_ip (rw) register accessor: an alias for `Reg<HIGH_IP_SPEC>`"]
pub type HIGH_IP = crate::Reg<high_ip::HIGH_IP_SPEC>;
#[doc = "High Interrupt Pending Register"]
pub mod high_ip;
#[doc = "low_ie (rw) register accessor: an alias for `Reg<LOW_IE_SPEC>`"]
pub type LOW_IE = crate::Reg<low_ie::LOW_IE_SPEC>;
#[doc = "Low Interrupt Enable Register"]
pub mod low_ie;
#[doc = "low_ip (rw) register accessor: an alias for `Reg<LOW_IP_SPEC>`"]
pub type LOW_IP = crate::Reg<low_ip::LOW_IP_SPEC>;
#[doc = "Low Interrupt Pending Register"]
pub mod low_ip;
#[doc = "iof_en (rw) register accessor: an alias for `Reg<IOF_EN_SPEC>`"]
pub type IOF_EN = crate::Reg<iof_en::IOF_EN_SPEC>;
#[doc = "HW I/O Function Enable Register"]
pub mod iof_en;
#[doc = "iof_sel (rw) register accessor: an alias for `Reg<IOF_SEL_SPEC>`"]
pub type IOF_SEL = crate::Reg<iof_sel::IOF_SEL_SPEC>;
#[doc = "HW I/O Function Select Register"]
pub mod iof_sel;
#[doc = "out_xor (rw) register accessor: an alias for `Reg<OUT_XOR_SPEC>`"]
pub type OUT_XOR = crate::Reg<out_xor::OUT_XOR_SPEC>;
#[doc = "Output XOR (invert) Register"]
pub mod out_xor;
