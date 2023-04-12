#[doc = "Register `CLIPWINDOWOFFSET3` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET3` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET3_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET3` reader - Horizontal position (X). (format is signed integer)"]
pub type CLIPWINDOWXOFFSET3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET3` writer - Horizontal position (X). (format is signed integer)"]
pub type CLIPWINDOWXOFFSET3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET3_SPEC, u16, u16, 15, O>;
#[doc = "Field `CLIPWINDOWYOFFSET3` reader - Vertical position (Y). (format is signed integer)"]
pub type CLIPWINDOWYOFFSET3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET3` writer - Vertical position (Y). (format is signed integer)"]
pub type CLIPWINDOWYOFFSET3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET3_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal position (X). (format is signed integer)"]
    #[inline(always)]
    pub fn clipwindowxoffset3(&self) -> CLIPWINDOWXOFFSET3_R {
        CLIPWINDOWXOFFSET3_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical position (Y). (format is signed integer)"]
    #[inline(always)]
    pub fn clipwindowyoffset3(&self) -> CLIPWINDOWYOFFSET3_R {
        CLIPWINDOWYOFFSET3_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal position (X). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset3(&mut self) -> CLIPWINDOWXOFFSET3_W<0> {
        CLIPWINDOWXOFFSET3_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical position (Y). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset3(&mut self) -> CLIPWINDOWYOFFSET3_W<16> {
        CLIPWINDOWYOFFSET3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window position for layer 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset3](index.html) module"]
pub struct CLIPWINDOWOFFSET3_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset3::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset3::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET3 to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
