#[doc = "Register `TX_Q_SEG_ALLOC_Q0TO7` reader"]
pub struct R(crate::R<TX_Q_SEG_ALLOC_Q0TO7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_Q_SEG_ALLOC_Q0TO7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_Q_SEG_ALLOC_Q0TO7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_Q_SEG_ALLOC_Q0TO7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_Q_SEG_ALLOC_Q0TO7` writer"]
pub struct W(crate::W<TX_Q_SEG_ALLOC_Q0TO7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_Q_SEG_ALLOC_Q0TO7_SPEC>;
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
impl From<crate::W<TX_Q_SEG_ALLOC_Q0TO7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_Q_SEG_ALLOC_Q0TO7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGMENT_ALLOC_Q0` reader - Number of segments allocated to q0. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted. The default value is determined by `gem_tx_pbuf_num_segments_q0"]
pub type SEGMENT_ALLOC_Q0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGMENT_ALLOC_Q0` writer - Number of segments allocated to q0. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted. The default value is determined by `gem_tx_pbuf_num_segments_q0"]
pub type SEGMENT_ALLOC_Q0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_Q_SEG_ALLOC_Q0TO7_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD_3_3` reader - N/A"]
pub type RSVD_3_3_R = crate::BitReader<bool>;
#[doc = "Field `SEGMENT_ALLOC_Q1` reader - Number of segments allocated to q1. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
pub type SEGMENT_ALLOC_Q1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGMENT_ALLOC_Q1` writer - Number of segments allocated to q1. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
pub type SEGMENT_ALLOC_Q1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_Q_SEG_ALLOC_Q0TO7_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD_7_7` reader - N/A"]
pub type RSVD_7_7_R = crate::BitReader<bool>;
#[doc = "Field `SEGMENT_ALLOC_Q2` reader - Number of segments allocated to q2. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
pub type SEGMENT_ALLOC_Q2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGMENT_ALLOC_Q2` writer - Number of segments allocated to q2. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
pub type SEGMENT_ALLOC_Q2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_Q_SEG_ALLOC_Q0TO7_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD_11_11` reader - Write ignore, read 0"]
pub type RSVD_11_11_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_14_12` reader - Write ignore, read 0"]
pub type REMOVED_14_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_15_15` reader - Write ignore, read 0"]
pub type RSVD_15_15_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_18_16` reader - Write ignore, read 0"]
pub type REMOVED_18_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_19_19` reader - Write ignore, read 0"]
pub type RSVD_19_19_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_22_20` reader - Write ignore, read 0"]
pub type REMOVED_22_20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_23` reader - Write ignore, read 0"]
pub type RSVD_23_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_26_24` reader - Write ignore, read 0"]
pub type REMOVED_26_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_27_27` reader - Write ignore, read 0"]
pub type RSVD_27_27_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_30_28` reader - Write ignore, read 0"]
pub type REMOVED_30_28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_31_31` reader - Write ignore, read 0"]
pub type RSVD_31_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Number of segments allocated to q0. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted. The default value is determined by `gem_tx_pbuf_num_segments_q0"]
    #[inline(always)]
    pub fn segment_alloc_q0(&self) -> SEGMENT_ALLOC_Q0_R {
        SEGMENT_ALLOC_Q0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rsvd_3_3(&self) -> RSVD_3_3_R {
        RSVD_3_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Number of segments allocated to q1. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
    #[inline(always)]
    pub fn segment_alloc_q1(&self) -> SEGMENT_ALLOC_Q1_R {
        SEGMENT_ALLOC_Q1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7_7(&self) -> RSVD_7_7_R {
        RSVD_7_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Number of segments allocated to q2. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
    #[inline(always)]
    pub fn segment_alloc_q2(&self) -> SEGMENT_ALLOC_Q2_R {
        SEGMENT_ALLOC_Q2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_11_11(&self) -> RSVD_11_11_R {
        RSVD_11_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_14_12(&self) -> REMOVED_14_12_R {
        REMOVED_14_12_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_15_15(&self) -> RSVD_15_15_R {
        RSVD_15_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_18_16(&self) -> REMOVED_18_16_R {
        REMOVED_18_16_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_19_19(&self) -> RSVD_19_19_R {
        RSVD_19_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_22_20(&self) -> REMOVED_22_20_R {
        REMOVED_22_20_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_23(&self) -> RSVD_23_R {
        RSVD_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_26_24(&self) -> REMOVED_26_24_R {
        REMOVED_26_24_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_27_27(&self) -> RSVD_27_27_R {
        RSVD_27_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_30_28(&self) -> REMOVED_30_28_R {
        REMOVED_30_28_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_31_31(&self) -> RSVD_31_31_R {
        RSVD_31_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of segments allocated to q0. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted. The default value is determined by `gem_tx_pbuf_num_segments_q0"]
    #[inline(always)]
    #[must_use]
    pub fn segment_alloc_q0(&mut self) -> SEGMENT_ALLOC_Q0_W<0> {
        SEGMENT_ALLOC_Q0_W::new(self)
    }
    #[doc = "Bits 4:6 - Number of segments allocated to q1. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
    #[inline(always)]
    #[must_use]
    pub fn segment_alloc_q1(&mut self) -> SEGMENT_ALLOC_Q1_W<4> {
        SEGMENT_ALLOC_Q1_W::new(self)
    }
    #[doc = "Bits 8:10 - Number of segments allocated to q2. This should be entered as a log 2, for example entering a value of 2 would grant 4 segments. A maximum of 32 segments can be granted"]
    #[inline(always)]
    #[must_use]
    pub fn segment_alloc_q2(&mut self) -> SEGMENT_ALLOC_Q2_W<8> {
        SEGMENT_ALLOC_Q2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register allows the user to distribute the Transmit SRAM used by the DMA across the priority queues, for queues 0 to 7. The SRAM itself is split into a number of evenly sized segments (this is defined in the verilog configuration defs file - for the configuration used to generate this register description, the total number of segments was set to '16'). Those segments can then be freely distributed across the active queues, in powers of 2. I.e. a value of 0 would mean 1 segment has been allocated to the queue. A value of 1 would mean 2 segments, a value of 2 means 4 segments and so on. The reset values of these registers are defined in the configuration defs file.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_q_seg_alloc_q0to7](index.html) module"]
pub struct TX_Q_SEG_ALLOC_Q0TO7_SPEC;
impl crate::RegisterSpec for TX_Q_SEG_ALLOC_Q0TO7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_q_seg_alloc_q0to7::R](R) reader structure"]
impl crate::Readable for TX_Q_SEG_ALLOC_Q0TO7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_q_seg_alloc_q0to7::W](W) writer structure"]
impl crate::Writable for TX_Q_SEG_ALLOC_Q0TO7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_Q_SEG_ALLOC_Q0TO7 to value 0x01"]
impl crate::Resettable for TX_Q_SEG_ALLOC_Q0TO7_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
