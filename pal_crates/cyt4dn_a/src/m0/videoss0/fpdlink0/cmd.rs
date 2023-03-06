#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVDS_TX_EN` reader - Enable the serializers and TX drivers for all lanes of s40efpdlink"]
pub type LVDS_TX_EN_R = crate::BitReader<LVDS_TX_EN_A>;
#[doc = "Enable the serializers and TX drivers for all lanes of s40efpdlink\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDS_TX_EN_A {
    #[doc = "0: Disable fpdlink TX operation"]
    FPDLINK_TX_DRIVER_DISABLED = 0,
    #[doc = "1: Enable fpdlink TX operation"]
    FPDLINK_TX_DRIVER_ENABLED = 1,
}
impl From<LVDS_TX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_TX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDS_TX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_TX_EN_A {
        match self.bits {
            false => LVDS_TX_EN_A::FPDLINK_TX_DRIVER_DISABLED,
            true => LVDS_TX_EN_A::FPDLINK_TX_DRIVER_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FPDLINK_TX_DRIVER_DISABLED`"]
    #[inline(always)]
    pub fn is_fpdlink_tx_driver_disabled(&self) -> bool {
        *self == LVDS_TX_EN_A::FPDLINK_TX_DRIVER_DISABLED
    }
    #[doc = "Checks if the value of the field is `FPDLINK_TX_DRIVER_ENABLED`"]
    #[inline(always)]
    pub fn is_fpdlink_tx_driver_enabled(&self) -> bool {
        *self == LVDS_TX_EN_A::FPDLINK_TX_DRIVER_ENABLED
    }
}
#[doc = "Field `LVDS_TX_EN` writer - Enable the serializers and TX drivers for all lanes of s40efpdlink"]
pub type LVDS_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, LVDS_TX_EN_A, O>;
impl<'a, const O: u8> LVDS_TX_EN_W<'a, O> {
    #[doc = "Disable fpdlink TX operation"]
    #[inline(always)]
    pub fn fpdlink_tx_driver_disabled(self) -> &'a mut W {
        self.variant(LVDS_TX_EN_A::FPDLINK_TX_DRIVER_DISABLED)
    }
    #[doc = "Enable fpdlink TX operation"]
    #[inline(always)]
    pub fn fpdlink_tx_driver_enabled(self) -> &'a mut W {
        self.variant(LVDS_TX_EN_A::FPDLINK_TX_DRIVER_ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the serializers and TX drivers for all lanes of s40efpdlink"]
    #[inline(always)]
    pub fn lvds_tx_en(&self) -> LVDS_TX_EN_R {
        LVDS_TX_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the serializers and TX drivers for all lanes of s40efpdlink"]
    #[inline(always)]
    #[must_use]
    pub fn lvds_tx_en(&mut self) -> LVDS_TX_EN_W<0> {
        LVDS_TX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command register for fpdlink\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
