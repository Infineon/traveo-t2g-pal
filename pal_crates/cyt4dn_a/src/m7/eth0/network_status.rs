#[doc = "Register `NETWORK_STATUS` reader"]
pub struct R(crate::R<NETWORK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NETWORK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NETWORK_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NETWORK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCS_LINK_STATE` reader - Returns status of PCS link state. If auto-negotiation is disabled this returns the synchronisation status. If auto-negotiation is enabled it is set in the LINK_OK state as long as a compatible duplex mode is resolved."]
pub type PCS_LINK_STATE_R = crate::BitReader<bool>;
#[doc = "Field `MDIO_IN` reader - Returns status of the mdio_in pin."]
pub type MDIO_IN_R = crate::BitReader<bool>;
#[doc = "Field `MAN_DONE` reader - The PHY management logic is idle (i.e. has completed)."]
pub type MAN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `MAC_FULL_DUPLEX` reader - PCS auto-negotiation duplex resolution. Set to one if the resolution function determines that both devices are capable of full duplex operation. If zero half-duplex operation is possible as long as bit 0 (PCS link state) is also one."]
pub type MAC_FULL_DUPLEX_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_5_4` reader - N/A"]
pub type REMOVED_5_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFC_NEGOTIATE_PCLK` reader - Set when PFC Priority Based Pause has been negotiated."]
pub type PFC_NEGOTIATE_PCLK_R = crate::BitReader<bool>;
#[doc = "Field `LPI_INDICATE_PCLK` reader - LPI Indication - Low power idle has been detected on receive. This bit is set when LPI is detected and reset when normal idle is detected. An interrupt is generated when the state of this bit changes."]
pub type LPI_INDICATE_PCLK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Returns status of PCS link state. If auto-negotiation is disabled this returns the synchronisation status. If auto-negotiation is enabled it is set in the LINK_OK state as long as a compatible duplex mode is resolved."]
    #[inline(always)]
    pub fn pcs_link_state(&self) -> PCS_LINK_STATE_R {
        PCS_LINK_STATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Returns status of the mdio_in pin."]
    #[inline(always)]
    pub fn mdio_in(&self) -> MDIO_IN_R {
        MDIO_IN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The PHY management logic is idle (i.e. has completed)."]
    #[inline(always)]
    pub fn man_done(&self) -> MAN_DONE_R {
        MAN_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCS auto-negotiation duplex resolution. Set to one if the resolution function determines that both devices are capable of full duplex operation. If zero half-duplex operation is possible as long as bit 0 (PCS link state) is also one."]
    #[inline(always)]
    pub fn mac_full_duplex(&self) -> MAC_FULL_DUPLEX_R {
        MAC_FULL_DUPLEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - N/A"]
    #[inline(always)]
    pub fn removed_5_4(&self) -> REMOVED_5_4_R {
        REMOVED_5_4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set when PFC Priority Based Pause has been negotiated."]
    #[inline(always)]
    pub fn pfc_negotiate_pclk(&self) -> PFC_NEGOTIATE_PCLK_R {
        PFC_NEGOTIATE_PCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPI Indication - Low power idle has been detected on receive. This bit is set when LPI is detected and reset when normal idle is detected. An interrupt is generated when the state of this bit changes."]
    #[inline(always)]
    pub fn lpi_indicate_pclk(&self) -> LPI_INDICATE_PCLK_R {
        LPI_INDICATE_PCLK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "The network status register returns status information with respect to the PHY management interface.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [network_status](index.html) module"]
pub struct NETWORK_STATUS_SPEC;
impl crate::RegisterSpec for NETWORK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [network_status::R](R) reader structure"]
impl crate::Readable for NETWORK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NETWORK_STATUS to value 0x06"]
impl crate::Resettable for NETWORK_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
