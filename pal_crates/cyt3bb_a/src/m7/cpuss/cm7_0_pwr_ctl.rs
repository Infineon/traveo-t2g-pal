#[doc = "Register `CM7_0_PWR_CTL` reader"]
pub struct R(crate::R<CM7_0_PWR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_0_PWR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_0_PWR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_0_PWR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_0_PWR_CTL` writer"]
pub struct W(crate::W<CM7_0_PWR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_0_PWR_CTL_SPEC>;
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
impl From<crate::W<CM7_0_PWR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_0_PWR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_MODE` reader - Power mode."]
pub type PWR_MODE_R = crate::FieldReader<u8, PWR_MODE_A>;
#[doc = "Power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Switch CM7_0 off Power off, clock off, isolate, reset and no retain."]
    OFF = 0,
    #[doc = "1: Reset CM7_0 Clock off, no isolated, no retain and reset. Note: The CM7_0 CPU has a AIRCR.SYSRESETREQ register field that allows the CM7_0 to reset the complete device (RESET only resets the CM7_0), resulting in a warm boot."]
    RESET = 1,
    #[doc = "2: Put CM7_0 in Retained mode This can only become effective if CM7_0 is in SleepDeep mode. Check PWR_DONE flag to see if CM7_0 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    RETAINED = 2,
    #[doc = "3: Switch CM7_0 on. Power on, clock on, no isolate, no reset and no retain."]
    ENABLED = 3,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
impl PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_MODE_A {
        match self.bits {
            0 => PWR_MODE_A::OFF,
            1 => PWR_MODE_A::RESET,
            2 => PWR_MODE_A::RETAINED,
            3 => PWR_MODE_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PWR_MODE_A::RESET
    }
    #[doc = "Checks if the value of the field is `RETAINED`"]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PWR_MODE_A::RETAINED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWR_MODE_A::ENABLED
    }
}
#[doc = "Field `PWR_MODE` writer - Power mode."]
pub type PWR_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CM7_0_PWR_CTL_SPEC, u8, PWR_MODE_A, 2, O>;
impl<'a, const O: u8> PWR_MODE_W<'a, O> {
    #[doc = "Switch CM7_0 off Power off, clock off, isolate, reset and no retain."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "Reset CM7_0 Clock off, no isolated, no retain and reset. Note: The CM7_0 CPU has a AIRCR.SYSRESETREQ register field that allows the CM7_0 to reset the complete device (RESET only resets the CM7_0), resulting in a warm boot."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RESET)
    }
    #[doc = "Put CM7_0 in Retained mode This can only become effective if CM7_0 is in SleepDeep mode. Check PWR_DONE flag to see if CM7_0 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "Switch CM7_0 on. Power on, clock on, no isolate, no reset and no retain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWR_MODE_A::ENABLED)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
pub type VECTKEYSTAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VECTKEYSTAT_R {
        VECTKEYSTAT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<0> {
        PWR_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM7 0 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_0_pwr_ctl](index.html) module"]
pub struct CM7_0_PWR_CTL_SPEC;
impl crate::RegisterSpec for CM7_0_PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_0_pwr_ctl::R](R) reader structure"]
impl crate::Readable for CM7_0_PWR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_0_pwr_ctl::W](W) writer structure"]
impl crate::Writable for CM7_0_PWR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_0_PWR_CTL to value 0xfa05_0000"]
impl crate::Resettable for CM7_0_PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0000;
}
