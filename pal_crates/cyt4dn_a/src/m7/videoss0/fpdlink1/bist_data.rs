#[doc = "Register `BIST_DATA` reader"]
pub struct R(crate::R<BIST_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_DATA` writer"]
pub struct W(crate::W<BIST_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_DATA_SPEC>;
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
impl From<crate::W<BIST_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_PATTERN` reader - This is the programmable test pattern used by BIST pattern generator and pattern matcher. The TEST_PATTERN bus value will be used as the payload data during the test. It should be set based on the expected BIST pattern to be sent or received."]
pub type TEST_PATTERN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TEST_PATTERN` writer - This is the programmable test pattern used by BIST pattern generator and pattern matcher. The TEST_PATTERN bus value will be used as the payload data during the test. It should be set based on the expected BIST pattern to be sent or received."]
pub type TEST_PATTERN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BIST_DATA_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - This is the programmable test pattern used by BIST pattern generator and pattern matcher. The TEST_PATTERN bus value will be used as the payload data during the test. It should be set based on the expected BIST pattern to be sent or received."]
    #[inline(always)]
    pub fn test_pattern(&self) -> TEST_PATTERN_R {
        TEST_PATTERN_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - This is the programmable test pattern used by BIST pattern generator and pattern matcher. The TEST_PATTERN bus value will be used as the payload data during the test. It should be set based on the expected BIST pattern to be sent or received."]
    #[inline(always)]
    #[must_use]
    pub fn test_pattern(&mut self) -> TEST_PATTERN_W<0> {
        TEST_PATTERN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data](index.html) module"]
pub struct BIST_DATA_SPEC;
impl crate::RegisterSpec for BIST_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_data::R](R) reader structure"]
impl crate::Readable for BIST_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_data::W](W) writer structure"]
impl crate::Writable for BIST_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIST_DATA to value 0"]
impl crate::Resettable for BIST_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
