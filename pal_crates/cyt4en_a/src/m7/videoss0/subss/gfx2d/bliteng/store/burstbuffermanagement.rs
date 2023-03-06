#[doc = "Register `BURSTBUFFERMANAGEMENT` reader"]
pub struct R(crate::R<BURSTBUFFERMANAGEMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BURSTBUFFERMANAGEMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BURSTBUFFERMANAGEMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BURSTBUFFERMANAGEMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BURSTBUFFERMANAGEMENT` writer"]
pub struct W(crate::W<BURSTBUFFERMANAGEMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BURSTBUFFERMANAGEMENT_SPEC>;
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
impl From<crate::W<BURSTBUFFERMANAGEMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BURSTBUFFERMANAGEMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETBURSTLENGTH` reader - Set this to the burst length that should be used on the AXI interface. Please note that SetBurstLength has to be smaller or equal to MaxBurstLength. Only a power of two may be specified as burst length. Please set this to at least 2 for 64bit pixels, otherwise performance will suffer."]
pub type SETBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETBURSTLENGTH` writer - Set this to the burst length that should be used on the AXI interface. Please note that SetBurstLength has to be smaller or equal to MaxBurstLength. Only a power of two may be specified as burst length. Please set this to at least 2 for 64bit pixels, otherwise performance will suffer."]
pub type SETBURSTLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BURSTBUFFERMANAGEMENT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 8:12 - Set this to the burst length that should be used on the AXI interface. Please note that SetBurstLength has to be smaller or equal to MaxBurstLength. Only a power of two may be specified as burst length. Please set this to at least 2 for 64bit pixels, otherwise performance will suffer."]
    #[inline(always)]
    pub fn setburstlength(&self) -> SETBURSTLENGTH_R {
        SETBURSTLENGTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - Set this to the burst length that should be used on the AXI interface. Please note that SetBurstLength has to be smaller or equal to MaxBurstLength. Only a power of two may be specified as burst length. Please set this to at least 2 for 64bit pixels, otherwise performance will suffer."]
    #[inline(always)]
    #[must_use]
    pub fn setburstlength(&mut self) -> SETBURSTLENGTH_W<8> {
        SETBURSTLENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst Buffer setup register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstbuffermanagement](index.html) module"]
pub struct BURSTBUFFERMANAGEMENT_SPEC;
impl crate::RegisterSpec for BURSTBUFFERMANAGEMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [burstbuffermanagement::R](R) reader structure"]
impl crate::Readable for BURSTBUFFERMANAGEMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [burstbuffermanagement::W](W) writer structure"]
impl crate::Writable for BURSTBUFFERMANAGEMENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BURSTBUFFERMANAGEMENT to value 0x0400"]
impl crate::Resettable for BURSTBUFFERMANAGEMENT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
