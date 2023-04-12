#[doc = "Register `DESIGNCFG_DEBUG7` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q0` reader - Takes the value of the `gem_tx_pbuf_num_segments_q0 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q1` reader - Takes the value of the `gem_tx_pbuf_num_segments_q1 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q2` reader - Takes the value of the `gem_tx_pbuf_num_segments_q2 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q3` reader - Takes the value of the `gem_tx_pbuf_num_segments_q3 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q4` reader - Takes the value of the `gem_tx_pbuf_num_segments_q4 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q5` reader - Takes the value of the `gem_tx_pbuf_num_segments_q5 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q6` reader - Takes the value of the `gem_tx_pbuf_num_segments_q6 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_PBUF_NUM_SEGMENTS_Q7` reader - Takes the value of the `gem_tx_pbuf_num_segments_q7 DEFINE"]
pub type X_PBUF_NUM_SEGMENTS_Q7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Takes the value of the `gem_tx_pbuf_num_segments_q0 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q0(&self) -> X_PBUF_NUM_SEGMENTS_Q0_R {
        X_PBUF_NUM_SEGMENTS_Q0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Takes the value of the `gem_tx_pbuf_num_segments_q1 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q1(&self) -> X_PBUF_NUM_SEGMENTS_Q1_R {
        X_PBUF_NUM_SEGMENTS_Q1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Takes the value of the `gem_tx_pbuf_num_segments_q2 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q2(&self) -> X_PBUF_NUM_SEGMENTS_Q2_R {
        X_PBUF_NUM_SEGMENTS_Q2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Takes the value of the `gem_tx_pbuf_num_segments_q3 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q3(&self) -> X_PBUF_NUM_SEGMENTS_Q3_R {
        X_PBUF_NUM_SEGMENTS_Q3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Takes the value of the `gem_tx_pbuf_num_segments_q4 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q4(&self) -> X_PBUF_NUM_SEGMENTS_Q4_R {
        X_PBUF_NUM_SEGMENTS_Q4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Takes the value of the `gem_tx_pbuf_num_segments_q5 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q5(&self) -> X_PBUF_NUM_SEGMENTS_Q5_R {
        X_PBUF_NUM_SEGMENTS_Q5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Takes the value of the `gem_tx_pbuf_num_segments_q6 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q6(&self) -> X_PBUF_NUM_SEGMENTS_Q6_R {
        X_PBUF_NUM_SEGMENTS_Q6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Takes the value of the `gem_tx_pbuf_num_segments_q7 DEFINE"]
    #[inline(always)]
    pub fn x_pbuf_num_segments_q7(&self) -> X_PBUF_NUM_SEGMENTS_Q7_R {
        X_PBUF_NUM_SEGMENTS_Q7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Design Configuration Register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug7](index.html) module"]
pub struct DESIGNCFG_DEBUG7_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug7::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG7 to value 0x01"]
impl crate::Resettable for DESIGNCFG_DEBUG7_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
