#[doc = "Register `EVALLOWERRIGHT3` reader"]
pub struct R(crate::R<EVALLOWERRIGHT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALLOWERRIGHT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALLOWERRIGHT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALLOWERRIGHT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALLOWERRIGHT3` writer"]
pub struct W(crate::W<EVALLOWERRIGHT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALLOWERRIGHT3_SPEC>;
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
impl From<crate::W<EVALLOWERRIGHT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALLOWERRIGHT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALLOWERRIGHT3` reader - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALLOWERRIGHT3` writer - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT3_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALLOWERRIGHT3` reader - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALLOWERRIGHT3` writer - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT3_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    pub fn xevallowerright3(&self) -> XEVALLOWERRIGHT3_R {
        XEVALLOWERRIGHT3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    pub fn yevallowerright3(&self) -> YEVALLOWERRIGHT3_R {
        YEVALLOWERRIGHT3_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn xevallowerright3(&mut self) -> XEVALLOWERRIGHT3_W<0> {
        XEVALLOWERRIGHT3_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn yevallowerright3(&mut self) -> YEVALLOWERRIGHT3_W<16> {
        YEVALLOWERRIGHT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower right corner of evaluation window 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evallowerright3](index.html) module"]
pub struct EVALLOWERRIGHT3_SPEC;
impl crate::RegisterSpec for EVALLOWERRIGHT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evallowerright3::R](R) reader structure"]
impl crate::Readable for EVALLOWERRIGHT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evallowerright3::W](W) writer structure"]
impl crate::Writable for EVALLOWERRIGHT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALLOWERRIGHT3 to value 0"]
impl crate::Resettable for EVALLOWERRIGHT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
