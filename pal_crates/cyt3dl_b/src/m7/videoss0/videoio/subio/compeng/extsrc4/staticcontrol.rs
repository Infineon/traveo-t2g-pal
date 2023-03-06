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
#[doc = "Field `SHDEN` reader - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `STARTSEL` reader - Start select selects the mode to start processing operations."]
pub type STARTSEL_R = crate::BitReader<STARTSEL_A>;
#[doc = "Start select selects the mode to start processing operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTSEL_A {
    #[doc = "0: Free running mode, use the incoming command word as trigger to start processing."]
    INPUT = 0,
    #[doc = "1: Keep to local start configuration, use external kick or SW trigger to start processing. The behavior is similar to fetch units."]
    LOCAL = 1,
}
impl From<STARTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: STARTSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTSEL_A {
        match self.bits {
            false => STARTSEL_A::INPUT,
            true => STARTSEL_A::LOCAL,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == STARTSEL_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == STARTSEL_A::LOCAL
    }
}
#[doc = "Field `STARTSEL` writer - Start select selects the mode to start processing operations."]
pub type STARTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, STARTSEL_A, O>;
impl<'a, const O: u8> STARTSEL_W<'a, O> {
    #[doc = "Free running mode, use the incoming command word as trigger to start processing."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(STARTSEL_A::INPUT)
    }
    #[doc = "Keep to local start configuration, use external kick or SW trigger to start processing. The behavior is similar to fetch units."]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(STARTSEL_A::LOCAL)
    }
}
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Start select selects the mode to start processing operations."]
    #[inline(always)]
    pub fn startsel(&self) -> STARTSEL_R {
        STARTSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bit 8 - Start select selects the mode to start processing operations."]
    #[inline(always)]
    #[must_use]
    pub fn startsel(&mut self) -> STARTSEL_W<8> {
        STARTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ExtSrc static control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
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
#[doc = "`reset()` method sets STATICCONTROL to value 0"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
