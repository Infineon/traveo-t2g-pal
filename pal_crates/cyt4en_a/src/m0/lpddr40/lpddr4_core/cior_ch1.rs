#[doc = "Register `CIOR_CH1` reader"]
pub struct R(crate::R<CIOR_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIOR_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIOR_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIOR_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIOR_CH1` writer"]
pub struct W(crate::W<CIOR_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIOR_CH1_SPEC>;
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
impl From<crate::W<CIOR_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIOR_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRVSEL` reader - 3-bit value to select how many copies of driver need to be turned on. Driver Setting (000-turns on 1 driver copy, 001- 2 driver copy, 010- 3 driver copy, 111- 8 driver copy) - Channel 1 Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
pub type DRVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRVSEL` writer - 3-bit value to select how many copies of driver need to be turned on. Driver Setting (000-turns on 1 driver copy, 001- 2 driver copy, 010- 3 driver copy, 111- 8 driver copy) - Channel 1 Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
pub type DRVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIOR_CH1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CMOS_EN` reader - Enable CMOS Receiver in IO - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type CMOS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMOS_EN` writer - Enable CMOS Receiver in IO - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type CMOS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIOR_CH1_SPEC, bool, O>;
#[doc = "Field `ODIS_RESETN` reader - Channel 0 mem_reset Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_RESETN_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_RESETN` writer - Channel 0 mem_reset Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_RESETN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIOR_CH1_SPEC, bool, O>;
#[doc = "Field `ODIS_ODT` reader - Channel 1 mem_odt Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_ODT_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_ODT` writer - Channel 1 mem_odt Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_ODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIOR_CH1_SPEC, bool, O>;
#[doc = "Field `ODIS_CS` reader - Channel 1 mem_cs Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CS_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_CS` writer - Channel 1 mem_cs Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIOR_CH1_SPEC, bool, O>;
#[doc = "Field `ODIS_CLK` reader - Channel 1 mem_clk Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CLK_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_CLK` writer - Channel 1 mem_clk Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIOR_CH1_SPEC, bool, O>;
#[doc = "Field `ODIS_CKE` reader - Channel 1 mem_cke Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CKE_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_CKE` writer - Channel 1 mem_cke Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIOR_CH1_SPEC, bool, O>;
#[doc = "Field `ODIS_CA` reader - Channel 1 mem_ca Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODIS_CA` writer - Channel 1 mem_ca Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIOR_CH1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:2 - 3-bit value to select how many copies of driver need to be turned on. Driver Setting (000-turns on 1 driver copy, 001- 2 driver copy, 010- 3 driver copy, 111- 8 driver copy) - Channel 1 Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
    #[inline(always)]
    pub fn drvsel(&self) -> DRVSEL_R {
        DRVSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable CMOS Receiver in IO - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn cmos_en(&self) -> CMOS_EN_R {
        CMOS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 mem_reset Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_resetn(&self) -> ODIS_RESETN_R {
        ODIS_RESETN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 mem_odt Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_odt(&self) -> ODIS_ODT_R {
        ODIS_ODT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 1 mem_cs Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_cs(&self) -> ODIS_CS_R {
        ODIS_CS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 mem_clk Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_clk(&self) -> ODIS_CLK_R {
        ODIS_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 1 mem_cke Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_cke(&self) -> ODIS_CKE_R {
        ODIS_CKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Channel 1 mem_ca Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_ca(&self) -> ODIS_CA_R {
        ODIS_CA_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 3-bit value to select how many copies of driver need to be turned on. Driver Setting (000-turns on 1 driver copy, 001- 2 driver copy, 010- 3 driver copy, 111- 8 driver copy) - Channel 1 Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
    #[inline(always)]
    #[must_use]
    pub fn drvsel(&mut self) -> DRVSEL_W<0> {
        DRVSEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable CMOS Receiver in IO - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn cmos_en(&mut self) -> CMOS_EN_W<3> {
        CMOS_EN_W::new(self)
    }
    #[doc = "Bit 4 - Channel 0 mem_reset Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_resetn(&mut self) -> ODIS_RESETN_W<4> {
        ODIS_RESETN_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 mem_odt Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_odt(&mut self) -> ODIS_ODT_W<5> {
        ODIS_ODT_W::new(self)
    }
    #[doc = "Bit 6 - Channel 1 mem_cs Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_cs(&mut self) -> ODIS_CS_W<6> {
        ODIS_CS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 1 mem_clk Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_clk(&mut self) -> ODIS_CLK_W<7> {
        ODIS_CLK_W::new(self)
    }
    #[doc = "Bit 8 - Channel 1 mem_cke Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_cke(&mut self) -> ODIS_CKE_W<8> {
        ODIS_CKE_W::new(self)
    }
    #[doc = "Bits 9:14 - Channel 1 mem_ca Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_ca(&mut self) -> ODIS_CA_W<9> {
        ODIS_CA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Command Module IO Control Register - Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cior_ch1](index.html) module"]
pub struct CIOR_CH1_SPEC;
impl crate::RegisterSpec for CIOR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cior_ch1::R](R) reader structure"]
impl crate::Readable for CIOR_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cior_ch1::W](W) writer structure"]
impl crate::Writable for CIOR_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIOR_CH1 to value 0"]
impl crate::Resettable for CIOR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
