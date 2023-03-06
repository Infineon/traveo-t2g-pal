#[doc = "Register `COLORLOOKUPTABLE[%s]` reader"]
pub struct R(crate::R<COLORLOOKUPTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORLOOKUPTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORLOOKUPTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORLOOKUPTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORLOOKUPTABLE[%s]` writer"]
pub struct W(crate::W<COLORLOOKUPTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORLOOKUPTABLE_SPEC>;
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
impl From<crate::W<COLORLOOKUPTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORLOOKUPTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLUTENTRY` reader - ClutEntry in LCD controller target color format (up to 18-bit)."]
pub type CLUTENTRY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CLUTENTRY` writer - ClutEntry in LCD controller target color format (up to 18-bit)."]
pub type CLUTENTRY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORLOOKUPTABLE_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - ClutEntry in LCD controller target color format (up to 18-bit)."]
    #[inline(always)]
    pub fn clutentry(&self) -> CLUTENTRY_R {
        CLUTENTRY_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - ClutEntry in LCD controller target color format (up to 18-bit)."]
    #[inline(always)]
    #[must_use]
    pub fn clutentry(&mut self) -> CLUTENTRY_W<0> {
        CLUTENTRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color lookup table memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorlookuptable](index.html) module"]
pub struct COLORLOOKUPTABLE_SPEC;
impl crate::RegisterSpec for COLORLOOKUPTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorlookuptable::R](R) reader structure"]
impl crate::Readable for COLORLOOKUPTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorlookuptable::W](W) writer structure"]
impl crate::Writable for COLORLOOKUPTABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORLOOKUPTABLE[%s]
to value 0"]
impl crate::Resettable for COLORLOOKUPTABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
