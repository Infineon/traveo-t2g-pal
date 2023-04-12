#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_COMPLETE_MASK` reader - Mask for corresponding field in INTR register."]
pub type FRAME_COMPLETE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_COMPLETE_MASK` writer - Mask for corresponding field in INTR register."]
pub type FRAME_COMPLETE_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `JPEG_CORE_MASK` reader - Mask for corresponding field in INTR register."]
pub type JPEG_CORE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `JPEG_CORE_MASK` writer - Mask for corresponding field in INTR register."]
pub type JPEG_CORE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `FETCH_ERROR_MASK` reader - Mask for corresponding field in INTR register."]
pub type FETCH_ERROR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `FETCH_ERROR_MASK` writer - Mask for corresponding field in INTR register."]
pub type FETCH_ERROR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `STORE_ERROR_MASK` reader - Mask for corresponding field in INTR register."]
pub type STORE_ERROR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `STORE_ERROR_MASK` writer - Mask for corresponding field in INTR register."]
pub type STORE_ERROR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn frame_complete_mask(&self) -> FRAME_COMPLETE_MASK_R {
        FRAME_COMPLETE_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn jpeg_core_mask(&self) -> JPEG_CORE_MASK_R {
        JPEG_CORE_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn fetch_error_mask(&self) -> FETCH_ERROR_MASK_R {
        FETCH_ERROR_MASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn store_error_mask(&self) -> STORE_ERROR_MASK_R {
        STORE_ERROR_MASK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_complete_mask(&mut self) -> FRAME_COMPLETE_MASK_W<0> {
        FRAME_COMPLETE_MASK_W::new(self)
    }
    #[doc = "Bit 8 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn jpeg_core_mask(&mut self) -> JPEG_CORE_MASK_W<8> {
        JPEG_CORE_MASK_W::new(self)
    }
    #[doc = "Bit 16 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn fetch_error_mask(&mut self) -> FETCH_ERROR_MASK_W<16> {
        FETCH_ERROR_MASK_W::new(self)
    }
    #[doc = "Bit 17 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn store_error_mask(&mut self) -> STORE_ERROR_MASK_W<17> {
        STORE_ERROR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
