#[doc = "Register `RX_FIFO_STATUS` reader"]
pub struct R(crate::R<RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Number of used/occupied entries in the RX FIFO. The field value is in the range \\[0, 128\\]. When '0', the FIFO is empty. When '128', the FIFO is full."]
pub type USED_R = crate::FieldReader<u8, USED_A>;
#[doc = "Number of used/occupied entries in the RX FIFO. The field value is in the range \\[0, 128\\]. When '0', the FIFO is empty. When '128', the FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USED_A {
    #[doc = "0: N/A"]
    EMPTY = 0,
    #[doc = "1: N/A"]
    USED_1 = 1,
    #[doc = "128: N/A"]
    FULL = 128,
}
impl From<USED_A> for u8 {
    #[inline(always)]
    fn from(variant: USED_A) -> Self {
        variant as _
    }
}
impl USED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USED_A> {
        match self.bits {
            0 => Some(USED_A::EMPTY),
            1 => Some(USED_A::USED_1),
            128 => Some(USED_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == USED_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `USED_1`"]
    #[inline(always)]
    pub fn is_used_1(&self) -> bool {
        *self == USED_A::USED_1
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == USED_A::FULL
    }
}
#[doc = "Field `RD_PTR` reader - RX FIFO read pointer: FIFO location from which a data is read. Note: This functionality is intended for debugging purposes."]
pub type RD_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_PTR` reader - RX FIFO write pointer: FIFO location at which a new data is written by the hardware. Note: This functionality is intended for debugging purposes."]
pub type WR_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of used/occupied entries in the RX FIFO. The field value is in the range \\[0, 128\\]. When '0', the FIFO is empty. When '128', the FIFO is full."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - RX FIFO read pointer: FIFO location from which a data is read. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - RX FIFO write pointer: FIFO location at which a new data is written by the hardware. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "RX FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_status](index.html) module"]
pub struct RX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for RX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_status::R](R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_STATUS to value 0"]
impl crate::Resettable for RX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
