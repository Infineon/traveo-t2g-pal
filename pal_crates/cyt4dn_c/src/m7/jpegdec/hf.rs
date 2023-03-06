#[doc = r"Register block"]
#[repr(C)]
pub struct HF {
    #[doc = "0x00 - AXI cache attributes."]
    pub axictl: AXICTL,
    #[doc = "0x04 - AXI burst settings."]
    pub burst: BURST,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Source buffer configuration (JPEG)."]
    pub fetchbuf0: FETCHBUF0,
    #[doc = "0x14 - Source buffer configuration (JPEG)."]
    pub fetchbuf1: FETCHBUF1,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - Destination buffer configuration (Y, YUV, RGB)."]
    pub store0buf0: STORE0BUF0,
    #[doc = "0x24 - Destination buffer configuration (Y, YUV, RGB)."]
    pub store0buf1: STORE0BUF1,
    #[doc = "0x28 - Destination buffer configuration (Y, YUV, RGB)."]
    pub store0buf2: STORE0BUF2,
    _reserved7: [u8; 0x14],
    #[doc = "0x40 - Destination buffer configuration (UV)."]
    pub store1buf0: STORE1BUF0,
    #[doc = "0x44 - Destination buffer configuration (UV)."]
    pub store1buf1: STORE1BUF1,
    #[doc = "0x48 - Destination buffer configuration (UV)."]
    pub store1buf2: STORE1BUF2,
    _reserved10: [u8; 0x14],
    #[doc = "0x60 - Destination buffer format settings."]
    pub storectl: STORECTL,
    _reserved11: [u8; 0x0c],
    #[doc = "0x70 - Start/Resume triggers."]
    pub cmd: CMD,
    _reserved12: [u8; 0x0c],
    #[doc = "0x80 - Decoder core decoding end interrupt mask."]
    pub decodeintrmask: DECODEINTRMASK,
    _reserved13: [u8; 0x0c],
    #[doc = "0x90 - Interrupt status bits."]
    pub intr: INTR,
    #[doc = "0x94 - Interrupt set bits."]
    pub intr_set: INTR_SET,
    #[doc = "0x98 - Interrupt mask bits."]
    pub intr_mask: INTR_MASK,
    #[doc = "0x9c - Interrupt masked bits."]
    pub intr_masked: INTR_MASKED,
    _reserved17: [u8; 0x10],
    #[doc = "0xb0 - Error status."]
    pub errorstatus: ERRORSTATUS,
    #[doc = "0xb4 - Operating status."]
    pub operatingstatus: OPERATINGSTATUS,
    #[doc = "0xb8 - Fetch Unit status."]
    pub fetchstatus: FETCHSTATUS,
    #[doc = "0xbc - Store Unit status."]
    pub storestatus: STORESTATUS,
}
#[doc = "AXICTL (rw) register accessor: an alias for `Reg<AXICTL_SPEC>`"]
pub type AXICTL = crate::Reg<axictl::AXICTL_SPEC>;
#[doc = "AXI cache attributes."]
pub mod axictl;
#[doc = "BURST (rw) register accessor: an alias for `Reg<BURST_SPEC>`"]
pub type BURST = crate::Reg<burst::BURST_SPEC>;
#[doc = "AXI burst settings."]
pub mod burst;
#[doc = "FETCHBUF0 (rw) register accessor: an alias for `Reg<FETCHBUF0_SPEC>`"]
pub type FETCHBUF0 = crate::Reg<fetchbuf0::FETCHBUF0_SPEC>;
#[doc = "Source buffer configuration (JPEG)."]
pub mod fetchbuf0;
#[doc = "FETCHBUF1 (rw) register accessor: an alias for `Reg<FETCHBUF1_SPEC>`"]
pub type FETCHBUF1 = crate::Reg<fetchbuf1::FETCHBUF1_SPEC>;
#[doc = "Source buffer configuration (JPEG)."]
pub mod fetchbuf1;
#[doc = "STORE0BUF0 (rw) register accessor: an alias for `Reg<STORE0BUF0_SPEC>`"]
pub type STORE0BUF0 = crate::Reg<store0buf0::STORE0BUF0_SPEC>;
#[doc = "Destination buffer configuration (Y, YUV, RGB)."]
pub mod store0buf0;
#[doc = "STORE0BUF1 (rw) register accessor: an alias for `Reg<STORE0BUF1_SPEC>`"]
pub type STORE0BUF1 = crate::Reg<store0buf1::STORE0BUF1_SPEC>;
#[doc = "Destination buffer configuration (Y, YUV, RGB)."]
pub mod store0buf1;
#[doc = "STORE0BUF2 (rw) register accessor: an alias for `Reg<STORE0BUF2_SPEC>`"]
pub type STORE0BUF2 = crate::Reg<store0buf2::STORE0BUF2_SPEC>;
#[doc = "Destination buffer configuration (Y, YUV, RGB)."]
pub mod store0buf2;
#[doc = "STORE1BUF0 (rw) register accessor: an alias for `Reg<STORE1BUF0_SPEC>`"]
pub type STORE1BUF0 = crate::Reg<store1buf0::STORE1BUF0_SPEC>;
#[doc = "Destination buffer configuration (UV)."]
pub mod store1buf0;
#[doc = "STORE1BUF1 (rw) register accessor: an alias for `Reg<STORE1BUF1_SPEC>`"]
pub type STORE1BUF1 = crate::Reg<store1buf1::STORE1BUF1_SPEC>;
#[doc = "Destination buffer configuration (UV)."]
pub mod store1buf1;
#[doc = "STORE1BUF2 (rw) register accessor: an alias for `Reg<STORE1BUF2_SPEC>`"]
pub type STORE1BUF2 = crate::Reg<store1buf2::STORE1BUF2_SPEC>;
#[doc = "Destination buffer configuration (UV)."]
pub mod store1buf2;
#[doc = "STORECTL (rw) register accessor: an alias for `Reg<STORECTL_SPEC>`"]
pub type STORECTL = crate::Reg<storectl::STORECTL_SPEC>;
#[doc = "Destination buffer format settings."]
pub mod storectl;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Start/Resume triggers."]
pub mod cmd;
#[doc = "DECODEINTRMASK (rw) register accessor: an alias for `Reg<DECODEINTRMASK_SPEC>`"]
pub type DECODEINTRMASK = crate::Reg<decodeintrmask::DECODEINTRMASK_SPEC>;
#[doc = "Decoder core decoding end interrupt mask."]
pub mod decodeintrmask;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt status bits."]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set bits."]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask bits."]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked bits."]
pub mod intr_masked;
#[doc = "ERRORSTATUS (r) register accessor: an alias for `Reg<ERRORSTATUS_SPEC>`"]
pub type ERRORSTATUS = crate::Reg<errorstatus::ERRORSTATUS_SPEC>;
#[doc = "Error status."]
pub mod errorstatus;
#[doc = "OPERATINGSTATUS (r) register accessor: an alias for `Reg<OPERATINGSTATUS_SPEC>`"]
pub type OPERATINGSTATUS = crate::Reg<operatingstatus::OPERATINGSTATUS_SPEC>;
#[doc = "Operating status."]
pub mod operatingstatus;
#[doc = "FETCHSTATUS (r) register accessor: an alias for `Reg<FETCHSTATUS_SPEC>`"]
pub type FETCHSTATUS = crate::Reg<fetchstatus::FETCHSTATUS_SPEC>;
#[doc = "Fetch Unit status."]
pub mod fetchstatus;
#[doc = "STORESTATUS (r) register accessor: an alias for `Reg<STORESTATUS_SPEC>`"]
pub type STORESTATUS = crate::Reg<storestatus::STORESTATUS_SPEC>;
#[doc = "Store Unit status."]
pub mod storestatus;
