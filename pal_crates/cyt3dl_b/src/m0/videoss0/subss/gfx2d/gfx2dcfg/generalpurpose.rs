#[doc = "Register `GENERALPURPOSE[%s]` reader"]
pub struct R(crate::R<GENERALPURPOSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENERALPURPOSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENERALPURPOSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENERALPURPOSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENERALPURPOSE[%s]` writer"]
pub struct W(crate::W<GENERALPURPOSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENERALPURPOSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GENERALPURPOSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENERALPURPOSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GENERALPURPOSE` reader - General purpose config memory entry, does not have any function."]
pub type GENERALPURPOSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GENERALPURPOSE` writer - General purpose config memory entry, does not have any function."]
pub type GENERALPURPOSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GENERALPURPOSE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - General purpose config memory entry, does not have any function."]
    #[inline(always)]
    pub fn generalpurpose(&self) -> GENERALPURPOSE_R {
        GENERALPURPOSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose config memory entry, does not have any function."]
    #[inline(always)]
    #[must_use]
    pub fn generalpurpose(&mut self) -> GENERALPURPOSE_W<0> {
        GENERALPURPOSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose config memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [generalpurpose](index.html) module"]
pub struct GENERALPURPOSE_SPEC;
impl crate::RegisterSpec for GENERALPURPOSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [generalpurpose::R](R) reader structure"]
impl crate::Readable for GENERALPURPOSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [generalpurpose::W](W) writer structure"]
impl crate::Writable for GENERALPURPOSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GENERALPURPOSE[%s]
to value 0"]
impl crate::Resettable for GENERALPURPOSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
