#[doc = "Register `CLIPWINDOWOFFSET` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET` reader - Clip window offset in X direction, relative to the frame origin."]
pub type CLIPWINDOWXOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET` writer - Clip window offset in X direction, relative to the frame origin."]
pub type CLIPWINDOWXOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET_SPEC, u16, u16, 14, O>;
#[doc = "Field `CLIPWINDOWYOFFSET` reader - Clip window offset in Y direction, relative to the frame origin."]
pub type CLIPWINDOWYOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET` writer - Clip window offset in Y direction, relative to the frame origin."]
pub type CLIPWINDOWYOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Clip window offset in X direction, relative to the frame origin."]
    #[inline(always)]
    pub fn clipwindowxoffset(&self) -> CLIPWINDOWXOFFSET_R {
        CLIPWINDOWXOFFSET_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Clip window offset in Y direction, relative to the frame origin."]
    #[inline(always)]
    pub fn clipwindowyoffset(&self) -> CLIPWINDOWYOFFSET_R {
        CLIPWINDOWYOFFSET_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Clip window offset in X direction, relative to the frame origin."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset(&mut self) -> CLIPWINDOWXOFFSET_W<0> {
        CLIPWINDOWXOFFSET_W::new(self)
    }
    #[doc = "Bits 16:29 - Clip window offset in Y direction, relative to the frame origin."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset(&mut self) -> CLIPWINDOWYOFFSET_W<16> {
        CLIPWINDOWYOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window offset, to generate a clipping of the frame. It has to be within the input frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset](index.html) module"]
pub struct CLIPWINDOWOFFSET_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
