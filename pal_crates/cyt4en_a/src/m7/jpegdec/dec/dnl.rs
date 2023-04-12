#[doc = "Register `DNL` reader"]
pub struct R(crate::R<DNL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DNL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DNL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DNL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DNL` reader - DNL value defined in the marker"]
pub type DNL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DNL value defined in the marker"]
    #[inline(always)]
    pub fn dnl(&self) -> DNL_R {
        DNL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DNL value downloaded from the JPEG image. This register is initialized by the decoding start trigger (CMD.START).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dnl](index.html) module"]
pub struct DNL_SPEC;
impl crate::RegisterSpec for DNL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dnl::R](R) reader structure"]
impl crate::Readable for DNL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DNL to value 0"]
impl crate::Resettable for DNL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
