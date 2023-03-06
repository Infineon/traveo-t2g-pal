#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSPCLKDIVIDE` reader - Controls generation of display clock signals for display stream."]
pub type DSPCLKDIVIDE_R = crate::BitReader<DSPCLKDIVIDE_A>;
#[doc = "Controls generation of display clock signals for display stream.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSPCLKDIVIDE_A {
    #[doc = "0: External display clock signal has pixel clock frequency."]
    DIV1 = 0,
    #[doc = "1: External display clock signal has twice the pixel clock frequency."]
    DIV2 = 1,
}
impl From<DSPCLKDIVIDE_A> for bool {
    #[inline(always)]
    fn from(variant: DSPCLKDIVIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl DSPCLKDIVIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSPCLKDIVIDE_A {
        match self.bits {
            false => DSPCLKDIVIDE_A::DIV1,
            true => DSPCLKDIVIDE_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DSPCLKDIVIDE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DSPCLKDIVIDE_A::DIV2
    }
}
#[doc = "Field `DSPCLKDIVIDE` writer - Controls generation of display clock signals for display stream."]
pub type DSPCLKDIVIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, DSPCLKDIVIDE_A, O>;
impl<'a, const O: u8> DSPCLKDIVIDE_W<'a, O> {
    #[doc = "External display clock signal has pixel clock frequency."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DSPCLKDIVIDE_A::DIV1)
    }
    #[doc = "External display clock signal has twice the pixel clock frequency."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DSPCLKDIVIDE_A::DIV2)
    }
}
impl R {
    #[doc = "Bit 0 - Controls generation of display clock signals for display stream."]
    #[inline(always)]
    pub fn dspclkdivide(&self) -> DSPCLKDIVIDE_R {
        DSPCLKDIVIDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls generation of display clock signals for display stream."]
    #[inline(always)]
    #[must_use]
    pub fn dspclkdivide(&mut self) -> DSPCLKDIVIDE_W<0> {
        DSPCLKDIVIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display engine control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x01"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
