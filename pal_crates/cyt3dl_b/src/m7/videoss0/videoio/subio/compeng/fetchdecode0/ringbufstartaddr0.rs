#[doc = "Register `RINGBUFSTARTADDR0` reader"]
pub struct R(crate::R<RINGBUFSTARTADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGBUFSTARTADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGBUFSTARTADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGBUFSTARTADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGBUFSTARTADDR0` writer"]
pub struct W(crate::W<RINGBUFSTARTADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGBUFSTARTADDR0_SPEC>;
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
impl From<crate::W<RINGBUFSTARTADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGBUFSTARTADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RINGBUFSTARTADDR0` reader - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
pub type RINGBUFSTARTADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RINGBUFSTARTADDR0` writer - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
pub type RINGBUFSTARTADDR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RINGBUFSTARTADDR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
    #[inline(always)]
    pub fn ringbufstartaddr0(&self) -> RINGBUFSTARTADDR0_R {
        RINGBUFSTARTADDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the ring buffer. Must be aligned to SetBurstLength x 8 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn ringbufstartaddr0(&mut self) -> RINGBUFSTARTADDR0_W<0> {
        RINGBUFSTARTADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring buffer setup for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringbufstartaddr0](index.html) module"]
pub struct RINGBUFSTARTADDR0_SPEC;
impl crate::RegisterSpec for RINGBUFSTARTADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringbufstartaddr0::R](R) reader structure"]
impl crate::Readable for RINGBUFSTARTADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringbufstartaddr0::W](W) writer structure"]
impl crate::Writable for RINGBUFSTARTADDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RINGBUFSTARTADDR0 to value 0"]
impl crate::Resettable for RINGBUFSTARTADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
