#[doc = "Register `BISTEDADDR_CH1` reader"]
pub struct R(crate::R<BISTEDADDR_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTEDADDR_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTEDADDR_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTEDADDR_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BISTEDADDR_CH1` writer"]
pub struct W(crate::W<BISTEDADDR_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BISTEDADDR_CH1_SPEC>;
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
impl From<crate::W<BISTEDADDR_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BISTEDADDR_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `END_ROW` reader - End row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
pub type END_ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `END_ROW` writer - End row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
pub type END_ROW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTEDADDR_CH1_SPEC, u16, u16, 15, O>;
#[doc = "Field `END_COL` reader - End column address (Width = DRAM_COL_WIDTH) - Channel 1"]
pub type END_COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `END_COL` writer - End column address (Width = DRAM_COL_WIDTH) - Channel 1"]
pub type END_COL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BISTEDADDR_CH1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:14 - End row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
    #[inline(always)]
    pub fn end_row(&self) -> END_ROW_R {
        END_ROW_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:24 - End column address (Width = DRAM_COL_WIDTH) - Channel 1"]
    #[inline(always)]
    pub fn end_col(&self) -> END_COL_R {
        END_COL_R::new(((self.bits >> 15) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - End row address (Width = DRAM_ROW_WIDTH) - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn end_row(&mut self) -> END_ROW_W<0> {
        END_ROW_W::new(self)
    }
    #[doc = "Bits 15:24 - End column address (Width = DRAM_COL_WIDTH) - Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn end_col(&mut self) -> END_COL_W<15> {
        END_COL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST End Address - Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bistedaddr_ch1](index.html) module"]
pub struct BISTEDADDR_CH1_SPEC;
impl crate::RegisterSpec for BISTEDADDR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bistedaddr_ch1::R](R) reader structure"]
impl crate::Readable for BISTEDADDR_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bistedaddr_ch1::W](W) writer structure"]
impl crate::Writable for BISTEDADDR_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BISTEDADDR_CH1 to value 0x0010_0003"]
impl crate::Resettable for BISTEDADDR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0003;
}
