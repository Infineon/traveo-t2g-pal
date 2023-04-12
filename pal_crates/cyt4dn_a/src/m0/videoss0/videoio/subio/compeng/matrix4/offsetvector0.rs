#[doc = "Register `OFFSETVECTOR0` reader"]
pub struct R(crate::R<OFFSETVECTOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFSETVECTOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFSETVECTOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFSETVECTOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFSETVECTOR0` writer"]
pub struct W(crate::W<OFFSETVECTOR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFSETVECTOR0_SPEC>;
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
impl From<crate::W<OFFSETVECTOR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFSETVECTOR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C1` reader - Red output offset."]
pub type C1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C1` writer - Red output offset."]
pub type C1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFFSETVECTOR0_SPEC, u16, u16, 13, O>;
#[doc = "Field `C2` reader - Green output offset."]
pub type C2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C2` writer - Green output offset."]
pub type C2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFFSETVECTOR0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Red output offset."]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Green output offset."]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Red output offset."]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1_W<0> {
        C1_W::new(self)
    }
    #[doc = "Bits 16:28 - Green output offset."]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<16> {
        C2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset vectors for red and green output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsetvector0](index.html) module"]
pub struct OFFSETVECTOR0_SPEC;
impl crate::RegisterSpec for OFFSETVECTOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [offsetvector0::R](R) reader structure"]
impl crate::Readable for OFFSETVECTOR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offsetvector0::W](W) writer structure"]
impl crate::Writable for OFFSETVECTOR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFFSETVECTOR0 to value 0"]
impl crate::Resettable for OFFSETVECTOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
