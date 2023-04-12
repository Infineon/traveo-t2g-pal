#[doc = "Register `TEST_CTL` reader"]
pub struct R(crate::R<TEST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTL` writer"]
pub struct W(crate::W<TEST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTL_SPEC>;
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
impl From<crate::W<TEST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_CAL_CUR_IN` reader - External current input switch control, for Temperature Sensor Calibration"]
pub type TS_CAL_CUR_IN_R = crate::BitReader<bool>;
#[doc = "Field `TS_CAL_CUR_IN` writer - External current input switch control, for Temperature Sensor Calibration"]
pub type TS_CAL_CUR_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TS_CAL_VB_OUT` reader - Voltage Base switch control, for Temperature Sensor Calibration"]
pub type TS_CAL_VB_OUT_R = crate::BitReader<bool>;
#[doc = "Field `TS_CAL_VB_OUT` writer - Voltage Base switch control, for Temperature Sensor Calibration"]
pub type TS_CAL_VB_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TS_CAL_VE_OUT` reader - Voltage Emitter switch control, for Temperature Sensor Calibration"]
pub type TS_CAL_VE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `TS_CAL_VE_OUT` writer - Voltage Emitter switch control, for Temperature Sensor Calibration"]
pub type TS_CAL_VE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TS_CAL_DIODE_EN` reader - Diode Enable, disconnect or connect the base and collector terminal of the BJT"]
pub type TS_CAL_DIODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TS_CAL_DIODE_EN` writer - Diode Enable, disconnect or connect the base and collector terminal of the BJT"]
pub type TS_CAL_DIODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TS_CAL_DIODE_PNP_EN` reader - Enable signal for PNP transistor. This transistor will be used only during calibration for accurate estimation of chip temp. 0 = Turn PNP off 1 = Configure PNP as a diode (short base and collector)"]
pub type TS_CAL_DIODE_PNP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TS_CAL_DIODE_PNP_EN` writer - Enable signal for PNP transistor. This transistor will be used only during calibration for accurate estimation of chip temp. 0 = Turn PNP off 1 = Configure PNP as a diode (short base and collector)"]
pub type TS_CAL_DIODE_PNP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TS_CAL_VI_SEL` reader - Select current or voltage output on 'v_temp' pin, for Temperature Sensor Calibration"]
pub type TS_CAL_VI_SEL_R = crate::BitReader<TS_CAL_VI_SEL_A>;
#[doc = "Select current or voltage output on 'v_temp' pin, for Temperature Sensor Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_CAL_VI_SEL_A {
    #[doc = "0: Current is selected"]
    CURRENT = 0,
    #[doc = "1: Voltage is selected"]
    VOLTAGE = 1,
}
impl From<TS_CAL_VI_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TS_CAL_VI_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_CAL_VI_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_CAL_VI_SEL_A {
        match self.bits {
            false => TS_CAL_VI_SEL_A::CURRENT,
            true => TS_CAL_VI_SEL_A::VOLTAGE,
        }
    }
    #[doc = "Checks if the value of the field is `CURRENT`"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == TS_CAL_VI_SEL_A::CURRENT
    }
    #[doc = "Checks if the value of the field is `VOLTAGE`"]
    #[inline(always)]
    pub fn is_voltage(&self) -> bool {
        *self == TS_CAL_VI_SEL_A::VOLTAGE
    }
}
#[doc = "Field `TS_CAL_VI_SEL` writer - Select current or voltage output on 'v_temp' pin, for Temperature Sensor Calibration"]
pub type TS_CAL_VI_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEST_CTL_SPEC, TS_CAL_VI_SEL_A, O>;
impl<'a, const O: u8> TS_CAL_VI_SEL_W<'a, O> {
    #[doc = "Current is selected"]
    #[inline(always)]
    pub fn current(self) -> &'a mut W {
        self.variant(TS_CAL_VI_SEL_A::CURRENT)
    }
    #[doc = "Voltage is selected"]
    #[inline(always)]
    pub fn voltage(self) -> &'a mut W {
        self.variant(TS_CAL_VI_SEL_A::VOLTAGE)
    }
}
#[doc = "Field `TS_CAL_CUR_SEL` reader - Select the current going into the BJT, for Temperature Sensor Calibration"]
pub type TS_CAL_CUR_SEL_R = crate::FieldReader<u8, TS_CAL_CUR_SEL_A>;
#[doc = "Select the current going into the BJT, for Temperature Sensor Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS_CAL_CUR_SEL_A {
    #[doc = "0: Select 1 uA"]
    I_1U = 0,
    #[doc = "1: Select 2 uA"]
    I_2U = 1,
    #[doc = "2: Select 5 uA"]
    I_5U = 2,
    #[doc = "3: Select 10 uA"]
    I_10U = 3,
}
impl From<TS_CAL_CUR_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_CAL_CUR_SEL_A) -> Self {
        variant as _
    }
}
impl TS_CAL_CUR_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_CAL_CUR_SEL_A {
        match self.bits {
            0 => TS_CAL_CUR_SEL_A::I_1U,
            1 => TS_CAL_CUR_SEL_A::I_2U,
            2 => TS_CAL_CUR_SEL_A::I_5U,
            3 => TS_CAL_CUR_SEL_A::I_10U,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `I_1U`"]
    #[inline(always)]
    pub fn is_i_1u(&self) -> bool {
        *self == TS_CAL_CUR_SEL_A::I_1U
    }
    #[doc = "Checks if the value of the field is `I_2U`"]
    #[inline(always)]
    pub fn is_i_2u(&self) -> bool {
        *self == TS_CAL_CUR_SEL_A::I_2U
    }
    #[doc = "Checks if the value of the field is `I_5U`"]
    #[inline(always)]
    pub fn is_i_5u(&self) -> bool {
        *self == TS_CAL_CUR_SEL_A::I_5U
    }
    #[doc = "Checks if the value of the field is `I_10U`"]
    #[inline(always)]
    pub fn is_i_10u(&self) -> bool {
        *self == TS_CAL_CUR_SEL_A::I_10U
    }
}
#[doc = "Field `TS_CAL_CUR_SEL` writer - Select the current going into the BJT, for Temperature Sensor Calibration"]
pub type TS_CAL_CUR_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TEST_CTL_SPEC, u8, TS_CAL_CUR_SEL_A, 2, O>;
impl<'a, const O: u8> TS_CAL_CUR_SEL_W<'a, O> {
    #[doc = "Select 1 uA"]
    #[inline(always)]
    pub fn i_1u(self) -> &'a mut W {
        self.variant(TS_CAL_CUR_SEL_A::I_1U)
    }
    #[doc = "Select 2 uA"]
    #[inline(always)]
    pub fn i_2u(self) -> &'a mut W {
        self.variant(TS_CAL_CUR_SEL_A::I_2U)
    }
    #[doc = "Select 5 uA"]
    #[inline(always)]
    pub fn i_5u(self) -> &'a mut W {
        self.variant(TS_CAL_CUR_SEL_A::I_5U)
    }
    #[doc = "Select 10 uA"]
    #[inline(always)]
    pub fn i_10u(self) -> &'a mut W {
        self.variant(TS_CAL_CUR_SEL_A::I_10U)
    }
}
#[doc = "Field `TS_CAL_SPARE` reader - Spare"]
pub type TS_CAL_SPARE_R = crate::BitReader<bool>;
#[doc = "Field `TS_CAL_SPARE` writer - Spare"]
pub type TS_CAL_SPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External current input switch control, for Temperature Sensor Calibration"]
    #[inline(always)]
    pub fn ts_cal_cur_in(&self) -> TS_CAL_CUR_IN_R {
        TS_CAL_CUR_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Base switch control, for Temperature Sensor Calibration"]
    #[inline(always)]
    pub fn ts_cal_vb_out(&self) -> TS_CAL_VB_OUT_R {
        TS_CAL_VB_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Emitter switch control, for Temperature Sensor Calibration"]
    #[inline(always)]
    pub fn ts_cal_ve_out(&self) -> TS_CAL_VE_OUT_R {
        TS_CAL_VE_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Diode Enable, disconnect or connect the base and collector terminal of the BJT"]
    #[inline(always)]
    pub fn ts_cal_diode_en(&self) -> TS_CAL_DIODE_EN_R {
        TS_CAL_DIODE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable signal for PNP transistor. This transistor will be used only during calibration for accurate estimation of chip temp. 0 = Turn PNP off 1 = Configure PNP as a diode (short base and collector)"]
    #[inline(always)]
    pub fn ts_cal_diode_pnp_en(&self) -> TS_CAL_DIODE_PNP_EN_R {
        TS_CAL_DIODE_PNP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select current or voltage output on 'v_temp' pin, for Temperature Sensor Calibration"]
    #[inline(always)]
    pub fn ts_cal_vi_sel(&self) -> TS_CAL_VI_SEL_R {
        TS_CAL_VI_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Select the current going into the BJT, for Temperature Sensor Calibration"]
    #[inline(always)]
    pub fn ts_cal_cur_sel(&self) -> TS_CAL_CUR_SEL_R {
        TS_CAL_CUR_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Spare"]
    #[inline(always)]
    pub fn ts_cal_spare(&self) -> TS_CAL_SPARE_R {
        TS_CAL_SPARE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External current input switch control, for Temperature Sensor Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_cur_in(&mut self) -> TS_CAL_CUR_IN_W<0> {
        TS_CAL_CUR_IN_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Base switch control, for Temperature Sensor Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_vb_out(&mut self) -> TS_CAL_VB_OUT_W<2> {
        TS_CAL_VB_OUT_W::new(self)
    }
    #[doc = "Bit 3 - Voltage Emitter switch control, for Temperature Sensor Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_ve_out(&mut self) -> TS_CAL_VE_OUT_W<3> {
        TS_CAL_VE_OUT_W::new(self)
    }
    #[doc = "Bit 4 - Diode Enable, disconnect or connect the base and collector terminal of the BJT"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_diode_en(&mut self) -> TS_CAL_DIODE_EN_W<4> {
        TS_CAL_DIODE_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable signal for PNP transistor. This transistor will be used only during calibration for accurate estimation of chip temp. 0 = Turn PNP off 1 = Configure PNP as a diode (short base and collector)"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_diode_pnp_en(&mut self) -> TS_CAL_DIODE_PNP_EN_W<5> {
        TS_CAL_DIODE_PNP_EN_W::new(self)
    }
    #[doc = "Bit 6 - Select current or voltage output on 'v_temp' pin, for Temperature Sensor Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_vi_sel(&mut self) -> TS_CAL_VI_SEL_W<6> {
        TS_CAL_VI_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Select the current going into the BJT, for Temperature Sensor Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_cur_sel(&mut self) -> TS_CAL_CUR_SEL_W<8> {
        TS_CAL_CUR_SEL_W::new(self)
    }
    #[doc = "Bit 12 - Spare"]
    #[inline(always)]
    #[must_use]
    pub fn ts_cal_spare(&mut self) -> TS_CAL_SPARE_W<12> {
        TS_CAL_SPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctl](index.html) module"]
pub struct TEST_CTL_SPEC;
impl crate::RegisterSpec for TEST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_ctl::R](R) reader structure"]
impl crate::Readable for TEST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctl::W](W) writer structure"]
impl crate::Writable for TEST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST_CTL to value 0"]
impl crate::Resettable for TEST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
