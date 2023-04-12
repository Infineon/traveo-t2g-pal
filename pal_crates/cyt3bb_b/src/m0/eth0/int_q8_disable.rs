#[doc = "Register `INT_Q8_DISABLE` reader"]
pub struct R(crate::R<INT_Q8_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_Q8_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_Q8_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_Q8_DISABLE_SPEC>) -> Self {
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
#[doc = "int_q8_disable to int_q15_disable doesn't present. Access to the register returns AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_q8_disable](index.html) module"]
pub struct INT_Q8_DISABLE_SPEC;
impl crate::RegisterSpec for INT_Q8_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_q8_disable::R](R) reader structure"]
impl crate::Readable for INT_Q8_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_Q8_DISABLE to value 0"]
impl crate::Resettable for INT_Q8_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
