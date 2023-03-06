#[doc = "Register `DESIGNCFG_DEBUG2` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JUMBO_MAX_LENGTH` reader - Takes the value of the `gem_jumbo_max_length DEFINE"]
pub type JUMBO_MAX_LENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPROT_VALUE` reader - Takes the value of the `gem_hprot_value DEFINE"]
pub type HPROT_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PKT_BUFFER` reader - Takes the value of the `gem_rx_pkt_buffer DEFINE"]
pub type RX_PKT_BUFFER_R = crate::BitReader<bool>;
#[doc = "Field `TX_PKT_BUFFER` reader - Takes the value of the `gem_tx_pkt_buffer DEFINE"]
pub type TX_PKT_BUFFER_R = crate::BitReader<bool>;
#[doc = "Field `RX_PBUF_ADDR` reader - Takes the value of the `gem_rx_pbuf_addr DEFINE"]
pub type RX_PBUF_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_ADDR` reader - Takes the value of the `gem_tx_pbuf_addr DEFINE"]
pub type TX_PBUF_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI` reader - Takes the value of the `gem_axi DEFINE"]
pub type AXI_R = crate::BitReader<bool>;
#[doc = "Field `SPRAM` reader - Takes the value of the `gem_spram DEFINE"]
pub type SPRAM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - Takes the value of the `gem_jumbo_max_length DEFINE"]
    #[inline(always)]
    pub fn jumbo_max_length(&self) -> JUMBO_MAX_LENGTH_R {
        JUMBO_MAX_LENGTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:19 - Takes the value of the `gem_hprot_value DEFINE"]
    #[inline(always)]
    pub fn hprot_value(&self) -> HPROT_VALUE_R {
        HPROT_VALUE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Takes the value of the `gem_rx_pkt_buffer DEFINE"]
    #[inline(always)]
    pub fn rx_pkt_buffer(&self) -> RX_PKT_BUFFER_R {
        RX_PKT_BUFFER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Takes the value of the `gem_tx_pkt_buffer DEFINE"]
    #[inline(always)]
    pub fn tx_pkt_buffer(&self) -> TX_PKT_BUFFER_R {
        TX_PKT_BUFFER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - Takes the value of the `gem_rx_pbuf_addr DEFINE"]
    #[inline(always)]
    pub fn rx_pbuf_addr(&self) -> RX_PBUF_ADDR_R {
        RX_PBUF_ADDR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - Takes the value of the `gem_tx_pbuf_addr DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_addr(&self) -> TX_PBUF_ADDR_R {
        TX_PBUF_ADDR_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Takes the value of the `gem_axi DEFINE"]
    #[inline(always)]
    pub fn axi(&self) -> AXI_R {
        AXI_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Takes the value of the `gem_spram DEFINE"]
    #[inline(always)]
    pub fn spram(&self) -> SPRAM_R {
        SPRAM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Design Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug2](index.html) module"]
pub struct DESIGNCFG_DEBUG2_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug2::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG2 to value 0xe631_0600"]
impl crate::Resettable for DESIGNCFG_DEBUG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xe631_0600;
}
