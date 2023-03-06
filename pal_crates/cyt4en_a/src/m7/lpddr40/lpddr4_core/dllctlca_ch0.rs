#[doc = "Register `DLLCTLCA_CH0` reader"]
pub struct R(crate::R<DLLCTLCA_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCTLCA_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCTLCA_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCTLCA_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLLCTLCA_CH0` writer"]
pub struct W(crate::W<DLLCTLCA_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCTLCA_CH0_SPEC>;
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
impl From<crate::W<DLLCTLCA_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCTLCA_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMIT` reader - Bandwidth for the digital loop. - Channel 0 The recommended value from simulation is 5. The optimum value must be determined from evaluation."]
pub type LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIMIT` writer - Bandwidth for the digital loop. - Channel 0 The recommended value from simulation is 5. The optimum value must be determined from evaluation."]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCTLCA_CH0_SPEC, u8, u8, 5, O>;
#[doc = "Field `EN` reader - DLL enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - DLL enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCTLCA_CH0_SPEC, bool, O>;
#[doc = "Field `UPD` reader - DLL Update. When the PHY is in active operation, DLL update should be turn off. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type UPD_R = crate::BitReader<bool>;
#[doc = "Field `UPD` writer - DLL Update. When the PHY is in active operation, DLL update should be turn off. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type UPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCTLCA_CH0_SPEC, bool, O>;
#[doc = "Field `BYP` reader - DLL bypass enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type BYP_R = crate::BitReader<bool>;
#[doc = "Field `BYP` writer - DLL bypass enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type BYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCTLCA_CH0_SPEC, bool, O>;
#[doc = "Field `BYPC` reader - DLL bypass code. - Channel 0"]
pub type BYPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYPC` writer - DLL bypass code. - Channel 0"]
pub type BYPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCTLCA_CH0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKDLY` reader - Phase shifting for mem_clk 6'b000000: No shifted 6'b111111: 0.5 cycle - Channel 0"]
pub type CLKDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDLY` writer - Phase shifting for mem_clk 6'b000000: No shifted 6'b111111: 0.5 cycle - Channel 0"]
pub type CLKDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCTLCA_CH0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:4 - Bandwidth for the digital loop. - Channel 0 The recommended value from simulation is 5. The optimum value must be determined from evaluation."]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - DLL enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DLL Update. When the PHY is in active operation, DLL update should be turn off. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DLL bypass enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn byp(&self) -> BYP_R {
        BYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - DLL bypass code. - Channel 0"]
    #[inline(always)]
    pub fn bypc(&self) -> BYPC_R {
        BYPC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Phase shifting for mem_clk 6'b000000: No shifted 6'b111111: 0.5 cycle - Channel 0"]
    #[inline(always)]
    pub fn clkdly(&self) -> CLKDLY_R {
        CLKDLY_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bandwidth for the digital loop. - Channel 0 The recommended value from simulation is 5. The optimum value must be determined from evaluation."]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<0> {
        LIMIT_W::new(self)
    }
    #[doc = "Bit 5 - DLL enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<5> {
        EN_W::new(self)
    }
    #[doc = "Bit 6 - DLL Update. When the PHY is in active operation, DLL update should be turn off. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn upd(&mut self) -> UPD_W<6> {
        UPD_W::new(self)
    }
    #[doc = "Bit 7 - DLL bypass enable. - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn byp(&mut self) -> BYP_W<7> {
        BYP_W::new(self)
    }
    #[doc = "Bits 8:15 - DLL bypass code. - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn bypc(&mut self) -> BYPC_W<8> {
        BYPC_W::new(self)
    }
    #[doc = "Bits 16:21 - Phase shifting for mem_clk 6'b000000: No shifted 6'b111111: 0.5 cycle - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkdly(&mut self) -> CLKDLY_W<16> {
        CLKDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLL Control Register for PHY Command Module - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllctlca_ch0](index.html) module"]
pub struct DLLCTLCA_CH0_SPEC;
impl crate::RegisterSpec for DLLCTLCA_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllctlca_ch0::R](R) reader structure"]
impl crate::Readable for DLLCTLCA_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dllctlca_ch0::W](W) writer structure"]
impl crate::Writable for DLLCTLCA_CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLLCTLCA_CH0 to value 0"]
impl crate::Resettable for DLLCTLCA_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
