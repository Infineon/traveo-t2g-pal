#[doc = "Register `LBHLINK3BUFFERCONFIG0` reader"]
pub struct R(crate::R<LBHLINK3BUFFERCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK3BUFFERCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK3BUFFERCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK3BUFFERCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LBHLINK3BUFFERCONFIG0` writer"]
pub struct W(crate::W<LBHLINK3BUFFERCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBHLINK3BUFFERCONFIG0_SPEC>;
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
impl From<crate::W<LBHLINK3BUFFERCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBHLINK3BUFFERCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBHLINK3ENABLE` reader - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
pub type LBHLINK3ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LBHLINK3ENABLE` writer - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
pub type LBHLINK3ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LBHLINK3BUFFERCONFIG0_SPEC, bool, O>;
#[doc = "Field `LBHLINK3SIZEENABLE` reader - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
pub type LBHLINK3SIZEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LBHLINK3SIZEENABLE` writer - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
pub type LBHLINK3SIZEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LBHLINK3BUFFERCONFIG0_SPEC, bool, O>;
#[doc = "Field `LBHLINK3BUFLINES` reader - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
pub type LBHLINK3BUFLINES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LBHLINK3BUFLINES` writer - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
pub type LBHLINK3BUFLINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK3BUFFERCONFIG0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
    #[inline(always)]
    pub fn lbhlink3enable(&self) -> LBHLINK3ENABLE_R {
        LBHLINK3ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
    #[inline(always)]
    pub fn lbhlink3sizeenable(&self) -> LBHLINK3SIZEENABLE_R {
        LBHLINK3SIZEENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
    #[inline(always)]
    pub fn lbhlink3buflines(&self) -> LBHLINK3BUFLINES_R {
        LBHLINK3BUFLINES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Activates the line-buffer handshake, i.e. will stall the store or the display fetch upon respective condition."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink3enable(&mut self) -> LBHLINK3ENABLE_W<0> {
        LBHLINK3ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Activates the ring buffer usage. This signal will be provided to the fetch and the store."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink3sizeenable(&mut self) -> LBHLINK3SIZEENABLE_W<4> {
        LBHLINK3SIZEENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - The number of lines in the line buffer in power of two. Max supported number of lines in ringbuffer is 16384 (14pow2=16384). Programming 15 results also in 16384 lines."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink3buflines(&mut self) -> LBHLINK3BUFLINES_W<8> {
        LBHLINK3BUFLINES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line buffer configuration register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink3bufferconfig0](index.html) module"]
pub struct LBHLINK3BUFFERCONFIG0_SPEC;
impl crate::RegisterSpec for LBHLINK3BUFFERCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink3bufferconfig0::R](R) reader structure"]
impl crate::Readable for LBHLINK3BUFFERCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lbhlink3bufferconfig0::W](W) writer structure"]
impl crate::Writable for LBHLINK3BUFFERCONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LBHLINK3BUFFERCONFIG0 to value 0"]
impl crate::Resettable for LBHLINK3BUFFERCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
