#[doc = "Register `TRANSLATIONVECTORY` reader"]
pub struct R(crate::R<TRANSLATIONVECTORY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSLATIONVECTORY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSLATIONVECTORY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSLATIONVECTORY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSLATIONVECTORY` writer"]
pub struct W(crate::W<TRANSLATIONVECTORY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSLATIONVECTORY_SPEC>;
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
impl From<crate::W<TRANSLATIONVECTORY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSLATIONVECTORY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANSLATIONY` reader - Vertical translation (Y), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the YOffset field of the LayerOffset register for subsequent Blit operation."]
pub type TRANSLATIONY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TRANSLATIONY` writer - Vertical translation (Y), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the YOffset field of the LayerOffset register for subsequent Blit operation."]
pub type TRANSLATIONY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSLATIONVECTORY_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Vertical translation (Y), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the YOffset field of the LayerOffset register for subsequent Blit operation."]
    #[inline(always)]
    pub fn translationy(&self) -> TRANSLATIONY_R {
        TRANSLATIONY_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Vertical translation (Y), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the YOffset field of the LayerOffset register for subsequent Blit operation."]
    #[inline(always)]
    #[must_use]
    pub fn translationy(&mut self) -> TRANSLATIONY_W<0> {
        TRANSLATIONY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical translation of the alpha frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [translationvectory](index.html) module"]
pub struct TRANSLATIONVECTORY_SPEC;
impl crate::RegisterSpec for TRANSLATIONVECTORY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [translationvectory::R](R) reader structure"]
impl crate::Readable for TRANSLATIONVECTORY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [translationvectory::W](W) writer structure"]
impl crate::Writable for TRANSLATIONVECTORY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSLATIONVECTORY to value 0"]
impl crate::Resettable for TRANSLATIONVECTORY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
