#[doc = "Register `CONSTANTCOLOR1` reader"]
pub struct R(crate::R<CONSTANTCOLOR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR1` writer"]
pub struct W(crate::W<CONSTANTCOLOR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR1_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA1` reader - Alpha."]
pub type CONSTANTALPHA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA1` writer - Alpha."]
pub type CONSTANTALPHA1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE1` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE1` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN1` reader - Green and U (chroma)."]
pub type CONSTANTGREEN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN1` writer - Green and U (chroma)."]
pub type CONSTANTGREEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED1` reader - Red and Y (luma)."]
pub type CONSTANTRED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED1` writer - Red and Y (luma)."]
pub type CONSTANTRED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha1(&self) -> CONSTANTALPHA1_R {
        CONSTANTALPHA1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue1(&self) -> CONSTANTBLUE1_R {
        CONSTANTBLUE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen1(&self) -> CONSTANTGREEN1_R {
        CONSTANTGREEN1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred1(&self) -> CONSTANTRED1_R {
        CONSTANTRED1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha1(&mut self) -> CONSTANTALPHA1_W<0> {
        CONSTANTALPHA1_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue1(&mut self) -> CONSTANTBLUE1_W<8> {
        CONSTANTBLUE1_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen1(&mut self) -> CONSTANTGREEN1_W<16> {
        CONSTANTGREEN1_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred1(&mut self) -> CONSTANTRED1_W<24> {
        CONSTANTRED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor1](index.html) module"]
pub struct CONSTANTCOLOR1_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor1::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor1::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR1 to value 0"]
impl crate::Resettable for CONSTANTCOLOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
