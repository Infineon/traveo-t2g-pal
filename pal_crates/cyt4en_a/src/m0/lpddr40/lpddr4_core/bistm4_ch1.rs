#[doc = "Register `BISTM4_CH1` reader"]
pub struct R(crate::R<BISTM4_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTM4_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTM4_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTM4_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BISTM4_CH1` writer"]
pub struct W(crate::W<BISTM4_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BISTM4_CH1_SPEC>;
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
impl From<crate::W<BISTM4_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BISTM4_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARCH_ELEMENT_4` reader - March element 4 - Channel 1"]
pub type MARCH_ELEMENT_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MARCH_ELEMENT_4` writer - March element 4 - Channel 1"]
pub type MARCH_ELEMENT_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTM4_CH1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - March element 4 - Channel 1"]
    #[inline(always)]
    pub fn march_element_4(&self) -> MARCH_ELEMENT_4_R {
        MARCH_ELEMENT_4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - March element 4 - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn march_element_4(&mut self) -> MARCH_ELEMENT_4_W<0> {
        MARCH_ELEMENT_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST March Element Register 4 - Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bistm4_ch1](index.html) module"]
pub struct BISTM4_CH1_SPEC;
impl crate::RegisterSpec for BISTM4_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bistm4_ch1::R](R) reader structure"]
impl crate::Readable for BISTM4_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bistm4_ch1::W](W) writer structure"]
impl crate::Writable for BISTM4_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BISTM4_CH1 to value 0x4e00_0000"]
impl crate::Resettable for BISTM4_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4e00_0000;
}
