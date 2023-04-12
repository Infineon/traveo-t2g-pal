#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFULL` reader - Write Access to the full coupling FIFO (bit locked when 1, cleared by writing StatusClear field)."]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFULL` writer - Write Access to the full coupling FIFO (bit locked when 1, cleared by writing StatusClear field)."]
pub type FIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `VSLATE` reader - VSYNC is detected too late. The received frame is too big (bit locked when 1, cleared by writing StatusClear field)."]
pub type VSLATE_R = crate::BitReader<bool>;
#[doc = "Field `VSLATE` writer - VSYNC is detected too late. The received frame is too big (bit locked when 1, cleared by writing StatusClear field)."]
pub type VSLATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `VSEARLY` reader - VSYNC is detected too early. The received frame is too small (bit locked when 1, cleared by writing StatusClear field)."]
pub type VSEARLY_R = crate::BitReader<bool>;
#[doc = "Field `VSEARLY` writer - VSYNC is detected too early. The received frame is too small (bit locked when 1, cleared by writing StatusClear field)."]
pub type VSEARLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `CSV_LOST` reader - Csv_Lost=1, capture clock is not available. Csv_Lost=0, capture clock is resumed.(bit locked when 1, cleared by writing StatusClear field)."]
pub type CSV_LOST_R = crate::BitReader<bool>;
#[doc = "Field `CSV_LOST` writer - Csv_Lost=1, capture clock is not available. Csv_Lost=0, capture clock is resumed.(bit locked when 1, cleared by writing StatusClear field)."]
pub type CSV_LOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `MDR_SIZECHANGE` reader - Mdr_SizeChange=1, frame size of the video frame detected is not consistent. (bit locked when 1, clear by writing StatusClear field). If frame size change, the clock measurement fails."]
pub type MDR_SIZECHANGE_R = crate::BitReader<bool>;
#[doc = "Field `MDR_SIZECHANGE` writer - Mdr_SizeChange=1, frame size of the video frame detected is not consistent. (bit locked when 1, clear by writing StatusClear field). If frame size change, the clock measurement fails."]
pub type MDR_SIZECHANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `MDRCMRDONE` reader - MdrCmrDone=1 means that the mode detection is completed and pixel clock measurement is done. (bit locked when 1, clear by writing StatusClear field)."]
pub type MDRCMRDONE_R = crate::BitReader<bool>;
#[doc = "Field `MDRCMRDONE` writer - MdrCmrDone=1 means that the mode detection is completed and pixel clock measurement is done. (bit locked when 1, clear by writing StatusClear field)."]
pub type MDRCMRDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `SYNCSTAT` reader - Indicates the current state of synchronization (0 = out of sync, 1 = in sync). Note: A loss of the video stream cannot be detected using this field. Use the framegen status fields for this purpose."]
pub type SYNCSTAT_R = crate::BitReader<bool>;
#[doc = "Field `SYNCSTAT` writer - Indicates the current state of synchronization (0 = out of sync, 1 = in sync). Note: A loss of the video stream cannot be detected using this field. Use the framegen status fields for this purpose."]
pub type SYNCSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write Access to the full coupling FIFO (bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC is detected too late. The received frame is too big (bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    pub fn vslate(&self) -> VSLATE_R {
        VSLATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VSYNC is detected too early. The received frame is too small (bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    pub fn vsearly(&self) -> VSEARLY_R {
        VSEARLY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Csv_Lost=1, capture clock is not available. Csv_Lost=0, capture clock is resumed.(bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    pub fn csv_lost(&self) -> CSV_LOST_R {
        CSV_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mdr_SizeChange=1, frame size of the video frame detected is not consistent. (bit locked when 1, clear by writing StatusClear field). If frame size change, the clock measurement fails."]
    #[inline(always)]
    pub fn mdr_sizechange(&self) -> MDR_SIZECHANGE_R {
        MDR_SIZECHANGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MdrCmrDone=1 means that the mode detection is completed and pixel clock measurement is done. (bit locked when 1, clear by writing StatusClear field)."]
    #[inline(always)]
    pub fn mdrcmrdone(&self) -> MDRCMRDONE_R {
        MDRCMRDONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates the current state of synchronization (0 = out of sync, 1 = in sync). Note: A loss of the video stream cannot be detected using this field. Use the framegen status fields for this purpose."]
    #[inline(always)]
    pub fn syncstat(&self) -> SYNCSTAT_R {
        SYNCSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Access to the full coupling FIFO (bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    #[must_use]
    pub fn fifofull(&mut self) -> FIFOFULL_W<0> {
        FIFOFULL_W::new(self)
    }
    #[doc = "Bit 1 - VSYNC is detected too late. The received frame is too big (bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    #[must_use]
    pub fn vslate(&mut self) -> VSLATE_W<1> {
        VSLATE_W::new(self)
    }
    #[doc = "Bit 2 - VSYNC is detected too early. The received frame is too small (bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    #[must_use]
    pub fn vsearly(&mut self) -> VSEARLY_W<2> {
        VSEARLY_W::new(self)
    }
    #[doc = "Bit 3 - Csv_Lost=1, capture clock is not available. Csv_Lost=0, capture clock is resumed.(bit locked when 1, cleared by writing StatusClear field)."]
    #[inline(always)]
    #[must_use]
    pub fn csv_lost(&mut self) -> CSV_LOST_W<3> {
        CSV_LOST_W::new(self)
    }
    #[doc = "Bit 4 - Mdr_SizeChange=1, frame size of the video frame detected is not consistent. (bit locked when 1, clear by writing StatusClear field). If frame size change, the clock measurement fails."]
    #[inline(always)]
    #[must_use]
    pub fn mdr_sizechange(&mut self) -> MDR_SIZECHANGE_W<4> {
        MDR_SIZECHANGE_W::new(self)
    }
    #[doc = "Bit 5 - MdrCmrDone=1 means that the mode detection is completed and pixel clock measurement is done. (bit locked when 1, clear by writing StatusClear field)."]
    #[inline(always)]
    #[must_use]
    pub fn mdrcmrdone(&mut self) -> MDRCMRDONE_W<5> {
        MDRCMRDONE_W::new(self)
    }
    #[doc = "Bit 8 - Indicates the current state of synchronization (0 = out of sync, 1 = in sync). Note: A loss of the video stream cannot be detected using this field. Use the framegen status fields for this purpose."]
    #[inline(always)]
    #[must_use]
    pub fn syncstat(&mut self) -> SYNCSTAT_W<8> {
        SYNCSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap status register. Current status of the FrameCap module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
