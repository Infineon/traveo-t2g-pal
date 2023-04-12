#[doc = "Register `STATIC_CONTROL` reader"]
pub struct R(crate::R<STATIC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATIC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATIC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATIC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATIC_CONTROL` writer"]
pub struct W(crate::W<STATIC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATIC_CONTROL_SPEC>;
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
impl From<crate::W<STATIC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATIC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_ENABLED` reader - Enable bit for channel 0"]
pub type CH0_ENABLED_R = crate::BitReader<CH0_ENABLED_A>;
#[doc = "Enable bit for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_ENABLED_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<CH0_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_ENABLED_A {
        match self.bits {
            false => CH0_ENABLED_A::DIS,
            true => CH0_ENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH0_ENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH0_ENABLED_A::EN
    }
}
#[doc = "Field `CH0_ENABLED` writer - Enable bit for channel 0"]
pub type CH0_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH0_ENABLED_A, O>;
impl<'a, const O: u8> CH0_ENABLED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH0_ENABLED_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH0_ENABLED_A::EN)
    }
}
#[doc = "Field `MEASUREMENT_EN` reader - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
pub type MEASUREMENT_EN_R = crate::BitReader<MEASUREMENT_EN_A>;
#[doc = "Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEASUREMENT_EN_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<MEASUREMENT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MEASUREMENT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MEASUREMENT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASUREMENT_EN_A {
        match self.bits {
            false => MEASUREMENT_EN_A::DIS,
            true => MEASUREMENT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MEASUREMENT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEASUREMENT_EN_A::EN
    }
}
#[doc = "Field `MEASUREMENT_EN` writer - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
pub type MEASUREMENT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, MEASUREMENT_EN_A, O>;
impl<'a, const O: u8> MEASUREMENT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEASUREMENT_EN_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEASUREMENT_EN_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bit for channel 0"]
    #[inline(always)]
    pub fn ch0_enabled(&self) -> CH0_ENABLED_R {
        CH0_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
    #[inline(always)]
    pub fn measurement_en(&self) -> MEASUREMENT_EN_R {
        MEASUREMENT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_enabled(&mut self) -> CH0_ENABLED_W<0> {
        CH0_ENABLED_W::new(self)
    }
    #[doc = "Bit 8 - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
    #[inline(always)]
    #[must_use]
    pub fn measurement_en(&mut self) -> MEASUREMENT_EN_W<8> {
        MEASUREMENT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [static_control](index.html) module"]
pub struct STATIC_CONTROL_SPEC;
impl crate::RegisterSpec for STATIC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [static_control::R](R) reader structure"]
impl crate::Readable for STATIC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [static_control::W](W) writer structure"]
impl crate::Writable for STATIC_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATIC_CONTROL to value 0"]
impl crate::Resettable for STATIC_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
