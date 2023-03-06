#[doc = "Register `SPR` reader"]
pub struct R(crate::R<SPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPR` writer"]
pub struct W(crate::W<SPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPR_SPEC>;
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
impl From<crate::W<SPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLEN` reader - Change polarity of video signal de."]
pub type POLEN_R = crate::BitReader<POLEN_A>;
#[doc = "Change polarity of video signal de.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLEN_A {
    #[doc = "0: Signal is active high (default)."]
    HIGHACT = 0,
    #[doc = "1: Signal is active low."]
    LOWACT = 1,
}
impl From<POLEN_A> for bool {
    #[inline(always)]
    fn from(variant: POLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl POLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLEN_A {
        match self.bits {
            false => POLEN_A::HIGHACT,
            true => POLEN_A::LOWACT,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHACT`"]
    #[inline(always)]
    pub fn is_highact(&self) -> bool {
        *self == POLEN_A::HIGHACT
    }
    #[doc = "Checks if the value of the field is `LOWACT`"]
    #[inline(always)]
    pub fn is_lowact(&self) -> bool {
        *self == POLEN_A::LOWACT
    }
}
#[doc = "Field `POLEN` writer - Change polarity of video signal de."]
pub type POLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPR_SPEC, POLEN_A, O>;
impl<'a, const O: u8> POLEN_W<'a, O> {
    #[doc = "Signal is active high (default)."]
    #[inline(always)]
    pub fn highact(self) -> &'a mut W {
        self.variant(POLEN_A::HIGHACT)
    }
    #[doc = "Signal is active low."]
    #[inline(always)]
    pub fn lowact(self) -> &'a mut W {
        self.variant(POLEN_A::LOWACT)
    }
}
#[doc = "Field `POLHS` reader - Change polarity of video signal hsync."]
pub type POLHS_R = crate::BitReader<POLHS_A>;
#[doc = "Change polarity of video signal hsync.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLHS_A {
    #[doc = "0: Signal is active low (default)."]
    LOWACT = 0,
    #[doc = "1: Signal is active high."]
    HIGHACT = 1,
}
impl From<POLHS_A> for bool {
    #[inline(always)]
    fn from(variant: POLHS_A) -> Self {
        variant as u8 != 0
    }
}
impl POLHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLHS_A {
        match self.bits {
            false => POLHS_A::LOWACT,
            true => POLHS_A::HIGHACT,
        }
    }
    #[doc = "Checks if the value of the field is `LOWACT`"]
    #[inline(always)]
    pub fn is_lowact(&self) -> bool {
        *self == POLHS_A::LOWACT
    }
    #[doc = "Checks if the value of the field is `HIGHACT`"]
    #[inline(always)]
    pub fn is_highact(&self) -> bool {
        *self == POLHS_A::HIGHACT
    }
}
#[doc = "Field `POLHS` writer - Change polarity of video signal hsync."]
pub type POLHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPR_SPEC, POLHS_A, O>;
impl<'a, const O: u8> POLHS_W<'a, O> {
    #[doc = "Signal is active low (default)."]
    #[inline(always)]
    pub fn lowact(self) -> &'a mut W {
        self.variant(POLHS_A::LOWACT)
    }
    #[doc = "Signal is active high."]
    #[inline(always)]
    pub fn highact(self) -> &'a mut W {
        self.variant(POLHS_A::HIGHACT)
    }
}
#[doc = "Field `POLVS` reader - Change polarity of video signal vsync."]
pub type POLVS_R = crate::BitReader<POLVS_A>;
#[doc = "Change polarity of video signal vsync.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLVS_A {
    #[doc = "0: Signal is active low (default)."]
    LOWACT = 0,
    #[doc = "1: Signal is active high."]
    HIGHACT = 1,
}
impl From<POLVS_A> for bool {
    #[inline(always)]
    fn from(variant: POLVS_A) -> Self {
        variant as u8 != 0
    }
}
impl POLVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLVS_A {
        match self.bits {
            false => POLVS_A::LOWACT,
            true => POLVS_A::HIGHACT,
        }
    }
    #[doc = "Checks if the value of the field is `LOWACT`"]
    #[inline(always)]
    pub fn is_lowact(&self) -> bool {
        *self == POLVS_A::LOWACT
    }
    #[doc = "Checks if the value of the field is `HIGHACT`"]
    #[inline(always)]
    pub fn is_highact(&self) -> bool {
        *self == POLVS_A::HIGHACT
    }
}
#[doc = "Field `POLVS` writer - Change polarity of video signal vsync."]
pub type POLVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPR_SPEC, POLVS_A, O>;
impl<'a, const O: u8> POLVS_W<'a, O> {
    #[doc = "Signal is active low (default)."]
    #[inline(always)]
    pub fn lowact(self) -> &'a mut W {
        self.variant(POLVS_A::LOWACT)
    }
    #[doc = "Signal is active high."]
    #[inline(always)]
    pub fn highact(self) -> &'a mut W {
        self.variant(POLVS_A::HIGHACT)
    }
}
impl R {
    #[doc = "Bit 0 - Change polarity of video signal de."]
    #[inline(always)]
    pub fn polen(&self) -> POLEN_R {
        POLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Change polarity of video signal hsync."]
    #[inline(always)]
    pub fn polhs(&self) -> POLHS_R {
        POLHS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change polarity of video signal vsync."]
    #[inline(always)]
    pub fn polvs(&self) -> POLVS_R {
        POLVS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Change polarity of video signal de."]
    #[inline(always)]
    #[must_use]
    pub fn polen(&mut self) -> POLEN_W<0> {
        POLEN_W::new(self)
    }
    #[doc = "Bit 1 - Change polarity of video signal hsync."]
    #[inline(always)]
    #[must_use]
    pub fn polhs(&mut self) -> POLHS_W<1> {
        POLHS_W::new(self)
    }
    #[doc = "Bit 2 - Change polarity of video signal vsync."]
    #[inline(always)]
    #[must_use]
    pub fn polvs(&mut self) -> POLVS_W<2> {
        POLVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap sync signal polarity configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr](index.html) module"]
pub struct SPR_SPEC;
impl crate::RegisterSpec for SPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr::R](R) reader structure"]
impl crate::Readable for SPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spr::W](W) writer structure"]
impl crate::Writable for SPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR to value 0"]
impl crate::Resettable for SPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
