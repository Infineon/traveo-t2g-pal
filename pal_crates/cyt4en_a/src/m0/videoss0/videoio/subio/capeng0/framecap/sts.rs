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
#[doc = "Field `FIFOFULL` reader - Write Access to the full coupling FIFO (bit locked when 1, cleared by writing StatusClear field)."]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `VSLATE` reader - VSYNC is detected too late. The received frame is too big (bit locked when 1, cleared by writing StatusClear field)."]
pub type VSLATE_R = crate::BitReader<bool>;
#[doc = "Field `VSEARLY` reader - VSYNC is detected too early. The received frame is too small (bit locked when 1, cleared by writing StatusClear field)."]
pub type VSEARLY_R = crate::BitReader<bool>;
#[doc = "Field `CSV_LOST` reader - Csv_Lost=1, capture clock is not available. Csv_Lost=0, capture clock is resumed.(bit locked when 1, cleared by writing StatusClear field)."]
pub type CSV_LOST_R = crate::BitReader<bool>;
#[doc = "Field `MDR_SIZECHANGE` reader - Mdr_SizeChange=1, frame size of the video frame detected is not consistent. (bit locked when 1, clear by writing StatusClear field). If frame size change, the clock measurement fails."]
pub type MDR_SIZECHANGE_R = crate::BitReader<bool>;
#[doc = "Field `MDRCMRDONE` reader - MdrCmrDone=1 means that the mode detection is completed and pixel clock measurement is done. (bit locked when 1, clear by writing StatusClear field)."]
pub type MDRCMRDONE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCSTAT` reader - Indicates the current state of synchronization (0 = out of sync, 1 = in sync). Note: A loss of the video stream cannot be detected using this field. Use the framegen status fields for this purpose."]
pub type SYNCSTAT_R = crate::BitReader<bool>;
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
#[doc = "FrameCap status register. Current status of the FrameCap module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
