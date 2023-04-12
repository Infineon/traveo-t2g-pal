#[doc = "Register `ILE` reader"]
pub struct R(crate::R<ILE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILE` writer"]
pub struct W(crate::W<ILE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILE_SPEC>;
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
impl From<crate::W<ILE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EINT0_` reader - Enable Interrupt Line 0 1 = Interrupt line eray_int0 enabled 0 = Interrupt line eray_int0 disabled"]
pub type EINT0__R = crate::BitReader<EINT0__A>;
#[doc = "Enable Interrupt Line 0 1 = Interrupt line eray_int0 enabled 0 = Interrupt line eray_int0 disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT0__A {
    #[doc = "0: N/A"]
    ERAY_INT0_DISABLED = 0,
    #[doc = "1: N/A"]
    ERAY_INT0_ENABLED = 1,
}
impl From<EINT0__A> for bool {
    #[inline(always)]
    fn from(variant: EINT0__A) -> Self {
        variant as u8 != 0
    }
}
impl EINT0__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT0__A {
        match self.bits {
            false => EINT0__A::ERAY_INT0_DISABLED,
            true => EINT0__A::ERAY_INT0_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_INT0_DISABLED`"]
    #[inline(always)]
    pub fn is_eray_int0_disabled(&self) -> bool {
        *self == EINT0__A::ERAY_INT0_DISABLED
    }
    #[doc = "Checks if the value of the field is `ERAY_INT0_ENABLED`"]
    #[inline(always)]
    pub fn is_eray_int0_enabled(&self) -> bool {
        *self == EINT0__A::ERAY_INT0_ENABLED
    }
}
#[doc = "Field `EINT0_` writer - Enable Interrupt Line 0 1 = Interrupt line eray_int0 enabled 0 = Interrupt line eray_int0 disabled"]
pub type EINT0__W<'a, const O: u8> = crate::BitWriter<'a, u32, ILE_SPEC, EINT0__A, O>;
impl<'a, const O: u8> EINT0__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_int0_disabled(self) -> &'a mut W {
        self.variant(EINT0__A::ERAY_INT0_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_int0_enabled(self) -> &'a mut W {
        self.variant(EINT0__A::ERAY_INT0_ENABLED)
    }
}
#[doc = "Field `EINT1_` reader - Enable Interrupt Line 1 1 = Interrupt line eray_int1 enabled 0 = Interrupt line eray_int1 disabled"]
pub type EINT1__R = crate::BitReader<EINT1__A>;
#[doc = "Enable Interrupt Line 1 1 = Interrupt line eray_int1 enabled 0 = Interrupt line eray_int1 disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT1__A {
    #[doc = "0: N/A"]
    ERAY_INT1_DISABLED = 0,
    #[doc = "1: N/A"]
    ERAY_INT1_ENABLED = 1,
}
impl From<EINT1__A> for bool {
    #[inline(always)]
    fn from(variant: EINT1__A) -> Self {
        variant as u8 != 0
    }
}
impl EINT1__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT1__A {
        match self.bits {
            false => EINT1__A::ERAY_INT1_DISABLED,
            true => EINT1__A::ERAY_INT1_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_INT1_DISABLED`"]
    #[inline(always)]
    pub fn is_eray_int1_disabled(&self) -> bool {
        *self == EINT1__A::ERAY_INT1_DISABLED
    }
    #[doc = "Checks if the value of the field is `ERAY_INT1_ENABLED`"]
    #[inline(always)]
    pub fn is_eray_int1_enabled(&self) -> bool {
        *self == EINT1__A::ERAY_INT1_ENABLED
    }
}
#[doc = "Field `EINT1_` writer - Enable Interrupt Line 1 1 = Interrupt line eray_int1 enabled 0 = Interrupt line eray_int1 disabled"]
pub type EINT1__W<'a, const O: u8> = crate::BitWriter<'a, u32, ILE_SPEC, EINT1__A, O>;
impl<'a, const O: u8> EINT1__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_int1_disabled(self) -> &'a mut W {
        self.variant(EINT1__A::ERAY_INT1_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_int1_enabled(self) -> &'a mut W {
        self.variant(EINT1__A::ERAY_INT1_ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Interrupt Line 0 1 = Interrupt line eray_int0 enabled 0 = Interrupt line eray_int0 disabled"]
    #[inline(always)]
    pub fn eint0_(&self) -> EINT0__R {
        EINT0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1 1 = Interrupt line eray_int1 enabled 0 = Interrupt line eray_int1 disabled"]
    #[inline(always)]
    pub fn eint1_(&self) -> EINT1__R {
        EINT1__R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0 1 = Interrupt line eray_int0 enabled 0 = Interrupt line eray_int0 disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eint0_(&mut self) -> EINT0__W<0> {
        EINT0__W::new(self)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1 1 = Interrupt line eray_int1 enabled 0 = Interrupt line eray_int1 disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eint1_(&mut self) -> EINT1__W<1> {
        EINT1__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Line Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ile](index.html) module"]
pub struct ILE_SPEC;
impl crate::RegisterSpec for ILE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ile::R](R) reader structure"]
impl crate::Readable for ILE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ile::W](W) writer structure"]
impl crate::Writable for ILE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for ILE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
