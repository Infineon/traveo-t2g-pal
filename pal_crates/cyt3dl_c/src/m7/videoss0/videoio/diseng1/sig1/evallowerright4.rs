#[doc = "Register `EVALLOWERRIGHT4` reader"]
pub struct R(crate::R<EVALLOWERRIGHT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALLOWERRIGHT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALLOWERRIGHT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALLOWERRIGHT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALLOWERRIGHT4` writer"]
pub struct W(crate::W<EVALLOWERRIGHT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALLOWERRIGHT4_SPEC>;
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
impl From<crate::W<EVALLOWERRIGHT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALLOWERRIGHT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALLOWERRIGHT4` reader - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALLOWERRIGHT4` writer - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT4_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALLOWERRIGHT4` reader - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALLOWERRIGHT4` writer - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT4_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    pub fn xevallowerright4(&self) -> XEVALLOWERRIGHT4_R {
        XEVALLOWERRIGHT4_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    pub fn yevallowerright4(&self) -> YEVALLOWERRIGHT4_R {
        YEVALLOWERRIGHT4_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn xevallowerright4(&mut self) -> XEVALLOWERRIGHT4_W<0> {
        XEVALLOWERRIGHT4_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn yevallowerright4(&mut self) -> YEVALLOWERRIGHT4_W<16> {
        YEVALLOWERRIGHT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower right corner of evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evallowerright4](index.html) module"]
pub struct EVALLOWERRIGHT4_SPEC;
impl crate::RegisterSpec for EVALLOWERRIGHT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evallowerright4::R](R) reader structure"]
impl crate::Readable for EVALLOWERRIGHT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evallowerright4::W](W) writer structure"]
impl crate::Writable for EVALLOWERRIGHT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALLOWERRIGHT4 to value 0"]
impl crate::Resettable for EVALLOWERRIGHT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
