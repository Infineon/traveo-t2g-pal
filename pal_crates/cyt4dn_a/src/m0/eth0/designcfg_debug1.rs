#[doc = "Register `DESIGNCFG_DEBUG1` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NO_PCS` reader - Takes the value of the `gem_no_pcs DEFINE"]
pub type NO_PCS_R = crate::BitReader<bool>;
#[doc = "Field `EXCLUDE_QBV` reader - Takes the value of the `gem_exclude_qbv DEFINE"]
pub type EXCLUDE_QBV_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_2` reader - N/A"]
pub type RSVD_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_LOOPBACK` reader - Takes the value of the `gem_int_loopback DEFINE"]
pub type INT_LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_5` reader - N/A"]
pub type RSVD_5_R = crate::BitReader<bool>;
#[doc = "Field `EXT_FIFO_INTERFACE` reader - Takes the value of the `gem_ext_fifo_interface DEFINE"]
pub type EXT_FIFO_INTERFACE_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_7` reader - N/A"]
pub type RSVD_7_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_8` reader - N/A"]
pub type RSVD_8_R = crate::BitReader<bool>;
#[doc = "Field `USER_IO` reader - Takes the value of the `gem_user_io DEFINE"]
pub type USER_IO_R = crate::BitReader<bool>;
#[doc = "Field `USER_OUT_WIDTH` reader - Takes the value of the `gem_user_out_width DEFINE if `gem_user_io is set"]
pub type USER_OUT_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USER_IN_WIDTH` reader - Takes the value of the `gem_user_in_width DEFINE `gem_user_io"]
pub type USER_IN_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_20` reader - N/A"]
pub type RSVD_20_R = crate::BitReader<bool>;
#[doc = "Field `NO_STATS` reader - Takes the value of the `gem_no_stats DEFINE"]
pub type NO_STATS_R = crate::BitReader<bool>;
#[doc = "Field `NO_SNAPSHOT` reader - Takes the value of the `gem_no_snapshot DEFINE"]
pub type NO_SNAPSHOT_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_READ_CLEAR` reader - Takes the value of the `gem_irq_read_clear DEFINE"]
pub type IRQ_READ_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `EXCLUDE_CBS` reader - Takes the value of the `gem_exclude_cbs DEFINE"]
pub type EXCLUDE_CBS_R = crate::BitReader<bool>;
#[doc = "Field `DMA_BUS_WIDTH` reader - Takes the value of bits 7:5 of the `gem_dma_bus_width DEFINE. So if the define is set to decimal 64 this will return binary 010."]
pub type DMA_BUS_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_CACHE_VALUE` reader - Takes the value of the `gem_axi_cache_value DEFINE"]
pub type AXI_CACHE_VALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Takes the value of the `gem_no_pcs DEFINE"]
    #[inline(always)]
    pub fn no_pcs(&self) -> NO_PCS_R {
        NO_PCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Takes the value of the `gem_exclude_qbv DEFINE"]
    #[inline(always)]
    pub fn exclude_qbv(&self) -> EXCLUDE_QBV_R {
        EXCLUDE_QBV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> RSVD_2_R {
        RSVD_2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Takes the value of the `gem_int_loopback DEFINE"]
    #[inline(always)]
    pub fn int_loopback(&self) -> INT_LOOPBACK_R {
        INT_LOOPBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> RSVD_5_R {
        RSVD_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Takes the value of the `gem_ext_fifo_interface DEFINE"]
    #[inline(always)]
    pub fn ext_fifo_interface(&self) -> EXT_FIFO_INTERFACE_R {
        EXT_FIFO_INTERFACE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn rsvd_8(&self) -> RSVD_8_R {
        RSVD_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Takes the value of the `gem_user_io DEFINE"]
    #[inline(always)]
    pub fn user_io(&self) -> USER_IO_R {
        USER_IO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - Takes the value of the `gem_user_out_width DEFINE if `gem_user_io is set"]
    #[inline(always)]
    pub fn user_out_width(&self) -> USER_OUT_WIDTH_R {
        USER_OUT_WIDTH_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Takes the value of the `gem_user_in_width DEFINE `gem_user_io"]
    #[inline(always)]
    pub fn user_in_width(&self) -> USER_IN_WIDTH_R {
        USER_IN_WIDTH_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn rsvd_20(&self) -> RSVD_20_R {
        RSVD_20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Takes the value of the `gem_no_stats DEFINE"]
    #[inline(always)]
    pub fn no_stats(&self) -> NO_STATS_R {
        NO_STATS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Takes the value of the `gem_no_snapshot DEFINE"]
    #[inline(always)]
    pub fn no_snapshot(&self) -> NO_SNAPSHOT_R {
        NO_SNAPSHOT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Takes the value of the `gem_irq_read_clear DEFINE"]
    #[inline(always)]
    pub fn irq_read_clear(&self) -> IRQ_READ_CLEAR_R {
        IRQ_READ_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Takes the value of the `gem_exclude_cbs DEFINE"]
    #[inline(always)]
    pub fn exclude_cbs(&self) -> EXCLUDE_CBS_R {
        EXCLUDE_CBS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Takes the value of bits 7:5 of the `gem_dma_bus_width DEFINE. So if the define is set to decimal 64 this will return binary 010."]
    #[inline(always)]
    pub fn dma_bus_width(&self) -> DMA_BUS_WIDTH_R {
        DMA_BUS_WIDTH_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Takes the value of the `gem_axi_cache_value DEFINE"]
    #[inline(always)]
    pub fn axi_cache_value(&self) -> AXI_CACHE_VALUE_R {
        AXI_CACHE_VALUE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "The GEM_GXL(3PIP) has many parameterisation options to configure the IP during compilation stage. This is achieved using Verilog define compiler directives in an include file called mxeth_defs.v.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug1](index.html) module"]
pub struct DESIGNCFG_DEBUG1_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug1::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG1 to value 0x0250_8503"]
impl crate::Resettable for DESIGNCFG_DEBUG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0250_8503;
}
