#[doc = "Register `CONSTANTCOLOR5` reader"]
pub struct R(crate::R<CONSTANTCOLOR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR5` writer"]
pub struct W(crate::W<CONSTANTCOLOR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR5_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA5` reader - Alpha."]
pub type CONSTANTALPHA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA5` writer - Alpha."]
pub type CONSTANTALPHA5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE5` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE5` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN5` reader - Green and U (chroma)."]
pub type CONSTANTGREEN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN5` writer - Green and U (chroma)."]
pub type CONSTANTGREEN5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED5` reader - Red and Y (luma)."]
pub type CONSTANTRED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED5` writer - Red and Y (luma)."]
pub type CONSTANTRED5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha5(&self) -> CONSTANTALPHA5_R {
        CONSTANTALPHA5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue5(&self) -> CONSTANTBLUE5_R {
        CONSTANTBLUE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen5(&self) -> CONSTANTGREEN5_R {
        CONSTANTGREEN5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred5(&self) -> CONSTANTRED5_R {
        CONSTANTRED5_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha5(&mut self) -> CONSTANTALPHA5_W<0> {
        CONSTANTALPHA5_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue5(&mut self) -> CONSTANTBLUE5_W<8> {
        CONSTANTBLUE5_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen5(&mut self) -> CONSTANTGREEN5_W<16> {
        CONSTANTGREEN5_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred5(&mut self) -> CONSTANTRED5_W<24> {
        CONSTANTRED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor5](index.html) module"]
pub struct CONSTANTCOLOR5_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor5::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor5::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR5 to value 0"]
impl crate::Resettable for CONSTANTCOLOR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
