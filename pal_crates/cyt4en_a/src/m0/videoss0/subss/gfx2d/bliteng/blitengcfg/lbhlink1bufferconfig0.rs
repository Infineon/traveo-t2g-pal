#[doc = "Register `LBHLINK1BUFFERCONFIG0` reader"]
pub struct R(crate::R<LBHLINK1BUFFERCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK1BUFFERCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK1BUFFERCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK1BUFFERCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LBHLINK1BUFFERCONFIG0` writer"]
pub struct W(crate::W<LBHLINK1BUFFERCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBHLINK1BUFFERCONFIG0_SPEC>;
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
impl From<crate::W<LBHLINK1BUFFERCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBHLINK1BUFFERCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBHLINK1ENABLE` reader - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
pub type LBHLINK1ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LBHLINK1ENABLE` writer - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
pub type LBHLINK1ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LBHLINK1BUFFERCONFIG0_SPEC, bool, O>;
#[doc = "Field `LBHLINK1SIZEENABLE` reader - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
pub type LBHLINK1SIZEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LBHLINK1SIZEENABLE` writer - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
pub type LBHLINK1SIZEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LBHLINK1BUFFERCONFIG0_SPEC, bool, O>;
#[doc = "Field `LBHLINK1BUFLINES` reader - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
pub type LBHLINK1BUFLINES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LBHLINK1BUFLINES` writer - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
pub type LBHLINK1BUFLINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK1BUFFERCONFIG0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
    #[inline(always)]
    pub fn lbhlink1enable(&self) -> LBHLINK1ENABLE_R {
        LBHLINK1ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
    #[inline(always)]
    pub fn lbhlink1sizeenable(&self) -> LBHLINK1SIZEENABLE_R {
        LBHLINK1SIZEENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
    #[inline(always)]
    pub fn lbhlink1buflines(&self) -> LBHLINK1BUFLINES_R {
        LBHLINK1BUFLINES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink1enable(&mut self) -> LBHLINK1ENABLE_W<0> {
        LBHLINK1ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink1sizeenable(&mut self) -> LBHLINK1SIZEENABLE_W<4> {
        LBHLINK1SIZEENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink1buflines(&mut self) -> LBHLINK1BUFLINES_W<8> {
        LBHLINK1BUFLINES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line buffer configuration register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink1bufferconfig0](index.html) module"]
pub struct LBHLINK1BUFFERCONFIG0_SPEC;
impl crate::RegisterSpec for LBHLINK1BUFFERCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink1bufferconfig0::R](R) reader structure"]
impl crate::Readable for LBHLINK1BUFFERCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lbhlink1bufferconfig0::W](W) writer structure"]
impl crate::Writable for LBHLINK1BUFFERCONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LBHLINK1BUFFERCONFIG0 to value 0"]
impl crate::Resettable for LBHLINK1BUFFERCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
