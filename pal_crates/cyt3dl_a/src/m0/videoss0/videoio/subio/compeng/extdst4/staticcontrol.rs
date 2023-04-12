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
#[doc = "Field `KICK_MODE` reader - Operation mode of generated kick signal"]
pub type KICK_MODE_R = crate::BitReader<KICK_MODE_A>;
#[doc = "Operation mode of generated kick signal\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KICK_MODE_A {
    #[doc = "0: kick generation by KICK field only"]
    SOFTWARE = 0,
    #[doc = "1: kick signal from external allowed"]
    EXTERNAL = 1,
}
impl From<KICK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: KICK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl KICK_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KICK_MODE_A {
        match self.bits {
            false => KICK_MODE_A::SOFTWARE,
            true => KICK_MODE_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == KICK_MODE_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == KICK_MODE_A::EXTERNAL
    }
}
#[doc = "Field `KICK_MODE` writer - Operation mode of generated kick signal"]
pub type KICK_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, KICK_MODE_A, O>;
impl<'a, const O: u8> KICK_MODE_W<'a, O> {
    #[doc = "kick generation by KICK field only"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(KICK_MODE_A::SOFTWARE)
    }
    #[doc = "kick signal from external allowed"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(KICK_MODE_A::EXTERNAL)
    }
}
#[doc = "Field `PERFCOUNTMODE` reader - Value 1 enables performance counter mode, which does not generate an output frame but processes input data as fast as possible instead. Can be used to determine the maximum possible read-out performance of display buffers."]
pub type PERFCOUNTMODE_R = crate::BitReader<bool>;
#[doc = "Field `PERFCOUNTMODE` writer - Value 1 enables performance counter mode, which does not generate an output frame but processes input data as fast as possible instead. Can be used to determine the maximum possible read-out performance of display buffers."]
pub type PERFCOUNTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Operation mode of generated kick signal"]
    #[inline(always)]
    pub fn kick_mode(&self) -> KICK_MODE_R {
        KICK_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Value 1 enables performance counter mode, which does not generate an output frame but processes input data as fast as possible instead. Can be used to determine the maximum possible read-out performance of display buffers."]
    #[inline(always)]
    pub fn perfcountmode(&self) -> PERFCOUNTMODE_R {
        PERFCOUNTMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bit 8 - Operation mode of generated kick signal"]
    #[inline(always)]
    #[must_use]
    pub fn kick_mode(&mut self) -> KICK_MODE_W<8> {
        KICK_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Value 1 enables performance counter mode, which does not generate an output frame but processes input data as fast as possible instead. Can be used to determine the maximum possible read-out performance of display buffers."]
    #[inline(always)]
    #[must_use]
    pub fn perfcountmode(&mut self) -> PERFCOUNTMODE_W<12> {
        PERFCOUNTMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Destination static control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
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
#[doc = "`reset()` method sets STATICCONTROL to value 0x0100"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
