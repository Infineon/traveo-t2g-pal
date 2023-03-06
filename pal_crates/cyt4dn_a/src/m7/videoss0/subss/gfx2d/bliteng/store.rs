#[doc = r"Register block"]
#[repr(C)]
pub struct STORE {
    #[doc = "0x00 - Store unit static control register."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Burst Buffer setup register."]
    pub burstbuffermanagement: BURSTBUFFERMANAGEMENT,
    #[doc = "0x08 - Ring buffer setup for destination."]
    pub ringbufstartaddr: RINGBUFSTARTADDR,
    #[doc = "0x0c - Ring buffer setup for destination."]
    pub ringbufwrapaddr: RINGBUFWRAPADDR,
    #[doc = "0x10 - Destination buffer base address."]
    pub baseaddress: BASEADDRESS,
    #[doc = "0x14 - Destination buffer attributes."]
    pub destinationbufferattributes: DESTINATIONBUFFERATTRIBUTES,
    #[doc = "0x18 - Destination buffer dimension."]
    pub destinationbufferdimension: DESTINATIONBUFFERDIMENSION,
    #[doc = "0x1c - Offset between destination frame and buffer."]
    pub frameoffset: FRAMEOFFSET,
    #[doc = "0x20 - Color component size of destination buffer"]
    pub colorcomponentbits: COLORCOMPONENTBITS,
    #[doc = "0x24 - Color component offset of destination buffer."]
    pub colorcomponentshift: COLORCOMPONENTSHIFT,
    #[doc = "0x28 - Store unit dynamic control register"]
    pub control: CONTROL,
    #[doc = "0x2c - Control options for RLAD compression."]
    pub encodecontrol: ENCODECONTROL,
    #[doc = "0x30 - Destination buffer length for compressed data."]
    pub destinationbufferlength: DESTINATIONBUFFERLENGTH,
    #[doc = "0x34 - Store unit start register"]
    pub start: START,
    #[doc = "0x38 - Status information of the RLAD encoder."]
    pub encoderstatus: ENCODERSTATUS,
    #[doc = "0x3c - Ring buffer synchronization."]
    pub writeaddress: WRITEADDRESS,
    #[doc = "0x40 - Ring buffer synchronization."]
    pub frameproperties: FRAMEPROPERTIES,
    #[doc = "0x44 - Burst Buffer Property register"]
    pub burstbufferproperties: BURSTBUFFERPROPERTIES,
    #[doc = "0x48 - Shows the last control word received"]
    pub lastcontrolword: LASTCONTROLWORD,
    #[doc = "0x4c - Performance counter result"]
    pub perfcounter: PERFCOUNTER,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Store unit static control register."]
pub mod staticcontrol;
#[doc = "BURSTBUFFERMANAGEMENT (rw) register accessor: an alias for `Reg<BURSTBUFFERMANAGEMENT_SPEC>`"]
pub type BURSTBUFFERMANAGEMENT = crate::Reg<burstbuffermanagement::BURSTBUFFERMANAGEMENT_SPEC>;
#[doc = "Burst Buffer setup register."]
pub mod burstbuffermanagement;
#[doc = "RINGBUFSTARTADDR (rw) register accessor: an alias for `Reg<RINGBUFSTARTADDR_SPEC>`"]
pub type RINGBUFSTARTADDR = crate::Reg<ringbufstartaddr::RINGBUFSTARTADDR_SPEC>;
#[doc = "Ring buffer setup for destination."]
pub mod ringbufstartaddr;
#[doc = "RINGBUFWRAPADDR (rw) register accessor: an alias for `Reg<RINGBUFWRAPADDR_SPEC>`"]
pub type RINGBUFWRAPADDR = crate::Reg<ringbufwrapaddr::RINGBUFWRAPADDR_SPEC>;
#[doc = "Ring buffer setup for destination."]
pub mod ringbufwrapaddr;
#[doc = "BASEADDRESS (rw) register accessor: an alias for `Reg<BASEADDRESS_SPEC>`"]
pub type BASEADDRESS = crate::Reg<baseaddress::BASEADDRESS_SPEC>;
#[doc = "Destination buffer base address."]
pub mod baseaddress;
#[doc = "DESTINATIONBUFFERATTRIBUTES (rw) register accessor: an alias for `Reg<DESTINATIONBUFFERATTRIBUTES_SPEC>`"]
pub type DESTINATIONBUFFERATTRIBUTES =
    crate::Reg<destinationbufferattributes::DESTINATIONBUFFERATTRIBUTES_SPEC>;
#[doc = "Destination buffer attributes."]
pub mod destinationbufferattributes;
#[doc = "DESTINATIONBUFFERDIMENSION (rw) register accessor: an alias for `Reg<DESTINATIONBUFFERDIMENSION_SPEC>`"]
pub type DESTINATIONBUFFERDIMENSION =
    crate::Reg<destinationbufferdimension::DESTINATIONBUFFERDIMENSION_SPEC>;
#[doc = "Destination buffer dimension."]
pub mod destinationbufferdimension;
#[doc = "FRAMEOFFSET (rw) register accessor: an alias for `Reg<FRAMEOFFSET_SPEC>`"]
pub type FRAMEOFFSET = crate::Reg<frameoffset::FRAMEOFFSET_SPEC>;
#[doc = "Offset between destination frame and buffer."]
pub mod frameoffset;
#[doc = "COLORCOMPONENTBITS (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS_SPEC>`"]
pub type COLORCOMPONENTBITS = crate::Reg<colorcomponentbits::COLORCOMPONENTBITS_SPEC>;
#[doc = "Color component size of destination buffer"]
pub mod colorcomponentbits;
#[doc = "COLORCOMPONENTSHIFT (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT_SPEC>`"]
pub type COLORCOMPONENTSHIFT = crate::Reg<colorcomponentshift::COLORCOMPONENTSHIFT_SPEC>;
#[doc = "Color component offset of destination buffer."]
pub mod colorcomponentshift;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Store unit dynamic control register"]
pub mod control;
#[doc = "ENCODECONTROL (rw) register accessor: an alias for `Reg<ENCODECONTROL_SPEC>`"]
pub type ENCODECONTROL = crate::Reg<encodecontrol::ENCODECONTROL_SPEC>;
#[doc = "Control options for RLAD compression."]
pub mod encodecontrol;
#[doc = "DESTINATIONBUFFERLENGTH (rw) register accessor: an alias for `Reg<DESTINATIONBUFFERLENGTH_SPEC>`"]
pub type DESTINATIONBUFFERLENGTH =
    crate::Reg<destinationbufferlength::DESTINATIONBUFFERLENGTH_SPEC>;
#[doc = "Destination buffer length for compressed data."]
pub mod destinationbufferlength;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Store unit start register"]
pub mod start;
#[doc = "ENCODERSTATUS (rw) register accessor: an alias for `Reg<ENCODERSTATUS_SPEC>`"]
pub type ENCODERSTATUS = crate::Reg<encoderstatus::ENCODERSTATUS_SPEC>;
#[doc = "Status information of the RLAD encoder."]
pub mod encoderstatus;
#[doc = "WRITEADDRESS (rw) register accessor: an alias for `Reg<WRITEADDRESS_SPEC>`"]
pub type WRITEADDRESS = crate::Reg<writeaddress::WRITEADDRESS_SPEC>;
#[doc = "Ring buffer synchronization."]
pub mod writeaddress;
#[doc = "FRAMEPROPERTIES (rw) register accessor: an alias for `Reg<FRAMEPROPERTIES_SPEC>`"]
pub type FRAMEPROPERTIES = crate::Reg<frameproperties::FRAMEPROPERTIES_SPEC>;
#[doc = "Ring buffer synchronization."]
pub mod frameproperties;
#[doc = "BURSTBUFFERPROPERTIES (r) register accessor: an alias for `Reg<BURSTBUFFERPROPERTIES_SPEC>`"]
pub type BURSTBUFFERPROPERTIES = crate::Reg<burstbufferproperties::BURSTBUFFERPROPERTIES_SPEC>;
#[doc = "Burst Buffer Property register"]
pub mod burstbufferproperties;
#[doc = "LASTCONTROLWORD (r) register accessor: an alias for `Reg<LASTCONTROLWORD_SPEC>`"]
pub type LASTCONTROLWORD = crate::Reg<lastcontrolword::LASTCONTROLWORD_SPEC>;
#[doc = "Shows the last control word received"]
pub mod lastcontrolword;
#[doc = "PERFCOUNTER (r) register accessor: an alias for `Reg<PERFCOUNTER_SPEC>`"]
pub type PERFCOUNTER = crate::Reg<perfcounter::PERFCOUNTER_SPEC>;
#[doc = "Performance counter result"]
pub mod perfcounter;
