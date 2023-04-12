#[doc = "Register `PBCR` reader"]
pub struct R(crate::R<PBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBCR` writer"]
pub struct W(crate::W<PBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBCR_SPEC>;
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
impl From<crate::W<PBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIST_EN` reader - PHY BIST mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type BIST_EN_R = crate::BitReader<bool>;
#[doc = "Field `BIST_EN` writer - PHY BIST mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type BIST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCR_SPEC, bool, O>;
#[doc = "Field `BIST_START` reader - Start PHY BIST mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type BIST_START_R = crate::BitReader<bool>;
#[doc = "Field `BIST_START` writer - Start PHY BIST mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type BIST_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCR_SPEC, bool, O>;
#[doc = "Field `LP_EN` reader - PHY Loopback mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type LP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LP_EN` writer - PHY Loopback mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type LP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCR_SPEC, bool, O>;
#[doc = "Field `VREFENCA_C0` reader - VREF enable for control block, in BIST/Loopback mode - Channel A Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type VREFENCA_C0_R = crate::BitReader<bool>;
#[doc = "Field `VREFENCA_C0` writer - VREF enable for control block, in BIST/Loopback mode - Channel A Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type VREFENCA_C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCR_SPEC, bool, O>;
#[doc = "Field `VREFSETCA_C0` reader - VREF setting for control block, in BIST/Loopback mode - Channel A"]
pub type VREFSETCA_C0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFSETCA_C0` writer - VREF setting for control block, in BIST/Loopback mode - Channel A"]
pub type VREFSETCA_C0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PBCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFENCA_C1` reader - VREF enable for control block, in BIST/Loopback mode - Channel B Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type VREFENCA_C1_R = crate::BitReader<bool>;
#[doc = "Field `VREFENCA_C1` writer - VREF enable for control block, in BIST/Loopback mode - Channel B Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type VREFENCA_C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCR_SPEC, bool, O>;
#[doc = "Field `VREFSETCA_C1` reader - VREF setting for control block, in BIST/Loopback mode - Channel B"]
pub type VREFSETCA_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFSETCA_C1` writer - VREF setting for control block, in BIST/Loopback mode - Channel B"]
pub type VREFSETCA_C1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PBCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - PHY BIST mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn bist_en(&self) -> BIST_EN_R {
        BIST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start PHY BIST mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn bist_start(&self) -> BIST_START_R {
        BIST_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY Loopback mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn lp_en(&self) -> LP_EN_R {
        LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREF enable for control block, in BIST/Loopback mode - Channel A Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn vrefenca_c0(&self) -> VREFENCA_C0_R {
        VREFENCA_C0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - VREF setting for control block, in BIST/Loopback mode - Channel A"]
    #[inline(always)]
    pub fn vrefsetca_c0(&self) -> VREFSETCA_C0_R {
        VREFSETCA_C0_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - VREF enable for control block, in BIST/Loopback mode - Channel B Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn vrefenca_c1(&self) -> VREFENCA_C1_R {
        VREFENCA_C1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - VREF setting for control block, in BIST/Loopback mode - Channel B"]
    #[inline(always)]
    pub fn vrefsetca_c1(&self) -> VREFSETCA_C1_R {
        VREFSETCA_C1_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PHY BIST mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn bist_en(&mut self) -> BIST_EN_W<0> {
        BIST_EN_W::new(self)
    }
    #[doc = "Bit 1 - Start PHY BIST mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn bist_start(&mut self) -> BIST_START_W<1> {
        BIST_START_W::new(self)
    }
    #[doc = "Bit 2 - PHY Loopback mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn lp_en(&mut self) -> LP_EN_W<2> {
        LP_EN_W::new(self)
    }
    #[doc = "Bit 3 - VREF enable for control block, in BIST/Loopback mode - Channel A Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn vrefenca_c0(&mut self) -> VREFENCA_C0_W<3> {
        VREFENCA_C0_W::new(self)
    }
    #[doc = "Bits 4:9 - VREF setting for control block, in BIST/Loopback mode - Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsetca_c0(&mut self) -> VREFSETCA_C0_W<4> {
        VREFSETCA_C0_W::new(self)
    }
    #[doc = "Bit 10 - VREF enable for control block, in BIST/Loopback mode - Channel B Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn vrefenca_c1(&mut self) -> VREFENCA_C1_W<10> {
        VREFENCA_C1_W::new(self)
    }
    #[doc = "Bits 11:16 - VREF setting for control block, in BIST/Loopback mode - Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsetca_c1(&mut self) -> VREFSETCA_C1_W<11> {
        VREFSETCA_C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY BIST Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcr](index.html) module"]
pub struct PBCR_SPEC;
impl crate::RegisterSpec for PBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbcr::R](R) reader structure"]
impl crate::Readable for PBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbcr::W](W) writer structure"]
impl crate::Writable for PBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBCR to value 0"]
impl crate::Resettable for PBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
