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
#[doc = "Field `SHDEN` reader - Enables shadowing of all RWS type fields of this unit (0 = write through, 1 = shadowed)."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enables shadowing of all RWS type fields of this unit (0 = write through, 1 = shadowed)."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `SHDLDSEL` reader - Controls when shadow fields of this unit are loaded into the active configuration in case that ShdEn is enabled."]
pub type SHDLDSEL_R = crate::FieldReader<u8, SHDLDSEL_A>;
#[doc = "Controls when shadow fields of this unit are loaded into the active configuration in case that ShdEn is enabled.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHDLDSEL_A {
    #[doc = "0: Load shadows with shadow load token on primary input (background plane)."]
    PRIMARY = 0,
    #[doc = "1: Load shadows with shadow load token on secondary input (foreground plane)."]
    SECONDARY = 1,
    #[doc = "2: Load shadows with shadow load token on any input."]
    BOTH = 2,
}
impl From<SHDLDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHDLDSEL_A) -> Self {
        variant as _
    }
}
impl SHDLDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SHDLDSEL_A> {
        match self.bits {
            0 => Some(SHDLDSEL_A::PRIMARY),
            1 => Some(SHDLDSEL_A::SECONDARY),
            2 => Some(SHDLDSEL_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == SHDLDSEL_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == SHDLDSEL_A::SECONDARY
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SHDLDSEL_A::BOTH
    }
}
#[doc = "Field `SHDLDSEL` writer - Controls when shadow fields of this unit are loaded into the active configuration in case that ShdEn is enabled."]
pub type SHDLDSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCONTROL_SPEC, u8, SHDLDSEL_A, 2, O>;
impl<'a, const O: u8> SHDLDSEL_W<'a, O> {
    #[doc = "Load shadows with shadow load token on primary input (background plane)."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(SHDLDSEL_A::PRIMARY)
    }
    #[doc = "Load shadows with shadow load token on secondary input (foreground plane)."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(SHDLDSEL_A::SECONDARY)
    }
    #[doc = "Load shadows with shadow load token on any input."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SHDLDSEL_A::BOTH)
    }
}
#[doc = "Field `SHDTOKSEL` reader - Controls when a shadow load token is generated on the output port, which controls shadow load in subsequent processing units."]
pub type SHDTOKSEL_R = crate::FieldReader<u8, SHDTOKSEL_A>;
#[doc = "Controls when a shadow load token is generated on the output port, which controls shadow load in subsequent processing units.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHDTOKSEL_A {
    #[doc = "0: When a token was received on the primary input (background plane)."]
    PRIMARY = 0,
    #[doc = "1: When a token was received on the secondary input (foreground plane)."]
    SECONDARY = 1,
    #[doc = "2: When a token was received on any input."]
    BOTH = 2,
}
impl From<SHDTOKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHDTOKSEL_A) -> Self {
        variant as _
    }
}
impl SHDTOKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SHDTOKSEL_A> {
        match self.bits {
            0 => Some(SHDTOKSEL_A::PRIMARY),
            1 => Some(SHDTOKSEL_A::SECONDARY),
            2 => Some(SHDTOKSEL_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == SHDTOKSEL_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == SHDTOKSEL_A::SECONDARY
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SHDTOKSEL_A::BOTH
    }
}
#[doc = "Field `SHDTOKSEL` writer - Controls when a shadow load token is generated on the output port, which controls shadow load in subsequent processing units."]
pub type SHDTOKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCONTROL_SPEC, u8, SHDTOKSEL_A, 2, O>;
impl<'a, const O: u8> SHDTOKSEL_W<'a, O> {
    #[doc = "When a token was received on the primary input (background plane)."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(SHDTOKSEL_A::PRIMARY)
    }
    #[doc = "When a token was received on the secondary input (foreground plane)."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(SHDTOKSEL_A::SECONDARY)
    }
    #[doc = "When a token was received on any input."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SHDTOKSEL_A::BOTH)
    }
}
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type fields of this unit (0 = write through, 1 = shadowed)."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Controls when shadow fields of this unit are loaded into the active configuration in case that ShdEn is enabled."]
    #[inline(always)]
    pub fn shdldsel(&self) -> SHDLDSEL_R {
        SHDLDSEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Controls when a shadow load token is generated on the output port, which controls shadow load in subsequent processing units."]
    #[inline(always)]
    pub fn shdtoksel(&self) -> SHDTOKSEL_R {
        SHDTOKSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type fields of this unit (0 = write through, 1 = shadowed)."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Controls when shadow fields of this unit are loaded into the active configuration in case that ShdEn is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn shdldsel(&mut self) -> SHDLDSEL_W<1> {
        SHDLDSEL_W::new(self)
    }
    #[doc = "Bits 3:4 - Controls when a shadow load token is generated on the output port, which controls shadow load in subsequent processing units."]
    #[inline(always)]
    #[must_use]
    pub fn shdtoksel(&mut self) -> SHDTOKSEL_W<3> {
        SHDTOKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static control settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
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
#[doc = "`reset()` method sets STATICCONTROL to value 0x14"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
