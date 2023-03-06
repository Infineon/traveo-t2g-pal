#[doc = "Register `FM_CODE_MARGIN` reader"]
pub struct R(crate::R<FM_CODE_MARGIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_CODE_MARGIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_CODE_MARGIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_CODE_MARGIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_CODE_MARGIN` writer"]
pub struct W(crate::W<FM_CODE_MARGIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_CODE_MARGIN_SPEC>;
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
impl From<crate::W<FM_CODE_MARGIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_CODE_MARGIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARGIN_DCS_TRIM` reader - see above table to set the DCS reference current value to be used during Margin mode. (default set to 5uS = 0x143) which gives a Margin to the Erase side. 7uA would probably be used for Margin to the PGM side"]
pub type MARGIN_DCS_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MARGIN_DCS_TRIM` writer - see above table to set the DCS reference current value to be used during Margin mode. (default set to 5uS = 0x143) which gives a Margin to the Erase side. 7uA would probably be used for Margin to the PGM side"]
pub type MARGIN_DCS_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FM_CODE_MARGIN_SPEC, u16, u16, 9, O>;
#[doc = "Field `MARGIN_DCS_TRIM_EN` reader - 0: internal device defaults used from Margin reads reference current 1: MARGIN_DCS_TRIM configuration is used during Margin read"]
pub type MARGIN_DCS_TRIM_EN_R = crate::BitReader<bool>;
#[doc = "Field `MARGIN_DCS_TRIM_EN` writer - 0: internal device defaults used from Margin reads reference current 1: MARGIN_DCS_TRIM configuration is used during Margin read"]
pub type MARGIN_DCS_TRIM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FM_CODE_MARGIN_SPEC, bool, O>;
#[doc = "Field `MARGIN_RDREG_TRIM` reader - rdreg_c trim to be used in Margin mode if enabled by MARGIN_MODE_RDREG_CHNG_EN"]
pub type MARGIN_RDREG_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MARGIN_RDREG_TRIM` writer - rdreg_c trim to be used in Margin mode if enabled by MARGIN_MODE_RDREG_CHNG_EN"]
pub type MARGIN_RDREG_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FM_CODE_MARGIN_SPEC, u8, u8, 6, O>;
#[doc = "Field `MARGIN_PGM_ERS_B` reader - 0: ERS Margin is checked 1: PGM Margin is checked"]
pub type MARGIN_PGM_ERS_B_R = crate::BitReader<bool>;
#[doc = "Field `MARGIN_PGM_ERS_B` writer - 0: ERS Margin is checked 1: PGM Margin is checked"]
pub type MARGIN_PGM_ERS_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FM_CODE_MARGIN_SPEC, bool, O>;
#[doc = "Field `MARGIN_MODE_RDREG_CHNG_EN` reader - when set will also use the MARGIN_RDREG_TRIM from above. Default is not to use"]
pub type MARGIN_MODE_RDREG_CHNG_EN_R = crate::BitReader<bool>;
#[doc = "Field `MARGIN_MODE_RDREG_CHNG_EN` writer - when set will also use the MARGIN_RDREG_TRIM from above. Default is not to use"]
pub type MARGIN_MODE_RDREG_CHNG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FM_CODE_MARGIN_SPEC, bool, O>;
#[doc = "Field `MARGIN_MODE_EN` reader - when set puts the s40ect Flash IP In Margin mode"]
pub type MARGIN_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `MARGIN_MODE_EN` writer - when set puts the s40ect Flash IP In Margin mode"]
pub type MARGIN_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FM_CODE_MARGIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - see above table to set the DCS reference current value to be used during Margin mode. (default set to 5uS = 0x143) which gives a Margin to the Erase side. 7uA would probably be used for Margin to the PGM side"]
    #[inline(always)]
    pub fn margin_dcs_trim(&self) -> MARGIN_DCS_TRIM_R {
        MARGIN_DCS_TRIM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 0: internal device defaults used from Margin reads reference current 1: MARGIN_DCS_TRIM configuration is used during Margin read"]
    #[inline(always)]
    pub fn margin_dcs_trim_en(&self) -> MARGIN_DCS_TRIM_EN_R {
        MARGIN_DCS_TRIM_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - rdreg_c trim to be used in Margin mode if enabled by MARGIN_MODE_RDREG_CHNG_EN"]
    #[inline(always)]
    pub fn margin_rdreg_trim(&self) -> MARGIN_RDREG_TRIM_R {
        MARGIN_RDREG_TRIM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 29 - 0: ERS Margin is checked 1: PGM Margin is checked"]
    #[inline(always)]
    pub fn margin_pgm_ers_b(&self) -> MARGIN_PGM_ERS_B_R {
        MARGIN_PGM_ERS_B_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - when set will also use the MARGIN_RDREG_TRIM from above. Default is not to use"]
    #[inline(always)]
    pub fn margin_mode_rdreg_chng_en(&self) -> MARGIN_MODE_RDREG_CHNG_EN_R {
        MARGIN_MODE_RDREG_CHNG_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - when set puts the s40ect Flash IP In Margin mode"]
    #[inline(always)]
    pub fn margin_mode_en(&self) -> MARGIN_MODE_EN_R {
        MARGIN_MODE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - see above table to set the DCS reference current value to be used during Margin mode. (default set to 5uS = 0x143) which gives a Margin to the Erase side. 7uA would probably be used for Margin to the PGM side"]
    #[inline(always)]
    #[must_use]
    pub fn margin_dcs_trim(&mut self) -> MARGIN_DCS_TRIM_W<0> {
        MARGIN_DCS_TRIM_W::new(self)
    }
    #[doc = "Bit 9 - 0: internal device defaults used from Margin reads reference current 1: MARGIN_DCS_TRIM configuration is used during Margin read"]
    #[inline(always)]
    #[must_use]
    pub fn margin_dcs_trim_en(&mut self) -> MARGIN_DCS_TRIM_EN_W<9> {
        MARGIN_DCS_TRIM_EN_W::new(self)
    }
    #[doc = "Bits 10:15 - rdreg_c trim to be used in Margin mode if enabled by MARGIN_MODE_RDREG_CHNG_EN"]
    #[inline(always)]
    #[must_use]
    pub fn margin_rdreg_trim(&mut self) -> MARGIN_RDREG_TRIM_W<10> {
        MARGIN_RDREG_TRIM_W::new(self)
    }
    #[doc = "Bit 29 - 0: ERS Margin is checked 1: PGM Margin is checked"]
    #[inline(always)]
    #[must_use]
    pub fn margin_pgm_ers_b(&mut self) -> MARGIN_PGM_ERS_B_W<29> {
        MARGIN_PGM_ERS_B_W::new(self)
    }
    #[doc = "Bit 30 - when set will also use the MARGIN_RDREG_TRIM from above. Default is not to use"]
    #[inline(always)]
    #[must_use]
    pub fn margin_mode_rdreg_chng_en(&mut self) -> MARGIN_MODE_RDREG_CHNG_EN_W<30> {
        MARGIN_MODE_RDREG_CHNG_EN_W::new(self)
    }
    #[doc = "Bit 31 - when set puts the s40ect Flash IP In Margin mode"]
    #[inline(always)]
    #[must_use]
    pub fn margin_mode_en(&mut self) -> MARGIN_MODE_EN_W<31> {
        MARGIN_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Macro Margin Mode on Code Flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_code_margin](index.html) module"]
pub struct FM_CODE_MARGIN_SPEC;
impl crate::RegisterSpec for FM_CODE_MARGIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_code_margin::R](R) reader structure"]
impl crate::Readable for FM_CODE_MARGIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_code_margin::W](W) writer structure"]
impl crate::Writable for FM_CODE_MARGIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_CODE_MARGIN to value 0x3943"]
impl crate::Resettable for FM_CODE_MARGIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x3943;
}
