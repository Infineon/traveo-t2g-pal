#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Status of all channels. 1 indicates incomplete transfer."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW` reader - This bit is set with any counter overflow (equal 0xffffffff). The counter will remain on the overflow value in this case. This field is updated with SAVE_AND_RESET_MEASUREMENTS pulse."]
pub type OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `CH0_BUSY` reader - Status of all channels. 0 indicates incomplete transfer."]
pub type CH0_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CH1_BUSY` reader - Status of all channels. 1 indicates incomplete transfer."]
pub type CH1_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CH2_BUSY` reader - Status of all channels. 2 indicates incomplete transfer."]
pub type CH2_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CH3_BUSY` reader - Status of all channels. 3 indicates incomplete transfer."]
pub type CH3_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Status of all channels. 1 indicates incomplete transfer."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set with any counter overflow (equal 0xffffffff). The counter will remain on the overflow value in this case. This field is updated with SAVE_AND_RESET_MEASUREMENTS pulse."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of all channels. 0 indicates incomplete transfer."]
    #[inline(always)]
    pub fn ch0_busy(&self) -> CH0_BUSY_R {
        CH0_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of all channels. 1 indicates incomplete transfer."]
    #[inline(always)]
    pub fn ch1_busy(&self) -> CH1_BUSY_R {
        CH1_BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status of all channels. 2 indicates incomplete transfer."]
    #[inline(always)]
    pub fn ch2_busy(&self) -> CH2_BUSY_R {
        CH2_BUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of all channels. 3 indicates incomplete transfer."]
    #[inline(always)]
    pub fn ch3_busy(&self) -> CH3_BUSY_R {
        CH3_BUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Status information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
