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
#[doc = "Field `BLUEWRITEENABLE` reader - Write enable for the blue color sampling point entries."]
pub type BLUEWRITEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BLUEWRITEENABLE` writer - Write enable for the blue color sampling point entries."]
pub type BLUEWRITEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `GREENWRITEENABLE` reader - Write enable for the green color sampling point entries."]
pub type GREENWRITEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GREENWRITEENABLE` writer - Write enable for the green color sampling point entries."]
pub type GREENWRITEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `REDWRITEENABLE` reader - Write enable for the red color sampling point entries."]
pub type REDWRITEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `REDWRITEENABLE` writer - Write enable for the red color sampling point entries."]
pub type REDWRITEENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write enable for the blue color sampling point entries."]
    #[inline(always)]
    pub fn bluewriteenable(&self) -> BLUEWRITEENABLE_R {
        BLUEWRITEENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write enable for the green color sampling point entries."]
    #[inline(always)]
    pub fn greenwriteenable(&self) -> GREENWRITEENABLE_R {
        GREENWRITEENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write enable for the red color sampling point entries."]
    #[inline(always)]
    pub fn redwriteenable(&self) -> REDWRITEENABLE_R {
        REDWRITEENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bit 1 - Write enable for the blue color sampling point entries."]
    #[inline(always)]
    #[must_use]
    pub fn bluewriteenable(&mut self) -> BLUEWRITEENABLE_W<1> {
        BLUEWRITEENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Write enable for the green color sampling point entries."]
    #[inline(always)]
    #[must_use]
    pub fn greenwriteenable(&mut self) -> GREENWRITEENABLE_W<2> {
        GREENWRITEENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Write enable for the red color sampling point entries."]
    #[inline(always)]
    #[must_use]
    pub fn redwriteenable(&mut self) -> REDWRITEENABLE_W<3> {
        REDWRITEENABLE_W::new(self)
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
#[doc = "`reset()` method sets STATICCONTROL to value 0x0e"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
