#[doc = "Register `CMSTS2` reader"]
pub struct R(crate::R<CMSTS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMSTS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMSTS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMSTS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMSTS2` writer"]
pub struct W(crate::W<CMSTS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMSTS2_SPEC>;
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
impl From<crate::W<CMSTS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMSTS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTWIDTHT` reader - cycle number of the total part of a video frame line for calculating pixel frame rate."]
pub type TOTWIDTHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTWIDTHT` writer - cycle number of the total part of a video frame line for calculating pixel frame rate."]
pub type TOTWIDTHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMSTS2_SPEC, u16, u16, 15, O>;
#[doc = "Field `ACTWIDTHT` reader - cycle number of the active part of a video frame line for calculating pixel frame rate."]
pub type ACTWIDTHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTWIDTHT` writer - cycle number of the active part of a video frame line for calculating pixel frame rate."]
pub type ACTWIDTHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMSTS2_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:14 - cycle number of the total part of a video frame line for calculating pixel frame rate."]
    #[inline(always)]
    pub fn totwidtht(&self) -> TOTWIDTHT_R {
        TOTWIDTHT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:28 - cycle number of the active part of a video frame line for calculating pixel frame rate."]
    #[inline(always)]
    pub fn actwidtht(&self) -> ACTWIDTHT_R {
        ACTWIDTHT_R::new(((self.bits >> 15) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - cycle number of the total part of a video frame line for calculating pixel frame rate."]
    #[inline(always)]
    #[must_use]
    pub fn totwidtht(&mut self) -> TOTWIDTHT_W<0> {
        TOTWIDTHT_W::new(self)
    }
    #[doc = "Bits 15:28 - cycle number of the active part of a video frame line for calculating pixel frame rate."]
    #[inline(always)]
    #[must_use]
    pub fn actwidtht(&mut self) -> ACTWIDTHT_W<15> {
        ACTWIDTHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_axi cycle number of totwidth and actwidth of a frame. (bit locked when MdrCmrDone=1).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmsts2](index.html) module"]
pub struct CMSTS2_SPEC;
impl crate::RegisterSpec for CMSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmsts2::R](R) reader structure"]
impl crate::Readable for CMSTS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmsts2::W](W) writer structure"]
impl crate::Writable for CMSTS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMSTS2 to value 0"]
impl crate::Resettable for CMSTS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
