#[doc = "Register `FRAMEOFFSET` reader"]
pub struct R(crate::R<FRAMEOFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEOFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEOFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEOFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEOFFSET` writer"]
pub struct W(crate::W<FRAMEOFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEOFFSET_SPEC>;
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
impl From<crate::W<FRAMEOFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEOFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEXOFFSET` reader - Horizontal offset (X). (format is signed integer)"]
pub type FRAMEXOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEXOFFSET` writer - Horizontal offset (X). (format is signed integer)"]
pub type FRAMEXOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMEOFFSET_SPEC, u16, u16, 15, O>;
#[doc = "Field `FRAMEYOFFSET` reader - Vertical offset (Y). (format is signed integer)"]
pub type FRAMEYOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEYOFFSET` writer - Vertical offset (Y). (format is signed integer)"]
pub type FRAMEYOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMEOFFSET_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X). (format is signed integer)"]
    #[inline(always)]
    pub fn framexoffset(&self) -> FRAMEXOFFSET_R {
        FRAMEXOFFSET_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y). (format is signed integer)"]
    #[inline(always)]
    pub fn frameyoffset(&self) -> FRAMEYOFFSET_R {
        FRAMEYOFFSET_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn framexoffset(&mut self) -> FRAMEXOFFSET_W<0> {
        FRAMEXOFFSET_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn frameyoffset(&mut self) -> FRAMEYOFFSET_W<16> {
        FRAMEYOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset between destination frame and buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameoffset](index.html) module"]
pub struct FRAMEOFFSET_SPEC;
impl crate::RegisterSpec for FRAMEOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameoffset::R](R) reader structure"]
impl crate::Readable for FRAMEOFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frameoffset::W](W) writer structure"]
impl crate::Writable for FRAMEOFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEOFFSET to value 0"]
impl crate::Resettable for FRAMEOFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
