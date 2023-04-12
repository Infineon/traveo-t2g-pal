#[doc = "Register `FRCNT` reader"]
pub struct R(crate::R<FRCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCNT` writer"]
pub struct W(crate::W<FRCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCNT_SPEC>;
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
impl From<crate::W<FRCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRCOUNT` reader - Shows frame duration of video input counted by axi clock cycles. Updated using vsync (for debugging purposes only)."]
pub type FRCOUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRCOUNT` writer - Shows frame duration of video input counted by axi clock cycles. Updated using vsync (for debugging purposes only)."]
pub type FRCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shows frame duration of video input counted by axi clock cycles. Updated using vsync (for debugging purposes only)."]
    #[inline(always)]
    pub fn frcount(&self) -> FRCOUNT_R {
        FRCOUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shows frame duration of video input counted by axi clock cycles. Updated using vsync (for debugging purposes only)."]
    #[inline(always)]
    #[must_use]
    pub fn frcount(&mut self) -> FRCOUNT_W<0> {
        FRCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap frame counter register indicating the corresponding clk_axi cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frcnt](index.html) module"]
pub struct FRCNT_SPEC;
impl crate::RegisterSpec for FRCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frcnt::R](R) reader structure"]
impl crate::Readable for FRCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frcnt::W](W) writer structure"]
impl crate::Writable for FRCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRCNT to value 0"]
impl crate::Resettable for FRCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
