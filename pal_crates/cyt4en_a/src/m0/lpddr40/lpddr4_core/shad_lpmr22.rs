#[doc = "Register `SHAD_LPMR22` reader"]
pub struct R(crate::R<SHAD_LPMR22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR22_FS0_SOCODT` reader - SoC ODT. Controller ODT value for VOH calibration for frequency set 0 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
pub type SHAD_LPMR22_FS0_SOCODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR22_FS0_ODTECK` reader - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type SHAD_LPMR22_FS0_ODTECK_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR22_FS0_ODTECS` reader - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type SHAD_LPMR22_FS0_ODTECS_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR22_FS0_ODTDCA` reader - ODTD-CA. CA ODT termination disable for frequency set 0 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
pub type SHAD_LPMR22_FS0_ODTDCA_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR22_ODTDX8` reader - ODTD for x8_2ch (byte) mode"]
pub type SHAD_LPMR22_ODTDX8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR22_FS1_SOCODT` reader - SoC ODT. Controller ODT value for VOH calibration for frequency set 1 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
pub type SHAD_LPMR22_FS1_SOCODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR22_FS1_ODTECK` reader - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type SHAD_LPMR22_FS1_ODTECK_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR22_FS1_ODTECS` reader - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type SHAD_LPMR22_FS1_ODTECS_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR22_FS1_ODTDCA` reader - ODTD-CA. CA ODT termination disable for frequency set 1 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
pub type SHAD_LPMR22_FS1_ODTDCA_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR22_RSVD` reader - N/A"]
pub type SHAD_LPMR22_RSVD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - SoC ODT. Controller ODT value for VOH calibration for frequency set 0 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
    #[inline(always)]
    pub fn shad_lpmr22_fs0_socodt(&self) -> SHAD_LPMR22_FS0_SOCODT_R {
        SHAD_LPMR22_FS0_SOCODT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn shad_lpmr22_fs0_odteck(&self) -> SHAD_LPMR22_FS0_ODTECK_R {
        SHAD_LPMR22_FS0_ODTECK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn shad_lpmr22_fs0_odtecs(&self) -> SHAD_LPMR22_FS0_ODTECS_R {
        SHAD_LPMR22_FS0_ODTECS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ODTD-CA. CA ODT termination disable for frequency set 0 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
    #[inline(always)]
    pub fn shad_lpmr22_fs0_odtdca(&self) -> SHAD_LPMR22_FS0_ODTDCA_R {
        SHAD_LPMR22_FS0_ODTDCA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - ODTD for x8_2ch (byte) mode"]
    #[inline(always)]
    pub fn shad_lpmr22_odtdx8(&self) -> SHAD_LPMR22_ODTDX8_R {
        SHAD_LPMR22_ODTDX8_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - SoC ODT. Controller ODT value for VOH calibration for frequency set 1 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
    #[inline(always)]
    pub fn shad_lpmr22_fs1_socodt(&self) -> SHAD_LPMR22_FS1_SOCODT_R {
        SHAD_LPMR22_FS1_SOCODT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn shad_lpmr22_fs1_odteck(&self) -> SHAD_LPMR22_FS1_ODTECK_R {
        SHAD_LPMR22_FS1_ODTECK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn shad_lpmr22_fs1_odtecs(&self) -> SHAD_LPMR22_FS1_ODTECS_R {
        SHAD_LPMR22_FS1_ODTECS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ODTD-CA. CA ODT termination disable for frequency set 1 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
    #[inline(always)]
    pub fn shad_lpmr22_fs1_odtdca(&self) -> SHAD_LPMR22_FS1_ODTDCA_R {
        SHAD_LPMR22_FS1_ODTDCA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr22_rsvd(&self) -> SHAD_LPMR22_RSVD_R {
        SHAD_LPMR22_RSVD_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Shadow LPDDR Mode Register 22\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr22](index.html) module"]
pub struct SHAD_LPMR22_SPEC;
impl crate::RegisterSpec for SHAD_LPMR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr22::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR22_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR22 to value 0"]
impl crate::Resettable for SHAD_LPMR22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
