#[doc = "Register `BISTM5_CH0` reader"]
pub struct R(crate::R<BISTM5_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTM5_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTM5_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTM5_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BISTM5_CH0` writer"]
pub struct W(crate::W<BISTM5_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BISTM5_CH0_SPEC>;
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
impl From<crate::W<BISTM5_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BISTM5_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARCH_ELEMENT_5` reader - March element 5 - Channel 0"]
pub type MARCH_ELEMENT_5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MARCH_ELEMENT_5` writer - March element 5 - Channel 0"]
pub type MARCH_ELEMENT_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTM5_CH0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - March element 5 - Channel 0"]
    #[inline(always)]
    pub fn march_element_5(&self) -> MARCH_ELEMENT_5_R {
        MARCH_ELEMENT_5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - March element 5 - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn march_element_5(&mut self) -> MARCH_ELEMENT_5_W<0> {
        MARCH_ELEMENT_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST March Element Register 5 - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bistm5_ch0](index.html) module"]
pub struct BISTM5_CH0_SPEC;
impl crate::RegisterSpec for BISTM5_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bistm5_ch0::R](R) reader structure"]
impl crate::Readable for BISTM5_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bistm5_ch0::W](W) writer structure"]
impl crate::Writable for BISTM5_CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BISTM5_CH0 to value 0"]
impl crate::Resettable for BISTM5_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
