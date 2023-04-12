#[doc = "Register `DECODECONTROL` reader"]
pub struct R(crate::R<DECODECONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODECONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODECONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODECONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECODECONTROL` writer"]
pub struct W(crate::W<DECODECONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECODECONTROL_SPEC>;
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
impl From<crate::W<DECODECONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECODECONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPRESSIONMODE` reader - Algorithm that the encoder used for compression."]
pub type COMPRESSIONMODE_R = crate::FieldReader<u8, COMPRESSIONMODE_A>;
#[doc = "Algorithm that the encoder used for compression.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMPRESSIONMODE_A {
    #[doc = "0: Run-Length Adaptive Dithering (lossy compression)."]
    RLAD = 0,
    #[doc = "1: Run-Length Adaptive Dithering (lossy compression; uniform package size)."]
    RLAD_UNIFORM = 1,
    #[doc = "2: Run-Length Adaptive (lossless compression)."]
    RLA = 2,
    #[doc = "3: Standard Run-Length. In contrary to other modes is in big endian format."]
    RL = 3,
}
impl From<COMPRESSIONMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPRESSIONMODE_A) -> Self {
        variant as _
    }
}
impl COMPRESSIONMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPRESSIONMODE_A {
        match self.bits {
            0 => COMPRESSIONMODE_A::RLAD,
            1 => COMPRESSIONMODE_A::RLAD_UNIFORM,
            2 => COMPRESSIONMODE_A::RLA,
            3 => COMPRESSIONMODE_A::RL,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `RLA`"]
    #[inline(always)]
    pub fn is_rla(&self) -> bool {
        *self == COMPRESSIONMODE_A::RLA
    }
    #[doc = "Checks if the value of the field is `RL`"]
    #[inline(always)]
    pub fn is_rl(&self) -> bool {
        *self == COMPRESSIONMODE_A::RL
    }
}
#[doc = "Field `COMPRESSIONMODE` writer - Algorithm that the encoder used for compression."]
pub type COMPRESSIONMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DECODECONTROL_SPEC, u8, COMPRESSIONMODE_A, 2, O>;
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
    #[doc = "Run-Length Adaptive (lossless compression)."]
    #[inline(always)]
    pub fn rla(self) -> &'a mut W {
        self.variant(COMPRESSIONMODE_A::RLA)
    }
    #[doc = "Standard Run-Length. In contrary to other modes is in big endian format."]
    #[inline(always)]
    pub fn rl(self) -> &'a mut W {
        self.variant(COMPRESSIONMODE_A::RL)
    }
}
#[doc = "Field `RLADCOMPBITSRED` reader - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSRED` writer - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECODECONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLADCOMPBITSGREEN` reader - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSGREEN` writer - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECODECONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLADCOMPBITSBLUE` reader - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSBLUE` writer - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECODECONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLADCOMPBITSALPHA` reader - Maximum for average number of bits per compressed pixel for Alpha channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLADCOMPBITSALPHA` writer - Maximum for average number of bits per compressed pixel for Alpha channel. This must match the corresponding encoder setting."]
pub type RLADCOMPBITSALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECODECONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Algorithm that the encoder used for compression."]
    #[inline(always)]
    pub fn compressionmode(&self) -> COMPRESSIONMODE_R {
        COMPRESSIONMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    pub fn rladcompbitsred(&self) -> RLADCOMPBITSRED_R {
        RLADCOMPBITSRED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    pub fn rladcompbitsgreen(&self) -> RLADCOMPBITSGREEN_R {
        RLADCOMPBITSGREEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    pub fn rladcompbitsblue(&self) -> RLADCOMPBITSBLUE_R {
        RLADCOMPBITSBLUE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Maximum for average number of bits per compressed pixel for Alpha channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    pub fn rladcompbitsalpha(&self) -> RLADCOMPBITSALPHA_R {
        RLADCOMPBITSALPHA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Algorithm that the encoder used for compression."]
    #[inline(always)]
    #[must_use]
    pub fn compressionmode(&mut self) -> COMPRESSIONMODE_W<0> {
        COMPRESSIONMODE_W::new(self)
    }
    #[doc = "Bits 16:19 - Maximum for average number of bits per compressed pixel for Red or Y (luma) channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsred(&mut self) -> RLADCOMPBITSRED_W<16> {
        RLADCOMPBITSRED_W::new(self)
    }
    #[doc = "Bits 20:23 - Maximum for average number of bits per compressed pixel for Green or U (chroma) channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsgreen(&mut self) -> RLADCOMPBITSGREEN_W<20> {
        RLADCOMPBITSGREEN_W::new(self)
    }
    #[doc = "Bits 24:27 - Maximum for average number of bits per compressed pixel for Blue or V (chroma) channel. This must match the corresponding encoder setting."]
    #[inline(always)]
    #[must_use]
    pub fn rladcompbitsblue(&mut self) -> RLADCOMPBITSBLUE_W<24> {
        RLADCOMPBITSBLUE_W::new(self)
    }
    #[doc = "Bits 28:31 - Maximum for average number of bits per compressed pixel for Alpha channel. This must match the corresponding encoder setting."]
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
#[doc = "Control options for RLAD decompression.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decodecontrol](index.html) module"]
pub struct DECODECONTROL_SPEC;
impl crate::RegisterSpec for DECODECONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decodecontrol::R](R) reader structure"]
impl crate::Readable for DECODECONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decodecontrol::W](W) writer structure"]
impl crate::Writable for DECODECONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECODECONTROL to value 0x8888_0001"]
impl crate::Resettable for DECODECONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8888_0001;
}
