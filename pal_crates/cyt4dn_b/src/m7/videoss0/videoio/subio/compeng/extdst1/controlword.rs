#[doc = "Register `CONTROLWORD` reader"]
pub struct R(crate::R<CONTROLWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROLWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROLWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROLWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CW_VAL` reader - Value of last received control word"]
pub type CW_VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of last received control word"]
    #[inline(always)]
    pub fn cw_val(&self) -> CW_VAL_R {
        CW_VAL_R::new(self.bits)
    }
}
#[doc = "Value of last received control word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [controlword](index.html) module"]
pub struct CONTROLWORD_SPEC;
impl crate::RegisterSpec for CONTROLWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [controlword::R](R) reader structure"]
impl crate::Readable for CONTROLWORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONTROLWORD to value 0"]
impl crate::Resettable for CONTROLWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
