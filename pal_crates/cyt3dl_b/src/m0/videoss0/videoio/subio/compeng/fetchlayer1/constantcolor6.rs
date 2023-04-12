#[doc = "Register `CONSTANTCOLOR6` reader"]
pub struct R(crate::R<CONSTANTCOLOR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR6` writer"]
pub struct W(crate::W<CONSTANTCOLOR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR6_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA6` reader - Alpha."]
pub type CONSTANTALPHA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA6` writer - Alpha."]
pub type CONSTANTALPHA6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE6` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE6` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN6` reader - Green and U (chroma)."]
pub type CONSTANTGREEN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN6` writer - Green and U (chroma)."]
pub type CONSTANTGREEN6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED6` reader - Red and Y (luma)."]
pub type CONSTANTRED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED6` writer - Red and Y (luma)."]
pub type CONSTANTRED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha6(&self) -> CONSTANTALPHA6_R {
        CONSTANTALPHA6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue6(&self) -> CONSTANTBLUE6_R {
        CONSTANTBLUE6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen6(&self) -> CONSTANTGREEN6_R {
        CONSTANTGREEN6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred6(&self) -> CONSTANTRED6_R {
        CONSTANTRED6_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha6(&mut self) -> CONSTANTALPHA6_W<0> {
        CONSTANTALPHA6_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue6(&mut self) -> CONSTANTBLUE6_W<8> {
        CONSTANTBLUE6_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen6(&mut self) -> CONSTANTGREEN6_W<16> {
        CONSTANTGREEN6_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred6(&mut self) -> CONSTANTRED6_W<24> {
        CONSTANTRED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor6](index.html) module"]
pub struct CONSTANTCOLOR6_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor6::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor6::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR6 to value 0"]
impl crate::Resettable for CONSTANTCOLOR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
