#[doc = "Register `POLARITYCTRL` reader"]
pub struct R(crate::R<POLARITYCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLARITYCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POLARITYCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POLARITYCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLARITYCTRL` writer"]
pub struct W(crate::W<POLARITYCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLARITYCTRL_SPEC>;
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
impl From<crate::W<POLARITYCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POLARITYCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLHS` reader - Polarity of hsync signal."]
pub type POLHS_R = crate::BitReader<POLHS_A>;
#[doc = "Polarity of hsync signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLHS_A {
    #[doc = "0: Low active"]
    LOW = 0,
    #[doc = "1: High active"]
    HIGH = 1,
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
            false => POLHS_A::LOW,
            true => POLHS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLHS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLHS_A::HIGH
    }
}
#[doc = "Field `POLHS` writer - Polarity of hsync signal."]
pub type POLHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITYCTRL_SPEC, POLHS_A, O>;
impl<'a, const O: u8> POLHS_W<'a, O> {
    #[doc = "Low active"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POLHS_A::LOW)
    }
    #[doc = "High active"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POLHS_A::HIGH)
    }
}
#[doc = "Field `POLVS` reader - Polarity of vsync signal."]
pub type POLVS_R = crate::BitReader<POLVS_A>;
#[doc = "Polarity of vsync signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLVS_A {
    #[doc = "0: Low active"]
    LOW = 0,
    #[doc = "1: High active"]
    HIGH = 1,
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
            false => POLVS_A::LOW,
            true => POLVS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLVS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLVS_A::HIGH
    }
}
#[doc = "Field `POLVS` writer - Polarity of vsync signal."]
pub type POLVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITYCTRL_SPEC, POLVS_A, O>;
impl<'a, const O: u8> POLVS_W<'a, O> {
    #[doc = "Low active"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POLVS_A::LOW)
    }
    #[doc = "High active"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POLVS_A::HIGH)
    }
}
#[doc = "Field `POLEN` reader - Polarity of Data_Enable signal."]
pub type POLEN_R = crate::BitReader<POLEN_A>;
#[doc = "Polarity of Data_Enable signal.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLEN_A {
    #[doc = "0: Low active"]
    LOW = 0,
    #[doc = "1: High active"]
    HIGH = 1,
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
            false => POLEN_A::LOW,
            true => POLEN_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLEN_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLEN_A::HIGH
    }
}
#[doc = "Field `POLEN` writer - Polarity of Data_Enable signal."]
pub type POLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITYCTRL_SPEC, POLEN_A, O>;
impl<'a, const O: u8> POLEN_W<'a, O> {
    #[doc = "Low active"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POLEN_A::LOW)
    }
    #[doc = "High active"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POLEN_A::HIGH)
    }
}
#[doc = "Field `PIXINV` reader - Inversion of RGB data."]
pub type PIXINV_R = crate::BitReader<PIXINV_A>;
#[doc = "Inversion of RGB data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXINV_A {
    #[doc = "0: No inversion of pixel data"]
    NONINV = 0,
    #[doc = "1: Pixel data inverted (1. complement)"]
    INV = 1,
}
impl From<PIXINV_A> for bool {
    #[inline(always)]
    fn from(variant: PIXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl PIXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXINV_A {
        match self.bits {
            false => PIXINV_A::NONINV,
            true => PIXINV_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NONINV`"]
    #[inline(always)]
    pub fn is_noninv(&self) -> bool {
        *self == PIXINV_A::NONINV
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == PIXINV_A::INV
    }
}
#[doc = "Field `PIXINV` writer - Inversion of RGB data."]
pub type PIXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITYCTRL_SPEC, PIXINV_A, O>;
impl<'a, const O: u8> PIXINV_W<'a, O> {
    #[doc = "No inversion of pixel data"]
    #[inline(always)]
    pub fn noninv(self) -> &'a mut W {
        self.variant(PIXINV_A::NONINV)
    }
    #[doc = "Pixel data inverted (1. complement)"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(PIXINV_A::INV)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of hsync signal."]
    #[inline(always)]
    pub fn polhs(&self) -> POLHS_R {
        POLHS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity of vsync signal."]
    #[inline(always)]
    pub fn polvs(&self) -> POLVS_R {
        POLVS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Polarity of Data_Enable signal."]
    #[inline(always)]
    pub fn polen(&self) -> POLEN_R {
        POLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Inversion of RGB data."]
    #[inline(always)]
    pub fn pixinv(&self) -> PIXINV_R {
        PIXINV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of hsync signal."]
    #[inline(always)]
    #[must_use]
    pub fn polhs(&mut self) -> POLHS_W<0> {
        POLHS_W::new(self)
    }
    #[doc = "Bit 1 - Polarity of vsync signal."]
    #[inline(always)]
    #[must_use]
    pub fn polvs(&mut self) -> POLVS_W<1> {
        POLVS_W::new(self)
    }
    #[doc = "Bit 2 - Polarity of Data_Enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn polen(&mut self) -> POLEN_W<2> {
        POLEN_W::new(self)
    }
    #[doc = "Bit 3 - Inversion of RGB data."]
    #[inline(always)]
    #[must_use]
    pub fn pixinv(&mut self) -> PIXINV_W<3> {
        PIXINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polarity control for TCon input and corresponding top-level output (TCon by-pass port).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarityctrl](index.html) module"]
pub struct POLARITYCTRL_SPEC;
impl crate::RegisterSpec for POLARITYCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [polarityctrl::R](R) reader structure"]
impl crate::Readable for POLARITYCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [polarityctrl::W](W) writer structure"]
impl crate::Writable for POLARITYCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POLARITYCTRL to value 0x04"]
impl crate::Resettable for POLARITYCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
