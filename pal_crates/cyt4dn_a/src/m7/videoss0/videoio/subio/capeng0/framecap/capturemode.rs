#[doc = "Register `CAPTUREMODE` reader"]
pub struct R(crate::R<CAPTUREMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTUREMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTUREMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTUREMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPTUREMODE` writer"]
pub struct W(crate::W<CAPTUREMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPTUREMODE_SPEC>;
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
impl From<crate::W<CAPTUREMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPTUREMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTUREMODE` reader - Selects a capture mode. Do not change during operation of FrameCap unit."]
pub type CAPTUREMODE_R = crate::FieldReader<u8, CAPTUREMODE_A>;
#[doc = "Selects a capture mode. Do not change during operation of FrameCap unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTUREMODE_A {
    #[doc = "0: Capture of 32-bit parallel pixel input data with dedicated frame synchronization signals"]
    ENHSVS_32BIT = 0,
    #[doc = "1: Capture of ITU656 10 bit data"]
    ITU656_10BIT = 1,
    #[doc = "2: Capture of ITU656 8 bit data"]
    ITU656_8BIT = 2,
    #[doc = "3: Capture of MIPI_CSI2 data with hsync at the beginning of lines"]
    MIPICSI2 = 3,
}
impl From<CAPTUREMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTUREMODE_A) -> Self {
        variant as _
    }
}
impl CAPTUREMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTUREMODE_A {
        match self.bits {
            0 => CAPTUREMODE_A::ENHSVS_32BIT,
            1 => CAPTUREMODE_A::ITU656_10BIT,
            2 => CAPTUREMODE_A::ITU656_8BIT,
            3 => CAPTUREMODE_A::MIPICSI2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENHSVS_32BIT`"]
    #[inline(always)]
    pub fn is_enhsvs_32bit(&self) -> bool {
        *self == CAPTUREMODE_A::ENHSVS_32BIT
    }
    #[doc = "Checks if the value of the field is `ITU656_10BIT`"]
    #[inline(always)]
    pub fn is_itu656_10bit(&self) -> bool {
        *self == CAPTUREMODE_A::ITU656_10BIT
    }
    #[doc = "Checks if the value of the field is `ITU656_8BIT`"]
    #[inline(always)]
    pub fn is_itu656_8bit(&self) -> bool {
        *self == CAPTUREMODE_A::ITU656_8BIT
    }
    #[doc = "Checks if the value of the field is `MIPICSI2`"]
    #[inline(always)]
    pub fn is_mipicsi2(&self) -> bool {
        *self == CAPTUREMODE_A::MIPICSI2
    }
}
#[doc = "Field `CAPTUREMODE` writer - Selects a capture mode. Do not change during operation of FrameCap unit."]
pub type CAPTUREMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAPTUREMODE_SPEC, u8, CAPTUREMODE_A, 2, O>;
impl<'a, const O: u8> CAPTUREMODE_W<'a, O> {
    #[doc = "Capture of 32-bit parallel pixel input data with dedicated frame synchronization signals"]
    #[inline(always)]
    pub fn enhsvs_32bit(self) -> &'a mut W {
        self.variant(CAPTUREMODE_A::ENHSVS_32BIT)
    }
    #[doc = "Capture of ITU656 10 bit data"]
    #[inline(always)]
    pub fn itu656_10bit(self) -> &'a mut W {
        self.variant(CAPTUREMODE_A::ITU656_10BIT)
    }
    #[doc = "Capture of ITU656 8 bit data"]
    #[inline(always)]
    pub fn itu656_8bit(self) -> &'a mut W {
        self.variant(CAPTUREMODE_A::ITU656_8BIT)
    }
    #[doc = "Capture of MIPI_CSI2 data with hsync at the beginning of lines"]
    #[inline(always)]
    pub fn mipicsi2(self) -> &'a mut W {
        self.variant(CAPTUREMODE_A::MIPICSI2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects a capture mode. Do not change during operation of FrameCap unit."]
    #[inline(always)]
    pub fn capturemode(&self) -> CAPTUREMODE_R {
        CAPTUREMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects a capture mode. Do not change during operation of FrameCap unit."]
    #[inline(always)]
    #[must_use]
    pub fn capturemode(&mut self) -> CAPTUREMODE_W<0> {
        CAPTUREMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture mode configuration.FrameCap input mode configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturemode](index.html) module"]
pub struct CAPTUREMODE_SPEC;
impl crate::RegisterSpec for CAPTUREMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capturemode::R](R) reader structure"]
impl crate::Readable for CAPTUREMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capturemode::W](W) writer structure"]
impl crate::Writable for CAPTUREMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPTUREMODE to value 0"]
impl crate::Resettable for CAPTUREMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
