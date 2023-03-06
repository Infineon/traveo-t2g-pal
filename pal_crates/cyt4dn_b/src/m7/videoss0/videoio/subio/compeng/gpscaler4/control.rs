#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Controls the sequence of horizontal and vertical operation for all operation modes. (format is unsigned integer)"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Controls the sequence of horizontal and vertical operation for all operation modes. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Neutral mode. Pixels are by-passed unchanged, all other settings are ignored."]
    NEUTRAL = 0,
    #[doc = "14: Automatic mode. Behaves like ACTIVE_VER_FIRST when operation_mode=SCALE and scale_mode_h=UPSCALE, like ACTIVE_HOR_FIRST otherwise."]
    ACTIVE_AUTO = 14,
    #[doc = "12: Horizontal is done before vertical operation."]
    ACTIVE_HOR_FIRST = 12,
    #[doc = "13: Vertical is done before horizontal operation."]
    ACTIVE_VER_FIRST = 13,
    #[doc = "8: Only horizontal operation is done."]
    ACTIVE_HOR_ONLY = 8,
    #[doc = "5: Only vertical operation is done."]
    ACTIVE_VER_ONLY = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::NEUTRAL),
            14 => Some(MODE_A::ACTIVE_AUTO),
            12 => Some(MODE_A::ACTIVE_HOR_FIRST),
            13 => Some(MODE_A::ACTIVE_VER_FIRST),
            8 => Some(MODE_A::ACTIVE_HOR_ONLY),
            5 => Some(MODE_A::ACTIVE_VER_ONLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == MODE_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `ACTIVE_AUTO`"]
    #[inline(always)]
    pub fn is_active_auto(&self) -> bool {
        *self == MODE_A::ACTIVE_AUTO
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HOR_FIRST`"]
    #[inline(always)]
    pub fn is_active_hor_first(&self) -> bool {
        *self == MODE_A::ACTIVE_HOR_FIRST
    }
    #[doc = "Checks if the value of the field is `ACTIVE_VER_FIRST`"]
    #[inline(always)]
    pub fn is_active_ver_first(&self) -> bool {
        *self == MODE_A::ACTIVE_VER_FIRST
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HOR_ONLY`"]
    #[inline(always)]
    pub fn is_active_hor_only(&self) -> bool {
        *self == MODE_A::ACTIVE_HOR_ONLY
    }
    #[doc = "Checks if the value of the field is `ACTIVE_VER_ONLY`"]
    #[inline(always)]
    pub fn is_active_ver_only(&self) -> bool {
        *self == MODE_A::ACTIVE_VER_ONLY
    }
}
#[doc = "Field `MODE` writer - Controls the sequence of horizontal and vertical operation for all operation modes. (format is unsigned integer)"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, MODE_A, 4, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Neutral mode. Pixels are by-passed unchanged, all other settings are ignored."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(MODE_A::NEUTRAL)
    }
    #[doc = "Automatic mode. Behaves like ACTIVE_VER_FIRST when operation_mode=SCALE and scale_mode_h=UPSCALE, like ACTIVE_HOR_FIRST otherwise."]
    #[inline(always)]
    pub fn active_auto(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE_AUTO)
    }
    #[doc = "Horizontal is done before vertical operation."]
    #[inline(always)]
    pub fn active_hor_first(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE_HOR_FIRST)
    }
    #[doc = "Vertical is done before horizontal operation."]
    #[inline(always)]
    pub fn active_ver_first(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE_VER_FIRST)
    }
    #[doc = "Only horizontal operation is done."]
    #[inline(always)]
    pub fn active_hor_only(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE_HOR_ONLY)
    }
    #[doc = "Only vertical operation is done."]
    #[inline(always)]
    pub fn active_ver_only(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE_VER_ONLY)
    }
}
#[doc = "Field `OPERATION_MODE` reader - Selects the kind of operation to execute. (format is unsigned integer)"]
pub type OPERATION_MODE_R = crate::BitReader<OPERATION_MODE_A>;
#[doc = "Selects the kind of operation to execute. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERATION_MODE_A {
    #[doc = "0: Scaler is active with the corresponding settings from scaler registers; filter registers are ignored."]
    SCALE = 0,
    #[doc = "1: FIR-Filter is active with the corresponding settings from filter registers; scaler registers are ignored."]
    FIR = 1,
}
impl From<OPERATION_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OPERATION_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl OPERATION_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPERATION_MODE_A {
        match self.bits {
            false => OPERATION_MODE_A::SCALE,
            true => OPERATION_MODE_A::FIR,
        }
    }
    #[doc = "Checks if the value of the field is `SCALE`"]
    #[inline(always)]
    pub fn is_scale(&self) -> bool {
        *self == OPERATION_MODE_A::SCALE
    }
    #[doc = "Checks if the value of the field is `FIR`"]
    #[inline(always)]
    pub fn is_fir(&self) -> bool {
        *self == OPERATION_MODE_A::FIR
    }
}
#[doc = "Field `OPERATION_MODE` writer - Selects the kind of operation to execute. (format is unsigned integer)"]
pub type OPERATION_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, OPERATION_MODE_A, O>;
impl<'a, const O: u8> OPERATION_MODE_W<'a, O> {
    #[doc = "Scaler is active with the corresponding settings from scaler registers; filter registers are ignored."]
    #[inline(always)]
    pub fn scale(self) -> &'a mut W {
        self.variant(OPERATION_MODE_A::SCALE)
    }
    #[doc = "FIR-Filter is active with the corresponding settings from filter registers; scaler registers are ignored."]
    #[inline(always)]
    pub fn fir(self) -> &'a mut W {
        self.variant(OPERATION_MODE_A::FIR)
    }
}
impl R {
    #[doc = "Bits 0:3 - Controls the sequence of horizontal and vertical operation for all operation modes. (format is unsigned integer)"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects the kind of operation to execute. (format is unsigned integer)"]
    #[inline(always)]
    pub fn operation_mode(&self) -> OPERATION_MODE_R {
        OPERATION_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controls the sequence of horizontal and vertical operation for all operation modes. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 8 - Selects the kind of operation to execute. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn operation_mode(&mut self) -> OPERATION_MODE_W<8> {
        OPERATION_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global operation control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
