#[doc = "Register `RINGBUFWRAPADDR` reader"]
pub struct R(crate::R<RINGBUFWRAPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGBUFWRAPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGBUFWRAPADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGBUFWRAPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGBUFWRAPADDR` writer"]
pub struct W(crate::W<RINGBUFWRAPADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGBUFWRAPADDR_SPEC>;
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
impl From<crate::W<RINGBUFWRAPADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGBUFWRAPADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RINGBUFWRAPADDR` reader - End address of the ring buffer (last byte of the buffer plus one)."]
pub type RINGBUFWRAPADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RINGBUFWRAPADDR` writer - End address of the ring buffer (last byte of the buffer plus one)."]
pub type RINGBUFWRAPADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RINGBUFWRAPADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - End address of the ring buffer (last byte of the buffer plus one)."]
    #[inline(always)]
    pub fn ringbufwrapaddr(&self) -> RINGBUFWRAPADDR_R {
        RINGBUFWRAPADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of the ring buffer (last byte of the buffer plus one)."]
    #[inline(always)]
    #[must_use]
    pub fn ringbufwrapaddr(&mut self) -> RINGBUFWRAPADDR_W<0> {
        RINGBUFWRAPADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring buffer setup for destination.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringbufwrapaddr](index.html) module"]
pub struct RINGBUFWRAPADDR_SPEC;
impl crate::RegisterSpec for RINGBUFWRAPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringbufwrapaddr::R](R) reader structure"]
impl crate::Readable for RINGBUFWRAPADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringbufwrapaddr::W](W) writer structure"]
impl crate::Writable for RINGBUFWRAPADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RINGBUFWRAPADDR to value 0"]
impl crate::Resettable for RINGBUFWRAPADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
