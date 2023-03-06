#[doc = "Register `CLIPWINDOWOFFSET2` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET2` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET2_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET2` reader - Horizontal position (X). (format is signed integer)"]
pub type CLIPWINDOWXOFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET2` writer - Horizontal position (X). (format is signed integer)"]
pub type CLIPWINDOWXOFFSET2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET2_SPEC, u16, u16, 15, O>;
#[doc = "Field `CLIPWINDOWYOFFSET2` reader - Vertical position (Y). (format is signed integer)"]
pub type CLIPWINDOWYOFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET2` writer - Vertical position (Y). (format is signed integer)"]
pub type CLIPWINDOWYOFFSET2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET2_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal position (X). (format is signed integer)"]
    #[inline(always)]
    pub fn clipwindowxoffset2(&self) -> CLIPWINDOWXOFFSET2_R {
        CLIPWINDOWXOFFSET2_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical position (Y). (format is signed integer)"]
    #[inline(always)]
    pub fn clipwindowyoffset2(&self) -> CLIPWINDOWYOFFSET2_R {
        CLIPWINDOWYOFFSET2_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal position (X). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset2(&mut self) -> CLIPWINDOWXOFFSET2_W<0> {
        CLIPWINDOWXOFFSET2_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical position (Y). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset2(&mut self) -> CLIPWINDOWYOFFSET2_W<16> {
        CLIPWINDOWYOFFSET2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window position for layer 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset2](index.html) module"]
pub struct CLIPWINDOWOFFSET2_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset2::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset2::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET2 to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
