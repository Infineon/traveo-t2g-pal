#[doc = "Register `SOURCEBUFFERLENGTH` reader"]
pub struct R(crate::R<SOURCEBUFFERLENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERLENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERLENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERLENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERLENGTH` writer"]
pub struct W(crate::W<SOURCEBUFFERLENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERLENGTH_SPEC>;
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
impl From<crate::W<SOURCEBUFFERLENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERLENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLEWORDS` reader - Number of 32-bit words minus one that are required to decode the run length encoded source buffer."]
pub type RLEWORDS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLEWORDS` writer - Number of 32-bit words minus one that are required to decode the run length encoded source buffer."]
pub type RLEWORDS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERLENGTH_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Number of 32-bit words minus one that are required to decode the run length encoded source buffer."]
    #[inline(always)]
    pub fn rlewords(&self) -> RLEWORDS_R {
        RLEWORDS_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Number of 32-bit words minus one that are required to decode the run length encoded source buffer."]
    #[inline(always)]
    #[must_use]
    pub fn rlewords(&mut self) -> RLEWORDS_W<0> {
        RLEWORDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer length for compressed data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferlength](index.html) module"]
pub struct SOURCEBUFFERLENGTH_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERLENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferlength::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERLENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferlength::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERLENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERLENGTH to value 0"]
impl crate::Resettable for SOURCEBUFFERLENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
