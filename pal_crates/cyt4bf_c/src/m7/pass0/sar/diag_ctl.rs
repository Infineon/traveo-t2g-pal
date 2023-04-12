#[doc = "Register `DIAG_CTL` reader"]
pub struct R(crate::R<DIAG_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIAG_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIAG_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIAG_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIAG_CTL` writer"]
pub struct W(crate::W<DIAG_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIAG_CTL_SPEC>;
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
impl From<crate::W<DIAG_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIAG_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAG_SEL` reader - Select Diagnostic Reference function"]
pub type DIAG_SEL_R = crate::FieldReader<u8, DIAG_SEL_A>;
#[doc = "Select Diagnostic Reference function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAG_SEL_A {
    #[doc = "0: DiagOut = VrefL"]
    VREFL = 0,
    #[doc = "1: DiagOut = VrefH * 1/8"]
    VREFH_1DIV8 = 1,
    #[doc = "2: DiagOut = VrefH * 2/8"]
    VREFH_2DIV8 = 2,
    #[doc = "3: DiagOut = VrefH * 3/8"]
    VREFH_3DIV8 = 3,
    #[doc = "4: DiagOut = VrefH * 4/8"]
    VREFH_4DIV8 = 4,
    #[doc = "5: DiagOut = VrefH * 5/8"]
    VREFH_5DIV8 = 5,
    #[doc = "6: DiagOut = VrefH * 6/8"]
    VREFH_6DIV8 = 6,
    #[doc = "7: DiagOut = VrefH * 7/8"]
    VREFH_7DIV8 = 7,
    #[doc = "8: DiagOut = VrefH"]
    VREFH = 8,
    #[doc = "9: DiagOut = VrefX = VrefH * 199/200"]
    VREFX = 9,
    #[doc = "10: DiagOut = Vbg from SRSS"]
    VBG = 10,
    #[doc = "11: DiagOut = Vin1"]
    VIN1 = 11,
    #[doc = "12: DiagOut = Vin2"]
    VIN2 = 12,
    #[doc = "13: DiagOut = Vin3"]
    VIN3 = 13,
    #[doc = "14: DiagOut = Isource (10uA)"]
    I_SOURCE = 14,
    #[doc = "15: DiagOut = Isink (10uA)"]
    I_SINK = 15,
}
impl From<DIAG_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG_SEL_A) -> Self {
        variant as _
    }
}
impl DIAG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAG_SEL_A {
        match self.bits {
            0 => DIAG_SEL_A::VREFL,
            1 => DIAG_SEL_A::VREFH_1DIV8,
            2 => DIAG_SEL_A::VREFH_2DIV8,
            3 => DIAG_SEL_A::VREFH_3DIV8,
            4 => DIAG_SEL_A::VREFH_4DIV8,
            5 => DIAG_SEL_A::VREFH_5DIV8,
            6 => DIAG_SEL_A::VREFH_6DIV8,
            7 => DIAG_SEL_A::VREFH_7DIV8,
            8 => DIAG_SEL_A::VREFH,
            9 => DIAG_SEL_A::VREFX,
            10 => DIAG_SEL_A::VBG,
            11 => DIAG_SEL_A::VIN1,
            12 => DIAG_SEL_A::VIN2,
            13 => DIAG_SEL_A::VIN3,
            14 => DIAG_SEL_A::I_SOURCE,
            15 => DIAG_SEL_A::I_SINK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFL`"]
    #[inline(always)]
    pub fn is_vrefl(&self) -> bool {
        *self == DIAG_SEL_A::VREFL
    }
    #[doc = "Checks if the value of the field is `VREFH_1DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_1div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_1DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH_2DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_2div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_2DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH_3DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_3div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_3DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH_4DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_4div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_4DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH_5DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_5div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_5DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH_6DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_6div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_6DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH_7DIV8`"]
    #[inline(always)]
    pub fn is_vrefh_7div8(&self) -> bool {
        *self == DIAG_SEL_A::VREFH_7DIV8
    }
    #[doc = "Checks if the value of the field is `VREFH`"]
    #[inline(always)]
    pub fn is_vrefh(&self) -> bool {
        *self == DIAG_SEL_A::VREFH
    }
    #[doc = "Checks if the value of the field is `VREFX`"]
    #[inline(always)]
    pub fn is_vrefx(&self) -> bool {
        *self == DIAG_SEL_A::VREFX
    }
    #[doc = "Checks if the value of the field is `VBG`"]
    #[inline(always)]
    pub fn is_vbg(&self) -> bool {
        *self == DIAG_SEL_A::VBG
    }
    #[doc = "Checks if the value of the field is `VIN1`"]
    #[inline(always)]
    pub fn is_vin1(&self) -> bool {
        *self == DIAG_SEL_A::VIN1
    }
    #[doc = "Checks if the value of the field is `VIN2`"]
    #[inline(always)]
    pub fn is_vin2(&self) -> bool {
        *self == DIAG_SEL_A::VIN2
    }
    #[doc = "Checks if the value of the field is `VIN3`"]
    #[inline(always)]
    pub fn is_vin3(&self) -> bool {
        *self == DIAG_SEL_A::VIN3
    }
    #[doc = "Checks if the value of the field is `I_SOURCE`"]
    #[inline(always)]
    pub fn is_i_source(&self) -> bool {
        *self == DIAG_SEL_A::I_SOURCE
    }
    #[doc = "Checks if the value of the field is `I_SINK`"]
    #[inline(always)]
    pub fn is_i_sink(&self) -> bool {
        *self == DIAG_SEL_A::I_SINK
    }
}
#[doc = "Field `DIAG_SEL` writer - Select Diagnostic Reference function"]
pub type DIAG_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DIAG_CTL_SPEC, u8, DIAG_SEL_A, 4, O>;
impl<'a, const O: u8> DIAG_SEL_W<'a, O> {
    #[doc = "DiagOut = VrefL"]
    #[inline(always)]
    pub fn vrefl(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFL)
    }
    #[doc = "DiagOut = VrefH * 1/8"]
    #[inline(always)]
    pub fn vrefh_1div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_1DIV8)
    }
    #[doc = "DiagOut = VrefH * 2/8"]
    #[inline(always)]
    pub fn vrefh_2div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_2DIV8)
    }
    #[doc = "DiagOut = VrefH * 3/8"]
    #[inline(always)]
    pub fn vrefh_3div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_3DIV8)
    }
    #[doc = "DiagOut = VrefH * 4/8"]
    #[inline(always)]
    pub fn vrefh_4div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_4DIV8)
    }
    #[doc = "DiagOut = VrefH * 5/8"]
    #[inline(always)]
    pub fn vrefh_5div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_5DIV8)
    }
    #[doc = "DiagOut = VrefH * 6/8"]
    #[inline(always)]
    pub fn vrefh_6div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_6DIV8)
    }
    #[doc = "DiagOut = VrefH * 7/8"]
    #[inline(always)]
    pub fn vrefh_7div8(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH_7DIV8)
    }
    #[doc = "DiagOut = VrefH"]
    #[inline(always)]
    pub fn vrefh(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFH)
    }
    #[doc = "DiagOut = VrefX = VrefH * 199/200"]
    #[inline(always)]
    pub fn vrefx(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VREFX)
    }
    #[doc = "DiagOut = Vbg from SRSS"]
    #[inline(always)]
    pub fn vbg(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VBG)
    }
    #[doc = "DiagOut = Vin1"]
    #[inline(always)]
    pub fn vin1(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VIN1)
    }
    #[doc = "DiagOut = Vin2"]
    #[inline(always)]
    pub fn vin2(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VIN2)
    }
    #[doc = "DiagOut = Vin3"]
    #[inline(always)]
    pub fn vin3(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::VIN3)
    }
    #[doc = "DiagOut = Isource (10uA)"]
    #[inline(always)]
    pub fn i_source(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::I_SOURCE)
    }
    #[doc = "DiagOut = Isink (10uA)"]
    #[inline(always)]
    pub fn i_sink(self) -> &'a mut W {
        self.variant(DIAG_SEL_A::I_SINK)
    }
}
#[doc = "Field `DIAG_EN` reader - Diagnostic Reference enable (only valid if ENABLED=1) - 0: Diagnostic Reference disabled (powered down resistor ladder and current mirrors, DiagOut = Vssa). - 1: Diagnostic Reference enabled, output signal select according to DIAG_SEL (note also EPASS_MMIO.PASS_CTL.REFBUF_EN must be set)."]
pub type DIAG_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIAG_EN` writer - Diagnostic Reference enable (only valid if ENABLED=1) - 0: Diagnostic Reference disabled (powered down resistor ladder and current mirrors, DiagOut = Vssa). - 1: Diagnostic Reference enabled, output signal select according to DIAG_SEL (note also EPASS_MMIO.PASS_CTL.REFBUF_EN must be set)."]
pub type DIAG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIAG_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Select Diagnostic Reference function"]
    #[inline(always)]
    pub fn diag_sel(&self) -> DIAG_SEL_R {
        DIAG_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Diagnostic Reference enable (only valid if ENABLED=1) - 0: Diagnostic Reference disabled (powered down resistor ladder and current mirrors, DiagOut = Vssa). - 1: Diagnostic Reference enabled, output signal select according to DIAG_SEL (note also EPASS_MMIO.PASS_CTL.REFBUF_EN must be set)."]
    #[inline(always)]
    pub fn diag_en(&self) -> DIAG_EN_R {
        DIAG_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select Diagnostic Reference function"]
    #[inline(always)]
    #[must_use]
    pub fn diag_sel(&mut self) -> DIAG_SEL_W<0> {
        DIAG_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Diagnostic Reference enable (only valid if ENABLED=1) - 0: Diagnostic Reference disabled (powered down resistor ladder and current mirrors, DiagOut = Vssa). - 1: Diagnostic Reference enabled, output signal select according to DIAG_SEL (note also EPASS_MMIO.PASS_CTL.REFBUF_EN must be set)."]
    #[inline(always)]
    #[must_use]
    pub fn diag_en(&mut self) -> DIAG_EN_W<31> {
        DIAG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Diagnostic Reference control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diag_ctl](index.html) module"]
pub struct DIAG_CTL_SPEC;
impl crate::RegisterSpec for DIAG_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diag_ctl::R](R) reader structure"]
impl crate::Readable for DIAG_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diag_ctl::W](W) writer structure"]
impl crate::Writable for DIAG_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIAG_CTL to value 0"]
impl crate::Resettable for DIAG_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
