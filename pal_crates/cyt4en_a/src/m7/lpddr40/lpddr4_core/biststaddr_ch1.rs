#[doc = "Register `BISTSTADDR_CH1` reader"]
pub struct R(crate::R<BISTSTADDR_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTSTADDR_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTSTADDR_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTSTADDR_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BISTSTADDR_CH1` writer"]
pub struct W(crate::W<BISTSTADDR_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BISTSTADDR_CH1_SPEC>;
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
impl From<crate::W<BISTSTADDR_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BISTSTADDR_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_ROW` reader - Start row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
pub type START_ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `START_ROW` writer - Start row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
pub type START_ROW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTSTADDR_CH1_SPEC, u16, u16, 15, O>;
#[doc = "Field `START_COL` reader - Start column address (Width = DRAM_COL_WIDTH) - Channel 1"]
pub type START_COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `START_COL` writer - Start column address (Width = DRAM_COL_WIDTH) - Channel 1"]
pub type START_COL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTSTADDR_CH1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:14 - Start row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
    #[inline(always)]
    pub fn start_row(&self) -> START_ROW_R {
        START_ROW_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:24 - Start column address (Width = DRAM_COL_WIDTH) - Channel 1"]
    #[inline(always)]
    pub fn start_col(&self) -> START_COL_R {
        START_COL_R::new(((self.bits >> 15) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Start row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn start_row(&mut self) -> START_ROW_W<0> {
        START_ROW_W::new(self)
    }
    #[doc = "Bits 15:24 - Start column address (Width = DRAM_COL_WIDTH) - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn start_col(&mut self) -> START_COL_W<15> {
        START_COL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST Start Address - Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biststaddr_ch1](index.html) module"]
pub struct BISTSTADDR_CH1_SPEC;
impl crate::RegisterSpec for BISTSTADDR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biststaddr_ch1::R](R) reader structure"]
impl crate::Readable for BISTSTADDR_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biststaddr_ch1::W](W) writer structure"]
impl crate::Writable for BISTSTADDR_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BISTSTADDR_CH1 to value 0"]
impl crate::Resettable for BISTSTADDR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
