#[doc = "Register `CORRECTIONCROPSTATUS` reader"]
pub struct R(crate::R<CORRECTIONCROPSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORRECTIONCROPSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORRECTIONCROPSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORRECTIONCROPSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERVAL` reader - Huffman coding segment data exceeding the DRI have been discarded."]
pub type INTERVAL_R = crate::BitReader<bool>;
#[doc = "Field `TOTALSIZE` reader - Pixels exceeding the total image size defined in the SOF segment have been discarded."]
pub type TOTALSIZE_R = crate::BitReader<bool>;
#[doc = "Field `CROPEND` reader - Crop has finished successfully."]
pub type CROPEND_R = crate::BitReader<bool>;
#[doc = "Field `CROPSIZEERROR` reader - Crop has been cancelled due to the illegal crop region specified in CROPSTARTX/Y and CROPSIZEX/Y. When this error occurs, a normal decoding without crop is performed and flags the normal end status (INTR_DEC.NORMALEND='1') when complete."]
pub type CROPSIZEERROR_R = crate::BitReader<bool>;
#[doc = "Field `MCUUNITERROR` reader - Crop has been cancelled since the crop region specified in CROPSTARTX/Y and CROPSIZEX/Y doesn't end at the MCU boundary. When this error occurs, a normal decoding without crop is performed and flags the normal end status (INTR_DEC.NORMALEND='1') when complete."]
pub type MCUUNITERROR_R = crate::BitReader<bool>;
#[doc = "Field `NODRI` reader - DRI marker is missing in the JPEG image. When this error occurs, a normal decoding without crop is performed and flags the normal end status (INTR_DEC.NORMALEND='1') when complete."]
pub type NODRI_R = crate::BitReader<bool>;
#[doc = "Field `NORST` reader - The crop region specified in CROPSTARTX/Y and CROPSIZEX/Y doesn't have a RSTm marker at the end of each line. When this error occurs, the decoding is stopped immediately, and the normal end status is flagged (INTR_DEC.NORMALEND='1')."]
pub type NORST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Huffman coding segment data exceeding the DRI have been discarded."]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pixels exceeding the total image size defined in the SOF segment have been discarded."]
    #[inline(always)]
    pub fn totalsize(&self) -> TOTALSIZE_R {
        TOTALSIZE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Crop has finished successfully."]
    #[inline(always)]
    pub fn cropend(&self) -> CROPEND_R {
        CROPEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Crop has been cancelled due to the illegal crop region specified in CROPSTARTX/Y and CROPSIZEX/Y. When this error occurs, a normal decoding without crop is performed and flags the normal end status (INTR_DEC.NORMALEND='1') when complete."]
    #[inline(always)]
    pub fn cropsizeerror(&self) -> CROPSIZEERROR_R {
        CROPSIZEERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Crop has been cancelled since the crop region specified in CROPSTARTX/Y and CROPSIZEX/Y doesn't end at the MCU boundary. When this error occurs, a normal decoding without crop is performed and flags the normal end status (INTR_DEC.NORMALEND='1') when complete."]
    #[inline(always)]
    pub fn mcuuniterror(&self) -> MCUUNITERROR_R {
        MCUUNITERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DRI marker is missing in the JPEG image. When this error occurs, a normal decoding without crop is performed and flags the normal end status (INTR_DEC.NORMALEND='1') when complete."]
    #[inline(always)]
    pub fn nodri(&self) -> NODRI_R {
        NODRI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The crop region specified in CROPSTARTX/Y and CROPSIZEX/Y doesn't have a RSTm marker at the end of each line. When this error occurs, the decoding is stopped immediately, and the normal end status is flagged (INTR_DEC.NORMALEND='1')."]
    #[inline(always)]
    pub fn norst(&self) -> NORST_R {
        NORST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Correction Mode and Crop Decoding status. This register is initialized by the decoding start trigger (CMD.START).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [correctioncropstatus](index.html) module"]
pub struct CORRECTIONCROPSTATUS_SPEC;
impl crate::RegisterSpec for CORRECTIONCROPSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [correctioncropstatus::R](R) reader structure"]
impl crate::Readable for CORRECTIONCROPSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORRECTIONCROPSTATUS to value 0"]
impl crate::Resettable for CORRECTIONCROPSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
