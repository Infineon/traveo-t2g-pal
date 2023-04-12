#[doc = "Register `LBHLINK0BUFFERCONFIG1` reader"]
pub struct R(crate::R<LBHLINK0BUFFERCONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK0BUFFERCONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK0BUFFERCONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK0BUFFERCONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LBHLINK0BUFFERCONFIG1` writer"]
pub struct W(crate::W<LBHLINK0BUFFERCONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBHLINK0BUFFERCONFIG1_SPEC>;
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
impl From<crate::W<LBHLINK0BUFFERCONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBHLINK0BUFFERCONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBHLINK0FRAMELINES` reader - Height of the frame (programmed with -1). This setting must be identical to the display fetch and BlitEng value."]
pub type LBHLINK0FRAMELINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK0FRAMELINES` writer - Height of the frame (programmed with -1). This setting must be identical to the display fetch and BlitEng value."]
pub type LBHLINK0FRAMELINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK0BUFFERCONFIG1_SPEC, u16, u16, 14, O>;
#[doc = "Field `LBHLINK0KEEPLINES` reader - Number of lines to keep in the line buffer even if the display fetch is already beyond this line. Required for re-sampling operations that span over several input lines."]
pub type LBHLINK0KEEPLINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK0KEEPLINES` writer - Number of lines to keep in the line buffer even if the display fetch is already beyond this line. Required for re-sampling operations that span over several input lines."]
pub type LBHLINK0KEEPLINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK0BUFFERCONFIG1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Height of the frame (programmed with -1). This setting must be identical to the display fetch and BlitEng value."]
    #[inline(always)]
    pub fn lbhlink0framelines(&self) -> LBHLINK0FRAMELINES_R {
        LBHLINK0FRAMELINES_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Number of lines to keep in the line buffer even if the display fetch is already beyond this line. Required for re-sampling operations that span over several input lines."]
    #[inline(always)]
    pub fn lbhlink0keeplines(&self) -> LBHLINK0KEEPLINES_R {
        LBHLINK0KEEPLINES_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Height of the frame (programmed with -1). This setting must be identical to the display fetch and BlitEng value."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink0framelines(&mut self) -> LBHLINK0FRAMELINES_W<0> {
        LBHLINK0FRAMELINES_W::new(self)
    }
    #[doc = "Bits 16:29 - Number of lines to keep in the line buffer even if the display fetch is already beyond this line. Required for re-sampling operations that span over several input lines."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink0keeplines(&mut self) -> LBHLINK0KEEPLINES_W<16> {
        LBHLINK0KEEPLINES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line buffer configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink0bufferconfig1](index.html) module"]
pub struct LBHLINK0BUFFERCONFIG1_SPEC;
impl crate::RegisterSpec for LBHLINK0BUFFERCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink0bufferconfig1::R](R) reader structure"]
impl crate::Readable for LBHLINK0BUFFERCONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lbhlink0bufferconfig1::W](W) writer structure"]
impl crate::Writable for LBHLINK0BUFFERCONFIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LBHLINK0BUFFERCONFIG1 to value 0"]
impl crate::Resettable for LBHLINK0BUFFERCONFIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
