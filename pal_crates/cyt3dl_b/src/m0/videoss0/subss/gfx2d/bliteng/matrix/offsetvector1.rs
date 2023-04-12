#[doc = "Register `OFFSETVECTOR1` reader"]
pub struct R(crate::R<OFFSETVECTOR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFSETVECTOR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFSETVECTOR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFSETVECTOR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFSETVECTOR1` writer"]
pub struct W(crate::W<OFFSETVECTOR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFSETVECTOR1_SPEC>;
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
impl From<crate::W<OFFSETVECTOR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFSETVECTOR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C3` reader - Blue output offset. (format is signed integer)"]
pub type C3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C3` writer - Blue output offset. (format is signed integer)"]
pub type C3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFFSETVECTOR1_SPEC, u16, u16, 13, O>;
#[doc = "Field `C4` reader - Alpha output offset. Note that the 8-bit alpha input is up-scaled to 10-bit, before the matrix and this offset is applied, and down-scaled to 8-bit for output afterwards. (format is signed integer)"]
pub type C4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C4` writer - Alpha output offset. Note that the 8-bit alpha input is up-scaled to 10-bit, before the matrix and this offset is applied, and down-scaled to 8-bit for output afterwards. (format is signed integer)"]
pub type C4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFFSETVECTOR1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Blue output offset. (format is signed integer)"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Alpha output offset. Note that the 8-bit alpha input is up-scaled to 10-bit, before the matrix and this offset is applied, and down-scaled to 8-bit for output afterwards. (format is signed integer)"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Blue output offset. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn c3(&mut self) -> C3_W<0> {
        C3_W::new(self)
    }
    #[doc = "Bits 16:28 - Alpha output offset. Note that the 8-bit alpha input is up-scaled to 10-bit, before the matrix and this offset is applied, and down-scaled to 8-bit for output afterwards. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn c4(&mut self) -> C4_W<16> {
        C4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset vectors for blue and alpha output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsetvector1](index.html) module"]
pub struct OFFSETVECTOR1_SPEC;
impl crate::RegisterSpec for OFFSETVECTOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [offsetvector1::R](R) reader structure"]
impl crate::Readable for OFFSETVECTOR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offsetvector1::W](W) writer structure"]
impl crate::Writable for OFFSETVECTOR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFFSETVECTOR1 to value 0"]
impl crate::Resettable for OFFSETVECTOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
