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
#[doc = "Field `ENABLED` reader - Indicates the enable status of the channel. '0': Disabled. '1': Enabled. When CH_CTL.ENABLED is set to '0' by SW or by the activation of an error interrupt cause or by disabling the entire DMAC, HW sets this field to '0' after the channel has completed AXI transactions. When SW sets CH_CTL.ENABLED to '1' and this field is '0', it changes from '0' to '1' immediately (if the entire DMAC is still disabled by CTL.ENABLED, it changes from '0' to '1' after the DMAC gets enabled). If this field is '1' at the time CH_CTL_ENABLED is changed from '0' to '1', then this field is first set to '0' by HW after the channel has completed AXI transactions, and it is set to '1' after one clock cycle if CH_CTL.ENABLED is still '1'. This ensures that the channel is properly initialized (except the retained registers)."]
pub type ENABLED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - Indicates the enable status of the channel. '0': Disabled. '1': Enabled. When CH_CTL.ENABLED is set to '0' by SW or by the activation of an error interrupt cause or by disabling the entire DMAC, HW sets this field to '0' after the channel has completed AXI transactions. When SW sets CH_CTL.ENABLED to '1' and this field is '0', it changes from '0' to '1' immediately (if the entire DMAC is still disabled by CTL.ENABLED, it changes from '0' to '1' after the DMAC gets enabled). If this field is '1' at the time CH_CTL_ENABLED is changed from '0' to '1', then this field is first set to '0' by HW after the channel has completed AXI transactions, and it is set to '1' after one clock cycle if CH_CTL.ENABLED is still '1'. This ensures that the channel is properly initialized (except the retained registers)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
