#[doc = "Register `RES_CAUSE2` reader"]
pub struct R(crate::R<RES_CAUSE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_CAUSE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_CAUSE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_CAUSE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_CAUSE2` writer"]
pub struct W(crate::W<RES_CAUSE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_CAUSE2_SPEC>;
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
impl From<crate::W<RES_CAUSE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_CAUSE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_CSV_HF` reader - Clock supervision logic requested a reset due to loss or frequency violation of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESET_CSV_HF` writer - Clock supervision logic requested a reset due to loss or frequency violation of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RES_CAUSE2_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESET_CSV_REF` reader - Clock supervision logic requested a reset due to loss or frequency violation of the reference clock source that is used to monitor the other HF clock sources."]
pub type RESET_CSV_REF_R = crate::BitReader<bool>;
#[doc = "Field `RESET_CSV_REF` writer - Clock supervision logic requested a reset due to loss or frequency violation of the reference clock source that is used to monitor the other HF clock sources."]
pub type RESET_CSV_REF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss or frequency violation of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf(&self) -> RESET_CSV_HF_R {
        RESET_CSV_HF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Clock supervision logic requested a reset due to loss or frequency violation of the reference clock source that is used to monitor the other HF clock sources."]
    #[inline(always)]
    pub fn reset_csv_ref(&self) -> RESET_CSV_REF_R {
        RESET_CSV_REF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss or frequency violation of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_hf(&mut self) -> RESET_CSV_HF_W<0> {
        RESET_CSV_HF_W::new(self)
    }
    #[doc = "Bit 16 - Clock supervision logic requested a reset due to loss or frequency violation of the reference clock source that is used to monitor the other HF clock sources."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_ref(&mut self) -> RESET_CSV_REF_W<16> {
        RESET_CSV_REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause Observation Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause2](index.html) module"]
pub struct RES_CAUSE2_SPEC;
impl crate::RegisterSpec for RES_CAUSE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_cause2::R](R) reader structure"]
impl crate::Readable for RES_CAUSE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_cause2::W](W) writer structure"]
impl crate::Writable for RES_CAUSE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RES_CAUSE2 to value 0"]
impl crate::Resettable for RES_CAUSE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
