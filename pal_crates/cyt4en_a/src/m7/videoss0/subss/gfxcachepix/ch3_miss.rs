#[doc = "Register `CH3_MISS` reader"]
pub struct R(crate::R<CH3_MISS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_MISS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3_MISS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3_MISS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH3_MISS` reader - Count all channel 3 AXI requests passing the cache but request a AXI request to memory. This field is updated with SAVE_AND_RESET_MEASUREMENTS pulse. See also OVERFLOW bit."]
pub type CH3_MISS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Count all channel 3 AXI requests passing the cache but request a AXI request to memory. This field is updated with SAVE_AND_RESET_MEASUREMENTS pulse. See also OVERFLOW bit."]
    #[inline(always)]
    pub fn ch3_miss(&self) -> CH3_MISS_R {
        CH3_MISS_R::new(self.bits)
    }
}
#[doc = "Cache miss counter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_miss](index.html) module"]
pub struct CH3_MISS_SPEC;
impl crate::RegisterSpec for CH3_MISS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_miss::R](R) reader structure"]
impl crate::Readable for CH3_MISS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH3_MISS to value 0"]
impl crate::Resettable for CH3_MISS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
