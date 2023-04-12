#[doc = "Register `ENCODERSTATUS` reader"]
pub struct R(crate::R<ENCODERSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENCODERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENCODERSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENCODERSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENCODERSTATUS` writer"]
pub struct W(crate::W<ENCODERSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENCODERSTATUS_SPEC>;
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
impl From<crate::W<ENCODERSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENCODERSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLEWORDS` reader - Number of 32-bit words minus one that was used for the compressed buffer."]
pub type RLEWORDS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLEWORDS` writer - Number of 32-bit words minus one that was used for the compressed buffer."]
pub type RLEWORDS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENCODERSTATUS_SPEC, u32, u32, 29, O>;
#[doc = "Field `BUFFERTOOSMALL` reader - The buffer size given by RLEWordsMax is too small. Not the complete input frame could be encoded."]
pub type BUFFERTOOSMALL_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERTOOSMALL` writer - The buffer size given by RLEWordsMax is too small. Not the complete input frame could be encoded."]
pub type BUFFERTOOSMALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENCODERSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - Number of 32-bit words minus one that was used for the compressed buffer."]
    #[inline(always)]
    pub fn rlewords(&self) -> RLEWORDS_R {
        RLEWORDS_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - The buffer size given by RLEWordsMax is too small. Not the complete input frame could be encoded."]
    #[inline(always)]
    pub fn buffertoosmall(&self) -> BUFFERTOOSMALL_R {
        BUFFERTOOSMALL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Number of 32-bit words minus one that was used for the compressed buffer."]
    #[inline(always)]
    #[must_use]
    pub fn rlewords(&mut self) -> RLEWORDS_W<0> {
        RLEWORDS_W::new(self)
    }
    #[doc = "Bit 31 - The buffer size given by RLEWordsMax is too small. Not the complete input frame could be encoded."]
    #[inline(always)]
    #[must_use]
    pub fn buffertoosmall(&mut self) -> BUFFERTOOSMALL_W<31> {
        BUFFERTOOSMALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status information of the RLAD encoder.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [encoderstatus](index.html) module"]
pub struct ENCODERSTATUS_SPEC;
impl crate::RegisterSpec for ENCODERSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [encoderstatus::R](R) reader structure"]
impl crate::Readable for ENCODERSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [encoderstatus::W](W) writer structure"]
impl crate::Writable for ENCODERSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENCODERSTATUS to value 0x1fff_ffff"]
impl crate::Resettable for ENCODERSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1fff_ffff;
}
