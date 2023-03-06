#[doc = "Register `CRC_INPUT1` reader"]
pub struct R(crate::R<CRC_INPUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_INPUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_INPUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_INPUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_INPUT1` writer"]
pub struct W(crate::W<CRC_INPUT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_INPUT1_SPEC>;
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
impl From<crate::W<CRC_INPUT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_INPUT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT` reader - Higher 32 input bits to the CRC engine."]
pub type INPUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INPUT` writer - Higher 32 input bits to the CRC engine."]
pub type INPUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_INPUT1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Higher 32 input bits to the CRC engine."]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Higher 32 input bits to the CRC engine."]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<0> {
        INPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC input 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_input1](index.html) module"]
pub struct CRC_INPUT1_SPEC;
impl crate::RegisterSpec for CRC_INPUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_input1::R](R) reader structure"]
impl crate::Readable for CRC_INPUT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_input1::W](W) writer structure"]
impl crate::Writable for CRC_INPUT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_INPUT1 to value 0"]
impl crate::Resettable for CRC_INPUT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
