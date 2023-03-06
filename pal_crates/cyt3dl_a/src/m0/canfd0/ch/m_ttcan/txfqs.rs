#[doc = "Register `TXFQS` reader"]
pub struct R(crate::R<TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFFL` reader - Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1')"]
pub type TFFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFGI` reader - Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured TXBC.TFQM = '1')."]
pub type TFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
pub type TFQPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full"]
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1')"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured TXBC.TFQM = '1')."]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Tx FIFO/Queue Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](index.html) module"]
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfqs::R](R) reader structure"]
impl crate::Readable for TXFQS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
