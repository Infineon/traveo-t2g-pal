#[doc = "Register `TR_STATUS` reader"]
pub struct R(crate::R<TR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INITIALIZED` reader - Reflects the state of the true random number generator: '0': Not initialized (TR_CTL0.INIT_DELAY has NOT passed). '1': Initialized (TR_CTL0.INIT_DELAY has passed)."]
pub type INITIALIZED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Reflects the state of the true random number generator: '0': Not initialized (TR_CTL0.INIT_DELAY has NOT passed). '1': Initialized (TR_CTL0.INIT_DELAY has passed)."]
    #[inline(always)]
    pub fn initialized(&self) -> INITIALIZED_R {
        INITIALIZED_R::new((self.bits & 1) != 0)
    }
}
#[doc = "True random status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_status](index.html) module"]
pub struct TR_STATUS_SPEC;
impl crate::RegisterSpec for TR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_status::R](R) reader structure"]
impl crate::Readable for TR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TR_STATUS to value 0"]
impl crate::Resettable for TR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
