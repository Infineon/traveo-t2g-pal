#[doc = "Register `DQSDQCR` reader"]
pub struct R(crate::R<DQSDQCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DQSDQCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DQSDQCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DQSDQCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DQSDQCR` writer"]
pub struct W(crate::W<DQSDQCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DQSDQCR_SPEC>;
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
impl From<crate::W<DQSDQCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DQSDQCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYOFFS` reader - DQS2DQ delay offset (unit of phase based on DRAM clock)"]
pub type DLYOFFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYOFFS` writer - DQS2DQ delay offset (unit of phase based on DRAM clock)"]
pub type DLYOFFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DQSDQCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSEL` reader - Or-select data module to update. Example: 0001: data module 0 0011: data module 0,1 0100: data module 2 1111: data module 0,1,2,3"]
pub type DQSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSEL` writer - Or-select data module to update. Example: 0001: data module 0 0011: data module 0,1 0100: data module 2 1111: data module 0,1,2,3"]
pub type DQSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DQSDQCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MUPD` reader - DQS2DQ delay manual update enable. This bit is self-clearing when the delay is updated AUTOMATIC = 0 Automatic update MANUAL = 1 Manual update"]
pub type MUPD_R = crate::BitReader<bool>;
#[doc = "Field `MUPD` writer - DQS2DQ delay manual update enable. This bit is self-clearing when the delay is updated AUTOMATIC = 0 Automatic update MANUAL = 1 Manual update"]
pub type MUPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DQSDQCR_SPEC, bool, O>;
#[doc = "Field `MPCRPT` reader - Maximum number of consecutive MPC FIFO Write/Read before checking data. Up to 5."]
pub type MPCRPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPCRPT` writer - Maximum number of consecutive MPC FIFO Write/Read before checking data. Up to 5."]
pub type MPCRPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DQSDQCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYMAX` reader - Maximum scan steps for a DQS2DQ training session. Default: 0xFF during PHY Init Training"]
pub type DLYMAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYMAX` writer - Maximum scan steps for a DQS2DQ training session. Default: 0xFF during PHY Init Training"]
pub type DLYMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DQSDQCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIR` reader - DQS2DQ retraining direction control: 0: Down - Decrease delay from old setting 1: Up - Increase delay from old setting DOWN = 0 DQS2DQ search downwards UP = 1 DQS2DQ search upwards"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DQS2DQ retraining direction control: 0: Down - Decrease delay from old setting 1: Up - Increase delay from old setting DOWN = 0 DQS2DQ search downwards UP = 1 DQS2DQ search upwards"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DQSDQCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - DQS2DQ delay offset (unit of phase based on DRAM clock)"]
    #[inline(always)]
    pub fn dlyoffs(&self) -> DLYOFFS_R {
        DLYOFFS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Or-select data module to update. Example: 0001: data module 0 0011: data module 0,1 0100: data module 2 1111: data module 0,1,2,3"]
    #[inline(always)]
    pub fn dqsel(&self) -> DQSEL_R {
        DQSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DQS2DQ delay manual update enable. This bit is self-clearing when the delay is updated AUTOMATIC = 0 Automatic update MANUAL = 1 Manual update"]
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Maximum number of consecutive MPC FIFO Write/Read before checking data. Up to 5."]
    #[inline(always)]
    pub fn mpcrpt(&self) -> MPCRPT_R {
        MPCRPT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Maximum scan steps for a DQS2DQ training session. Default: 0xFF during PHY Init Training"]
    #[inline(always)]
    pub fn dlymax(&self) -> DLYMAX_R {
        DLYMAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DQS2DQ retraining direction control: 0: Down - Decrease delay from old setting 1: Up - Increase delay from old setting DOWN = 0 DQS2DQ search downwards UP = 1 DQS2DQ search upwards"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQS2DQ delay offset (unit of phase based on DRAM clock)"]
    #[inline(always)]
    #[must_use]
    pub fn dlyoffs(&mut self) -> DLYOFFS_W<0> {
        DLYOFFS_W::new(self)
    }
    #[doc = "Bits 8:11 - Or-select data module to update. Example: 0001: data module 0 0011: data module 0,1 0100: data module 2 1111: data module 0,1,2,3"]
    #[inline(always)]
    #[must_use]
    pub fn dqsel(&mut self) -> DQSEL_W<8> {
        DQSEL_W::new(self)
    }
    #[doc = "Bit 12 - DQS2DQ delay manual update enable. This bit is self-clearing when the delay is updated AUTOMATIC = 0 Automatic update MANUAL = 1 Manual update"]
    #[inline(always)]
    #[must_use]
    pub fn mupd(&mut self) -> MUPD_W<12> {
        MUPD_W::new(self)
    }
    #[doc = "Bits 13:15 - Maximum number of consecutive MPC FIFO Write/Read before checking data. Up to 5."]
    #[inline(always)]
    #[must_use]
    pub fn mpcrpt(&mut self) -> MPCRPT_W<13> {
        MPCRPT_W::new(self)
    }
    #[doc = "Bits 16:23 - Maximum scan steps for a DQS2DQ training session. Default: 0xFF during PHY Init Training"]
    #[inline(always)]
    #[must_use]
    pub fn dlymax(&mut self) -> DLYMAX_W<16> {
        DLYMAX_W::new(self)
    }
    #[doc = "Bit 24 - DQS2DQ retraining direction control: 0: Down - Decrease delay from old setting 1: Up - Increase delay from old setting DOWN = 0 DQS2DQ search downwards UP = 1 DQS2DQ search upwards"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<24> {
        DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DQS2DQ Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dqsdqcr](index.html) module"]
pub struct DQSDQCR_SPEC;
impl crate::RegisterSpec for DQSDQCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dqsdqcr::R](R) reader structure"]
impl crate::Readable for DQSDQCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dqsdqcr::W](W) writer structure"]
impl crate::Writable for DQSDQCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DQSDQCR to value 0"]
impl crate::Resettable for DQSDQCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
