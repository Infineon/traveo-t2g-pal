#[doc = "Register `CLIPWINDOWOFFSET0` reader"]
pub struct R(crate::R<CLIPWINDOWOFFSET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIPWINDOWOFFSET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIPWINDOWOFFSET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIPWINDOWOFFSET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIPWINDOWOFFSET0` writer"]
pub struct W(crate::W<CLIPWINDOWOFFSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIPWINDOWOFFSET0_SPEC>;
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
impl From<crate::W<CLIPWINDOWOFFSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIPWINDOWOFFSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIPWINDOWXOFFSET0` reader - Horizontal position (X)."]
pub type CLIPWINDOWXOFFSET0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWXOFFSET0` writer - Horizontal position (X)."]
pub type CLIPWINDOWXOFFSET0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET0_SPEC, u16, u16, 15, O>;
#[doc = "Field `CLIPWINDOWYOFFSET0` reader - Vertical position (Y)."]
pub type CLIPWINDOWYOFFSET0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLIPWINDOWYOFFSET0` writer - Vertical position (Y)."]
pub type CLIPWINDOWYOFFSET0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLIPWINDOWOFFSET0_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal position (X)."]
    #[inline(always)]
    pub fn clipwindowxoffset0(&self) -> CLIPWINDOWXOFFSET0_R {
        CLIPWINDOWXOFFSET0_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical position (Y)."]
    #[inline(always)]
    pub fn clipwindowyoffset0(&self) -> CLIPWINDOWYOFFSET0_R {
        CLIPWINDOWYOFFSET0_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal position (X)."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowxoffset0(&mut self) -> CLIPWINDOWXOFFSET0_W<0> {
        CLIPWINDOWXOFFSET0_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical position (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowyoffset0(&mut self) -> CLIPWINDOWYOFFSET0_W<16> {
        CLIPWINDOWYOFFSET0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clip window position for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clipwindowoffset0](index.html) module"]
pub struct CLIPWINDOWOFFSET0_SPEC;
impl crate::RegisterSpec for CLIPWINDOWOFFSET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clipwindowoffset0::R](R) reader structure"]
impl crate::Readable for CLIPWINDOWOFFSET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clipwindowoffset0::W](W) writer structure"]
impl crate::Writable for CLIPWINDOWOFFSET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLIPWINDOWOFFSET0 to value 0"]
impl crate::Resettable for CLIPWINDOWOFFSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
