#[doc = "Register `CLIPWINDOWDIMENSIONS0` reader"]
pub struct R(crate::R<CLIPWINDOWDIMENSIONS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWDIMENSIONS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWDIMENSIONS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWDIMENSIONS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWDIMENSIONS0` writer"]
pub struct W(crate::W<CLIPWINDOWDIMENSIONS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWDIMENSIONS0_SPEC>;
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
impl From<crate::W<CLIPWINDOWDIMENSIONS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWDIMENSIONS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWWIDTH0` reader - Width."]
pub type CLIPWINDOWWIDTH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWWIDTH0` writer - Width."]
pub type CLIPWINDOWWIDTH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSIONS0_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIPWINDOWHEIGHT0` reader - Height."]
pub type CLIPWINDOWHEIGHT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWHEIGHT0` writer - Height."]
pub type CLIPWINDOWHEIGHT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSIONS0_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width."]
    #[inline(always)]
    pub fn clipwindowwidth0(&self) -> CLIPWINDOWWIDTH0_R {
        CLIPWINDOWWIDTH0_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Height."]
    #[inline(always)]
    pub fn clipwindowheight0(&self) -> CLIPWINDOWHEIGHT0_R {
        CLIPWINDOWHEIGHT0_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowwidth0(&mut self) -> CLIPWINDOWWIDTH0_W<0> {
        CLIPWINDOWWIDTH0_W::new(self)
    }
    #[doc = "Bits 16:29 - Height."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowheight0(&mut self) -> CLIPWINDOWHEIGHT0_W<16> {
        CLIPWINDOWHEIGHT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window size for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowdimensions0](index.html) module"]
pub struct CLIPWINDOWDIMENSIONS0_SPEC;
impl crate::RegisterSpec for CLIPWINDOWDIMENSIONS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowdimensions0::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWDIMENSIONS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowdimensions0::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWDIMENSIONS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWDIMENSIONS0 to value 0"]
impl crate::Resettable for CLIPWINDOWDIMENSIONS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
