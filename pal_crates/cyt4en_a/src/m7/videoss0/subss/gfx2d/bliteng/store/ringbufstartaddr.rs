#[doc = "Register `RINGBUFSTARTADDR` reader"]
pub struct R(crate::R<RINGBUFSTARTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGBUFSTARTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGBUFSTARTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGBUFSTARTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGBUFSTARTADDR` writer"]
pub struct W(crate::W<RINGBUFSTARTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGBUFSTARTADDR_SPEC>;
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
impl From<crate::W<RINGBUFSTARTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGBUFSTARTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RINGBUFSTARTADDR` reader - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
pub type RINGBUFSTARTADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RINGBUFSTARTADDR` writer - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
pub type RINGBUFSTARTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RINGBUFSTARTADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
    #[inline(always)]
    pub fn ringbufstartaddr(&self) -> RINGBUFSTARTADDR_R {
        RINGBUFSTARTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn ringbufstartaddr(&mut self) -> RINGBUFSTARTADDR_W<0> {
        RINGBUFSTARTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring buffer setup for destination.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringbufstartaddr](index.html) module"]
pub struct RINGBUFSTARTADDR_SPEC;
impl crate::RegisterSpec for RINGBUFSTARTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringbufstartaddr::R](R) reader structure"]
impl crate::Readable for RINGBUFSTARTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringbufstartaddr::W](W) writer structure"]
impl crate::Writable for RINGBUFSTARTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RINGBUFSTARTADDR to value 0"]
impl crate::Resettable for RINGBUFSTARTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
