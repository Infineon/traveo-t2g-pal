#[doc = "Register `DESIGNCFG_DEBUG9` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q8` reader - Takes the value of the `gem_tx_pbuf_num_segments_q8 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q9` reader - Takes the value of the `gem_tx_pbuf_num_segments_q9 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q10` reader - Takes the value of the `gem_tx_pbuf_num_segments_q10 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q11` reader - Takes the value of the `gem_tx_pbuf_num_segments_q11 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q12` reader - Takes the value of the `gem_tx_pbuf_num_segments_q12 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q13` reader - Takes the value of the `gem_tx_pbuf_num_segments_q13 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q14` reader - Takes the value of the `gem_tx_pbuf_num_segments_q14 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_NUM_SEGMENTS_Q15` reader - Takes the value of the `gem_tx_pbuf_num_segments_q15 DEFINE"]
pub type TX_PBUF_NUM_SEGMENTS_Q15_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Takes the value of the `gem_tx_pbuf_num_segments_q8 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q8(&self) -> TX_PBUF_NUM_SEGMENTS_Q8_R {
        TX_PBUF_NUM_SEGMENTS_Q8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Takes the value of the `gem_tx_pbuf_num_segments_q9 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q9(&self) -> TX_PBUF_NUM_SEGMENTS_Q9_R {
        TX_PBUF_NUM_SEGMENTS_Q9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Takes the value of the `gem_tx_pbuf_num_segments_q10 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q10(&self) -> TX_PBUF_NUM_SEGMENTS_Q10_R {
        TX_PBUF_NUM_SEGMENTS_Q10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Takes the value of the `gem_tx_pbuf_num_segments_q11 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q11(&self) -> TX_PBUF_NUM_SEGMENTS_Q11_R {
        TX_PBUF_NUM_SEGMENTS_Q11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Takes the value of the `gem_tx_pbuf_num_segments_q12 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q12(&self) -> TX_PBUF_NUM_SEGMENTS_Q12_R {
        TX_PBUF_NUM_SEGMENTS_Q12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Takes the value of the `gem_tx_pbuf_num_segments_q13 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q13(&self) -> TX_PBUF_NUM_SEGMENTS_Q13_R {
        TX_PBUF_NUM_SEGMENTS_Q13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Takes the value of the `gem_tx_pbuf_num_segments_q14 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q14(&self) -> TX_PBUF_NUM_SEGMENTS_Q14_R {
        TX_PBUF_NUM_SEGMENTS_Q14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Takes the value of the `gem_tx_pbuf_num_segments_q15 DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_num_segments_q15(&self) -> TX_PBUF_NUM_SEGMENTS_Q15_R {
        TX_PBUF_NUM_SEGMENTS_Q15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Design Configuration Register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug9](index.html) module"]
pub struct DESIGNCFG_DEBUG9_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug9::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG9 to value 0"]
impl crate::Resettable for DESIGNCFG_DEBUG9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
