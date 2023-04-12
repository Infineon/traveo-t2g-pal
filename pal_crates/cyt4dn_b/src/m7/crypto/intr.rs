#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR_FF_LEVEL` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
pub type INSTR_FF_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_FF_LEVEL` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
pub type INSTR_FF_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INSTR_FF_OVERFLOW` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
pub type INSTR_FF_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_FF_OVERFLOW` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
pub type INSTR_FF_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TR_INITIALIZED` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
pub type TR_INITIALIZED_R = crate::BitReader<bool>;
#[doc = "Field `TR_INITIALIZED` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
pub type TR_INITIALIZED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TR_DATA_AVAILABLE` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
pub type TR_DATA_AVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `TR_DATA_AVAILABLE` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
pub type TR_DATA_AVAILABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `PR_DATA_AVAILABLE` reader - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
pub type PR_DATA_AVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `PR_DATA_AVAILABLE` writer - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
pub type PR_DATA_AVAILABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INSTR_OPC_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type INSTR_OPC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_OPC_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type INSTR_OPC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INSTR_CC_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type INSTR_CC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_CC_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type INSTR_CC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `BUS_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TR_AP_DETECT_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
pub type TR_AP_DETECT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TR_AP_DETECT_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
pub type TR_AP_DETECT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TR_RC_DETECT_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
pub type TR_RC_DETECT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TR_RC_DETECT_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
pub type TR_RC_DETECT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INSTR_DEV_KEY_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
pub type INSTR_DEV_KEY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_DEV_KEY_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
pub type INSTR_DEV_KEY_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
    #[inline(always)]
    pub fn instr_ff_level(&self) -> INSTR_FF_LEVEL_R {
        INSTR_FF_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
    #[inline(always)]
    pub fn instr_ff_overflow(&self) -> INSTR_FF_OVERFLOW_R {
        INSTR_FF_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
    #[inline(always)]
    pub fn tr_initialized(&self) -> TR_INITIALIZED_R {
        TR_INITIALIZED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
    #[inline(always)]
    pub fn tr_data_available(&self) -> TR_DATA_AVAILABLE_R {
        TR_DATA_AVAILABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
    #[inline(always)]
    pub fn pr_data_available(&self) -> PR_DATA_AVAILABLE_R {
        PR_DATA_AVAILABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    pub fn instr_opc_error(&self) -> INSTR_OPC_ERROR_R {
        INSTR_OPC_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    pub fn instr_cc_error(&self) -> INSTR_CC_ERROR_R {
        INSTR_CC_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
    #[inline(always)]
    pub fn tr_ap_detect_error(&self) -> TR_AP_DETECT_ERROR_R {
        TR_AP_DETECT_ERROR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
    #[inline(always)]
    pub fn tr_rc_detect_error(&self) -> TR_RC_DETECT_ERROR_R {
        TR_RC_DETECT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
    #[inline(always)]
    pub fn instr_dev_key_error(&self) -> INSTR_DEV_KEY_ERROR_R {
        INSTR_DEV_KEY_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
    #[inline(always)]
    #[must_use]
    pub fn instr_ff_level(&mut self) -> INSTR_FF_LEVEL_W<0> {
        INSTR_FF_LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn instr_ff_overflow(&mut self) -> INSTR_FF_OVERFLOW_W<1> {
        INSTR_FF_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 2 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
    #[inline(always)]
    #[must_use]
    pub fn tr_initialized(&mut self) -> TR_INITIALIZED_W<2> {
        TR_INITIALIZED_W::new(self)
    }
    #[doc = "Bit 3 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
    #[inline(always)]
    #[must_use]
    pub fn tr_data_available(&mut self) -> TR_DATA_AVAILABLE_W<3> {
        TR_DATA_AVAILABLE_W::new(self)
    }
    #[doc = "Bit 4 - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
    #[inline(always)]
    #[must_use]
    pub fn pr_data_available(&mut self) -> PR_DATA_AVAILABLE_W<4> {
        PR_DATA_AVAILABLE_W::new(self)
    }
    #[doc = "Bit 16 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn instr_opc_error(&mut self) -> INSTR_OPC_ERROR_W<16> {
        INSTR_OPC_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn instr_cc_error(&mut self) -> INSTR_CC_ERROR_W<17> {
        INSTR_CC_ERROR_W::new(self)
    }
    #[doc = "Bit 18 - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn bus_error(&mut self) -> BUS_ERROR_W<18> {
        BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 19 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
    #[inline(always)]
    #[must_use]
    pub fn tr_ap_detect_error(&mut self) -> TR_AP_DETECT_ERROR_W<19> {
        TR_AP_DETECT_ERROR_W::new(self)
    }
    #[doc = "Bit 20 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rc_detect_error(&mut self) -> TR_RC_DETECT_ERROR_W<20> {
        TR_RC_DETECT_ERROR_W::new(self)
    }
    #[doc = "Bit 21 - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn instr_dev_key_error(&mut self) -> INSTR_DEV_KEY_ERROR_W<21> {
        INSTR_DEV_KEY_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
