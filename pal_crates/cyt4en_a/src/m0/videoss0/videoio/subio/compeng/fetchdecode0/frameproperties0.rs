#[doc = "Register `FRAMEPROPERTIES0` reader"]
pub struct R(crate::R<FRAMEPROPERTIES0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEPROPERTIES0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEPROPERTIES0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEPROPERTIES0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEPROPERTIES0` writer"]
pub struct W(crate::W<FRAMEPROPERTIES0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEPROPERTIES0_SPEC>;
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
impl From<crate::W<FRAMEPROPERTIES0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEPROPERTIES0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELDID0` reader - Field identifier that is generated for subsequent units (0 = progressive frame or interlaced field with even line indices, 1 = odd field)."]
pub type FIELDID0_R = crate::BitReader<bool>;
#[doc = "Field `FIELDID0` writer - Field identifier that is generated for subsequent units (0 = progressive frame or interlaced field with even line indices, 1 = odd field)."]
pub type FIELDID0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMEPROPERTIES0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Field identifier that is generated for subsequent units (0 = progressive frame or interlaced field with even line indices, 1 = odd field)."]
    #[inline(always)]
    pub fn fieldid0(&self) -> FIELDID0_R {
        FIELDID0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Field identifier that is generated for subsequent units (0 = progressive frame or interlaced field with even line indices, 1 = odd field)."]
    #[inline(always)]
    #[must_use]
    pub fn fieldid0(&mut self) -> FIELDID0_W<0> {
        FIELDID0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame property setup for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameproperties0](index.html) module"]
pub struct FRAMEPROPERTIES0_SPEC;
impl crate::RegisterSpec for FRAMEPROPERTIES0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameproperties0::R](R) reader structure"]
impl crate::Readable for FRAMEPROPERTIES0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frameproperties0::W](W) writer structure"]
impl crate::Writable for FRAMEPROPERTIES0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEPROPERTIES0 to value 0"]
impl crate::Resettable for FRAMEPROPERTIES0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
