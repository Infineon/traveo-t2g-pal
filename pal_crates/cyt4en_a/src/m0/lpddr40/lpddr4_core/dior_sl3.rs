#[doc = "Register `DIOR_SL3` reader"]
pub struct R(crate::R<DIOR_SL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIOR_SL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIOR_SL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIOR_SL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIOR_SL3` writer"]
pub struct W(crate::W<DIOR_SL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIOR_SL3_SPEC>;
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
impl From<crate::W<DIOR_SL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIOR_SL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRVSEL` reader - PHY Data Module IO Control Register Slice 3. Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
pub type DRVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRVSEL` writer - PHY Data Module IO Control Register Slice 3. Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
pub type DRVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIOR_SL3_SPEC, u8, u8, 3, O>;
#[doc = "Field `CMOS_EN` reader - Enable CMOS Receiver in IO Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type CMOS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMOS_EN` writer - Enable CMOS Receiver in IO Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type CMOS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIOR_SL3_SPEC, bool, O>;
#[doc = "Field `FENA_RCV` reader - PHY Data Module IO Control Register for Slice 3. Driver output enable DRV_TRISTATE = 0 Driver in tristate DRV_ENABLE = 1 Driver enabled FENA_RCV has higher priority than ODIS and OE when it is high. Enable = FENA_RCV + !ODIS &amp; OE"]
pub type FENA_RCV_R = crate::BitReader<bool>;
#[doc = "Field `FENA_RCV` writer - PHY Data Module IO Control Register for Slice 3. Driver output enable DRV_TRISTATE = 0 Driver in tristate DRV_ENABLE = 1 Driver enabled FENA_RCV has higher priority than ODIS and OE when it is high. Enable = FENA_RCV + !ODIS &amp; OE"]
pub type FENA_RCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIOR_SL3_SPEC, bool, O>;
#[doc = "Field `RTT_EN` reader - Enable On-Die Termination resistor RTT for Slice 3. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type RTT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTT_EN` writer - Enable On-Die Termination resistor RTT for Slice 3. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type RTT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIOR_SL3_SPEC, bool, O>;
#[doc = "Field `RTT_SEL` reader - Data Slice 3 RTT impedance when rtt_en=1. Values according to JESD209-4: RTT240 = 001b RTT = 240Ohm RTT120 = 010b RTT = 120Ohm RTT80 = 011b RTT = 80Ohm RTT60 = 100b RTT = 60Ohm RTT48 = 101b RTT = 48Ohm RTT40 = 110b RTT = 40Ohm"]
pub type RTT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTT_SEL` writer - Data Slice 3 RTT impedance when rtt_en=1. Values according to JESD209-4: RTT240 = 001b RTT = 240Ohm RTT120 = 010b RTT = 120Ohm RTT80 = 011b RTT = 80Ohm RTT60 = 100b RTT = 60Ohm RTT48 = 101b RTT = 48Ohm RTT40 = 110b RTT = 40Ohm"]
pub type RTT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIOR_SL3_SPEC, u8, u8, 3, O>;
#[doc = "Field `ODIS_DQ` reader - Data Slice 3 DQ Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_DQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODIS_DQ` writer - Data Slice 3 DQ Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_DQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIOR_SL3_SPEC, u8, u8, 8, O>;
#[doc = "Field `ODIS_DM` reader - Data Slice 3 DM Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_DM_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_DM` writer - Data Slice 3 DM Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIOR_SL3_SPEC, bool, O>;
#[doc = "Field `ODIS_DQS` reader - Data Slice 3 DQS Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_DQS_R = crate::BitReader<bool>;
#[doc = "Field `ODIS_DQS` writer - Data Slice 3 DQS Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
pub type ODIS_DQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIOR_SL3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - PHY Data Module IO Control Register Slice 3. Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
    #[inline(always)]
    pub fn drvsel(&self) -> DRVSEL_R {
        DRVSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable CMOS Receiver in IO Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn cmos_en(&self) -> CMOS_EN_R {
        CMOS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Data Module IO Control Register for Slice 3. Driver output enable DRV_TRISTATE = 0 Driver in tristate DRV_ENABLE = 1 Driver enabled FENA_RCV has higher priority than ODIS and OE when it is high. Enable = FENA_RCV + !ODIS &amp; OE"]
    #[inline(always)]
    pub fn fena_rcv(&self) -> FENA_RCV_R {
        FENA_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable On-Die Termination resistor RTT for Slice 3. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn rtt_en(&self) -> RTT_EN_R {
        RTT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Data Slice 3 RTT impedance when rtt_en=1. Values according to JESD209-4: RTT240 = 001b RTT = 240Ohm RTT120 = 010b RTT = 120Ohm RTT80 = 011b RTT = 80Ohm RTT60 = 100b RTT = 60Ohm RTT48 = 101b RTT = 48Ohm RTT40 = 110b RTT = 40Ohm"]
    #[inline(always)]
    pub fn rtt_sel(&self) -> RTT_SEL_R {
        RTT_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:16 - Data Slice 3 DQ Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_dq(&self) -> ODIS_DQ_R {
        ODIS_DQ_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Data Slice 3 DM Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_dm(&self) -> ODIS_DM_R {
        ODIS_DM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Slice 3 DQS Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    pub fn odis_dqs(&self) -> ODIS_DQS_R {
        ODIS_DQS_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PHY Data Module IO Control Register Slice 3. Selects how many copies of drivers are turned on. Number of drivers: DRV1 = 000b 1 driver DRV2 = 001b 2 drivers DRV3 = 010b 3 drivers DRV4 = 011b 4 drivers DRV5 = 100b 5 drivers DRV6 = 101b 6 drivers DRV7 = 110b 7 drivers DRV8 = 111b 8 drivers"]
    #[inline(always)]
    #[must_use]
    pub fn drvsel(&mut self) -> DRVSEL_W<0> {
        DRVSEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable CMOS Receiver in IO Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn cmos_en(&mut self) -> CMOS_EN_W<3> {
        CMOS_EN_W::new(self)
    }
    #[doc = "Bit 4 - PHY Data Module IO Control Register for Slice 3. Driver output enable DRV_TRISTATE = 0 Driver in tristate DRV_ENABLE = 1 Driver enabled FENA_RCV has higher priority than ODIS and OE when it is high. Enable = FENA_RCV + !ODIS &amp; OE"]
    #[inline(always)]
    #[must_use]
    pub fn fena_rcv(&mut self) -> FENA_RCV_W<4> {
        FENA_RCV_W::new(self)
    }
    #[doc = "Bit 5 - Enable On-Die Termination resistor RTT for Slice 3. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn rtt_en(&mut self) -> RTT_EN_W<5> {
        RTT_EN_W::new(self)
    }
    #[doc = "Bits 6:8 - Data Slice 3 RTT impedance when rtt_en=1. Values according to JESD209-4: RTT240 = 001b RTT = 240Ohm RTT120 = 010b RTT = 120Ohm RTT80 = 011b RTT = 80Ohm RTT60 = 100b RTT = 60Ohm RTT48 = 101b RTT = 48Ohm RTT40 = 110b RTT = 40Ohm"]
    #[inline(always)]
    #[must_use]
    pub fn rtt_sel(&mut self) -> RTT_SEL_W<6> {
        RTT_SEL_W::new(self)
    }
    #[doc = "Bits 9:16 - Data Slice 3 DQ Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_dq(&mut self) -> ODIS_DQ_W<9> {
        ODIS_DQ_W::new(self)
    }
    #[doc = "Bit 17 - Data Slice 3 DM Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_dm(&mut self) -> ODIS_DM_W<17> {
        ODIS_DM_W::new(self)
    }
    #[doc = "Bit 18 - Data Slice 3 DQS Output Driver Disable. When set to 1, the corresponding output is forced to high-Z. NORMAL = 0 Normal operation FORCE_HIZ = 1 Forced to high-Z"]
    #[inline(always)]
    #[must_use]
    pub fn odis_dqs(&mut self) -> ODIS_DQS_W<18> {
        ODIS_DQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Data Module IO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dior_sl3](index.html) module"]
pub struct DIOR_SL3_SPEC;
impl crate::RegisterSpec for DIOR_SL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dior_sl3::R](R) reader structure"]
impl crate::Readable for DIOR_SL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dior_sl3::W](W) writer structure"]
impl crate::Writable for DIOR_SL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIOR_SL3 to value 0"]
impl crate::Resettable for DIOR_SL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
