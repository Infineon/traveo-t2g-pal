#[doc = "Register `BW_RATE_LIMIT_Q12TO15` reader"]
pub struct R(crate::R<BW_RATE_LIMIT_Q12TO15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BW_RATE_LIMIT_Q12TO15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BW_RATE_LIMIT_Q12TO15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BW_RATE_LIMIT_Q12TO15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMOVED_31_0` reader - Write ignore, read 0"]
pub type REMOVED_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_31_0(&self) -> REMOVED_31_0_R {
        REMOVED_31_0_R::new(self.bits)
    }
}
#[doc = "Not presents. MXETH has only 3 queues. Access to the register returns AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bw_rate_limit_q12to15](index.html) module"]
pub struct BW_RATE_LIMIT_Q12TO15_SPEC;
impl crate::RegisterSpec for BW_RATE_LIMIT_Q12TO15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bw_rate_limit_q12to15::R](R) reader structure"]
impl crate::Readable for BW_RATE_LIMIT_Q12TO15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BW_RATE_LIMIT_Q12TO15 to value 0"]
impl crate::Resettable for BW_RATE_LIMIT_Q12TO15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
