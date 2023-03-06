#[doc = "Register `CLIPWINDOWOFFSET7` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET7` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET7_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET7` reader - Horizontal position (X)."]
pub type CLIPWINDOWXOFFSET7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET7` writer - Horizontal position (X)."]
pub type CLIPWINDOWXOFFSET7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET7_SPEC, u16, u16, 15, O>;
#[doc = "Field `CLIPWINDOWYOFFSET7` reader - Vertical position (Y)."]
pub type CLIPWINDOWYOFFSET7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET7` writer - Vertical position (Y)."]
pub type CLIPWINDOWYOFFSET7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET7_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal position (X)."]
    #[inline(always)]
    pub fn clipwindowxoffset7(&self) -> CLIPWINDOWXOFFSET7_R {
        CLIPWINDOWXOFFSET7_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical position (Y)."]
    #[inline(always)]
    pub fn clipwindowyoffset7(&self) -> CLIPWINDOWYOFFSET7_R {
        CLIPWINDOWYOFFSET7_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal position (X)."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset7(&mut self) -> CLIPWINDOWXOFFSET7_W<0> {
        CLIPWINDOWXOFFSET7_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical position (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset7(&mut self) -> CLIPWINDOWYOFFSET7_W<16> {
        CLIPWINDOWYOFFSET7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window position for layer 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset7](index.html) module"]
pub struct CLIPWINDOWOFFSET7_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset7::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset7::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET7 to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
