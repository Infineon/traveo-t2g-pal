#[doc = "Register `FPDLINKCFG` reader"]
pub struct R(crate::R<FPDLINKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPDLINKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPDLINKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPDLINKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPDLINKCFG` writer"]
pub struct W(crate::W<FPDLINKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPDLINKCFG_SPEC>;
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
impl From<crate::W<FPDLINKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPDLINKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPDLINKSEL` reader - Selects the channel mode for the two FDP-link interfaces."]
pub type FPDLINKSEL_R = crate::BitReader<FPDLINKSEL_A>;
#[doc = "Selects the channel mode for the two FDP-link interfaces.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDLINKSEL_A {
    #[doc = "0: FPD-link 0 and 1 are operating independently, driven by the two Display Engine 0 and 1."]
    SINGLE = 0,
    #[doc = "1: FPD-link 0 and 1 are operating synchronously, both driven by Display Engine 0."]
    DUAL = 1,
}
impl From<FPDLINKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FPDLINKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FPDLINKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDLINKSEL_A {
        match self.bits {
            false => FPDLINKSEL_A::SINGLE,
            true => FPDLINKSEL_A::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == FPDLINKSEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == FPDLINKSEL_A::DUAL
    }
}
#[doc = "Field `FPDLINKSEL` writer - Selects the channel mode for the two FDP-link interfaces."]
pub type FPDLINKSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FPDLINKCFG_SPEC, FPDLINKSEL_A, O>;
impl<'a, const O: u8> FPDLINKSEL_W<'a, O> {
    #[doc = "FPD-link 0 and 1 are operating independently, driven by the two Display Engine 0 and 1."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(FPDLINKSEL_A::SINGLE)
    }
    #[doc = "FPD-link 0 and 1 are operating synchronously, both driven by Display Engine 0."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(FPDLINKSEL_A::DUAL)
    }
}
impl R {
    #[doc = "Bit 0 - Selects the channel mode for the two FDP-link interfaces."]
    #[inline(always)]
    pub fn fpdlinksel(&self) -> FPDLINKSEL_R {
        FPDLINKSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the channel mode for the two FDP-link interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn fpdlinksel(&mut self) -> FPDLINKSEL_W<0> {
        FPDLINKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FPDLink Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdlinkcfg](index.html) module"]
pub struct FPDLINKCFG_SPEC;
impl crate::RegisterSpec for FPDLINKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpdlinkcfg::R](R) reader structure"]
impl crate::Readable for FPDLINKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpdlinkcfg::W](W) writer structure"]
impl crate::Writable for FPDLINKCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPDLINKCFG to value 0"]
impl crate::Resettable for FPDLINKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
