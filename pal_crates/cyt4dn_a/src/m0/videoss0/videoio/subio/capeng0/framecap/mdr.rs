#[doc = "Register `MDR` reader"]
pub struct R(crate::R<MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDR` writer"]
pub struct W(crate::W<MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDR_SPEC>;
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
impl From<crate::W<MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDR_EN` reader - Mdr_en=1, enable bit for FrameCap mode detection and pixel frame rate measurement for the captured frame. Make sure that MdSts0.MdrCmrDone is 1 before starting it again. Mdr_en=0, disable bit for FrameCap mode detection."]
pub type MDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `MDR_EN` writer - Mdr_en=1, enable bit for FrameCap mode detection and pixel frame rate measurement for the captured frame. Make sure that MdSts0.MdrCmrDone is 1 before starting it again. Mdr_en=0, disable bit for FrameCap mode detection."]
pub type MDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDR_SPEC, bool, O>;
#[doc = "Field `FRAMET_OFFSET` reader - Frame size tolerance which will not generate a mode detection size mismatch. Set an offset for frame time duration."]
pub type FRAMET_OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRAMET_OFFSET` writer - Frame size tolerance which will not generate a mode detection size mismatch. Set an offset for frame time duration."]
pub type FRAMET_OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDR_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - Mdr_en=1, enable bit for FrameCap mode detection and pixel frame rate measurement for the captured frame. Make sure that MdSts0.MdrCmrDone is 1 before starting it again. Mdr_en=0, disable bit for FrameCap mode detection."]
    #[inline(always)]
    pub fn mdr_en(&self) -> MDR_EN_R {
        MDR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Frame size tolerance which will not generate a mode detection size mismatch. Set an offset for frame time duration."]
    #[inline(always)]
    pub fn framet_offset(&self) -> FRAMET_OFFSET_R {
        FRAMET_OFFSET_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Mdr_en=1, enable bit for FrameCap mode detection and pixel frame rate measurement for the captured frame. Make sure that MdSts0.MdrCmrDone is 1 before starting it again. Mdr_en=0, disable bit for FrameCap mode detection."]
    #[inline(always)]
    #[must_use]
    pub fn mdr_en(&mut self) -> MDR_EN_W<0> {
        MDR_EN_W::new(self)
    }
    #[doc = "Bits 1:31 - Frame size tolerance which will not generate a mode detection size mismatch. Set an offset for frame time duration."]
    #[inline(always)]
    #[must_use]
    pub fn framet_offset(&mut self) -> FRAMET_OFFSET_W<1> {
        FRAMET_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap control register for starting the mode detection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
