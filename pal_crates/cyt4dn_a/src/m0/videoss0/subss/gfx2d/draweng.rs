#[doc = r"Register block"]
#[repr(C)]
pub struct DRAWENG {
    #[doc = "0x00 - Drawing Engine static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Burst Buffer setup register"]
    pub staticburstbuffermanagement: STATICBURSTBUFFERMANAGEMENT,
    #[doc = "0x08 - Lowest command buffer address"]
    pub staticcmdbufferaddress: STATICCMDBUFFERADDRESS,
    #[doc = "0x0c - Command buffer size"]
    pub staticcmdbuffersize: STATICCMDBUFFERSIZE,
    #[doc = "0x10 - Buffer end address of command list"]
    pub cmdbufferwriteptr: CMDBUFFERWRITEPTR,
    #[doc = "0x14 - Component A11 of transformation Matrix A"]
    pub transformationmatrixa11: TRANSFORMATIONMATRIXA11,
    #[doc = "0x18 - Component A21 of transformation Matrix A"]
    pub transformationmatrixa21: TRANSFORMATIONMATRIXA21,
    #[doc = "0x1c - Component A12 of transformation Matrix A"]
    pub transformationmatrixa12: TRANSFORMATIONMATRIXA12,
    #[doc = "0x20 - Component A22 of transformation Matrix A"]
    pub transformationmatrixa22: TRANSFORMATIONMATRIXA22,
    #[doc = "0x24 - Horizontal translation of the alpha frame."]
    pub translationvectorx: TRANSLATIONVECTORX,
    #[doc = "0x28 - Vertical translation of the alpha frame."]
    pub translationvectory: TRANSLATIONVECTORY,
    #[doc = "0x2c - Drawing Engine main processing control register"]
    pub processingcontrol: PROCESSINGCONTROL,
    #[doc = "0x30 - Store unit dynamic control register"]
    pub storecontrol: STORECONTROL,
    #[doc = "0x34 - Maximum dimensions of the destination buffer. The resulting alpha frame is limited to this dimensions. However, the active area can be smaller and is given by ActiveDimensions register."]
    pub destinationbufferdimensions: DESTINATIONBUFFERDIMENSIONS,
    #[doc = "0x38 - Destination buffer base address"]
    pub destinationbuffer: DESTINATIONBUFFER,
    #[doc = "0x3c - Drawing Engine start register"]
    pub start: START,
    #[doc = "0x40 - Drawing Engine read acknowledge register"]
    pub readack: READACK,
    #[doc = "0x44 - Buffer end address of command list"]
    pub cmdbufferreadptr: CMDBUFFERREADPTR,
    #[doc = "0x48 - Shows active dimensions of the output alpha frame. This register is aligned to the ClipWindowDimensions register of the fetch."]
    pub activedimensions: ACTIVEDIMENSIONS,
    #[doc = "0x4c - Desired position of the alpha frame for the following Blit operation. This register is aligned to the LayerOffset of the fetch and given as signed 16.0 values."]
    pub layeroffset: LAYEROFFSET,
    #[doc = "0x50 - Error status of finished operation. If register is unequal to zero the resulting raster image is most likely corrupted."]
    pub error: ERROR,
    #[doc = "0x54 - Burst Buffer Property register"]
    pub burstbufferproperties: BURSTBUFFERPROPERTIES,
    #[doc = "0x58 - Status bits."]
    pub status: STATUS,
    #[doc = "0x5c - Performance counter result"]
    pub perfcounterfront: PERFCOUNTERFRONT,
    #[doc = "0x60 - Performance counter result"]
    pub perfcounterback: PERFCOUNTERBACK,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Drawing Engine static control register"]
pub mod staticcontrol;
#[doc = "STATICBURSTBUFFERMANAGEMENT (rw) register accessor: an alias for `Reg<STATICBURSTBUFFERMANAGEMENT_SPEC>`"]
pub type STATICBURSTBUFFERMANAGEMENT =
    crate::Reg<staticburstbuffermanagement::STATICBURSTBUFFERMANAGEMENT_SPEC>;
#[doc = "Burst Buffer setup register"]
pub mod staticburstbuffermanagement;
#[doc = "STATICCMDBUFFERADDRESS (rw) register accessor: an alias for `Reg<STATICCMDBUFFERADDRESS_SPEC>`"]
pub type STATICCMDBUFFERADDRESS = crate::Reg<staticcmdbufferaddress::STATICCMDBUFFERADDRESS_SPEC>;
#[doc = "Lowest command buffer address"]
pub mod staticcmdbufferaddress;
#[doc = "STATICCMDBUFFERSIZE (rw) register accessor: an alias for `Reg<STATICCMDBUFFERSIZE_SPEC>`"]
pub type STATICCMDBUFFERSIZE = crate::Reg<staticcmdbuffersize::STATICCMDBUFFERSIZE_SPEC>;
#[doc = "Command buffer size"]
pub mod staticcmdbuffersize;
#[doc = "CMDBUFFERWRITEPTR (rw) register accessor: an alias for `Reg<CMDBUFFERWRITEPTR_SPEC>`"]
pub type CMDBUFFERWRITEPTR = crate::Reg<cmdbufferwriteptr::CMDBUFFERWRITEPTR_SPEC>;
#[doc = "Buffer end address of command list"]
pub mod cmdbufferwriteptr;
#[doc = "TRANSFORMATIONMATRIXA11 (rw) register accessor: an alias for `Reg<TRANSFORMATIONMATRIXA11_SPEC>`"]
pub type TRANSFORMATIONMATRIXA11 =
    crate::Reg<transformationmatrixa11::TRANSFORMATIONMATRIXA11_SPEC>;
#[doc = "Component A11 of transformation Matrix A"]
pub mod transformationmatrixa11;
#[doc = "TRANSFORMATIONMATRIXA21 (rw) register accessor: an alias for `Reg<TRANSFORMATIONMATRIXA21_SPEC>`"]
pub type TRANSFORMATIONMATRIXA21 =
    crate::Reg<transformationmatrixa21::TRANSFORMATIONMATRIXA21_SPEC>;
#[doc = "Component A21 of transformation Matrix A"]
pub mod transformationmatrixa21;
#[doc = "TRANSFORMATIONMATRIXA12 (rw) register accessor: an alias for `Reg<TRANSFORMATIONMATRIXA12_SPEC>`"]
pub type TRANSFORMATIONMATRIXA12 =
    crate::Reg<transformationmatrixa12::TRANSFORMATIONMATRIXA12_SPEC>;
#[doc = "Component A12 of transformation Matrix A"]
pub mod transformationmatrixa12;
#[doc = "TRANSFORMATIONMATRIXA22 (rw) register accessor: an alias for `Reg<TRANSFORMATIONMATRIXA22_SPEC>`"]
pub type TRANSFORMATIONMATRIXA22 =
    crate::Reg<transformationmatrixa22::TRANSFORMATIONMATRIXA22_SPEC>;
#[doc = "Component A22 of transformation Matrix A"]
pub mod transformationmatrixa22;
#[doc = "TRANSLATIONVECTORX (rw) register accessor: an alias for `Reg<TRANSLATIONVECTORX_SPEC>`"]
pub type TRANSLATIONVECTORX = crate::Reg<translationvectorx::TRANSLATIONVECTORX_SPEC>;
#[doc = "Horizontal translation of the alpha frame."]
pub mod translationvectorx;
#[doc = "TRANSLATIONVECTORY (rw) register accessor: an alias for `Reg<TRANSLATIONVECTORY_SPEC>`"]
pub type TRANSLATIONVECTORY = crate::Reg<translationvectory::TRANSLATIONVECTORY_SPEC>;
#[doc = "Vertical translation of the alpha frame."]
pub mod translationvectory;
#[doc = "PROCESSINGCONTROL (rw) register accessor: an alias for `Reg<PROCESSINGCONTROL_SPEC>`"]
pub type PROCESSINGCONTROL = crate::Reg<processingcontrol::PROCESSINGCONTROL_SPEC>;
#[doc = "Drawing Engine main processing control register"]
pub mod processingcontrol;
#[doc = "STORECONTROL (rw) register accessor: an alias for `Reg<STORECONTROL_SPEC>`"]
pub type STORECONTROL = crate::Reg<storecontrol::STORECONTROL_SPEC>;
#[doc = "Store unit dynamic control register"]
pub mod storecontrol;
#[doc = "DESTINATIONBUFFERDIMENSIONS (rw) register accessor: an alias for `Reg<DESTINATIONBUFFERDIMENSIONS_SPEC>`"]
pub type DESTINATIONBUFFERDIMENSIONS =
    crate::Reg<destinationbufferdimensions::DESTINATIONBUFFERDIMENSIONS_SPEC>;
#[doc = "Maximum dimensions of the destination buffer. The resulting alpha frame is limited to this dimensions. However, the active area can be smaller and is given by ActiveDimensions register."]
pub mod destinationbufferdimensions;
#[doc = "DESTINATIONBUFFER (rw) register accessor: an alias for `Reg<DESTINATIONBUFFER_SPEC>`"]
pub type DESTINATIONBUFFER = crate::Reg<destinationbuffer::DESTINATIONBUFFER_SPEC>;
#[doc = "Destination buffer base address"]
pub mod destinationbuffer;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Drawing Engine start register"]
pub mod start;
#[doc = "READACK (w) register accessor: an alias for `Reg<READACK_SPEC>`"]
pub type READACK = crate::Reg<readack::READACK_SPEC>;
#[doc = "Drawing Engine read acknowledge register"]
pub mod readack;
#[doc = "CMDBUFFERREADPTR (r) register accessor: an alias for `Reg<CMDBUFFERREADPTR_SPEC>`"]
pub type CMDBUFFERREADPTR = crate::Reg<cmdbufferreadptr::CMDBUFFERREADPTR_SPEC>;
#[doc = "Buffer end address of command list"]
pub mod cmdbufferreadptr;
#[doc = "ACTIVEDIMENSIONS (r) register accessor: an alias for `Reg<ACTIVEDIMENSIONS_SPEC>`"]
pub type ACTIVEDIMENSIONS = crate::Reg<activedimensions::ACTIVEDIMENSIONS_SPEC>;
#[doc = "Shows active dimensions of the output alpha frame. This register is aligned to the ClipWindowDimensions register of the fetch."]
pub mod activedimensions;
#[doc = "LAYEROFFSET (r) register accessor: an alias for `Reg<LAYEROFFSET_SPEC>`"]
pub type LAYEROFFSET = crate::Reg<layeroffset::LAYEROFFSET_SPEC>;
#[doc = "Desired position of the alpha frame for the following Blit operation. This register is aligned to the LayerOffset of the fetch and given as signed 16.0 values."]
pub mod layeroffset;
#[doc = "ERROR (r) register accessor: an alias for `Reg<ERROR_SPEC>`"]
pub type ERROR = crate::Reg<error::ERROR_SPEC>;
#[doc = "Error status of finished operation. If register is unequal to zero the resulting raster image is most likely corrupted."]
pub mod error;
#[doc = "BURSTBUFFERPROPERTIES (r) register accessor: an alias for `Reg<BURSTBUFFERPROPERTIES_SPEC>`"]
pub type BURSTBUFFERPROPERTIES = crate::Reg<burstbufferproperties::BURSTBUFFERPROPERTIES_SPEC>;
#[doc = "Burst Buffer Property register"]
pub mod burstbufferproperties;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status bits."]
pub mod status;
#[doc = "PERFCOUNTERFRONT (r) register accessor: an alias for `Reg<PERFCOUNTERFRONT_SPEC>`"]
pub type PERFCOUNTERFRONT = crate::Reg<perfcounterfront::PERFCOUNTERFRONT_SPEC>;
#[doc = "Performance counter result"]
pub mod perfcounterfront;
#[doc = "PERFCOUNTERBACK (r) register accessor: an alias for `Reg<PERFCOUNTERBACK_SPEC>`"]
pub type PERFCOUNTERBACK = crate::Reg<perfcounterback::PERFCOUNTERBACK_SPEC>;
#[doc = "Performance counter result"]
pub mod perfcounterback;
