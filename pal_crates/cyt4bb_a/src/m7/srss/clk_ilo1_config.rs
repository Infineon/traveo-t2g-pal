#[doc = "Register `CLK_ILO1_CONFIG` reader"]
pub struct R(crate::R<CLK_ILO1_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ILO1_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ILO1_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ILO1_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_ILO1_CONFIG` writer"]
pub struct W(crate::W<CLK_ILO1_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_ILO1_CONFIG_SPEC>;
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
impl From<crate::W<CLK_ILO1_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_ILO1_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILO1_MON_ENABLE` reader - N/A"]
pub type ILO1_MON_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ILO1_MON_ENABLE` writer - N/A"]
pub type ILO1_MON_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_ILO1_CONFIG_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Master enable for ILO1. HT-variant: After enabling, the first ILO1 cycle occurs within 12us and is +/-10 percent accuracy. Thereafter, ILO1 is +/-5 percent accurate."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for ILO1. HT-variant: After enabling, the first ILO1 cycle occurs within 12us and is +/-10 percent accuracy. Thereafter, ILO1 is +/-5 percent accurate."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_ILO1_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn ilo1_mon_enable(&self) -> ILO1_MON_ENABLE_R {
        ILO1_MON_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ILO1. HT-variant: After enabling, the first ILO1 cycle occurs within 12us and is +/-10 percent accuracy. Thereafter, ILO1 is +/-5 percent accurate."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ilo1_mon_enable(&mut self) -> ILO1_MON_ENABLE_W<30> {
        ILO1_MON_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for ILO1. HT-variant: After enabling, the first ILO1 cycle occurs within 12us and is +/-10 percent accuracy. Thereafter, ILO1 is +/-5 percent accurate."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ILO1 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ilo1_config](index.html) module"]
pub struct CLK_ILO1_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_ILO1_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ilo1_config::R](R) reader structure"]
impl crate::Readable for CLK_ILO1_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ilo1_config::W](W) writer structure"]
impl crate::Writable for CLK_ILO1_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ILO1_CONFIG to value 0"]
impl crate::Resettable for CLK_ILO1_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
