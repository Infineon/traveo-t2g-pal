#[doc = "Register `CLIPWINUPPERLEFT` reader"]
pub struct R(crate::R<CLIPWINUPPERLEFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINUPPERLEFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINUPPERLEFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINUPPERLEFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINUPPERLEFT` writer"]
pub struct W(crate::W<CLIPWINUPPERLEFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINUPPERLEFT_SPEC>;
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
impl From<crate::W<CLIPWINUPPERLEFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINUPPERLEFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIP_X` reader - X coordinate of upper left corner of clip window."]
pub type CLIP_X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIP_X` writer - X coordinate of upper left corner of clip window."]
pub type CLIP_X_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINUPPERLEFT_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIP_Y` reader - Y coordinate of upper left corner of clip window."]
pub type CLIP_Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIP_Y` writer - Y coordinate of upper left corner of clip window."]
pub type CLIP_Y_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINUPPERLEFT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - X coordinate of upper left corner of clip window."]
    #[inline(always)]
    pub fn clip_x(&self) -> CLIP_X_R {
        CLIP_X_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Y coordinate of upper left corner of clip window."]
    #[inline(always)]
    pub fn clip_y(&self) -> CLIP_Y_R {
        CLIP_Y_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - X coordinate of upper left corner of clip window."]
    #[inline(always)]
    #[must_use]
    pub fn clip_x(&mut self) -> CLIP_X_W<0> {
        CLIP_X_W::new(self)
    }
    #[doc = "Bits 16:29 - Y coordinate of upper left corner of clip window."]
    #[inline(always)]
    #[must_use]
    pub fn clip_y(&mut self) -> CLIP_Y_W<16> {
        CLIP_Y_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of upper left corner of clip window.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwinupperleft](index.html) module"]
pub struct CLIPWINUPPERLEFT_SPEC;
impl crate::RegisterSpec for CLIPWINUPPERLEFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwinupperleft::R](R) reader structure"]
impl crate::Readable for CLIPWINUPPERLEFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwinupperleft::W](W) writer structure"]
impl crate::Writable for CLIPWINUPPERLEFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINUPPERLEFT to value 0"]
impl crate::Resettable for CLIPWINUPPERLEFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
