#[doc = "Register `CLIPWINDOWDIMENSIONS3` reader"]
pub struct R(crate::R<CLIPWINDOWDIMENSIONS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWDIMENSIONS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWDIMENSIONS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWDIMENSIONS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWDIMENSIONS3` writer"]
pub struct W(crate::W<CLIPWINDOWDIMENSIONS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWDIMENSIONS3_SPEC>;
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
impl From<crate::W<CLIPWINDOWDIMENSIONS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWDIMENSIONS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWWIDTH3` reader - Width."]
pub type CLIPWINDOWWIDTH3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWWIDTH3` writer - Width."]
pub type CLIPWINDOWWIDTH3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSIONS3_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIPWINDOWHEIGHT3` reader - Height."]
pub type CLIPWINDOWHEIGHT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWHEIGHT3` writer - Height."]
pub type CLIPWINDOWHEIGHT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSIONS3_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width."]
    #[inline(always)]
    pub fn clipwindowwidth3(&self) -> CLIPWINDOWWIDTH3_R {
        CLIPWINDOWWIDTH3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Height."]
    #[inline(always)]
    pub fn clipwindowheight3(&self) -> CLIPWINDOWHEIGHT3_R {
        CLIPWINDOWHEIGHT3_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowwidth3(&mut self) -> CLIPWINDOWWIDTH3_W<0> {
        CLIPWINDOWWIDTH3_W::new(self)
    }
    #[doc = "Bits 16:29 - Height."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowheight3(&mut self) -> CLIPWINDOWHEIGHT3_W<16> {
        CLIPWINDOWHEIGHT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window size for layer 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowdimensions3](index.html) module"]
pub struct CLIPWINDOWDIMENSIONS3_SPEC;
impl crate::RegisterSpec for CLIPWINDOWDIMENSIONS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowdimensions3::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWDIMENSIONS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowdimensions3::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWDIMENSIONS3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWDIMENSIONS3 to value 0"]
impl crate::Resettable for CLIPWINDOWDIMENSIONS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
