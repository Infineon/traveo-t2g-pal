#[doc = "Register `CLIPWINSIZE` reader"]
pub struct R(crate::R<CLIPWINSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINSIZE` writer"]
pub struct W(crate::W<CLIPWINSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINSIZE_SPEC>;
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
impl From<crate::W<CLIPWINSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIP_WIDTH` reader - Width of clip window, given minus one. (format is unsigned integer)"]
pub type CLIP_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIP_WIDTH` writer - Width of clip window, given minus one. (format is unsigned integer)"]
pub type CLIP_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINSIZE_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIP_HEIGHT` reader - Height of clip window, given minus one. (format is unsigned integer)"]
pub type CLIP_HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIP_HEIGHT` writer - Height of clip window, given minus one. (format is unsigned integer)"]
pub type CLIP_HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINSIZE_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width of clip window, given minus one. (format is unsigned integer)"]
    #[inline(always)]
    pub fn clip_width(&self) -> CLIP_WIDTH_R {
        CLIP_WIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Height of clip window, given minus one. (format is unsigned integer)"]
    #[inline(always)]
    pub fn clip_height(&self) -> CLIP_HEIGHT_R {
        CLIP_HEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width of clip window, given minus one. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clip_width(&mut self) -> CLIP_WIDTH_W<0> {
        CLIP_WIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Height of clip window, given minus one. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clip_height(&mut self) -> CLIP_HEIGHT_W<16> {
        CLIP_HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dimensions of clip window.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwinsize](index.html) module"]
pub struct CLIPWINSIZE_SPEC;
impl crate::RegisterSpec for CLIPWINSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwinsize::R](R) reader structure"]
impl crate::Readable for CLIPWINSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwinsize::W](W) writer structure"]
impl crate::Writable for CLIPWINSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINSIZE to value 0"]
impl crate::Resettable for CLIPWINSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
