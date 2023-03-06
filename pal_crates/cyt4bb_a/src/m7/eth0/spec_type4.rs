#[doc = "Register `SPEC_TYPE4` reader"]
pub struct R(crate::R<SPEC_TYPE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEC_TYPE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEC_TYPE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEC_TYPE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPEC_TYPE4` writer"]
pub struct W(crate::W<SPEC_TYPE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPEC_TYPE4_SPEC>;
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
impl From<crate::W<SPEC_TYPE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPEC_TYPE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - Type ID match 4. For use in comparisons with received frames type ID/length field."]
pub type MATCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCH` writer - Type ID match 4. For use in comparisons with received frames type ID/length field."]
pub type MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPEC_TYPE4_SPEC, u16, u16, 16, O>;
#[doc = "Field `ENABLE_COPY` reader - Enable copying of type ID match 4 matched frames."]
pub type ENABLE_COPY_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_COPY` writer - Enable copying of type ID match 4 matched frames."]
pub type ENABLE_COPY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPEC_TYPE4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID match 4. For use in comparisons with received frames type ID/length field."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 4 matched frames."]
    #[inline(always)]
    pub fn enable_copy(&self) -> ENABLE_COPY_R {
        ENABLE_COPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 4. For use in comparisons with received frames type ID/length field."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<0> {
        MATCH_W::new(self)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 4 matched frames."]
    #[inline(always)]
    #[must_use]
    pub fn enable_copy(&mut self) -> ENABLE_COPY_W<31> {
        ENABLE_COPY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Type ID Match 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spec_type4](index.html) module"]
pub struct SPEC_TYPE4_SPEC;
impl crate::RegisterSpec for SPEC_TYPE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spec_type4::R](R) reader structure"]
impl crate::Readable for SPEC_TYPE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spec_type4::W](W) writer structure"]
impl crate::Writable for SPEC_TYPE4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPEC_TYPE4 to value 0"]
impl crate::Resettable for SPEC_TYPE4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
