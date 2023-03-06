#[doc = "Register `SAR_TR_OUT_SEL[%s]` reader"]
pub struct R(crate::R<SAR_TR_OUT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TR_OUT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TR_OUT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TR_OUT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TR_OUT_SEL[%s]` writer"]
pub struct W(crate::W<SAR_TR_OUT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TR_OUT_SEL_SPEC>;
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
impl From<crate::W<SAR_TR_OUT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TR_OUT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT0_SEL` reader - Select SAR output trigger for generic trigger output 0 0-31: selects a tr_sar_ch_done trigger 32-63: selects a tr_sar_ch_rangvio trigger"]
pub type OUT0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT0_SEL` writer - Select SAR output trigger for generic trigger output 0 0-31: selects a tr_sar_ch_done trigger 32-63: selects a tr_sar_ch_rangvio trigger"]
pub type OUT0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TR_OUT_SEL_SPEC, u8, u8, 6, O>;
#[doc = "Field `OUT1_SEL` reader - Select SAR output trigger for generic trigger output 1"]
pub type OUT1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT1_SEL` writer - Select SAR output trigger for generic trigger output 1"]
pub type OUT1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TR_OUT_SEL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Select SAR output trigger for generic trigger output 0 0-31: selects a tr_sar_ch_done trigger 32-63: selects a tr_sar_ch_rangvio trigger"]
    #[inline(always)]
    pub fn out0_sel(&self) -> OUT0_SEL_R {
        OUT0_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select SAR output trigger for generic trigger output 1"]
    #[inline(always)]
    pub fn out1_sel(&self) -> OUT1_SEL_R {
        OUT1_SEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select SAR output trigger for generic trigger output 0 0-31: selects a tr_sar_ch_done trigger 32-63: selects a tr_sar_ch_rangvio trigger"]
    #[inline(always)]
    #[must_use]
    pub fn out0_sel(&mut self) -> OUT0_SEL_W<0> {
        OUT0_SEL_W::new(self)
    }
    #[doc = "Bits 8:13 - Select SAR output trigger for generic trigger output 1"]
    #[inline(always)]
    #[must_use]
    pub fn out1_sel(&mut self) -> OUT1_SEL_W<8> {
        OUT1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "per SAR generic output trigger select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tr_out_sel](index.html) module"]
pub struct SAR_TR_OUT_SEL_SPEC;
impl crate::RegisterSpec for SAR_TR_OUT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tr_out_sel::R](R) reader structure"]
impl crate::Readable for SAR_TR_OUT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tr_out_sel::W](W) writer structure"]
impl crate::Writable for SAR_TR_OUT_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TR_OUT_SEL[%s]
to value 0x0100"]
impl crate::Resettable for SAR_TR_OUT_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
