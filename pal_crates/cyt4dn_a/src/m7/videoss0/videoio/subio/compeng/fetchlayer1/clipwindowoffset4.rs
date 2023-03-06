#[doc = "Register `CLIPWINDOWOFFSET4` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET4` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET4_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET4` reader - Horizontal position (X)."]
pub type CLIPWINDOWXOFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET4` writer - Horizontal position (X)."]
pub type CLIPWINDOWXOFFSET4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET4_SPEC, u16, u16, 15, O>;
#[doc = "Field `CLIPWINDOWYOFFSET4` reader - Vertical position (Y)."]
pub type CLIPWINDOWYOFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET4` writer - Vertical position (Y)."]
pub type CLIPWINDOWYOFFSET4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET4_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal position (X)."]
    #[inline(always)]
    pub fn clipwindowxoffset4(&self) -> CLIPWINDOWXOFFSET4_R {
        CLIPWINDOWXOFFSET4_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical position (Y)."]
    #[inline(always)]
    pub fn clipwindowyoffset4(&self) -> CLIPWINDOWYOFFSET4_R {
        CLIPWINDOWYOFFSET4_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal position (X)."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset4(&mut self) -> CLIPWINDOWXOFFSET4_W<0> {
        CLIPWINDOWXOFFSET4_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical position (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset4(&mut self) -> CLIPWINDOWYOFFSET4_W<16> {
        CLIPWINDOWYOFFSET4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window position for layer 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset4](index.html) module"]
pub struct CLIPWINDOWOFFSET4_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset4::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset4::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET4 to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
