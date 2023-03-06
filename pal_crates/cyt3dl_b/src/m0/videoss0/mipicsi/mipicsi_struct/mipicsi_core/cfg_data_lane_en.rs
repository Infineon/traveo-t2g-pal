#[doc = "Register `CFG_DATA_LANE_EN` reader"]
pub struct R(crate::R<CFG_DATA_LANE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA_LANE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA_LANE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA_LANE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA_LANE_EN` writer"]
pub struct W(crate::W<CFG_DATA_LANE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA_LANE_EN_SPEC>;
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
impl From<crate::W<CFG_DATA_LANE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA_LANE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG_DATA_LANE_EN` reader - Setting bits to a '1' value enabled the according data lanes of the PHY."]
pub type CFG_DATA_LANE_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG_DATA_LANE_EN` writer - Setting bits to a '1' value enabled the according data lanes of the PHY."]
pub type CFG_DATA_LANE_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DATA_LANE_EN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Setting bits to a '1' value enabled the according data lanes of the PHY."]
    #[inline(always)]
    pub fn cfg_data_lane_en(&self) -> CFG_DATA_LANE_EN_R {
        CFG_DATA_LANE_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Setting bits to a '1' value enabled the according data lanes of the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_data_lane_en(&mut self) -> CFG_DATA_LANE_EN_W<0> {
        CFG_DATA_LANE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG_DATA_LANE_EN is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data_lane_en](index.html) module"]
pub struct CFG_DATA_LANE_EN_SPEC;
impl crate::RegisterSpec for CFG_DATA_LANE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data_lane_en::R](R) reader structure"]
impl crate::Readable for CFG_DATA_LANE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data_lane_en::W](W) writer structure"]
impl crate::Writable for CFG_DATA_LANE_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DATA_LANE_EN to value 0"]
impl crate::Resettable for CFG_DATA_LANE_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
