#[doc = "Register `PASS_CTL` reader"]
pub struct R(crate::R<PASS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASS_CTL` writer"]
pub struct W(crate::W<PASS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASS_CTL_SPEC>;
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
impl From<crate::W<PASS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUPPLY_MON_EN_A` reader - Supply monitor enable for AMUXBUS_A (amuxbus_a_mon)"]
pub type SUPPLY_MON_EN_A_R = crate::BitReader<bool>;
#[doc = "Field `SUPPLY_MON_EN_A` writer - Supply monitor enable for AMUXBUS_A (amuxbus_a_mon)"]
pub type SUPPLY_MON_EN_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, PASS_CTL_SPEC, bool, O>;
#[doc = "Field `SUPPLY_MON_LVL_A` reader - Supply monitor level select for AMUXBUS_A"]
pub type SUPPLY_MON_LVL_A_R = crate::BitReader<SUPPLY_MON_LVL_A_A>;
#[doc = "Supply monitor level select for AMUXBUS_A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUPPLY_MON_LVL_A_A {
    #[doc = "0: amuxbus_a_mon = VRL"]
    VRL = 0,
    #[doc = "1: amuxbus_a_mon = VRH"]
    VRH = 1,
}
impl From<SUPPLY_MON_LVL_A_A> for bool {
    #[inline(always)]
    fn from(variant: SUPPLY_MON_LVL_A_A) -> Self {
        variant as u8 != 0
    }
}
impl SUPPLY_MON_LVL_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPPLY_MON_LVL_A_A {
        match self.bits {
            false => SUPPLY_MON_LVL_A_A::VRL,
            true => SUPPLY_MON_LVL_A_A::VRH,
        }
    }
    #[doc = "Checks if the value of the field is `VRL`"]
    #[inline(always)]
    pub fn is_vrl(&self) -> bool {
        *self == SUPPLY_MON_LVL_A_A::VRL
    }
    #[doc = "Checks if the value of the field is `VRH`"]
    #[inline(always)]
    pub fn is_vrh(&self) -> bool {
        *self == SUPPLY_MON_LVL_A_A::VRH
    }
}
#[doc = "Field `SUPPLY_MON_LVL_A` writer - Supply monitor level select for AMUXBUS_A"]
pub type SUPPLY_MON_LVL_A_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PASS_CTL_SPEC, SUPPLY_MON_LVL_A_A, O>;
impl<'a, const O: u8> SUPPLY_MON_LVL_A_W<'a, O> {
    #[doc = "amuxbus_a_mon = VRL"]
    #[inline(always)]
    pub fn vrl(self) -> &'a mut W {
        self.variant(SUPPLY_MON_LVL_A_A::VRL)
    }
    #[doc = "amuxbus_a_mon = VRH"]
    #[inline(always)]
    pub fn vrh(self) -> &'a mut W {
        self.variant(SUPPLY_MON_LVL_A_A::VRH)
    }
}
#[doc = "Field `SUPPLY_MON_EN_B` reader - Supply monitor enable for AMUXBUS_B (amuxbus_b_mon)"]
pub type SUPPLY_MON_EN_B_R = crate::BitReader<bool>;
#[doc = "Field `SUPPLY_MON_EN_B` writer - Supply monitor enable for AMUXBUS_B (amuxbus_b_mon)"]
pub type SUPPLY_MON_EN_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, PASS_CTL_SPEC, bool, O>;
#[doc = "Field `SUPPLY_MON_LVL_B` reader - Supply monitor level select for AMUXBUS_B"]
pub type SUPPLY_MON_LVL_B_R = crate::BitReader<SUPPLY_MON_LVL_B_A>;
#[doc = "Supply monitor level select for AMUXBUS_B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUPPLY_MON_LVL_B_A {
    #[doc = "0: amuxbus_b_mon = VRL"]
    VRL = 0,
    #[doc = "1: amuxbus_b_mon = VRH"]
    VRH = 1,
}
impl From<SUPPLY_MON_LVL_B_A> for bool {
    #[inline(always)]
    fn from(variant: SUPPLY_MON_LVL_B_A) -> Self {
        variant as u8 != 0
    }
}
impl SUPPLY_MON_LVL_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPPLY_MON_LVL_B_A {
        match self.bits {
            false => SUPPLY_MON_LVL_B_A::VRL,
            true => SUPPLY_MON_LVL_B_A::VRH,
        }
    }
    #[doc = "Checks if the value of the field is `VRL`"]
    #[inline(always)]
    pub fn is_vrl(&self) -> bool {
        *self == SUPPLY_MON_LVL_B_A::VRL
    }
    #[doc = "Checks if the value of the field is `VRH`"]
    #[inline(always)]
    pub fn is_vrh(&self) -> bool {
        *self == SUPPLY_MON_LVL_B_A::VRH
    }
}
#[doc = "Field `SUPPLY_MON_LVL_B` writer - Supply monitor level select for AMUXBUS_B"]
pub type SUPPLY_MON_LVL_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PASS_CTL_SPEC, SUPPLY_MON_LVL_B_A, O>;
impl<'a, const O: u8> SUPPLY_MON_LVL_B_W<'a, O> {
    #[doc = "amuxbus_b_mon = VRL"]
    #[inline(always)]
    pub fn vrl(self) -> &'a mut W {
        self.variant(SUPPLY_MON_LVL_B_A::VRL)
    }
    #[doc = "amuxbus_b_mon = VRH"]
    #[inline(always)]
    pub fn vrh(self) -> &'a mut W {
        self.variant(SUPPLY_MON_LVL_B_A::VRH)
    }
}
#[doc = "Field `REFBUF_MODE` reader - Reference mode. The reference needs to be present when using TEMP sensor or diagnostic reference (in addition to SAR.DIAG_CTL.DIAG_EN). Note that setting this mode is not required for the ADC operation itself."]
pub type REFBUF_MODE_R = crate::FieldReader<u8, REFBUF_MODE_A>;
#[doc = "Reference mode. The reference needs to be present when using TEMP sensor or diagnostic reference (in addition to SAR.DIAG_CTL.DIAG_EN). Note that setting this mode is not required for the ADC operation itself.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFBUF_MODE_A {
    #[doc = "0: No reference"]
    OFF = 0,
    #[doc = "1: Reference = buffered Vbg from SRSS"]
    ON = 1,
    #[doc = "2: undefined"]
    RSVD = 2,
    #[doc = "3: Reference = unbuffered Vbg from SRSS"]
    BYPASS = 3,
}
impl From<REFBUF_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REFBUF_MODE_A) -> Self {
        variant as _
    }
}
impl REFBUF_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBUF_MODE_A {
        match self.bits {
            0 => REFBUF_MODE_A::OFF,
            1 => REFBUF_MODE_A::ON,
            2 => REFBUF_MODE_A::RSVD,
            3 => REFBUF_MODE_A::BYPASS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == REFBUF_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == REFBUF_MODE_A::ON
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == REFBUF_MODE_A::RSVD
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == REFBUF_MODE_A::BYPASS
    }
}
#[doc = "Field `REFBUF_MODE` writer - Reference mode. The reference needs to be present when using TEMP sensor or diagnostic reference (in addition to SAR.DIAG_CTL.DIAG_EN). Note that setting this mode is not required for the ADC operation itself."]
pub type REFBUF_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PASS_CTL_SPEC, u8, REFBUF_MODE_A, 2, O>;
impl<'a, const O: u8> REFBUF_MODE_W<'a, O> {
    #[doc = "No reference"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(REFBUF_MODE_A::OFF)
    }
    #[doc = "Reference = buffered Vbg from SRSS"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(REFBUF_MODE_A::ON)
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(REFBUF_MODE_A::RSVD)
    }
    #[doc = "Reference = unbuffered Vbg from SRSS"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(REFBUF_MODE_A::BYPASS)
    }
}
#[doc = "Field `DBG_FREEZE_EN` reader - Debug pause enable, 1 per ADC. When set a high tr_debug_freeze trigger will prevent the scheduler from starting acquisitions on a new channel. Note that averaging for an already started channel will be completed."]
pub type DBG_FREEZE_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_FREEZE_EN` writer - Debug pause enable, 1 per ADC. When set a high tr_debug_freeze trigger will prevent the scheduler from starting acquisitions on a new channel. Note that averaging for an already started channel will be completed."]
pub type DBG_FREEZE_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PASS_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Supply monitor enable for AMUXBUS_A (amuxbus_a_mon)"]
    #[inline(always)]
    pub fn supply_mon_en_a(&self) -> SUPPLY_MON_EN_A_R {
        SUPPLY_MON_EN_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Supply monitor level select for AMUXBUS_A"]
    #[inline(always)]
    pub fn supply_mon_lvl_a(&self) -> SUPPLY_MON_LVL_A_R {
        SUPPLY_MON_LVL_A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Supply monitor enable for AMUXBUS_B (amuxbus_b_mon)"]
    #[inline(always)]
    pub fn supply_mon_en_b(&self) -> SUPPLY_MON_EN_B_R {
        SUPPLY_MON_EN_B_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Supply monitor level select for AMUXBUS_B"]
    #[inline(always)]
    pub fn supply_mon_lvl_b(&self) -> SUPPLY_MON_LVL_B_R {
        SUPPLY_MON_LVL_B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Reference mode. The reference needs to be present when using TEMP sensor or diagnostic reference (in addition to SAR.DIAG_CTL.DIAG_EN). Note that setting this mode is not required for the ADC operation itself."]
    #[inline(always)]
    pub fn refbuf_mode(&self) -> REFBUF_MODE_R {
        REFBUF_MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Debug pause enable, 1 per ADC. When set a high tr_debug_freeze trigger will prevent the scheduler from starting acquisitions on a new channel. Note that averaging for an already started channel will be completed."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Supply monitor enable for AMUXBUS_A (amuxbus_a_mon)"]
    #[inline(always)]
    #[must_use]
    pub fn supply_mon_en_a(&mut self) -> SUPPLY_MON_EN_A_W<0> {
        SUPPLY_MON_EN_A_W::new(self)
    }
    #[doc = "Bit 1 - Supply monitor level select for AMUXBUS_A"]
    #[inline(always)]
    #[must_use]
    pub fn supply_mon_lvl_a(&mut self) -> SUPPLY_MON_LVL_A_W<1> {
        SUPPLY_MON_LVL_A_W::new(self)
    }
    #[doc = "Bit 4 - Supply monitor enable for AMUXBUS_B (amuxbus_b_mon)"]
    #[inline(always)]
    #[must_use]
    pub fn supply_mon_en_b(&mut self) -> SUPPLY_MON_EN_B_W<4> {
        SUPPLY_MON_EN_B_W::new(self)
    }
    #[doc = "Bit 5 - Supply monitor level select for AMUXBUS_B"]
    #[inline(always)]
    #[must_use]
    pub fn supply_mon_lvl_b(&mut self) -> SUPPLY_MON_LVL_B_W<5> {
        SUPPLY_MON_LVL_B_W::new(self)
    }
    #[doc = "Bits 21:22 - Reference mode. The reference needs to be present when using TEMP sensor or diagnostic reference (in addition to SAR.DIAG_CTL.DIAG_EN). Note that setting this mode is not required for the ADC operation itself."]
    #[inline(always)]
    #[must_use]
    pub fn refbuf_mode(&mut self) -> REFBUF_MODE_W<21> {
        REFBUF_MODE_W::new(self)
    }
    #[doc = "Bits 28:31 - Debug pause enable, 1 per ADC. When set a high tr_debug_freeze trigger will prevent the scheduler from starting acquisitions on a new channel. Note that averaging for an already started channel will be completed."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W<28> {
        DBG_FREEZE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PASS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pass_ctl](index.html) module"]
pub struct PASS_CTL_SPEC;
impl crate::RegisterSpec for PASS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pass_ctl::R](R) reader structure"]
impl crate::Readable for PASS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pass_ctl::W](W) writer structure"]
impl crate::Writable for PASS_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PASS_CTL to value 0"]
impl crate::Resettable for PASS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
