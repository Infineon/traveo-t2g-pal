#[doc = "Register `FRAMEPROPERTIES` reader"]
pub struct R(crate::R<FRAMEPROPERTIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEPROPERTIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEPROPERTIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEPROPERTIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEPROPERTIES` writer"]
pub struct W(crate::W<FRAMEPROPERTIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEPROPERTIES_SPEC>;
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
impl From<crate::W<FRAMEPROPERTIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEPROPERTIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELDID` reader - Field identifier for interlaced video streams (0/1 = even/odd line indices of progressive frame). Status is updated with begin of a new field."]
pub type FIELDID_R = crate::BitReader<bool>;
#[doc = "Field `FIELDID` writer - Field identifier for interlaced video streams (0/1 = even/odd line indices of progressive frame). Status is updated with begin of a new field."]
pub type FIELDID_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMEPROPERTIES_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Field identifier for interlaced video streams (0/1 = even/odd line indices of progressive frame). Status is updated with begin of a new field."]
    #[inline(always)]
    pub fn fieldid(&self) -> FIELDID_R {
        FIELDID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Field identifier for interlaced video streams (0/1 = even/odd line indices of progressive frame). Status is updated with begin of a new field."]
    #[inline(always)]
    #[must_use]
    pub fn fieldid(&mut self) -> FIELDID_W<0> {
        FIELDID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring buffer synchronization.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameproperties](index.html) module"]
pub struct FRAMEPROPERTIES_SPEC;
impl crate::RegisterSpec for FRAMEPROPERTIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameproperties::R](R) reader structure"]
impl crate::Readable for FRAMEPROPERTIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frameproperties::W](W) writer structure"]
impl crate::Writable for FRAMEPROPERTIES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEPROPERTIES to value 0"]
impl crate::Resettable for FRAMEPROPERTIES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
