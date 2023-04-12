#[doc = "Register `PCCR_CH1` reader"]
pub struct R(crate::R<PCCR_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCR_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCR_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCR_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCR_CH1` writer"]
pub struct W(crate::W<PCCR_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCR_CH1_SPEC>;
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
impl From<crate::W<PCCR_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCR_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCCR_CH1_SRST` reader - Channel 1 Compensation Soft-Reset Compensation soft-reset can be triggered by user's writing 0 to the srst field of the pccr_chX register followed by a writing 1. The srst field must stay at logic 1 indefinitely if user doesn't want to reset the compensation block. Users must poll the field (srstc - Compensation soft-reset complete) in the pcsr_chX register for the ready of the compensation block."]
pub type PCCR_CH1_SRST_R = crate::BitReader<bool>;
#[doc = "Field `PCCR_CH1_SRST` writer - Channel 1 Compensation Soft-Reset Compensation soft-reset can be triggered by user's writing 0 to the srst field of the pccr_chX register followed by a writing 1. The srst field must stay at logic 1 indefinitely if user doesn't want to reset the compensation block. Users must poll the field (srstc - Compensation soft-reset complete) in the pcsr_chX register for the ready of the compensation block."]
pub type PCCR_CH1_SRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR_CH1_SPEC, bool, O>;
#[doc = "Field `PCCR_CH1_TPADEN` reader - Testpad enable - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_TPADEN_R = crate::BitReader<bool>;
#[doc = "Field `PCCR_CH1_TPADEN` writer - Testpad enable - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_TPADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR_CH1_SPEC, bool, O>;
#[doc = "Field `PCCR_CH1_MVG` reader - Enable moving average for compensation - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_MVG_R = crate::BitReader<bool>;
#[doc = "Field `PCCR_CH1_MVG` writer - Enable moving average for compensation - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_MVG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR_CH1_SPEC, bool, O>;
#[doc = "Field `PCCR_CH1_EN` reader - Enable compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_EN_R = crate::BitReader<bool>;
#[doc = "Field `PCCR_CH1_EN` writer - Enable compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR_CH1_SPEC, bool, O>;
#[doc = "Field `PCCR_CH1_UPD` reader - Update compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_UPD_R = crate::BitReader<bool>;
#[doc = "Field `PCCR_CH1_UPD` writer - Update compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_UPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR_CH1_SPEC, bool, O>;
#[doc = "Field `PCCR_CH1_BYPEN` reader - Bypass internal compensation setting - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_BYPEN_R = crate::BitReader<bool>;
#[doc = "Field `PCCR_CH1_BYPEN` writer - Bypass internal compensation setting - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PCCR_CH1_BYPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR_CH1_SPEC, bool, O>;
#[doc = "Field `PCCR_CH1_BYP_N` reader - Bypass setting - Channel 1"]
pub type PCCR_CH1_BYP_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCCR_CH1_BYP_N` writer - Bypass setting - Channel 1"]
pub type PCCR_CH1_BYP_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCCR_CH1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCCR_CH1_BYP_P` reader - Bypass setting - Channel 1"]
pub type PCCR_CH1_BYP_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCCR_CH1_BYP_P` writer - Bypass setting - Channel 1"]
pub type PCCR_CH1_BYP_P_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCCR_CH1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCCR_CH1_INITCNT` reader - Channel 1 PHY compensation initialization counter - Channel 0. Always write 1024."]
pub type PCCR_CH1_INITCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCCR_CH1_INITCNT` writer - Channel 1 PHY compensation initialization counter - Channel 0. Always write 1024."]
pub type PCCR_CH1_INITCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCCR_CH1_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 0 - Channel 1 Compensation Soft-Reset Compensation soft-reset can be triggered by user's writing 0 to the srst field of the pccr_chX register followed by a writing 1. The srst field must stay at logic 1 indefinitely if user doesn't want to reset the compensation block. Users must poll the field (srstc - Compensation soft-reset complete) in the pcsr_chX register for the ready of the compensation block."]
    #[inline(always)]
    pub fn pccr_ch1_srst(&self) -> PCCR_CH1_SRST_R {
        PCCR_CH1_SRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Testpad enable - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pccr_ch1_tpaden(&self) -> PCCR_CH1_TPADEN_R {
        PCCR_CH1_TPADEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable moving average for compensation - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pccr_ch1_mvg(&self) -> PCCR_CH1_MVG_R {
        PCCR_CH1_MVG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pccr_ch1_en(&self) -> PCCR_CH1_EN_R {
        PCCR_CH1_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Update compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pccr_ch1_upd(&self) -> PCCR_CH1_UPD_R {
        PCCR_CH1_UPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass internal compensation setting - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pccr_ch1_bypen(&self) -> PCCR_CH1_BYPEN_R {
        PCCR_CH1_BYPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Bypass setting - Channel 1"]
    #[inline(always)]
    pub fn pccr_ch1_byp_n(&self) -> PCCR_CH1_BYP_N_R {
        PCCR_CH1_BYP_N_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Bypass setting - Channel 1"]
    #[inline(always)]
    pub fn pccr_ch1_byp_p(&self) -> PCCR_CH1_BYP_P_R {
        PCCR_CH1_BYP_P_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:24 - Channel 1 PHY compensation initialization counter - Channel 0. Always write 1024."]
    #[inline(always)]
    pub fn pccr_ch1_initcnt(&self) -> PCCR_CH1_INITCNT_R {
        PCCR_CH1_INITCNT_R::new(((self.bits >> 14) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 Compensation Soft-Reset Compensation soft-reset can be triggered by user's writing 0 to the srst field of the pccr_chX register followed by a writing 1. The srst field must stay at logic 1 indefinitely if user doesn't want to reset the compensation block. Users must poll the field (srstc - Compensation soft-reset complete) in the pcsr_chX register for the ready of the compensation block."]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_srst(&mut self) -> PCCR_CH1_SRST_W<0> {
        PCCR_CH1_SRST_W::new(self)
    }
    #[doc = "Bit 1 - Testpad enable - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_tpaden(&mut self) -> PCCR_CH1_TPADEN_W<1> {
        PCCR_CH1_TPADEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable moving average for compensation - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_mvg(&mut self) -> PCCR_CH1_MVG_W<2> {
        PCCR_CH1_MVG_W::new(self)
    }
    #[doc = "Bit 3 - Enable compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_en(&mut self) -> PCCR_CH1_EN_W<3> {
        PCCR_CH1_EN_W::new(self)
    }
    #[doc = "Bit 4 - Update compensation block - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_upd(&mut self) -> PCCR_CH1_UPD_W<4> {
        PCCR_CH1_UPD_W::new(self)
    }
    #[doc = "Bit 5 - Bypass internal compensation setting - Channel 1 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_bypen(&mut self) -> PCCR_CH1_BYPEN_W<5> {
        PCCR_CH1_BYPEN_W::new(self)
    }
    #[doc = "Bits 6:9 - Bypass setting - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_byp_n(&mut self) -> PCCR_CH1_BYP_N_W<6> {
        PCCR_CH1_BYP_N_W::new(self)
    }
    #[doc = "Bits 10:13 - Bypass setting - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_byp_p(&mut self) -> PCCR_CH1_BYP_P_W<10> {
        PCCR_CH1_BYP_P_W::new(self)
    }
    #[doc = "Bits 14:24 - Channel 1 PHY compensation initialization counter - Channel 0. Always write 1024."]
    #[inline(always)]
    #[must_use]
    pub fn pccr_ch1_initcnt(&mut self) -> PCCR_CH1_INITCNT_W<14> {
        PCCR_CH1_INITCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Compensation Control Register - Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccr_ch1](index.html) module"]
pub struct PCCR_CH1_SPEC;
impl crate::RegisterSpec for PCCR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccr_ch1::R](R) reader structure"]
impl crate::Readable for PCCR_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccr_ch1::W](W) writer structure"]
impl crate::Writable for PCCR_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCCR_CH1 to value 0x0100_001f"]
impl crate::Resettable for PCCR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_001f;
}
