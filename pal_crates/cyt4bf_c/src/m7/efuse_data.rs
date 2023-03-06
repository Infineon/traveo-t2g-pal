#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x2c],
    #[doc = "0x2c - Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state."]
    pub secure_hash_word0: SECURE_HASH_WORD0,
    #[doc = "0x30 - Secure 128 bits HASH word 1 that is used for authentication in SECURE protection state."]
    pub secure_hash_word1: SECURE_HASH_WORD1,
    #[doc = "0x34 - Secure 128 bits HASH word 2 that is used for authentication in SECURE protection state."]
    pub secure_hash_word2: SECURE_HASH_WORD2,
    #[doc = "0x38 - Secure 128 bits HASH word 3 that is used for authentication in SECURE protection state."]
    pub secure_hash_word3: SECURE_HASH_WORD3,
    #[doc = "0x3c - Access restrictions for SECURE protection state in SECURE lifecycle stage"]
    pub secure_access_restrict: SECURE_ACCESS_RESTRICT,
    #[doc = "0x40 - Access restrictions for DEAD protection state in SECURE lifecycle stage and number of zeros for Secure fuse group"]
    pub secure_dead_access_restrict_zeros: SECURE_DEAD_ACCESS_RESTRICT_ZEROS,
    _reserved6: [u8; 0x24],
    #[doc = "0x68..0x80 - Available EFUSE bits for customer usage.They can be programmed in NORMAL protection state via CMx/DAP and in SECURE protection state via CMx."]
    pub customer_data: [CUSTOMER_DATA; 6],
}
#[doc = "SECURE_HASH_WORD0 (rw) register accessor: an alias for `Reg<SECURE_HASH_WORD0_SPEC>`"]
pub type SECURE_HASH_WORD0 = crate::Reg<secure_hash_word0::SECURE_HASH_WORD0_SPEC>;
#[doc = "Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state."]
pub mod secure_hash_word0;
#[doc = "SECURE_HASH_WORD1 (rw) register accessor: an alias for `Reg<SECURE_HASH_WORD1_SPEC>`"]
pub type SECURE_HASH_WORD1 = crate::Reg<secure_hash_word1::SECURE_HASH_WORD1_SPEC>;
#[doc = "Secure 128 bits HASH word 1 that is used for authentication in SECURE protection state."]
pub mod secure_hash_word1;
#[doc = "SECURE_HASH_WORD2 (rw) register accessor: an alias for `Reg<SECURE_HASH_WORD2_SPEC>`"]
pub type SECURE_HASH_WORD2 = crate::Reg<secure_hash_word2::SECURE_HASH_WORD2_SPEC>;
#[doc = "Secure 128 bits HASH word 2 that is used for authentication in SECURE protection state."]
pub mod secure_hash_word2;
#[doc = "SECURE_HASH_WORD3 (rw) register accessor: an alias for `Reg<SECURE_HASH_WORD3_SPEC>`"]
pub type SECURE_HASH_WORD3 = crate::Reg<secure_hash_word3::SECURE_HASH_WORD3_SPEC>;
#[doc = "Secure 128 bits HASH word 3 that is used for authentication in SECURE protection state."]
pub mod secure_hash_word3;
#[doc = "SECURE_ACCESS_RESTRICT (rw) register accessor: an alias for `Reg<SECURE_ACCESS_RESTRICT_SPEC>`"]
pub type SECURE_ACCESS_RESTRICT = crate::Reg<secure_access_restrict::SECURE_ACCESS_RESTRICT_SPEC>;
#[doc = "Access restrictions for SECURE protection state in SECURE lifecycle stage"]
pub mod secure_access_restrict;
#[doc = "SECURE_DEAD_ACCESS_RESTRICT_ZEROS (rw) register accessor: an alias for `Reg<SECURE_DEAD_ACCESS_RESTRICT_ZEROS_SPEC>`"]
pub type SECURE_DEAD_ACCESS_RESTRICT_ZEROS =
    crate::Reg<secure_dead_access_restrict_zeros::SECURE_DEAD_ACCESS_RESTRICT_ZEROS_SPEC>;
#[doc = "Access restrictions for DEAD protection state in SECURE lifecycle stage and number of zeros for Secure fuse group"]
pub mod secure_dead_access_restrict_zeros;
#[doc = "CUSTOMER_DATA (rw) register accessor: an alias for `Reg<CUSTOMER_DATA_SPEC>`"]
pub type CUSTOMER_DATA = crate::Reg<customer_data::CUSTOMER_DATA_SPEC>;
#[doc = "Available EFUSE bits for customer usage.They can be programmed in NORMAL protection state via CMx/DAP and in SECURE protection state via CMx."]
pub mod customer_data;
