#[doc = "Register `CLIPWINDOWDIMENSIONS1` reader"]
pub struct R(crate::R<CLIPWINDOWDIMENSIONS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWDIMENSIONS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWDIMENSIONS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWDIMENSIONS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWDIMENSIONS1` writer"]
pub struct W(crate::W<CLIPWINDOWDIMENSIONS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWDIMENSIONS1_SPEC>;
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
impl From<crate::W<CLIPWINDOWDIMENSIONS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWDIMENSIONS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWWIDTH1` reader - Width."]
pub type CLIPWINDOWWIDTH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWWIDTH1` writer - Width."]
pub type CLIPWINDOWWIDTH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSIONS1_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIPWINDOWHEIGHT1` reader - Height."]
pub type CLIPWINDOWHEIGHT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWHEIGHT1` writer - Height."]
pub type CLIPWINDOWHEIGHT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWDIMENSIONS1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width."]
    #[inline(always)]
    pub fn clipwindowwidth1(&self) -> CLIPWINDOWWIDTH1_R {
        CLIPWINDOWWIDTH1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Height."]
    #[inline(always)]
    pub fn clipwindowheight1(&self) -> CLIPWINDOWHEIGHT1_R {
        CLIPWINDOWHEIGHT1_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowwidth1(&mut self) -> CLIPWINDOWWIDTH1_W<0> {
        CLIPWINDOWWIDTH1_W::new(self)
    }
    #[doc = "Bits 16:29 - Height."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowheight1(&mut self) -> CLIPWINDOWHEIGHT1_W<16> {
        CLIPWINDOWHEIGHT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window size for layer 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowdimensions1](index.html) module"]
pub struct CLIPWINDOWDIMENSIONS1_SPEC;
impl crate::RegisterSpec for CLIPWINDOWDIMENSIONS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowdimensions1::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWDIMENSIONS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowdimensions1::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWDIMENSIONS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWDIMENSIONS1 to value 0"]
impl crate::Resettable for CLIPWINDOWDIMENSIONS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
