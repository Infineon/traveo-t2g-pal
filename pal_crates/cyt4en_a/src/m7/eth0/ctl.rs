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
#[doc = "Field `ETH_MODE` reader - Set ethernet mode. The speed of 10M/100M/1G is selected by programming network_config after IP is enabled. The field also specifies the clock source of internal tx_clk and rx_clk domain."]
pub type ETH_MODE_R = crate::FieldReader<u8, ETH_MODE_A>;
#[doc = "Set ethernet mode. The speed of 10M/100M/1G is selected by programming network_config after IP is enabled. The field also specifies the clock source of internal tx_clk and rx_clk domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETH_MODE_A {
    #[doc = "0: MII mode (10/100MHz speed is determined by network_config\\[0\\])."]
    MII = 0,
    #[doc = "1: GMII mode (network_config\\[10\\]
must be set to enable 1G operation)."]
    GMII = 1,
    #[doc = "2: RGMII mode (10M/100M/1G speed is determined by network_config\\[10\\]
and network_config\\[0\\])."]
    RGMII = 2,
    #[doc = "3: RMII mode (10M/100M speed is determined by network_config\\[0\\])."]
    RMII = 3,
}
impl From<ETH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ETH_MODE_A) -> Self {
        variant as _
    }
}
impl ETH_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH_MODE_A {
        match self.bits {
            0 => ETH_MODE_A::MII,
            1 => ETH_MODE_A::GMII,
            2 => ETH_MODE_A::RGMII,
            3 => ETH_MODE_A::RMII,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MII`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == ETH_MODE_A::MII
    }
    #[doc = "Checks if the value of the field is `GMII`"]
    #[inline(always)]
    pub fn is_gmii(&self) -> bool {
        *self == ETH_MODE_A::GMII
    }
    #[doc = "Checks if the value of the field is `RGMII`"]
    #[inline(always)]
    pub fn is_rgmii(&self) -> bool {
        *self == ETH_MODE_A::RGMII
    }
    #[doc = "Checks if the value of the field is `RMII`"]
    #[inline(always)]
    pub fn is_rmii(&self) -> bool {
        *self == ETH_MODE_A::RMII
    }
}
#[doc = "Field `ETH_MODE` writer - Set ethernet mode. The speed of 10M/100M/1G is selected by programming network_config after IP is enabled. The field also specifies the clock source of internal tx_clk and rx_clk domain."]
pub type ETH_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, ETH_MODE_A, 2, O>;
impl<'a, const O: u8> ETH_MODE_W<'a, O> {
    #[doc = "MII mode (10/100MHz speed is determined by network_config\\[0\\])."]
    #[inline(always)]
    pub fn mii(self) -> &'a mut W {
        self.variant(ETH_MODE_A::MII)
    }
    #[doc = "GMII mode (network_config\\[10\\]
must be set to enable 1G operation)."]
    #[inline(always)]
    pub fn gmii(self) -> &'a mut W {
        self.variant(ETH_MODE_A::GMII)
    }
    #[doc = "RGMII mode (10M/100M/1G speed is determined by network_config\\[10\\]
and network_config\\[0\\])."]
    #[inline(always)]
    pub fn rgmii(self) -> &'a mut W {
        self.variant(ETH_MODE_A::RGMII)
    }
    #[doc = "RMII mode (10M/100M speed is determined by network_config\\[0\\])."]
    #[inline(always)]
    pub fn rmii(self) -> &'a mut W {
        self.variant(ETH_MODE_A::RMII)
    }
}
#[doc = "Field `REFCLK_SRC_SEL` reader - Select the source for ref_clk. 0: Ref_clk comes from REF_CLK_IN input port (HSIO). 1: Ref_clk comes from REF_CLK_INT_IN input port (PLL)."]
pub type REFCLK_SRC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `REFCLK_SRC_SEL` writer - Select the source for ref_clk. 0: Ref_clk comes from REF_CLK_IN input port (HSIO). 1: Ref_clk comes from REF_CLK_INT_IN input port (PLL)."]
pub type REFCLK_SRC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `REFCLK_DIV` reader - Specify the ref_clk divider. Integer division by (1+REFCLK_DIV). Allows for integer divisions in the range \\[1, 256\\]."]
pub type REFCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFCLK_DIV` writer - Specify the ref_clk divider. Integer division by (1+REFCLK_DIV). Allows for integer divisions in the range \\[1, 256\\]."]
pub type REFCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ENABLED` reader - MXETH enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - MXETH enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Set ethernet mode. The speed of 10M/100M/1G is selected by programming network_config after IP is enabled. The field also specifies the clock source of internal tx_clk and rx_clk domain."]
    #[inline(always)]
    pub fn eth_mode(&self) -> ETH_MODE_R {
        ETH_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Select the source for ref_clk. 0: Ref_clk comes from REF_CLK_IN input port (HSIO). 1: Ref_clk comes from REF_CLK_INT_IN input port (PLL)."]
    #[inline(always)]
    pub fn refclk_src_sel(&self) -> REFCLK_SRC_SEL_R {
        REFCLK_SRC_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Specify the ref_clk divider. Integer division by (1+REFCLK_DIV). Allows for integer divisions in the range \\[1, 256\\]."]
    #[inline(always)]
    pub fn refclk_div(&self) -> REFCLK_DIV_R {
        REFCLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - MXETH enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set ethernet mode. The speed of 10M/100M/1G is selected by programming network_config after IP is enabled. The field also specifies the clock source of internal tx_clk and rx_clk domain."]
    #[inline(always)]
    #[must_use]
    pub fn eth_mode(&mut self) -> ETH_MODE_W<0> {
        ETH_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Select the source for ref_clk. 0: Ref_clk comes from REF_CLK_IN input port (HSIO). 1: Ref_clk comes from REF_CLK_INT_IN input port (PLL)."]
    #[inline(always)]
    #[must_use]
    pub fn refclk_src_sel(&mut self) -> REFCLK_SRC_SEL_W<2> {
        REFCLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Specify the ref_clk divider. Integer division by (1+REFCLK_DIV). Allows for integer divisions in the range \\[1, 256\\]."]
    #[inline(always)]
    #[must_use]
    pub fn refclk_div(&mut self) -> REFCLK_DIV_W<8> {
        REFCLK_DIV_W::new(self)
    }
    #[doc = "Bit 31 - MXETH enable: '0': Disabled. '1': Enabled."]
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
#[doc = "MXETH Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
