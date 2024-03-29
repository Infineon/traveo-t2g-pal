#[doc = "Register `RAM1_PWR_CTL` reader"]
pub struct R(crate::R<RAM1_PWR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM1_PWR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM1_PWR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM1_PWR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM1_PWR_CTL` writer"]
pub struct W(crate::W<RAM1_PWR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM1_PWR_CTL_SPEC>;
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
impl From<crate::W<RAM1_PWR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM1_PWR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_MODE` reader - Power mode."]
pub type PWR_MODE_R = crate::FieldReader<u8, PWR_MODE_A>;
#[doc = "Power mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Refer RAM0_PWR_MACRO_CTL description."]
    OFF = 0,
    #[doc = "1: Undefined."]
    RSVD = 1,
    #[doc = "2: Refer RAM0_PWR_MACRO_CTL description."]
    RETAINED = 2,
    #[doc = "3: Refer RAM0_PWR_MACRO_CTL description."]
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
            1 => PWR_MODE_A::RSVD,
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
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PWR_MODE_A::RSVD
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
    crate::FieldWriterSafe<'a, u32, RAM1_PWR_CTL_SPEC, u8, PWR_MODE_A, 2, O>;
impl<'a, const O: u8> PWR_MODE_W<'a, O> {
    #[doc = "Refer RAM0_PWR_MACRO_CTL description."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "Undefined."]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RSVD)
    }
    #[doc = "Refer RAM0_PWR_MACRO_CTL description."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "Refer RAM0_PWR_MACRO_CTL description."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWR_MODE_A::ENABLED)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Refer RAM0_PWR_MACRO_CTL description."]
pub type VECTKEYSTAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Refer RAM0_PWR_MACRO_CTL description."]
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
#[doc = "RAM 1 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_pwr_ctl](index.html) module"]
pub struct RAM1_PWR_CTL_SPEC;
impl crate::RegisterSpec for RAM1_PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram1_pwr_ctl::R](R) reader structure"]
impl crate::Readable for RAM1_PWR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram1_pwr_ctl::W](W) writer structure"]
impl crate::Writable for RAM1_PWR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM1_PWR_CTL to value 0xfa05_0003"]
impl crate::Resettable for RAM1_PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0003;
}
