#[doc = "Register `IMAGEPROPERTY` reader"]
pub struct R(crate::R<IMAGEPROPERTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMAGEPROPERTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMAGEPROPERTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMAGEPROPERTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMAGEPROPERTY` writer"]
pub struct W(crate::W<IMAGEPROPERTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMAGEPROPERTY_SPEC>;
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
impl From<crate::W<IMAGEPROPERTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMAGEPROPERTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSAMPLE` reader - Sub-sampling mode defined in SOF marker segment 000 4:4:4 001 4:2:2 110 4:1:1 010 4:2:0 011 Gray Scale 100 CMYK 111 Invalid"]
pub type SUBSAMPLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDIANSTORE` reader - N/A"]
pub type ENDIANSTORE_R = crate::BitReader<bool>;
#[doc = "Field `ENDIANSTORE` writer - N/A"]
pub type ENDIANSTORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMAGEPROPERTY_SPEC, bool, O>;
#[doc = "Field `ENDIANFETCH` reader - N/A"]
pub type ENDIANFETCH_R = crate::BitReader<bool>;
#[doc = "Field `ENDIANFETCH` writer - N/A"]
pub type ENDIANFETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMAGEPROPERTY_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Sub-sampling mode defined in SOF marker segment 000 4:4:4 001 4:2:2 110 4:1:1 010 4:2:0 011 Gray Scale 100 CMYK 111 Invalid"]
    #[inline(always)]
    pub fn subsample(&self) -> SUBSAMPLE_R {
        SUBSAMPLE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn endianstore(&self) -> ENDIANSTORE_R {
        ENDIANSTORE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn endianfetch(&self) -> ENDIANFETCH_R {
        ENDIANFETCH_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn endianstore(&mut self) -> ENDIANSTORE_W<4> {
        ENDIANSTORE_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn endianfetch(&mut self) -> ENDIANFETCH_W<5> {
        ENDIANFETCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Image property settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imageproperty](index.html) module"]
pub struct IMAGEPROPERTY_SPEC;
impl crate::RegisterSpec for IMAGEPROPERTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imageproperty::R](R) reader structure"]
impl crate::Readable for IMAGEPROPERTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imageproperty::W](W) writer structure"]
impl crate::Writable for IMAGEPROPERTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMAGEPROPERTY to value 0"]
impl crate::Resettable for IMAGEPROPERTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
