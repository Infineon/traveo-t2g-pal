#[doc = "Register `CH0_HIT` reader"]
pub struct R(crate::R<CH0_HIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0_HIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0_HIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0_HIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0_HIT` reader - Count all channel 0 AXI requests resulting in a cache hit. This field is updated with SAVE_AND_RESET_MEASUREMENTS pulse. See also OVERFLOW bit."]
pub type CH0_HIT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Count all channel 0 AXI requests resulting in a cache hit. This field is updated with SAVE_AND_RESET_MEASUREMENTS pulse. See also OVERFLOW bit."]
    #[inline(always)]
    pub fn ch0_hit(&self) -> CH0_HIT_R {
        CH0_HIT_R::new(self.bits)
    }
}
#[doc = "Cache hit counter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_hit](index.html) module"]
pub struct CH0_HIT_SPEC;
impl crate::RegisterSpec for CH0_HIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0_hit::R](R) reader structure"]
impl crate::Readable for CH0_HIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH0_HIT to value 0"]
impl crate::Resettable for CH0_HIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
