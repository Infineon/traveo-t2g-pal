#[doc = "Register `RXFIFOSTATUS` reader"]
pub struct R(crate::R<RXFIFOSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFOSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFOSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFOSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXLEVEL` reader - Number of entries in RxFifo."]
pub type RXLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXEMPTY` reader - Indicates if RxFifo is empty."]
pub type RXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - Indicates if RxFifo is full."]
pub type RXFULL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 16:20 - Number of entries in RxFifo."]
    #[inline(always)]
    pub fn rxlevel(&self) -> RXLEVEL_R {
        RXLEVEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Indicates if RxFifo is empty."]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates if RxFifo is full."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Reception fifo status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifostatus](index.html) module"]
pub struct RXFIFOSTATUS_SPEC;
impl crate::RegisterSpec for RXFIFOSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifostatus::R](R) reader structure"]
impl crate::Readable for RXFIFOSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIFOSTATUS to value 0x0100_0000"]
impl crate::Resettable for RXFIFOSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
