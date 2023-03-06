#[doc = "Register `CLIPWINDOWDIMENSION` reader"]
pub struct R(crate::R<CLIPWINDOWDIMENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWDIMENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWDIMENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWDIMENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWDIMENSION` writer"]
pub struct W(crate::W<CLIPWINDOWDIMENSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWDIMENSION_SPEC>;
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
impl From<crate::W<CLIPWINDOWDIMENSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWDIMENSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWWIDTH` reader - Clip window width in pixels minus one."]
pub type CLIPWINDOWWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWWIDTH` writer - Clip window width in pixels minus one."]
pub type CLIPWINDOWWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSION_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIPWINDOWHEIGHT` reader - Clip window height in pixels minus one."]
pub type CLIPWINDOWHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWHEIGHT` writer - Clip window height in pixels minus one."]
pub type CLIPWINDOWHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSION_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Clip window width in pixels minus one."]
    #[inline(always)]
    pub fn clipwindowwidth(&self) -> CLIPWINDOWWIDTH_R {
        CLIPWINDOWWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Clip window height in pixels minus one."]
    #[inline(always)]
    pub fn clipwindowheight(&self) -> CLIPWINDOWHEIGHT_R {
        CLIPWINDOWHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Clip window width in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowwidth(&mut self) -> CLIPWINDOWWIDTH_W<0> {
        CLIPWINDOWWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Clip window height in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowheight(&mut self) -> CLIPWINDOWHEIGHT_W<16> {
        CLIPWINDOWHEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Define the clip window dimension. If the clip window feature is enabled this dimension is used for the new frame dimension. Note that the clip window has to be smaller or equal to the original frame dimensions. The new frame has to be within the active area of the original frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowdimension](index.html) module"]
pub struct CLIPWINDOWDIMENSION_SPEC;
impl crate::RegisterSpec for CLIPWINDOWDIMENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowdimension::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWDIMENSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowdimension::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWDIMENSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWDIMENSION to value 0"]
impl crate::Resettable for CLIPWINDOWDIMENSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
