#[doc = "Register `FRLINECOUNT` reader"]
pub struct R(crate::R<FRLINECOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRLINECOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRLINECOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRLINECOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRLINECOUNT` writer"]
pub struct W(crate::W<FRLINECOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRLINECOUNT_SPEC>;
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
impl From<crate::W<FRLINECOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRLINECOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRLINECOUNT` reader - Shows current line number of input timing. Incremented with leading edge of hsync signal, set to 0 with leading edge of vsync signal."]
pub type FRLINECOUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRLINECOUNT` writer - Shows current line number of input timing. Incremented with leading edge of hsync signal, set to 0 with leading edge of vsync signal."]
pub type FRLINECOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRLINECOUNT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Shows current line number of input timing. Incremented with leading edge of hsync signal, set to 0 with leading edge of vsync signal."]
    #[inline(always)]
    pub fn frlinecount(&self) -> FRLINECOUNT_R {
        FRLINECOUNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Shows current line number of input timing. Incremented with leading edge of hsync signal, set to 0 with leading edge of vsync signal."]
    #[inline(always)]
    #[must_use]
    pub fn frlinecount(&mut self) -> FRLINECOUNT_W<0> {
        FRLINECOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap line counter register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frlinecount](index.html) module"]
pub struct FRLINECOUNT_SPEC;
impl crate::RegisterSpec for FRLINECOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frlinecount::R](R) reader structure"]
impl crate::Readable for FRLINECOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frlinecount::W](W) writer structure"]
impl crate::Writable for FRLINECOUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRLINECOUNT to value 0"]
impl crate::Resettable for FRLINECOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
