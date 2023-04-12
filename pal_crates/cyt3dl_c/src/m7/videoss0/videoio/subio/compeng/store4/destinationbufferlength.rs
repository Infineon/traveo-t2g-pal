#[doc = "Register `DESTINATIONBUFFERLENGTH` reader"]
pub struct R(crate::R<DESTINATIONBUFFERLENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATIONBUFFERLENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATIONBUFFERLENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATIONBUFFERLENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATIONBUFFERLENGTH` writer"]
pub struct W(crate::W<DESTINATIONBUFFERLENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATIONBUFFERLENGTH_SPEC>;
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
impl From<crate::W<DESTINATIONBUFFERLENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATIONBUFFERLENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLEWORDSMAX` reader - N/A"]
pub type RLEWORDSMAX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLEWORDSMAX` writer - N/A"]
pub type RLEWORDSMAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERLENGTH_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - N/A"]
    #[inline(always)]
    pub fn rlewordsmax(&self) -> RLEWORDSMAX_R {
        RLEWORDSMAX_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rlewordsmax(&mut self) -> RLEWORDSMAX_W<0> {
        RLEWORDSMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer length for compressed data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destinationbufferlength](index.html) module"]
pub struct DESTINATIONBUFFERLENGTH_SPEC;
impl crate::RegisterSpec for DESTINATIONBUFFERLENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destinationbufferlength::R](R) reader structure"]
impl crate::Readable for DESTINATIONBUFFERLENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destinationbufferlength::W](W) writer structure"]
impl crate::Writable for DESTINATIONBUFFERLENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATIONBUFFERLENGTH to value 0"]
impl crate::Resettable for DESTINATIONBUFFERLENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
