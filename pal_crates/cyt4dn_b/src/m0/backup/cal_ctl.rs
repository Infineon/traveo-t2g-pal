#[doc = "Register `CAL_CTL` reader"]
pub struct R(crate::R<CAL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL` writer"]
pub struct W(crate::W<CAL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL_SPEC>;
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
impl From<crate::W<CAL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIB_VAL` reader - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)) when CAL_COMP_PER_MIN is set at default value. Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) when CAL_COMP_PER_MIN is set at default value . Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments when CAL_COMP_PER_MIN is set at default value ."]
pub type CALIB_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB_VAL` writer - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)) when CAL_COMP_PER_MIN is set at default value. Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) when CAL_COMP_PER_MIN is set at default value . Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments when CAL_COMP_PER_MIN is set at default value ."]
pub type CALIB_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CALIB_SIGN` reader - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CALIB_SIGN_R = crate::BitReader<bool>;
#[doc = "Field `CALIB_SIGN` writer - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CALIB_SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL_SPEC, bool, O>;
#[doc = "Field `CAL_COMP_PER_MIN` reader - Select how many time calibration is performed per minute per step of 64 , each time a 64 step is added or substracted one unit 2/4/8/16*CALIB_VAL is substracted."]
pub type CAL_COMP_PER_MIN_R = crate::FieldReader<u8, CAL_COMP_PER_MIN_A>;
#[doc = "Select how many time calibration is performed per minute per step of 64 , each time a 64 step is added or substracted one unit 2/4/8/16*CALIB_VAL is substracted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAL_COMP_PER_MIN_A {
    #[doc = "0: Calibration of 64 each is performed twice per minute for 2*CALIB_VAL per hour"]
    _2 = 0,
    #[doc = "1: Calibration of 64 each is performed four times per minute for 4*CALIB_VAL per hour"]
    _4 = 1,
    #[doc = "2: Calibration of 64 each is performed eight times per minute for 8*CALIB_VAL per hour"]
    _8 = 2,
    #[doc = "3: Ca6libration of 64 each is performed sixteen times per minute for 16*CALIB_VAL per hour"]
    _16 = 3,
}
impl From<CAL_COMP_PER_MIN_A> for u8 {
    #[inline(always)]
    fn from(variant: CAL_COMP_PER_MIN_A) -> Self {
        variant as _
    }
}
impl CAL_COMP_PER_MIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_COMP_PER_MIN_A {
        match self.bits {
            0 => CAL_COMP_PER_MIN_A::_2,
            1 => CAL_COMP_PER_MIN_A::_4,
            2 => CAL_COMP_PER_MIN_A::_8,
            3 => CAL_COMP_PER_MIN_A::_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CAL_COMP_PER_MIN_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == CAL_COMP_PER_MIN_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CAL_COMP_PER_MIN_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CAL_COMP_PER_MIN_A::_16
    }
}
#[doc = "Field `CAL_COMP_PER_MIN` writer - Select how many time calibration is performed per minute per step of 64 , each time a 64 step is added or substracted one unit 2/4/8/16*CALIB_VAL is substracted."]
pub type CAL_COMP_PER_MIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAL_CTL_SPEC, u8, CAL_COMP_PER_MIN_A, 2, O>;
impl<'a, const O: u8> CAL_COMP_PER_MIN_W<'a, O> {
    #[doc = "Calibration of 64 each is performed twice per minute for 2*CALIB_VAL per hour"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CAL_COMP_PER_MIN_A::_2)
    }
    #[doc = "Calibration of 64 each is performed four times per minute for 4*CALIB_VAL per hour"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CAL_COMP_PER_MIN_A::_4)
    }
    #[doc = "Calibration of 64 each is performed eight times per minute for 8*CALIB_VAL per hour"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CAL_COMP_PER_MIN_A::_8)
    }
    #[doc = "Ca6libration of 64 each is performed sixteen times per minute for 16*CALIB_VAL per hour"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CAL_COMP_PER_MIN_A::_16)
    }
}
#[doc = "Field `CAL_SEL` reader - Select calibration wave output signal"]
pub type CAL_SEL_R = crate::FieldReader<u8, CAL_SEL_A>;
#[doc = "Select calibration wave output signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAL_SEL_A {
    #[doc = "0: 512Hz wave, not affected by calibration setting (not supported for 50/60Hz input clock: CTL.PRESCALER!=0)"]
    CAL512 = 0,
    #[doc = "1: N/A"]
    RSVD = 1,
    #[doc = "2: 2Hz wave, includes the effect of the calibration setting, (not supported for 50/60Hz input clock: CTL.PRESCALER!=0)"]
    CAL2 = 2,
    #[doc = "3: 1Hz wave, includes the effect of the calibration setting (supported for all input clocks)"]
    CAL1 = 3,
}
impl From<CAL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CAL_SEL_A) -> Self {
        variant as _
    }
}
impl CAL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_SEL_A {
        match self.bits {
            0 => CAL_SEL_A::CAL512,
            1 => CAL_SEL_A::RSVD,
            2 => CAL_SEL_A::CAL2,
            3 => CAL_SEL_A::CAL1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAL512`"]
    #[inline(always)]
    pub fn is_cal512(&self) -> bool {
        *self == CAL_SEL_A::CAL512
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == CAL_SEL_A::RSVD
    }
    #[doc = "Checks if the value of the field is `CAL2`"]
    #[inline(always)]
    pub fn is_cal2(&self) -> bool {
        *self == CAL_SEL_A::CAL2
    }
    #[doc = "Checks if the value of the field is `CAL1`"]
    #[inline(always)]
    pub fn is_cal1(&self) -> bool {
        *self == CAL_SEL_A::CAL1
    }
}
#[doc = "Field `CAL_SEL` writer - Select calibration wave output signal"]
pub type CAL_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAL_CTL_SPEC, u8, CAL_SEL_A, 2, O>;
impl<'a, const O: u8> CAL_SEL_W<'a, O> {
    #[doc = "512Hz wave, not affected by calibration setting (not supported for 50/60Hz input clock: CTL.PRESCALER!=0)"]
    #[inline(always)]
    pub fn cal512(self) -> &'a mut W {
        self.variant(CAL_SEL_A::CAL512)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(CAL_SEL_A::RSVD)
    }
    #[doc = "2Hz wave, includes the effect of the calibration setting, (not supported for 50/60Hz input clock: CTL.PRESCALER!=0)"]
    #[inline(always)]
    pub fn cal2(self) -> &'a mut W {
        self.variant(CAL_SEL_A::CAL2)
    }
    #[doc = "1Hz wave, includes the effect of the calibration setting (supported for all input clocks)"]
    #[inline(always)]
    pub fn cal1(self) -> &'a mut W {
        self.variant(CAL_SEL_A::CAL1)
    }
}
#[doc = "Field `CAL_OUT` reader - Output enable for wave signal for calibration and allow CALIB_VAL to be written."]
pub type CAL_OUT_R = crate::BitReader<bool>;
#[doc = "Field `CAL_OUT` writer - Output enable for wave signal for calibration and allow CALIB_VAL to be written."]
pub type CAL_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)) when CAL_COMP_PER_MIN is set at default value. Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) when CAL_COMP_PER_MIN is set at default value . Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments when CAL_COMP_PER_MIN is set at default value ."]
    #[inline(always)]
    pub fn calib_val(&self) -> CALIB_VAL_R {
        CALIB_VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&self) -> CALIB_SIGN_R {
        CALIB_SIGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select how many time calibration is performed per minute per step of 64 , each time a 64 step is added or substracted one unit 2/4/8/16*CALIB_VAL is substracted."]
    #[inline(always)]
    pub fn cal_comp_per_min(&self) -> CAL_COMP_PER_MIN_R {
        CAL_COMP_PER_MIN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Select calibration wave output signal"]
    #[inline(always)]
    pub fn cal_sel(&self) -> CAL_SEL_R {
        CAL_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Output enable for wave signal for calibration and allow CALIB_VAL to be written."]
    #[inline(always)]
    pub fn cal_out(&self) -> CAL_OUT_R {
        CAL_OUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)) when CAL_COMP_PER_MIN is set at default value. Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) when CAL_COMP_PER_MIN is set at default value . Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments when CAL_COMP_PER_MIN is set at default value ."]
    #[inline(always)]
    #[must_use]
    pub fn calib_val(&mut self) -> CALIB_VAL_W<0> {
        CALIB_VAL_W::new(self)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    #[must_use]
    pub fn calib_sign(&mut self) -> CALIB_SIGN_W<6> {
        CALIB_SIGN_W::new(self)
    }
    #[doc = "Bits 16:17 - Select how many time calibration is performed per minute per step of 64 , each time a 64 step is added or substracted one unit 2/4/8/16*CALIB_VAL is substracted."]
    #[inline(always)]
    #[must_use]
    pub fn cal_comp_per_min(&mut self) -> CAL_COMP_PER_MIN_W<16> {
        CAL_COMP_PER_MIN_W::new(self)
    }
    #[doc = "Bits 28:29 - Select calibration wave output signal"]
    #[inline(always)]
    #[must_use]
    pub fn cal_sel(&mut self) -> CAL_SEL_W<28> {
        CAL_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Output enable for wave signal for calibration and allow CALIB_VAL to be written."]
    #[inline(always)]
    #[must_use]
    pub fn cal_out(&mut self) -> CAL_OUT_W<31> {
        CAL_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator calibration for absolute frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl](index.html) module"]
pub struct CAL_CTL_SPEC;
impl crate::RegisterSpec for CAL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl::R](R) reader structure"]
impl crate::Readable for CAL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl::W](W) writer structure"]
impl crate::Writable for CAL_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL to value 0"]
impl crate::Resettable for CAL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
