#[doc = "Register `CONSTANTCOLOR3` reader"]
pub struct R(crate::R<CONSTANTCOLOR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR3` writer"]
pub struct W(crate::W<CONSTANTCOLOR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR3_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA3` reader - Alpha."]
pub type CONSTANTALPHA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA3` writer - Alpha."]
pub type CONSTANTALPHA3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE3` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE3` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN3` reader - Green and U (chroma)."]
pub type CONSTANTGREEN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN3` writer - Green and U (chroma)."]
pub type CONSTANTGREEN3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED3` reader - Red and Y (luma)."]
pub type CONSTANTRED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED3` writer - Red and Y (luma)."]
pub type CONSTANTRED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha3(&self) -> CONSTANTALPHA3_R {
        CONSTANTALPHA3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue3(&self) -> CONSTANTBLUE3_R {
        CONSTANTBLUE3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen3(&self) -> CONSTANTGREEN3_R {
        CONSTANTGREEN3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred3(&self) -> CONSTANTRED3_R {
        CONSTANTRED3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha3(&mut self) -> CONSTANTALPHA3_W<0> {
        CONSTANTALPHA3_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue3(&mut self) -> CONSTANTBLUE3_W<8> {
        CONSTANTBLUE3_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen3(&mut self) -> CONSTANTGREEN3_W<16> {
        CONSTANTGREEN3_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred3(&mut self) -> CONSTANTRED3_W<24> {
        CONSTANTRED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor3](index.html) module"]
pub struct CONSTANTCOLOR3_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor3::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor3::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR3 to value 0"]
impl crate::Resettable for CONSTANTCOLOR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
