#[doc = r"Register block"]
#[repr(C)]
pub struct SMIF_CRYPTO {
    #[doc = "0x00 - Cryptography command"]
    pub crypto_cmd: CRYPTO_CMD,
    #[doc = "0x04 - Cryptography base address"]
    pub crypto_addr: CRYPTO_ADDR,
    #[doc = "0x08 - Cryptography mask"]
    pub crypto_mask: CRYPTO_MASK,
    #[doc = "0x0c - Cryptography subregion disable"]
    pub crypto_subregion: CRYPTO_SUBREGION,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - Cryptography input 0"]
    pub crypto_input0: CRYPTO_INPUT0,
    #[doc = "0x24 - Cryptography input 1"]
    pub crypto_input1: CRYPTO_INPUT1,
    #[doc = "0x28 - Cryptography input 2"]
    pub crypto_input2: CRYPTO_INPUT2,
    #[doc = "0x2c - Cryptography input 3"]
    pub crypto_input3: CRYPTO_INPUT3,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - Cryptography key 0"]
    pub crypto_key0: CRYPTO_KEY0,
    #[doc = "0x44 - Cryptography key 1"]
    pub crypto_key1: CRYPTO_KEY1,
    #[doc = "0x48 - Cryptography key 2"]
    pub crypto_key2: CRYPTO_KEY2,
    #[doc = "0x4c - Cryptography key 3"]
    pub crypto_key3: CRYPTO_KEY3,
    _reserved12: [u8; 0x10],
    #[doc = "0x60 - Cryptography output 0"]
    pub crypto_output0: CRYPTO_OUTPUT0,
    #[doc = "0x64 - Cryptography output 1"]
    pub crypto_output1: CRYPTO_OUTPUT1,
    #[doc = "0x68 - Cryptography output 2"]
    pub crypto_output2: CRYPTO_OUTPUT2,
    #[doc = "0x6c - Cryptography output 3"]
    pub crypto_output3: CRYPTO_OUTPUT3,
}
#[doc = "CRYPTO_CMD (rw) register accessor: an alias for `Reg<CRYPTO_CMD_SPEC>`"]
pub type CRYPTO_CMD = crate::Reg<crypto_cmd::CRYPTO_CMD_SPEC>;
#[doc = "Cryptography command"]
pub mod crypto_cmd;
#[doc = "CRYPTO_ADDR (rw) register accessor: an alias for `Reg<CRYPTO_ADDR_SPEC>`"]
pub type CRYPTO_ADDR = crate::Reg<crypto_addr::CRYPTO_ADDR_SPEC>;
#[doc = "Cryptography base address"]
pub mod crypto_addr;
#[doc = "CRYPTO_MASK (rw) register accessor: an alias for `Reg<CRYPTO_MASK_SPEC>`"]
pub type CRYPTO_MASK = crate::Reg<crypto_mask::CRYPTO_MASK_SPEC>;
#[doc = "Cryptography mask"]
pub mod crypto_mask;
#[doc = "CRYPTO_SUBREGION (rw) register accessor: an alias for `Reg<CRYPTO_SUBREGION_SPEC>`"]
pub type CRYPTO_SUBREGION = crate::Reg<crypto_subregion::CRYPTO_SUBREGION_SPEC>;
#[doc = "Cryptography subregion disable"]
pub mod crypto_subregion;
#[doc = "CRYPTO_INPUT0 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT0_SPEC>`"]
pub type CRYPTO_INPUT0 = crate::Reg<crypto_input0::CRYPTO_INPUT0_SPEC>;
#[doc = "Cryptography input 0"]
pub mod crypto_input0;
#[doc = "CRYPTO_INPUT1 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT1_SPEC>`"]
pub type CRYPTO_INPUT1 = crate::Reg<crypto_input1::CRYPTO_INPUT1_SPEC>;
#[doc = "Cryptography input 1"]
pub mod crypto_input1;
#[doc = "CRYPTO_INPUT2 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT2_SPEC>`"]
pub type CRYPTO_INPUT2 = crate::Reg<crypto_input2::CRYPTO_INPUT2_SPEC>;
#[doc = "Cryptography input 2"]
pub mod crypto_input2;
#[doc = "CRYPTO_INPUT3 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT3_SPEC>`"]
pub type CRYPTO_INPUT3 = crate::Reg<crypto_input3::CRYPTO_INPUT3_SPEC>;
#[doc = "Cryptography input 3"]
pub mod crypto_input3;
#[doc = "CRYPTO_KEY0 (w) register accessor: an alias for `Reg<CRYPTO_KEY0_SPEC>`"]
pub type CRYPTO_KEY0 = crate::Reg<crypto_key0::CRYPTO_KEY0_SPEC>;
#[doc = "Cryptography key 0"]
pub mod crypto_key0;
#[doc = "CRYPTO_KEY1 (w) register accessor: an alias for `Reg<CRYPTO_KEY1_SPEC>`"]
pub type CRYPTO_KEY1 = crate::Reg<crypto_key1::CRYPTO_KEY1_SPEC>;
#[doc = "Cryptography key 1"]
pub mod crypto_key1;
#[doc = "CRYPTO_KEY2 (w) register accessor: an alias for `Reg<CRYPTO_KEY2_SPEC>`"]
pub type CRYPTO_KEY2 = crate::Reg<crypto_key2::CRYPTO_KEY2_SPEC>;
#[doc = "Cryptography key 2"]
pub mod crypto_key2;
#[doc = "CRYPTO_KEY3 (w) register accessor: an alias for `Reg<CRYPTO_KEY3_SPEC>`"]
pub type CRYPTO_KEY3 = crate::Reg<crypto_key3::CRYPTO_KEY3_SPEC>;
#[doc = "Cryptography key 3"]
pub mod crypto_key3;
#[doc = "CRYPTO_OUTPUT0 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT0_SPEC>`"]
pub type CRYPTO_OUTPUT0 = crate::Reg<crypto_output0::CRYPTO_OUTPUT0_SPEC>;
#[doc = "Cryptography output 0"]
pub mod crypto_output0;
#[doc = "CRYPTO_OUTPUT1 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT1_SPEC>`"]
pub type CRYPTO_OUTPUT1 = crate::Reg<crypto_output1::CRYPTO_OUTPUT1_SPEC>;
#[doc = "Cryptography output 1"]
pub mod crypto_output1;
#[doc = "CRYPTO_OUTPUT2 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT2_SPEC>`"]
pub type CRYPTO_OUTPUT2 = crate::Reg<crypto_output2::CRYPTO_OUTPUT2_SPEC>;
#[doc = "Cryptography output 2"]
pub mod crypto_output2;
#[doc = "CRYPTO_OUTPUT3 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT3_SPEC>`"]
pub type CRYPTO_OUTPUT3 = crate::Reg<crypto_output3::CRYPTO_OUTPUT3_SPEC>;
#[doc = "Cryptography output 3"]
pub mod crypto_output3;
