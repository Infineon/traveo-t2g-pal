#[doc = "Register `CLIPWINDOWOFFSET5` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET5` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET5_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET5` reader - Horizontal position (X). (format is signed integer)"]
pub type CLIPWINDOWXOFFSET5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET5` writer - Horizontal position (X). (format is signed integer)"]
pub type CLIPWINDOWXOFFSET5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET5_SPEC, u16, u16, 15, O>;
#[doc = "Field `CLIPWINDOWYOFFSET5` reader - Vertical position (Y). (format is signed integer)"]
pub type CLIPWINDOWYOFFSET5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET5` writer - Vertical position (Y). (format is signed integer)"]
pub type CLIPWINDOWYOFFSET5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET5_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal position (X). (format is signed integer)"]
    #[inline(always)]
    pub fn clipwindowxoffset5(&self) -> CLIPWINDOWXOFFSET5_R {
        CLIPWINDOWXOFFSET5_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical position (Y). (format is signed integer)"]
    #[inline(always)]
    pub fn clipwindowyoffset5(&self) -> CLIPWINDOWYOFFSET5_R {
        CLIPWINDOWYOFFSET5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal position (X). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset5(&mut self) -> CLIPWINDOWXOFFSET5_W<0> {
        CLIPWINDOWXOFFSET5_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical position (Y). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset5(&mut self) -> CLIPWINDOWYOFFSET5_W<16> {
        CLIPWINDOWYOFFSET5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window position for layer 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset5](index.html) module"]
pub struct CLIPWINDOWOFFSET5_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset5::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset5::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET5 to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
