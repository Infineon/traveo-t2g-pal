#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_COMPLETE` reader - This interrupt occurs when Store Unit finishes storing all pixels in the destination."]
pub type FRAME_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_COMPLETE` writer - This interrupt occurs when Store Unit finishes storing all pixels in the destination."]
pub type FRAME_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `JPEG_CORE` reader - HW sets this field to '1' when a flag in INTR_DEC is set with corresponding propagation field enabled (DECODEINTRMASK or INTR_DEC_EN). See description of INTR_DEC_EN/DECODEINTRMASK for more detail."]
pub type JPEG_CORE_R = crate::BitReader<bool>;
#[doc = "Field `JPEG_CORE` writer - HW sets this field to '1' when a flag in INTR_DEC is set with corresponding propagation field enabled (DECODEINTRMASK or INTR_DEC_EN). See description of INTR_DEC_EN/DECODEINTRMASK for more detail."]
pub type JPEG_CORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `FETCH_ERROR` reader - HW sets this field to '1' when there is an error in the on-going AXI transaction on fetch side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
pub type FETCH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `FETCH_ERROR` writer - HW sets this field to '1' when there is an error in the on-going AXI transaction on fetch side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
pub type FETCH_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `STORE_ERROR` reader - HW sets this field to '1' when there is an error in the on-going AXI transaction on store side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
pub type STORE_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `STORE_ERROR` writer - HW sets this field to '1' when there is an error in the on-going AXI transaction on store side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
pub type STORE_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This interrupt occurs when Store Unit finishes storing all pixels in the destination."]
    #[inline(always)]
    pub fn frame_complete(&self) -> FRAME_COMPLETE_R {
        FRAME_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1' when a flag in INTR_DEC is set with corresponding propagation field enabled (DECODEINTRMASK or INTR_DEC_EN). See description of INTR_DEC_EN/DECODEINTRMASK for more detail."]
    #[inline(always)]
    pub fn jpeg_core(&self) -> JPEG_CORE_R {
        JPEG_CORE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - HW sets this field to '1' when there is an error in the on-going AXI transaction on fetch side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
    #[inline(always)]
    pub fn fetch_error(&self) -> FETCH_ERROR_R {
        FETCH_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HW sets this field to '1' when there is an error in the on-going AXI transaction on store side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
    #[inline(always)]
    pub fn store_error(&self) -> STORE_ERROR_R {
        STORE_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt occurs when Store Unit finishes storing all pixels in the destination."]
    #[inline(always)]
    #[must_use]
    pub fn frame_complete(&mut self) -> FRAME_COMPLETE_W<0> {
        FRAME_COMPLETE_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1' when a flag in INTR_DEC is set with corresponding propagation field enabled (DECODEINTRMASK or INTR_DEC_EN). See description of INTR_DEC_EN/DECODEINTRMASK for more detail."]
    #[inline(always)]
    #[must_use]
    pub fn jpeg_core(&mut self) -> JPEG_CORE_W<8> {
        JPEG_CORE_W::new(self)
    }
    #[doc = "Bit 16 - HW sets this field to '1' when there is an error in the on-going AXI transaction on fetch side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
    #[inline(always)]
    #[must_use]
    pub fn fetch_error(&mut self) -> FETCH_ERROR_W<16> {
        FETCH_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - HW sets this field to '1' when there is an error in the on-going AXI transaction on store side (i.e. when one of ERRORSTATUS fields is set). Read ERRORSTATUS to see what event set this bit."]
    #[inline(always)]
    #[must_use]
    pub fn store_error(&mut self) -> STORE_ERROR_W<17> {
        STORE_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
