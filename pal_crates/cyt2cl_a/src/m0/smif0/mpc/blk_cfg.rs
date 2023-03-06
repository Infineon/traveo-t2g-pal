#[doc = "Register `BLK_CFG` reader"]
pub struct R(crate::R<BLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK_SIZE` reader - Block size of individually protected blocks (0: 32B, 1: 64B, ... up to 15: 1MB) Block size= (1&lt;&lt;(BLOCK_SIZE+5)) The number and size blocks in an MPC is design time configurable and for embedded memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
pub type BLOCK_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INIT_IN_PROGRESS` reader - During initialization INIT_IN_PROGRESS is '1' and MMIO register accesses to BLK_LUT is blocked (BLK_IDX increment is also ignored). The block attributes are retained in DeepSleep (and obviously Active) power mode. Initialization is only required from a power mode in which the block attributes are not retained. E.g., initialization is required for a cold boot (after a Power-on-Reset). HW initializes the block attributes: the NS attributes are set to '0' (secure), the R attributes are set to '1' (read access allowed) and the W attributes are set to '1' (write access allowed). During initialization, the MPC supports memory accesses (memory accesses are NOT blocked) with the initialization block attribute values as mentioned above. This e.g. allows MPC initialization to proceed in parallel with boot program memory accesses (as opposed to serializing the two), improving device boot time."]
pub type INIT_IN_PROGRESS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Block size of individually protected blocks (0: 32B, 1: 64B, ... up to 15: 1MB) Block size= (1&lt;&lt;(BLOCK_SIZE+5)) The number and size blocks in an MPC is design time configurable and for embedded memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - During initialization INIT_IN_PROGRESS is '1' and MMIO register accesses to BLK_LUT is blocked (BLK_IDX increment is also ignored). The block attributes are retained in DeepSleep (and obviously Active) power mode. Initialization is only required from a power mode in which the block attributes are not retained. E.g., initialization is required for a cold boot (after a Power-on-Reset). HW initializes the block attributes: the NS attributes are set to '0' (secure), the R attributes are set to '1' (read access allowed) and the W attributes are set to '1' (write access allowed). During initialization, the MPC supports memory accesses (memory accesses are NOT blocked) with the initialization block attribute values as mentioned above. This e.g. allows MPC initialization to proceed in parallel with boot program memory accesses (as opposed to serializing the two), improving device boot time."]
    #[inline(always)]
    pub fn init_in_progress(&self) -> INIT_IN_PROGRESS_R {
        INIT_IN_PROGRESS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Block size &amp; initialization in progress\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_cfg](index.html) module"]
pub struct BLK_CFG_SPEC;
impl crate::RegisterSpec for BLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_cfg::R](R) reader structure"]
impl crate::Readable for BLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK_CFG to value 0x8000_0007"]
impl crate::Resettable for BLK_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0007;
}
