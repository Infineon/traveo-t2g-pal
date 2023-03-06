#[doc = "Register `TSU_NSEC_CMP` reader"]
pub struct R(crate::R<TSU_NSEC_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_NSEC_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_NSEC_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_NSEC_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_NSEC_CMP` writer"]
pub struct W(crate::W<TSU_NSEC_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_NSEC_CMP_SPEC>;
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
impl From<crate::W<TSU_NSEC_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_NSEC_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARISON_NSEC` reader - TSU timer comparison value (ns). Value is compared to the bits\\[45:24\\]
of the TSU timer count value (upper 22 bits of nanosecond value)."]
pub type COMPARISON_NSEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMPARISON_NSEC` writer - TSU timer comparison value (ns). Value is compared to the bits\\[45:24\\]
of the TSU timer count value (upper 22 bits of nanosecond value)."]
pub type COMPARISON_NSEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_NSEC_CMP_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - TSU timer comparison value (ns). Value is compared to the bits\\[45:24\\]
of the TSU timer count value (upper 22 bits of nanosecond value)."]
    #[inline(always)]
    pub fn comparison_nsec(&self) -> COMPARISON_NSEC_R {
        COMPARISON_NSEC_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - TSU timer comparison value (ns). Value is compared to the bits\\[45:24\\]
of the TSU timer count value (upper 22 bits of nanosecond value)."]
    #[inline(always)]
    #[must_use]
    pub fn comparison_nsec(&mut self) -> COMPARISON_NSEC_W<0> {
        COMPARISON_NSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSU timer comparison value nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_nsec_cmp](index.html) module"]
pub struct TSU_NSEC_CMP_SPEC;
impl crate::RegisterSpec for TSU_NSEC_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_nsec_cmp::R](R) reader structure"]
impl crate::Readable for TSU_NSEC_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_nsec_cmp::W](W) writer structure"]
impl crate::Writable for TSU_NSEC_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_NSEC_CMP to value 0"]
impl crate::Resettable for TSU_NSEC_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
