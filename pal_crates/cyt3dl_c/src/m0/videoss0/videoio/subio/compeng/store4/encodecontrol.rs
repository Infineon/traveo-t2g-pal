#[doc = "Register `ENCODECONTROL` reader"]
pub struct R(crate::R<ENCODECONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENCODECONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENCODECONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENCODECONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENCODECONTROL` writer"]
pub struct W(crate::W<ENCODECONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENCODECONTROL_SPEC>;
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
impl From<crate::W<ENCODECONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENCODECONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPRESSIONMODE` reader - Algorithm to use for compression."]
pub type COMPRESSIONMODE_R = crate::BitReader<COMPRESSIONMODE_A>;
#[doc = "Algorithm to use for compression.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPRESSIONMODE_A {
    #[doc = "0: Run-Length Adaptive Dithering (lossy compression)."]
    RLAD = 0,
    #[doc = "1: Run-Length Adaptive Dithering (lossy compression; uniform package size)."]
    RLAD_UNIFORM = 1,
}
impl From<COMPRESSIONMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPRESSIONMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPRESSIONMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPRESSIONMODE_A {
        match self.bits {
            false => COMPRESSIONMODE_A::RLAD,
            true => COMPRESSIONMODE_A::RLAD_UNIFORM,
        }
    }
    #[doc = "Checks if the value of the field is `RLAD`"]
    #[inline(always)]
    pub fn is_rlad(&self) -> bool {
        *self == COMPRESSIONMODE_A::RLAD
    }
    #[doc = "Checks if the value of the field is `RLAD_UNIFORM`"]
    #[inline(always)]
    pub fn is_rlad_uniform(&self) -> bool {
        *self == COMPRESSIONMODE_A::RLAD_UNIFORM
    }
}
#[doc = "Field `COMPRESSIONMODE` writer - Algorithm to use for compression."]
pub type COMPRESSIONMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENCODECONTROL_SPEC, COMPRESSIONMODE_A, O>;
impl<'a, const O: u8> COMPRESSIONMODE_W<'a, O> {
    #[doc = "Run-Length Adaptive Dithering (lossy compression)."]
    #[inline(always)]
    pub fn rlad(self) -> &'a mut W {
        self.variant(COMPRESSIONMODE_A::RLAD)
    }
    #[doc = "Run-Length Adaptive Dithering (lossy compression; uniform package size)."]
    #[inline(always)]
    pub fn rlad_uniform(self) -> &'a mut W {
        self.variant(COMPRESSIONMODE_A::RLAD_UNIFORM)
    }
}
#[doc = "Field `RLADCOMPBITSRED` reader - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel."]
pub type RLADCOMPBITSRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSRED` writer - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel."]
pub type RLADCOMPBITSRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENCODECONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLADCOMPBITSGREEN` reader - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel."]
pub type RLADCOMPBITSGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSGREEN` writer - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel."]
pub type RLADCOMPBITSGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENCODECONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLADCOMPBITSBLUE` reader - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel."]
pub type RLADCOMPBITSBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSBLUE` writer - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel."]
pub type RLADCOMPBITSBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENCODECONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLADCOMPBITSALPHA` reader - Maximum for average number of bits per compressed pixel for Alpha channel."]
pub type RLADCOMPBITSALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSALPHA` writer - Maximum for average number of bits per compressed pixel for Alpha channel."]
pub type RLADCOMPBITSALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENCODECONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Algorithm to use for compression."]
    #[inline(always)]
    pub fn compressionmode(&self) -> COMPRESSIONMODE_R {
        COMPRESSIONMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:19 - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel."]
    #[inline(always)]
    pub fn rladcompbitsred(&self) -> RLADCOMPBITSRED_R {
        RLADCOMPBITSRED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel."]
    #[inline(always)]
    pub fn rladcompbitsgreen(&self) -> RLADCOMPBITSGREEN_R {
        RLADCOMPBITSGREEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel."]
    #[inline(always)]
    pub fn rladcompbitsblue(&self) -> RLADCOMPBITSBLUE_R {
        RLADCOMPBITSBLUE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Maximum for average number of bits per compressed pixel for Alpha channel."]
    #[inline(always)]
    pub fn rladcompbitsalpha(&self) -> RLADCOMPBITSALPHA_R {
        RLADCOMPBITSALPHA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Algorithm to use for compression."]
    #[inline(always)]
    #[must_use]
    pub fn compressionmode(&mut self) -> COMPRESSIONMODE_W<0> {
        COMPRESSIONMODE_W::new(self)
    }
    #[doc = "Bits 16:19 - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsred(&mut self) -> RLADCOMPBITSRED_W<16> {
        RLADCOMPBITSRED_W::new(self)
    }
    #[doc = "Bits 20:23 - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsgreen(&mut self) -> RLADCOMPBITSGREEN_W<20> {
        RLADCOMPBITSGREEN_W::new(self)
    }
    #[doc = "Bits 24:27 - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsblue(&mut self) -> RLADCOMPBITSBLUE_W<24> {
        RLADCOMPBITSBLUE_W::new(self)
    }
    #[doc = "Bits 28:31 - Maximum for average number of bits per compressed pixel for Alpha channel."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsalpha(&mut self) -> RLADCOMPBITSALPHA_W<28> {
        RLADCOMPBITSALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control options for RLAD compression.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [encodecontrol](index.html) module"]
pub struct ENCODECONTROL_SPEC;
impl crate::RegisterSpec for ENCODECONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [encodecontrol::R](R) reader structure"]
impl crate::Readable for ENCODECONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [encodecontrol::W](W) writer structure"]
impl crate::Writable for ENCODECONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENCODECONTROL to value 0x8888_0001"]
impl crate::Resettable for ENCODECONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8888_0001;
}
