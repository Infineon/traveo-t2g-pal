#[doc = "Register `BISTCFG_CH0` reader"]
pub struct R(crate::R<BISTCFG_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTCFG_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTCFG_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTCFG_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BISTCFG_CH0` writer"]
pub struct W(crate::W<BISTCFG_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BISTCFG_CH0_SPEC>;
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
impl From<crate::W<BISTCFG_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BISTCFG_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_BANK` reader - Start bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
pub type START_BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `START_BANK` writer - Start bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
pub type START_BANK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 3, O>;
#[doc = "Field `END_BANK` reader - End bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
pub type END_BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `END_BANK` writer - End bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
pub type END_BANK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 3, O>;
#[doc = "Field `START_BACKGROUND` reader - Start background index - Channel 0"]
pub type START_BACKGROUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `START_BACKGROUND` writer - Start background index - Channel 0"]
pub type START_BACKGROUND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 3, O>;
#[doc = "Field `END_BACKGROUND` reader - End background index - Channel 0"]
pub type END_BACKGROUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `END_BACKGROUND` writer - End background index - Channel 0"]
pub type END_BACKGROUND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 3, O>;
#[doc = "Field `ELEMENT` reader - March element number - Channel 0"]
pub type ELEMENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ELEMENT` writer - March element number - Channel 0"]
pub type ELEMENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `OPERATION` reader - March operation number - Channel 0"]
pub type OPERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPERATION` writer - March operation number - Channel 0"]
pub type OPERATION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RETENTION` reader - Count-to value of retention counter (Width = RETENTION_WIDTH) - Channel 0"]
pub type RETENTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETENTION` writer - Count-to value of retention counter (Width = RETENTION_WIDTH) - Channel 0"]
pub type RETENTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BISTCFG_CH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIAGNOSIS_EN` reader - Enable diagnosis mode - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type DIAGNOSIS_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIAGNOSIS_EN` writer - Enable diagnosis mode - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type DIAGNOSIS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BISTCFG_CH0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Start bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
    #[inline(always)]
    pub fn start_bank(&self) -> START_BANK_R {
        START_BANK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - End bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
    #[inline(always)]
    pub fn end_bank(&self) -> END_BANK_R {
        END_BANK_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Start background index - Channel 0"]
    #[inline(always)]
    pub fn start_background(&self) -> START_BACKGROUND_R {
        START_BACKGROUND_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - End background index - Channel 0"]
    #[inline(always)]
    pub fn end_background(&self) -> END_BACKGROUND_R {
        END_BACKGROUND_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - March element number - Channel 0"]
    #[inline(always)]
    pub fn element(&self) -> ELEMENT_R {
        ELEMENT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - March operation number - Channel 0"]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Count-to value of retention counter (Width = RETENTION_WIDTH) - Channel 0"]
    #[inline(always)]
    pub fn retention(&self) -> RETENTION_R {
        RETENTION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable diagnosis mode - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn diagnosis_en(&self) -> DIAGNOSIS_EN_R {
        DIAGNOSIS_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Start bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn start_bank(&mut self) -> START_BANK_W<0> {
        START_BANK_W::new(self)
    }
    #[doc = "Bits 3:5 - End bank (Width = DRAM_BANK_WIDTH) - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn end_bank(&mut self) -> END_BANK_W<3> {
        END_BANK_W::new(self)
    }
    #[doc = "Bits 6:8 - Start background index - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn start_background(&mut self) -> START_BACKGROUND_W<6> {
        START_BACKGROUND_W::new(self)
    }
    #[doc = "Bits 9:11 - End background index - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn end_background(&mut self) -> END_BACKGROUND_W<9> {
        END_BACKGROUND_W::new(self)
    }
    #[doc = "Bits 12:15 - March element number - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn element(&mut self) -> ELEMENT_W<12> {
        ELEMENT_W::new(self)
    }
    #[doc = "Bits 16:19 - March operation number - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn operation(&mut self) -> OPERATION_W<16> {
        OPERATION_W::new(self)
    }
    #[doc = "Bits 20:23 - Count-to value of retention counter (Width = RETENTION_WIDTH) - Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn retention(&mut self) -> RETENTION_W<20> {
        RETENTION_W::new(self)
    }
    #[doc = "Bit 24 - Enable diagnosis mode - Channel 0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn diagnosis_en(&mut self) -> DIAGNOSIS_EN_W<24> {
        DIAGNOSIS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST Configuration Register - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bistcfg_ch0](index.html) module"]
pub struct BISTCFG_CH0_SPEC;
impl crate::RegisterSpec for BISTCFG_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bistcfg_ch0::R](R) reader structure"]
impl crate::Readable for BISTCFG_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bistcfg_ch0::W](W) writer structure"]
impl crate::Writable for BISTCFG_CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BISTCFG_CH0 to value 0x0011_4618"]
impl crate::Resettable for BISTCFG_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_4618;
}
