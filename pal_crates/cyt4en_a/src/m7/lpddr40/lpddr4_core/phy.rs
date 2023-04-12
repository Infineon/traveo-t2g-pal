#[doc = "Register `PHY` reader"]
pub struct R(crate::R<PHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY` writer"]
pub struct W(crate::W<PHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_SPEC>;
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
impl From<crate::W<PHY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTI_DRAM_CLK_DIS` reader - This register is used by the MC to inform the PHY when to enable/disable the clock to the memory device Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type DTI_DRAM_CLK_DIS_R = crate::BitReader<bool>;
#[doc = "Field `DTI_DRAM_CLK_DIS` writer - This register is used by the MC to inform the PHY when to enable/disable the clock to the memory device Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type DTI_DRAM_CLK_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_SPEC, bool, O>;
#[doc = "Field `DTI_DATA_BYTE_DIS` reader - Signal to disable operations on the specified byte-lane dti_data_byte_dis\\[0\\]
= 0: Enable byte-lane 0 dti_data_byte_dis\\[0\\]
= 1: Disable byte-lane 0 ... dti_data_byte_dis\\[3\\]
= 0: Enable byte-lane 3 dti_data_byte_dis\\[3\\]
= 1: Disable byte-lane 3"]
pub type DTI_DATA_BYTE_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTI_DATA_BYTE_DIS` writer - Signal to disable operations on the specified byte-lane dti_data_byte_dis\\[0\\]
= 0: Enable byte-lane 0 dti_data_byte_dis\\[0\\]
= 1: Disable byte-lane 0 ... dti_data_byte_dis\\[3\\]
= 0: Enable byte-lane 3 dti_data_byte_dis\\[3\\]
= 1: Disable byte-lane 3"]
pub type DTI_DATA_BYTE_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - This register is used by the MC to inform the PHY when to enable/disable the clock to the memory device Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn dti_dram_clk_dis(&self) -> DTI_DRAM_CLK_DIS_R {
        DTI_DRAM_CLK_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Signal to disable operations on the specified byte-lane dti_data_byte_dis\\[0\\]
= 0: Enable byte-lane 0 dti_data_byte_dis\\[0\\]
= 1: Disable byte-lane 0 ... dti_data_byte_dis\\[3\\]
= 0: Enable byte-lane 3 dti_data_byte_dis\\[3\\]
= 1: Disable byte-lane 3"]
    #[inline(always)]
    pub fn dti_data_byte_dis(&self) -> DTI_DATA_BYTE_DIS_R {
        DTI_DATA_BYTE_DIS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used by the MC to inform the PHY when to enable/disable the clock to the memory device Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn dti_dram_clk_dis(&mut self) -> DTI_DRAM_CLK_DIS_W<0> {
        DTI_DRAM_CLK_DIS_W::new(self)
    }
    #[doc = "Bits 1:4 - Signal to disable operations on the specified byte-lane dti_data_byte_dis\\[0\\]
= 0: Enable byte-lane 0 dti_data_byte_dis\\[0\\]
= 1: Disable byte-lane 0 ... dti_data_byte_dis\\[3\\]
= 0: Enable byte-lane 3 dti_data_byte_dis\\[3\\]
= 1: Disable byte-lane 3"]
    #[inline(always)]
    #[must_use]
    pub fn dti_data_byte_dis(&mut self) -> DTI_DATA_BYTE_DIS_W<1> {
        DTI_DATA_BYTE_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy](index.html) module"]
pub struct PHY_SPEC;
impl crate::RegisterSpec for PHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy::R](R) reader structure"]
impl crate::Readable for PHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy::W](W) writer structure"]
impl crate::Writable for PHY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY to value 0"]
impl crate::Resettable for PHY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
