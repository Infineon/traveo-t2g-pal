#[doc = "Register `CONSTANTCOLOR0` reader"]
pub struct R(crate::R<CONSTANTCOLOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR0` writer"]
pub struct W(crate::W<CONSTANTCOLOR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR0_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA0` reader - Alpha."]
pub type CONSTANTALPHA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA0` writer - Alpha."]
pub type CONSTANTALPHA0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE0` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE0` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN0` reader - Green and U (chroma)."]
pub type CONSTANTGREEN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN0` writer - Green and U (chroma)."]
pub type CONSTANTGREEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED0` reader - Red and Y (luma)."]
pub type CONSTANTRED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED0` writer - Red and Y (luma)."]
pub type CONSTANTRED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha0(&self) -> CONSTANTALPHA0_R {
        CONSTANTALPHA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue0(&self) -> CONSTANTBLUE0_R {
        CONSTANTBLUE0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen0(&self) -> CONSTANTGREEN0_R {
        CONSTANTGREEN0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred0(&self) -> CONSTANTRED0_R {
        CONSTANTRED0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha0(&mut self) -> CONSTANTALPHA0_W<0> {
        CONSTANTALPHA0_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue0(&mut self) -> CONSTANTBLUE0_W<8> {
        CONSTANTBLUE0_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen0(&mut self) -> CONSTANTGREEN0_W<16> {
        CONSTANTGREEN0_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred0(&mut self) -> CONSTANTRED0_W<24> {
        CONSTANTRED0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor0](index.html) module"]
pub struct CONSTANTCOLOR0_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor0::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor0::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR0 to value 0"]
impl crate::Resettable for CONSTANTCOLOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
