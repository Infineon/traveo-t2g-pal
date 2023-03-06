#[doc = "Register `RAM0_CTL0` reader"]
pub struct R(crate::R<RAM0_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM0_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM0_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM0_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM0_CTL0` writer"]
pub struct W(crate::W<RAM0_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM0_CTL0_SPEC>;
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
impl From<crate::W<RAM0_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM0_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow') intefrace. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
pub type SLOW_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow') intefrace. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
pub type SLOW_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM0_CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast_0/1') interface. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
pub type FAST_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast_0/1') interface. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
pub type FAST_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM0_CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM0_CTL0_SPEC, bool, O>;
#[doc = "Field `ECC_AUTO_CORRECT` reader - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
pub type ECC_AUTO_CORRECT_R = crate::BitReader<bool>;
#[doc = "Field `ECC_AUTO_CORRECT` writer - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
pub type ECC_AUTO_CORRECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM0_CTL0_SPEC, bool, O>;
#[doc = "Field `ECC_INJ_EN` reader - Enable ECC parity injection. Instead of calculating the parity from the write data to the RAM, the parity is taken from ECC_CTL PARITY upon a match of ECC_CTL WORD_ADDR. The write data can be of any size. This bit is ignored when ECC_EN=0. Note: Parity injection invalidates the write buffer for this word address. If only a part of 64-bit data is written and consistency should be maintained, RAMC#_STATUS WB_EMPTY=1 should be checked before."]
pub type ECC_INJ_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECC_INJ_EN` writer - Enable ECC parity injection. Instead of calculating the parity from the write data to the RAM, the parity is taken from ECC_CTL PARITY upon a match of ECC_CTL WORD_ADDR. The write data can be of any size. This bit is ignored when ECC_EN=0. Note: Parity injection invalidates the write buffer for this word address. If only a part of 64-bit data is written and consistency should be maintained, RAMC#_STATUS WB_EMPTY=1 should be checked before."]
pub type ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM0_CTL0_SPEC, bool, O>;
#[doc = "Field `ECC_CHECK_DIS` reader - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when ECC_EN=0."]
pub type ECC_CHECK_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ECC_CHECK_DIS` writer - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when ECC_EN=0."]
pub type ECC_CHECK_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM0_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow') intefrace. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast_0/1') interface. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> ECC_AUTO_CORRECT_R {
        ECC_AUTO_CORRECT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable ECC parity injection. Instead of calculating the parity from the write data to the RAM, the parity is taken from ECC_CTL PARITY upon a match of ECC_CTL WORD_ADDR. The write data can be of any size. This bit is ignored when ECC_EN=0. Note: Parity injection invalidates the write buffer for this word address. If only a part of 64-bit data is written and consistency should be maintained, RAMC#_STATUS WB_EMPTY=1 should be checked before."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when ECC_EN=0."]
    #[inline(always)]
    pub fn ecc_check_dis(&self) -> ECC_CHECK_DIS_R {
        ECC_CHECK_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow') intefrace. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SLOW_WS_W<0> {
        SLOW_WS_W::new(self)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast_0/1') interface. The number of wait states is expressed in 'clk_mem' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FAST_WS_W<8> {
        FAST_WS_W::new(self)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> ECC_EN_W<16> {
        ECC_EN_W::new(self)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_auto_correct(&mut self) -> ECC_AUTO_CORRECT_W<17> {
        ECC_AUTO_CORRECT_W::new(self)
    }
    #[doc = "Bit 18 - Enable ECC parity injection. Instead of calculating the parity from the write data to the RAM, the parity is taken from ECC_CTL PARITY upon a match of ECC_CTL WORD_ADDR. The write data can be of any size. This bit is ignored when ECC_EN=0. Note: Parity injection invalidates the write buffer for this word address. If only a part of 64-bit data is written and consistency should be maintained, RAMC#_STATUS WB_EMPTY=1 should be checked before."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W<18> {
        ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 19 - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when ECC_EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_check_dis(&mut self) -> ECC_CHECK_DIS_W<19> {
        ECC_CHECK_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM 0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_ctl0](index.html) module"]
pub struct RAM0_CTL0_SPEC;
impl crate::RegisterSpec for RAM0_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram0_ctl0::R](R) reader structure"]
impl crate::Readable for RAM0_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram0_ctl0::W](W) writer structure"]
impl crate::Writable for RAM0_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM0_CTL0 to value 0x0003_0001"]
impl crate::Resettable for RAM0_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0001;
}
