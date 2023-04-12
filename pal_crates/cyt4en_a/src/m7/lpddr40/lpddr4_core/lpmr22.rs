#[doc = "Register `LPMR22` reader"]
pub struct R(crate::R<LPMR22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR22` writer"]
pub struct W(crate::W<LPMR22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR22_SPEC>;
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
impl From<crate::W<LPMR22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_SOCODT` reader - SoC ODT. Controller ODT value for VOH calibration for frequency set 0 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
pub type FS0_SOCODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_SOCODT` writer - SoC ODT. Controller ODT value for VOH calibration for frequency set 0 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
pub type FS0_SOCODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR22_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS0_ODTECK` reader - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS0_ODTECK_R = crate::BitReader<bool>;
#[doc = "Field `FS0_ODTECK` writer - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS0_ODTECK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR22_SPEC, bool, O>;
#[doc = "Field `FS0_ODTECS` reader - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS0_ODTECS_R = crate::BitReader<bool>;
#[doc = "Field `FS0_ODTECS` writer - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS0_ODTECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR22_SPEC, bool, O>;
#[doc = "Field `FS0_ODTDCA` reader - ODTD-CA. CA ODT termination disable for frequency set 0 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
pub type FS0_ODTDCA_R = crate::BitReader<bool>;
#[doc = "Field `FS0_ODTDCA` writer - ODTD-CA. CA ODT termination disable for frequency set 0 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
pub type FS0_ODTDCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR22_SPEC, bool, O>;
#[doc = "Field `ODTDX8` reader - ODTD for x8_2ch (byte) mode"]
pub type ODTDX8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODTDX8` writer - ODTD for x8_2ch (byte) mode"]
pub type ODTDX8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR22_SPEC, u8, u8, 2, O>;
#[doc = "Field `FS1_SOCODT` reader - SoC ODT. Controller ODT value for VOH calibration for frequency set 1 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
pub type FS1_SOCODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_SOCODT` writer - SoC ODT. Controller ODT value for VOH calibration for frequency set 1 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
pub type FS1_SOCODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR22_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_ODTECK` reader - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS1_ODTECK_R = crate::BitReader<bool>;
#[doc = "Field `FS1_ODTECK` writer - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS1_ODTECK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR22_SPEC, bool, O>;
#[doc = "Field `FS1_ODTECS` reader - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS1_ODTECS_R = crate::BitReader<bool>;
#[doc = "Field `FS1_ODTECS` writer - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
pub type FS1_ODTECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR22_SPEC, bool, O>;
#[doc = "Field `FS1_ODTDCA` reader - ODTD-CA. CA ODT termination disable for frequency set 1 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
pub type FS1_ODTDCA_R = crate::BitReader<bool>;
#[doc = "Field `FS1_ODTDCA` writer - ODTD-CA. CA ODT termination disable for frequency set 1 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
pub type FS1_ODTDCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR22_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - SoC ODT. Controller ODT value for VOH calibration for frequency set 0 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
    #[inline(always)]
    pub fn fs0_socodt(&self) -> FS0_SOCODT_R {
        FS0_SOCODT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn fs0_odteck(&self) -> FS0_ODTECK_R {
        FS0_ODTECK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn fs0_odtecs(&self) -> FS0_ODTECS_R {
        FS0_ODTECS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ODTD-CA. CA ODT termination disable for frequency set 0 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
    #[inline(always)]
    pub fn fs0_odtdca(&self) -> FS0_ODTDCA_R {
        FS0_ODTDCA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - ODTD for x8_2ch (byte) mode"]
    #[inline(always)]
    pub fn odtdx8(&self) -> ODTDX8_R {
        ODTDX8_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - SoC ODT. Controller ODT value for VOH calibration for frequency set 1 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
    #[inline(always)]
    pub fn fs1_socodt(&self) -> FS1_SOCODT_R {
        FS1_SOCODT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn fs1_odteck(&self) -> FS1_ODTECK_R {
        FS1_ODTECK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    pub fn fs1_odtecs(&self) -> FS1_ODTECS_R {
        FS1_ODTECS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ODTD-CA. CA ODT termination disable for frequency set 1 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
    #[inline(always)]
    pub fn fs1_odtdca(&self) -> FS1_ODTDCA_R {
        FS1_ODTDCA_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SoC ODT. Controller ODT value for VOH calibration for frequency set 0 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_socodt(&mut self) -> FS0_SOCODT_W<0> {
        FS0_SOCODT_W::new(self)
    }
    #[doc = "Bit 3 - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_odteck(&mut self) -> FS0_ODTECK_W<3> {
        FS0_ODTECK_W::new(self)
    }
    #[doc = "Bit 4 - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 0 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_odtecs(&mut self) -> FS0_ODTECS_W<4> {
        FS0_ODTECS_W::new(self)
    }
    #[doc = "Bit 5 - ODTD-CA. CA ODT termination disable for frequency set 0 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_odtdca(&mut self) -> FS0_ODTDCA_W<5> {
        FS0_ODTDCA_W::new(self)
    }
    #[doc = "Bits 6:7 - ODTD for x8_2ch (byte) mode"]
    #[inline(always)]
    #[must_use]
    pub fn odtdx8(&mut self) -> ODTDX8_W<6> {
        ODTDX8_W::new(self)
    }
    #[doc = "Bits 8:10 - SoC ODT. Controller ODT value for VOH calibration for frequency set 1 Controller ODT values according to JESD209-4: DISABLE = 000B Disable (Default) RZQ_DIV_1 = 001B RZQ/1 RZQ_DIV_2 = 010B RZQ/2 RZQ_DIV_3 = 011B RZQ/3 RZQ_DIV_4 = 100B RZQ/4 RZQ_DIV_5 = 101B RZQ/5 RZQ_DIV_6 = 110B RZQ/6 RFU = 111B RFU"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_socodt(&mut self) -> FS1_SOCODT_W<8> {
        FS1_SOCODT_W::new(self)
    }
    #[doc = "Bit 11 - ODTE-CK. CK ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_odteck(&mut self) -> FS1_ODTECK_W<11> {
        FS1_ODTECK_W::new(self)
    }
    #[doc = "Bit 12 - ODTE-CS. CS ODT enable for non-terminating rank for frequency set 1 ODT Enable for non-terminating rank. Values according to JESD209-4: OVERRIDE_DISABLED = 0B ODT-CK Over-ride Disabled (Default) OVERRIDE_ENABLED = 1B ODT-CK Over-ride Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_odtecs(&mut self) -> FS1_ODTECS_W<12> {
        FS1_ODTECS_W::new(self)
    }
    #[doc = "Bit 13 - ODTD-CA. CA ODT termination disable for frequency set 1 Command/Address bus ODT termination disable. Values according to JESD209-4: ODT_CA_PAD = 0B ODT-CA Obeys ODT_CA bond pad (default) ODT_CA_DISABLED = 1B ODT-CA Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_odtdca(&mut self) -> FS1_ODTDCA_W<13> {
        FS1_ODTDCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr22](index.html) module"]
pub struct LPMR22_SPEC;
impl crate::RegisterSpec for LPMR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr22::R](R) reader structure"]
impl crate::Readable for LPMR22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr22::W](W) writer structure"]
impl crate::Writable for LPMR22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR22 to value 0"]
impl crate::Resettable for LPMR22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
