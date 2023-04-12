#[doc = "Register `DPHY_CTL` reader"]
pub struct R(crate::R<DPHY_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_CTL` writer"]
pub struct W(crate::W<DPHY_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_CTL_SPEC>;
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
impl From<crate::W<DPHY_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_DPHY` reader - D-PHY power down: '1': PHY is in power down. '0': PHY is in operation."]
pub type PD_DPHY_R = crate::BitReader<bool>;
#[doc = "Field `PD_DPHY` writer - D-PHY power down: '1': PHY is in power down. '0': PHY is in operation."]
pub type PD_DPHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPHY_CTL_SPEC, bool, O>;
#[doc = "Field `AUTO_PD_EN` reader - Powers down inactive lanes reported by CFG_NUM_LANES input bus. 1'b0: inactive lanes are powered up and driving LP11. 1'b1: inactive lanes are powered down (Hi-Z)."]
pub type AUTO_PD_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_PD_EN` writer - Powers down inactive lanes reported by CFG_NUM_LANES input bus. 1'b0: inactive lanes are powered up and driving LP11. 1'b1: inactive lanes are powered down (Hi-Z)."]
pub type AUTO_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPHY_CTL_SPEC, bool, O>;
#[doc = "Field `CONT_CLK_MODE` reader - Enables the slave clock lane feature to maintain HS reception state during continuous clock mode operation. 1'b1: Feature enabled 1'b0: Feature disabled Detail: This bit enables an advanced feature in continues clock mode operation. If the CSI link was configured to continuous clock mode, the master clock lane sends HS entry command only once and remains in HS mode. In such case, if the slave clock lane got accidentally out of HS (For example: ESD on CKP/CKN lines caused LP11 level) the master side will not sense such a disruption and will continue sending packets. Having slave clock lane out of HS mode, all packets will be not be received. Setting CONT_CLK_MODE pin to 1'b1, will help getting the slave clock lane back into HS mode, without receiving a new HS entry command from the master clock lane. It will basically rely on the fact that slave data lanes will receive HS entry command per every burst. If you think that such a feature is not required, you can set this bit to 1'b0."]
pub type CONT_CLK_MODE_R = crate::BitReader<bool>;
#[doc = "Field `CONT_CLK_MODE` writer - Enables the slave clock lane feature to maintain HS reception state during continuous clock mode operation. 1'b1: Feature enabled 1'b0: Feature disabled Detail: This bit enables an advanced feature in continues clock mode operation. If the CSI link was configured to continuous clock mode, the master clock lane sends HS entry command only once and remains in HS mode. In such case, if the slave clock lane got accidentally out of HS (For example: ESD on CKP/CKN lines caused LP11 level) the master side will not sense such a disruption and will continue sending packets. Having slave clock lane out of HS mode, all packets will be not be received. Setting CONT_CLK_MODE pin to 1'b1, will help getting the slave clock lane back into HS mode, without receiving a new HS entry command from the master clock lane. It will basically rely on the fact that slave data lanes will receive HS entry command per every burst. If you think that such a feature is not required, you can set this bit to 1'b0."]
pub type CONT_CLK_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPHY_CTL_SPEC, bool, O>;
#[doc = "Field `HSEL` reader - High Speed Select. 1'b0: up to 1.0 Gbps operation 1'b1: above 1.0 Gbps operation"]
pub type HSEL_R = crate::BitReader<bool>;
#[doc = "Field `HSEL` writer - High Speed Select. 1'b0: up to 1.0 Gbps operation 1'b1: above 1.0 Gbps operation"]
pub type HSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPHY_CTL_SPEC, bool, O>;
#[doc = "Field `ENP_DESER` reader - To override the deserializer token detector and enable token detection in CIL. 1'b1: Feature enabled 1'b0: Feature disabled Detail: By default, deserializer token detection and byte alignment is part of the hard macro and this is the setting you need to apply in your normal operation. For characterization and debug, we can disable this feature and enable it in the CIL."]
pub type ENP_DESER_R = crate::BitReader<bool>;
#[doc = "Field `ENP_DESER` writer - To override the deserializer token detector and enable token detection in CIL. 1'b1: Feature enabled 1'b0: Feature disabled Detail: By default, deserializer token detection and byte alignment is part of the hard macro and this is the setting you need to apply in your normal operation. For characterization and debug, we can disable this feature and enable it in the CIL."]
pub type ENP_DESER_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPHY_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - D-PHY power down: '1': PHY is in power down. '0': PHY is in operation."]
    #[inline(always)]
    pub fn pd_dphy(&self) -> PD_DPHY_R {
        PD_DPHY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Powers down inactive lanes reported by CFG_NUM_LANES input bus. 1'b0: inactive lanes are powered up and driving LP11. 1'b1: inactive lanes are powered down (Hi-Z)."]
    #[inline(always)]
    pub fn auto_pd_en(&self) -> AUTO_PD_EN_R {
        AUTO_PD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the slave clock lane feature to maintain HS reception state during continuous clock mode operation. 1'b1: Feature enabled 1'b0: Feature disabled Detail: This bit enables an advanced feature in continues clock mode operation. If the CSI link was configured to continuous clock mode, the master clock lane sends HS entry command only once and remains in HS mode. In such case, if the slave clock lane got accidentally out of HS (For example: ESD on CKP/CKN lines caused LP11 level) the master side will not sense such a disruption and will continue sending packets. Having slave clock lane out of HS mode, all packets will be not be received. Setting CONT_CLK_MODE pin to 1'b1, will help getting the slave clock lane back into HS mode, without receiving a new HS entry command from the master clock lane. It will basically rely on the fact that slave data lanes will receive HS entry command per every burst. If you think that such a feature is not required, you can set this bit to 1'b0."]
    #[inline(always)]
    pub fn cont_clk_mode(&self) -> CONT_CLK_MODE_R {
        CONT_CLK_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High Speed Select. 1'b0: up to 1.0 Gbps operation 1'b1: above 1.0 Gbps operation"]
    #[inline(always)]
    pub fn hsel(&self) -> HSEL_R {
        HSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - To override the deserializer token detector and enable token detection in CIL. 1'b1: Feature enabled 1'b0: Feature disabled Detail: By default, deserializer token detection and byte alignment is part of the hard macro and this is the setting you need to apply in your normal operation. For characterization and debug, we can disable this feature and enable it in the CIL."]
    #[inline(always)]
    pub fn enp_deser(&self) -> ENP_DESER_R {
        ENP_DESER_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D-PHY power down: '1': PHY is in power down. '0': PHY is in operation."]
    #[inline(always)]
    #[must_use]
    pub fn pd_dphy(&mut self) -> PD_DPHY_W<0> {
        PD_DPHY_W::new(self)
    }
    #[doc = "Bit 8 - Powers down inactive lanes reported by CFG_NUM_LANES input bus. 1'b0: inactive lanes are powered up and driving LP11. 1'b1: inactive lanes are powered down (Hi-Z)."]
    #[inline(always)]
    #[must_use]
    pub fn auto_pd_en(&mut self) -> AUTO_PD_EN_W<8> {
        AUTO_PD_EN_W::new(self)
    }
    #[doc = "Bit 16 - Enables the slave clock lane feature to maintain HS reception state during continuous clock mode operation. 1'b1: Feature enabled 1'b0: Feature disabled Detail: This bit enables an advanced feature in continues clock mode operation. If the CSI link was configured to continuous clock mode, the master clock lane sends HS entry command only once and remains in HS mode. In such case, if the slave clock lane got accidentally out of HS (For example: ESD on CKP/CKN lines caused LP11 level) the master side will not sense such a disruption and will continue sending packets. Having slave clock lane out of HS mode, all packets will be not be received. Setting CONT_CLK_MODE pin to 1'b1, will help getting the slave clock lane back into HS mode, without receiving a new HS entry command from the master clock lane. It will basically rely on the fact that slave data lanes will receive HS entry command per every burst. If you think that such a feature is not required, you can set this bit to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn cont_clk_mode(&mut self) -> CONT_CLK_MODE_W<16> {
        CONT_CLK_MODE_W::new(self)
    }
    #[doc = "Bit 17 - High Speed Select. 1'b0: up to 1.0 Gbps operation 1'b1: above 1.0 Gbps operation"]
    #[inline(always)]
    #[must_use]
    pub fn hsel(&mut self) -> HSEL_W<17> {
        HSEL_W::new(self)
    }
    #[doc = "Bit 18 - To override the deserializer token detector and enable token detection in CIL. 1'b1: Feature enabled 1'b0: Feature disabled Detail: By default, deserializer token detection and byte alignment is part of the hard macro and this is the setting you need to apply in your normal operation. For characterization and debug, we can disable this feature and enable it in the CIL."]
    #[inline(always)]
    #[must_use]
    pub fn enp_deser(&mut self) -> ENP_DESER_W<18> {
        ENP_DESER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D-PHY Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_ctl](index.html) module"]
pub struct DPHY_CTL_SPEC;
impl crate::RegisterSpec for DPHY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_ctl::R](R) reader structure"]
impl crate::Readable for DPHY_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_ctl::W](W) writer structure"]
impl crate::Writable for DPHY_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_CTL to value 0x0003_0101"]
impl crate::Resettable for DPHY_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0101;
}
