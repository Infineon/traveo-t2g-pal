#[doc = "Register `JUMBO_MAX_LENGTH` reader"]
pub struct R(crate::R<JUMBO_MAX_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JUMBO_MAX_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JUMBO_MAX_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JUMBO_MAX_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JUMBO_MAX_LENGTH` writer"]
pub struct W(crate::W<JUMBO_MAX_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JUMBO_MAX_LENGTH_SPEC>;
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
impl From<crate::W<JUMBO_MAX_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JUMBO_MAX_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JUMBO_MAX_LENGTH` reader - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JUMBO_MAX_LENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JUMBO_MAX_LENGTH` writer - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JUMBO_MAX_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, JUMBO_MAX_LENGTH_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbo_max_length(&self) -> JUMBO_MAX_LENGTH_R {
        JUMBO_MAX_LENGTH_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    #[must_use]
    pub fn jumbo_max_length(&mut self) -> JUMBO_MAX_LENGTH_W<0> {
        JUMBO_MAX_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum Jumbo Frame Size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jumbo_max_length](index.html) module"]
pub struct JUMBO_MAX_LENGTH_SPEC;
impl crate::RegisterSpec for JUMBO_MAX_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jumbo_max_length::R](R) reader structure"]
impl crate::Readable for JUMBO_MAX_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jumbo_max_length::W](W) writer structure"]
impl crate::Writable for JUMBO_MAX_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JUMBO_MAX_LENGTH to value 0x0600"]
impl crate::Resettable for JUMBO_MAX_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
