#[doc = "Register `BURST` reader"]
pub struct R(crate::R<BURST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BURST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BURST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BURST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BURST` writer"]
pub struct W(crate::W<BURST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BURST_SPEC>;
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
impl From<crate::W<BURST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BURST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHBURSTLENGTH` reader - Maximum burst length for Fetch Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
pub type FETCHBURSTLENGTH_R = crate::BitReader<bool>;
#[doc = "Field `FETCHBURSTLENGTH` writer - Maximum burst length for Fetch Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
pub type FETCHBURSTLENGTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, BURST_SPEC, bool, O>;
#[doc = "Field `STOREBURSTLENGTH` reader - Maximum burst length for Store Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
pub type STOREBURSTLENGTH_R = crate::BitReader<bool>;
#[doc = "Field `STOREBURSTLENGTH` writer - Maximum burst length for Store Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
pub type STOREBURSTLENGTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, BURST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Maximum burst length for Fetch Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
    #[inline(always)]
    pub fn fetchburstlength(&self) -> FETCHBURSTLENGTH_R {
        FETCHBURSTLENGTH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Maximum burst length for Store Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
    #[inline(always)]
    pub fn storeburstlength(&self) -> STOREBURSTLENGTH_R {
        STOREBURSTLENGTH_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Maximum burst length for Fetch Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
    #[inline(always)]
    #[must_use]
    pub fn fetchburstlength(&mut self) -> FETCHBURSTLENGTH_W<0> {
        FETCHBURSTLENGTH_W::new(self)
    }
    #[doc = "Bit 8 - Maximum burst length for Store Unit. Must not be changed during decoding operation. 0: Length is 4 1: Length is 8"]
    #[inline(always)]
    #[must_use]
    pub fn storeburstlength(&mut self) -> STOREBURSTLENGTH_W<8> {
        STOREBURSTLENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI burst settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burst](index.html) module"]
pub struct BURST_SPEC;
impl crate::RegisterSpec for BURST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [burst::R](R) reader structure"]
impl crate::Readable for BURST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [burst::W](W) writer structure"]
impl crate::Writable for BURST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BURST to value 0"]
impl crate::Resettable for BURST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
