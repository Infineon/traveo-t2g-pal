#[doc = "Register `STORECTL` reader"]
pub struct R(crate::R<STORECTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORECTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORECTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORECTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORECTL` writer"]
pub struct W(crate::W<STORECTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORECTL_SPEC>;
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
impl From<crate::W<STORECTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORECTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPSAMPLING` reader - Decoded pixels are up-sampled to YUV 4:4:4. If the sub-sampling mode in the encoded JPEG data is NOT YUV 4:2:2, YUV 4:1:1 or YUV 4:2:0, this field is ignored. RWS field, which is activated by START or RESUME command."]
pub type UPSAMPLING_R = crate::BitReader<bool>;
#[doc = "Field `UPSAMPLING` writer - Decoded pixels are up-sampled to YUV 4:4:4. If the sub-sampling mode in the encoded JPEG data is NOT YUV 4:2:2, YUV 4:1:1 or YUV 4:2:0, this field is ignored. RWS field, which is activated by START or RESUME command."]
pub type UPSAMPLING_W<'a, const O: u8> = crate::BitWriter<'a, u32, STORECTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Decoded pixels are up-sampled to YUV 4:4:4. If the sub-sampling mode in the encoded JPEG data is NOT YUV 4:2:2, YUV 4:1:1 or YUV 4:2:0, this field is ignored. RWS field, which is activated by START or RESUME command."]
    #[inline(always)]
    pub fn upsampling(&self) -> UPSAMPLING_R {
        UPSAMPLING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Decoded pixels are up-sampled to YUV 4:4:4. If the sub-sampling mode in the encoded JPEG data is NOT YUV 4:2:2, YUV 4:1:1 or YUV 4:2:0, this field is ignored. RWS field, which is activated by START or RESUME command."]
    #[inline(always)]
    #[must_use]
    pub fn upsampling(&mut self) -> UPSAMPLING_W<0> {
        UPSAMPLING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer format settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [storectl](index.html) module"]
pub struct STORECTL_SPEC;
impl crate::RegisterSpec for STORECTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [storectl::R](R) reader structure"]
impl crate::Readable for STORECTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [storectl::W](W) writer structure"]
impl crate::Writable for STORECTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORECTL to value 0"]
impl crate::Resettable for STORECTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
