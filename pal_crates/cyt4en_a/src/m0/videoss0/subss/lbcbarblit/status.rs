#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - When bit is 0, the module is idle and no AXI transactions are pending. It is always 0 in NEUTRAL mode."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `FAULT_LOAD` reader - For debugging purposes only. The bit is set in DYNAMIC_LOAD mode when the transfer counter reaches a value below 0 or above 1023 and keeps value until reset of the IP. It is a fault state, which indicates undefined system behaviour due to invalid state of the AXI interconnect."]
pub type FAULT_LOAD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - When bit is 0, the module is idle and no AXI transactions are pending. It is always 0 in NEUTRAL mode."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - For debugging purposes only. The bit is set in DYNAMIC_LOAD mode when the transfer counter reaches a value below 0 or above 1023 and keeps value until reset of the IP. It is a fault state, which indicates undefined system behaviour due to invalid state of the AXI interconnect."]
    #[inline(always)]
    pub fn fault_load(&self) -> FAULT_LOAD_R {
        FAULT_LOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Common status information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
