#[doc = "Register `STRETCH_RATIO` reader"]
pub struct R(crate::R<STRETCH_RATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STRETCH_RATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STRETCH_RATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STRETCH_RATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STRETCH_RATIO` writer"]
pub struct W(crate::W<STRETCH_RATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STRETCH_RATIO_SPEC>;
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
impl From<crate::W<STRETCH_RATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STRETCH_RATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPG_STRETCH` reader - IPG Stretch. Bits 7:0 are multiplied with the previously transmitted frame length (including preamble) bits 15:8 +1 divide the frame length. If the resulting number is greater than 96 and bit 28 is set in the network configuration register then the resulting number is used for the transmit inter-packet-gap. 1 is added to bits 15:8 to prevent a divide by zero."]
pub type IPG_STRETCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IPG_STRETCH` writer - IPG Stretch. Bits 7:0 are multiplied with the previously transmitted frame length (including preamble) bits 15:8 +1 divide the frame length. If the resulting number is greater than 96 and bit 28 is set in the network configuration register then the resulting number is used for the transmit inter-packet-gap. 1 is added to bits 15:8 to prevent a divide by zero."]
pub type IPG_STRETCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STRETCH_RATIO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IPG Stretch. Bits 7:0 are multiplied with the previously transmitted frame length (including preamble) bits 15:8 +1 divide the frame length. If the resulting number is greater than 96 and bit 28 is set in the network configuration register then the resulting number is used for the transmit inter-packet-gap. 1 is added to bits 15:8 to prevent a divide by zero."]
    #[inline(always)]
    pub fn ipg_stretch(&self) -> IPG_STRETCH_R {
        IPG_STRETCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IPG Stretch. Bits 7:0 are multiplied with the previously transmitted frame length (including preamble) bits 15:8 +1 divide the frame length. If the resulting number is greater than 96 and bit 28 is set in the network configuration register then the resulting number is used for the transmit inter-packet-gap. 1 is added to bits 15:8 to prevent a divide by zero."]
    #[inline(always)]
    #[must_use]
    pub fn ipg_stretch(&mut self) -> IPG_STRETCH_W<0> {
        IPG_STRETCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPG stretch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stretch_ratio](index.html) module"]
pub struct STRETCH_RATIO_SPEC;
impl crate::RegisterSpec for STRETCH_RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stretch_ratio::R](R) reader structure"]
impl crate::Readable for STRETCH_RATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stretch_ratio::W](W) writer structure"]
impl crate::Writable for STRETCH_RATIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STRETCH_RATIO to value 0"]
impl crate::Resettable for STRETCH_RATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
