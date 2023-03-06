#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - CaptureEnable=1, enable bit for FrameCap unit. Capture starts with next frame sync signal. CaptureEnable=0, disable bit for FrameCap unit. Current frame is completed before FrameCap is stopped."]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - CaptureEnable=1, enable bit for FrameCap unit. Capture starts with next frame sync signal. CaptureEnable=0, disable bit for FrameCap unit. Current frame is completed before FrameCap is stopped."]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `BURSTTHRESHOLD` reader - When setting a burst threshold larger 0, the generation of output pixels is initially locked. It is unlocked once the number of pixels captured into an internal FIFO is larger than the programmed value or when the last active pixel of an input frame has been received. Once unlocked it is locked again when the FIFO runs empty. This function can have a positive effect on memory access efficiency, depending on the system's Video RAM type."]
pub type BURSTTHRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BURSTTHRESHOLD` writer - When setting a burst threshold larger 0, the generation of output pixels is initially locked. It is unlocked once the number of pixels captured into an internal FIFO is larger than the programmed value or when the last active pixel of an input frame has been received. Once unlocked it is locked again when the FIFO runs empty. This function can have a positive effect on memory access efficiency, depending on the system's Video RAM type."]
pub type BURSTTHRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 0 - CaptureEnable=1, enable bit for FrameCap unit. Capture starts with next frame sync signal. CaptureEnable=0, disable bit for FrameCap unit. Current frame is completed before FrameCap is stopped."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:24 - When setting a burst threshold larger 0, the generation of output pixels is initially locked. It is unlocked once the number of pixels captured into an internal FIFO is larger than the programmed value or when the last active pixel of an input frame has been received. Once unlocked it is locked again when the FIFO runs empty. This function can have a positive effect on memory access efficiency, depending on the system's Video RAM type."]
    #[inline(always)]
    pub fn burstthreshold(&self) -> BURSTTHRESHOLD_R {
        BURSTTHRESHOLD_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CaptureEnable=1, enable bit for FrameCap unit. Capture starts with next frame sync signal. CaptureEnable=0, disable bit for FrameCap unit. Current frame is completed before FrameCap is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bits 16:24 - When setting a burst threshold larger 0, the generation of output pixels is initially locked. It is unlocked once the number of pixels captured into an internal FIFO is larger than the programmed value or when the last active pixel of an input frame has been received. Once unlocked it is locked again when the FIFO runs empty. This function can have a positive effect on memory access efficiency, depending on the system's Video RAM type."]
    #[inline(always)]
    #[must_use]
    pub fn burstthreshold(&mut self) -> BURSTTHRESHOLD_W<16> {
        BURSTTHRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap control register for starting the unit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
