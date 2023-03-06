#[doc = "Register `FETCHBUF1` reader"]
pub struct R(crate::R<FETCHBUF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHBUF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHBUF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHBUF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCHBUF1` writer"]
pub struct W(crate::W<FETCHBUF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCHBUF1_SPEC>;
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
impl From<crate::W<FETCHBUF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCHBUF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHLENGTH` reader - JPEG image length in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on the configured burst length. So reading beyond the actual end address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
pub type FETCHLENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FETCHLENGTH` writer - JPEG image length in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on the configured burst length. So reading beyond the actual end address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
pub type FETCHLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FETCHBUF1_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - JPEG image length in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on the configured burst length. So reading beyond the actual end address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
    #[inline(always)]
    pub fn fetchlength(&self) -> FETCHLENGTH_R {
        FETCHLENGTH_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - JPEG image length in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on the configured burst length. So reading beyond the actual end address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
    #[inline(always)]
    #[must_use]
    pub fn fetchlength(&mut self) -> FETCHLENGTH_W<0> {
        FETCHLENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer configuration (JPEG).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchbuf1](index.html) module"]
pub struct FETCHBUF1_SPEC;
impl crate::RegisterSpec for FETCHBUF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchbuf1::R](R) reader structure"]
impl crate::Readable for FETCHBUF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetchbuf1::W](W) writer structure"]
impl crate::Writable for FETCHBUF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FETCHBUF1 to value 0"]
impl crate::Resettable for FETCHBUF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
