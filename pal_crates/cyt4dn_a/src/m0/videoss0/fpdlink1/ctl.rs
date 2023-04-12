#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_TX` reader - Power down bit, when high all blocks inside the s40efpdlink block shall be powered down"]
pub type PD_TX_R = crate::BitReader<PD_TX_A>;
#[doc = "Power down bit, when high all blocks inside the s40efpdlink block shall be powered down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD_TX_A {
    #[doc = "1: Power down FPDLINK, this is the default reset value"]
    FPDLINK_POWER_DOWN = 1,
    #[doc = "0: Power up FPDLINK"]
    FPDLINK_POWER_UP = 0,
}
impl From<PD_TX_A> for bool {
    #[inline(always)]
    fn from(variant: PD_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl PD_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_TX_A {
        match self.bits {
            true => PD_TX_A::FPDLINK_POWER_DOWN,
            false => PD_TX_A::FPDLINK_POWER_UP,
        }
    }
    #[doc = "Checks if the value of the field is `FPDLINK_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_fpdlink_power_down(&self) -> bool {
        *self == PD_TX_A::FPDLINK_POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `FPDLINK_POWER_UP`"]
    #[inline(always)]
    pub fn is_fpdlink_power_up(&self) -> bool {
        *self == PD_TX_A::FPDLINK_POWER_UP
    }
}
#[doc = "Field `PD_TX` writer - Power down bit, when high all blocks inside the s40efpdlink block shall be powered down"]
pub type PD_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, PD_TX_A, O>;
impl<'a, const O: u8> PD_TX_W<'a, O> {
    #[doc = "Power down FPDLINK, this is the default reset value"]
    #[inline(always)]
    pub fn fpdlink_power_down(self) -> &'a mut W {
        self.variant(PD_TX_A::FPDLINK_POWER_DOWN)
    }
    #[doc = "Power up FPDLINK"]
    #[inline(always)]
    pub fn fpdlink_power_up(self) -> &'a mut W {
        self.variant(PD_TX_A::FPDLINK_POWER_UP)
    }
}
#[doc = "Field `ENABLED` reader - Enable bit for the FPDLink"]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "Enable bit for the FPDLink\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: Disable the entire mxvideoss_fpdlink"]
    DIS = 0,
    #[doc = "1: Enable FPDLink"]
    EN = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DIS,
            true => ENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ENABLED_A::EN
    }
}
#[doc = "Field `ENABLED` writer - Enable bit for the FPDLink"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, ENABLED_A, O>;
impl<'a, const O: u8> ENABLED_W<'a, O> {
    #[doc = "Disable the entire mxvideoss_fpdlink"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENABLED_A::DIS)
    }
    #[doc = "Enable FPDLink"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ENABLED_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - Power down bit, when high all blocks inside the s40efpdlink block shall be powered down"]
    #[inline(always)]
    pub fn pd_tx(&self) -> PD_TX_R {
        PD_TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Enable bit for the FPDLink"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down bit, when high all blocks inside the s40efpdlink block shall be powered down"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tx(&mut self) -> PD_TX_W<0> {
        PD_TX_W::new(self)
    }
    #[doc = "Bit 31 - Enable bit for the FPDLink"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Control register for fpdlink\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x01"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
