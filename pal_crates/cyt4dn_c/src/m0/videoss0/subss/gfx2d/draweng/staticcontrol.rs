#[doc = "Register `STATICCONTROL` reader"]
pub struct R(crate::R<STATICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCONTROL` writer"]
pub struct W(crate::W<STATICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONTROL_SPEC>;
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
impl From<crate::W<STATICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRESET` reader - Puts the Drawing Engine in software reset."]
pub type SWRESET_R = crate::BitReader<SWRESET_A>;
#[doc = "Puts the Drawing Engine in software reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRESET_A {
    #[doc = "0: Normal Operation"]
    OPERATION = 0,
    #[doc = "1: Software Reset"]
    SW_RESET = 1,
}
impl From<SWRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SWRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRESET_A {
        match self.bits {
            false => SWRESET_A::OPERATION,
            true => SWRESET_A::SW_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == SWRESET_A::OPERATION
    }
    #[doc = "Checks if the value of the field is `SW_RESET`"]
    #[inline(always)]
    pub fn is_sw_reset(&self) -> bool {
        *self == SWRESET_A::SW_RESET
    }
}
#[doc = "Field `SWRESET` writer - Puts the Drawing Engine in software reset."]
pub type SWRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, SWRESET_A, O>;
impl<'a, const O: u8> SWRESET_W<'a, O> {
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(SWRESET_A::OPERATION)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn sw_reset(self) -> &'a mut W {
        self.variant(SWRESET_A::SW_RESET)
    }
}
#[doc = "Field `CLOCKDISABLE` reader - Deactivates most internal clocks of Drawing Engine. Can only be activated if SWReset field is set to SW_RESET."]
pub type CLOCKDISABLE_R = crate::BitReader<CLOCKDISABLE_A>;
#[doc = "Deactivates most internal clocks of Drawing Engine. Can only be activated if SWReset field is set to SW_RESET.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCKDISABLE_A {
    #[doc = "0: Normal Operation"]
    OPERATION = 0,
    #[doc = "1: Clocks are off"]
    POWERDOWN = 1,
}
impl From<CLOCKDISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCKDISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCKDISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCKDISABLE_A {
        match self.bits {
            false => CLOCKDISABLE_A::OPERATION,
            true => CLOCKDISABLE_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == CLOCKDISABLE_A::OPERATION
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == CLOCKDISABLE_A::POWERDOWN
    }
}
#[doc = "Field `CLOCKDISABLE` writer - Deactivates most internal clocks of Drawing Engine. Can only be activated if SWReset field is set to SW_RESET."]
pub type CLOCKDISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, CLOCKDISABLE_A, O>;
impl<'a, const O: u8> CLOCKDISABLE_W<'a, O> {
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(CLOCKDISABLE_A::OPERATION)
    }
    #[doc = "Clocks are off"]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(CLOCKDISABLE_A::POWERDOWN)
    }
}
impl R {
    #[doc = "Bit 4 - Puts the Drawing Engine in software reset."]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Deactivates most internal clocks of Drawing Engine. Can only be activated if SWReset field is set to SW_RESET."]
    #[inline(always)]
    pub fn clockdisable(&self) -> CLOCKDISABLE_R {
        CLOCKDISABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Puts the Drawing Engine in software reset."]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SWRESET_W<4> {
        SWRESET_W::new(self)
    }
    #[doc = "Bit 5 - Deactivates most internal clocks of Drawing Engine. Can only be activated if SWReset field is set to SW_RESET."]
    #[inline(always)]
    #[must_use]
    pub fn clockdisable(&mut self) -> CLOCKDISABLE_W<5> {
        CLOCKDISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Drawing Engine static control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
pub struct STATICCONTROL_SPEC;
impl crate::RegisterSpec for STATICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticcontrol::R](R) reader structure"]
impl crate::Readable for STATICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticcontrol::W](W) writer structure"]
impl crate::Writable for STATICCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICCONTROL to value 0x30"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
