#[doc = "Register `DECODERSTATUS` reader"]
pub struct R(crate::R<DECODERSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODERSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODERSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECODERSTATUS` writer"]
pub struct W(crate::W<DECODERSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECODERSTATUS_SPEC>;
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
impl From<crate::W<DECODERSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECODERSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFERTOOSMALL` reader - The buffer size given by RLEWords is too small. No complete output frame could be decoded."]
pub type BUFFERTOOSMALL_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERTOOSMALL` writer - The buffer size given by RLEWords is too small. No complete output frame could be decoded."]
pub type BUFFERTOOSMALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECODERSTATUS_SPEC, bool, O>;
#[doc = "Field `BUFFERTOOLARGE` reader - The buffer size given by RLEWords is too large. A complete output frame could be decoded, but more data was read than necessary."]
pub type BUFFERTOOLARGE_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERTOOLARGE` writer - The buffer size given by RLEWords is too large. A complete output frame could be decoded, but more data was read than necessary."]
pub type BUFFERTOOLARGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECODERSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The buffer size given by RLEWords is too small. No complete output frame could be decoded."]
    #[inline(always)]
    pub fn buffertoosmall(&self) -> BUFFERTOOSMALL_R {
        BUFFERTOOSMALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The buffer size given by RLEWords is too large. A complete output frame could be decoded, but more data was read than necessary."]
    #[inline(always)]
    pub fn buffertoolarge(&self) -> BUFFERTOOLARGE_R {
        BUFFERTOOLARGE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The buffer size given by RLEWords is too small. No complete output frame could be decoded."]
    #[inline(always)]
    #[must_use]
    pub fn buffertoosmall(&mut self) -> BUFFERTOOSMALL_W<0> {
        BUFFERTOOSMALL_W::new(self)
    }
    #[doc = "Bit 1 - The buffer size given by RLEWords is too large. A complete output frame could be decoded, but more data was read than necessary."]
    #[inline(always)]
    #[must_use]
    pub fn buffertoolarge(&mut self) -> BUFFERTOOLARGE_W<1> {
        BUFFERTOOLARGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status information of the RLAD decoder.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decoderstatus](index.html) module"]
pub struct DECODERSTATUS_SPEC;
impl crate::RegisterSpec for DECODERSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decoderstatus::R](R) reader structure"]
impl crate::Readable for DECODERSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decoderstatus::W](W) writer structure"]
impl crate::Writable for DECODERSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECODERSTATUS to value 0"]
impl crate::Resettable for DECODERSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
