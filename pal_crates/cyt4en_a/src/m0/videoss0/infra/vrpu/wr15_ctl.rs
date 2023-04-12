#[doc = "Register `WR15_CTL` reader"]
pub struct R(crate::R<WR15_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR15_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR15_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR15_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P` reader - N/A"]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `NS` reader - N/A"]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `PRIO` reader - N/A"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC_MASK_0` reader - N/A"]
pub type PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `PC_MASK_15_TO_1` reader - N/A"]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - N/A"]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
#[doc = "VRAM protection for write requests with ID=15 (not used).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr15_ctl](index.html) module"]
pub struct WR15_CTL_SPEC;
impl crate::RegisterSpec for WR15_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr15_ctl::R](R) reader structure"]
impl crate::Readable for WR15_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WR15_CTL to value 0"]
impl crate::Resettable for WR15_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
