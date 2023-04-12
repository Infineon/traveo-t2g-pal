#[doc = "Register `FLASH_CTL` reader"]
pub struct R(crate::R<FLASH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CTL` writer"]
pub struct W(crate::W<FLASH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CTL_SPEC>;
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
impl From<crate::W<FLASH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WS` reader - FLASH macro wait states (same for main and work interfaces): '0': 0 wait states. ... '15': 15 wait states"]
pub type WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WS` writer - FLASH macro wait states (same for main and work interfaces): '0': 0 wait states. ... '15': 15 wait states"]
pub type WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAIN_MAP` reader - N/A"]
pub type MAIN_MAP_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_MAP` writer - N/A"]
pub type MAIN_MAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `WORK_MAP` reader - N/A"]
pub type WORK_MAP_R = crate::BitReader<bool>;
#[doc = "Field `WORK_MAP` writer - N/A"]
pub type WORK_MAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `MAIN_BANK_MODE` reader - N/A"]
pub type MAIN_BANK_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_BANK_MODE` writer - N/A"]
pub type MAIN_BANK_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `WORK_BANK_MODE` reader - N/A"]
pub type WORK_BANK_MODE_R = crate::BitReader<bool>;
#[doc = "Field `WORK_BANK_MODE` writer - N/A"]
pub type WORK_BANK_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `MAIN_ECC_EN` reader - N/A"]
pub type MAIN_ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_ECC_EN` writer - N/A"]
pub type MAIN_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `MAIN_ECC_INJ_EN` reader - N/A"]
pub type MAIN_ECC_INJ_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_ECC_INJ_EN` writer - N/A"]
pub type MAIN_ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `MAIN_ERR_SILENT` reader - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error or a FLASH macro main interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error."]
pub type MAIN_ERR_SILENT_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_ERR_SILENT` writer - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error or a FLASH macro main interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error."]
pub type MAIN_ERR_SILENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `WORK_ECC_EN` reader - N/A"]
pub type WORK_ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `WORK_ECC_EN` writer - N/A"]
pub type WORK_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `WORK_ECC_INJ_EN` reader - N/A"]
pub type WORK_ECC_INJ_EN_R = crate::BitReader<bool>;
#[doc = "Field `WORK_ECC_INJ_EN` writer - N/A"]
pub type WORK_ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `WORK_ERR_SILENT` reader - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error or a FLASH macro work interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error."]
pub type WORK_ERR_SILENT_R = crate::BitReader<bool>;
#[doc = "Field `WORK_ERR_SILENT` writer - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error or a FLASH macro work interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error."]
pub type WORK_ERR_SILENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
#[doc = "Field `WORK_SEQ_RD_EN` reader - Enable sequential read mode for Work Flash for read accesses on the AXI port. 0: Sequential read mode for Work Flash is disabled. 1: Sequential read mode for Work Flash is enabled. This speeds up subsequent AXI read accesses to the same page (64 bytes) of the Work Flash. Setting this field to '1' has a side effect on the round-robin arbitration of the Work Flash macro between the different ports. When this field is set to '1', then as soon as the arbitration of the Work Flash is given to the AXI port, it is not released before the current AXI read burst either has been completed or leaves the current Work Flash page (64 bytes). This means that AHB masters accessing the Work Flash that have the same priority as the AXI port will not get their fair share of the Work Flash bandwidth. While an AHB master can perform one Work Flash access per round-robin cycle (4 bytes), an AXI master can read a whole AXI burst, or up to the next Work Flash page boundary. So if this field is set to '1', and an AHB master requires a high Work Flash bandwidth, then it should have a higher priority than the AXI masters accessing the Work Flash."]
pub type WORK_SEQ_RD_EN_R = crate::BitReader<bool>;
#[doc = "Field `WORK_SEQ_RD_EN` writer - Enable sequential read mode for Work Flash for read accesses on the AXI port. 0: Sequential read mode for Work Flash is disabled. 1: Sequential read mode for Work Flash is enabled. This speeds up subsequent AXI read accesses to the same page (64 bytes) of the Work Flash. Setting this field to '1' has a side effect on the round-robin arbitration of the Work Flash macro between the different ports. When this field is set to '1', then as soon as the arbitration of the Work Flash is given to the AXI port, it is not released before the current AXI read burst either has been completed or leaves the current Work Flash page (64 bytes). This means that AHB masters accessing the Work Flash that have the same priority as the AXI port will not get their fair share of the Work Flash bandwidth. While an AHB master can perform one Work Flash access per round-robin cycle (4 bytes), an AXI master can read a whole AXI burst, or up to the next Work Flash page boundary. So if this field is set to '1', and an AHB master requires a high Work Flash bandwidth, then it should have a higher priority than the AXI masters accessing the Work Flash."]
pub type WORK_SEQ_RD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - FLASH macro wait states (same for main and work interfaces): '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn ws(&self) -> WS_R {
        WS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn main_map(&self) -> MAIN_MAP_R {
        MAIN_MAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn work_map(&self) -> WORK_MAP_R {
        WORK_MAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn main_bank_mode(&self) -> MAIN_BANK_MODE_R {
        MAIN_BANK_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn work_bank_mode(&self) -> WORK_BANK_MODE_R {
        WORK_BANK_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn main_ecc_en(&self) -> MAIN_ECC_EN_R {
        MAIN_ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn main_ecc_inj_en(&self) -> MAIN_ECC_INJ_EN_R {
        MAIN_ECC_INJ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error or a FLASH macro main interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error."]
    #[inline(always)]
    pub fn main_err_silent(&self) -> MAIN_ERR_SILENT_R {
        MAIN_ERR_SILENT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn work_ecc_en(&self) -> WORK_ECC_EN_R {
        WORK_ECC_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn work_ecc_inj_en(&self) -> WORK_ECC_INJ_EN_R {
        WORK_ECC_INJ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error or a FLASH macro work interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error."]
    #[inline(always)]
    pub fn work_err_silent(&self) -> WORK_ERR_SILENT_R {
        WORK_ERR_SILENT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable sequential read mode for Work Flash for read accesses on the AXI port. 0: Sequential read mode for Work Flash is disabled. 1: Sequential read mode for Work Flash is enabled. This speeds up subsequent AXI read accesses to the same page (64 bytes) of the Work Flash. Setting this field to '1' has a side effect on the round-robin arbitration of the Work Flash macro between the different ports. When this field is set to '1', then as soon as the arbitration of the Work Flash is given to the AXI port, it is not released before the current AXI read burst either has been completed or leaves the current Work Flash page (64 bytes). This means that AHB masters accessing the Work Flash that have the same priority as the AXI port will not get their fair share of the Work Flash bandwidth. While an AHB master can perform one Work Flash access per round-robin cycle (4 bytes), an AXI master can read a whole AXI burst, or up to the next Work Flash page boundary. So if this field is set to '1', and an AHB master requires a high Work Flash bandwidth, then it should have a higher priority than the AXI masters accessing the Work Flash."]
    #[inline(always)]
    pub fn work_seq_rd_en(&self) -> WORK_SEQ_RD_EN_R {
        WORK_SEQ_RD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLASH macro wait states (same for main and work interfaces): '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    #[must_use]
    pub fn ws(&mut self) -> WS_W<0> {
        WS_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn main_map(&mut self) -> MAIN_MAP_W<8> {
        MAIN_MAP_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn work_map(&mut self) -> WORK_MAP_W<9> {
        WORK_MAP_W::new(self)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn main_bank_mode(&mut self) -> MAIN_BANK_MODE_W<12> {
        MAIN_BANK_MODE_W::new(self)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn work_bank_mode(&mut self) -> WORK_BANK_MODE_W<13> {
        WORK_BANK_MODE_W::new(self)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn main_ecc_en(&mut self) -> MAIN_ECC_EN_W<16> {
        MAIN_ECC_EN_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn main_ecc_inj_en(&mut self) -> MAIN_ECC_INJ_EN_W<17> {
        MAIN_ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error or a FLASH macro main interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error."]
    #[inline(always)]
    #[must_use]
    pub fn main_err_silent(&mut self) -> MAIN_ERR_SILENT_W<18> {
        MAIN_ERR_SILENT_W::new(self)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn work_ecc_en(&mut self) -> WORK_ECC_EN_W<20> {
        WORK_ECC_EN_W::new(self)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn work_ecc_inj_en(&mut self) -> WORK_ECC_INJ_EN_W<21> {
        WORK_ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error or a FLASH macro work interface internal error): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS, CM7_0_STATUS and CM7_1_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error."]
    #[inline(always)]
    #[must_use]
    pub fn work_err_silent(&mut self) -> WORK_ERR_SILENT_W<22> {
        WORK_ERR_SILENT_W::new(self)
    }
    #[doc = "Bit 24 - Enable sequential read mode for Work Flash for read accesses on the AXI port. 0: Sequential read mode for Work Flash is disabled. 1: Sequential read mode for Work Flash is enabled. This speeds up subsequent AXI read accesses to the same page (64 bytes) of the Work Flash. Setting this field to '1' has a side effect on the round-robin arbitration of the Work Flash macro between the different ports. When this field is set to '1', then as soon as the arbitration of the Work Flash is given to the AXI port, it is not released before the current AXI read burst either has been completed or leaves the current Work Flash page (64 bytes). This means that AHB masters accessing the Work Flash that have the same priority as the AXI port will not get their fair share of the Work Flash bandwidth. While an AHB master can perform one Work Flash access per round-robin cycle (4 bytes), an AXI master can read a whole AXI burst, or up to the next Work Flash page boundary. So if this field is set to '1', and an AHB master requires a high Work Flash bandwidth, then it should have a higher priority than the AXI masters accessing the Work Flash."]
    #[inline(always)]
    #[must_use]
    pub fn work_seq_rd_en(&mut self) -> WORK_SEQ_RD_EN_W<24> {
        WORK_SEQ_RD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](index.html) module"]
pub struct FLASH_CTL_SPEC;
impl crate::RegisterSpec for FLASH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ctl::R](R) reader structure"]
impl crate::Readable for FLASH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](W) writer structure"]
impl crate::Writable for FLASH_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_CTL to value 0x0100_0000"]
impl crate::Resettable for FLASH_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
