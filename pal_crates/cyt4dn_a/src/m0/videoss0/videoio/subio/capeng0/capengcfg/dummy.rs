#[doc = "Register `DUMMY` reader"]
pub struct R(crate::R<DUMMY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUMMY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUMMY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUMMY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUMMY` writer"]
pub struct W(crate::W<DUMMY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUMMY_SPEC>;
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
impl From<crate::W<DUMMY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUMMY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUMMY` reader - Do not use it. This register is set for avoiding sub_dec tpl error."]
pub type DUMMY_R = crate::BitReader<bool>;
#[doc = "Field `DUMMY` writer - Do not use it. This register is set for avoiding sub_dec tpl error."]
pub type DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DUMMY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Do not use it. This register is set for avoiding sub_dec tpl error."]
    #[inline(always)]
    pub fn dummy(&self) -> DUMMY_R {
        DUMMY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Do not use it. This register is set for avoiding sub_dec tpl error."]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DUMMY_W<31> {
        DUMMY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DUMMY Register for .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dummy](index.html) module"]
pub struct DUMMY_SPEC;
impl crate::RegisterSpec for DUMMY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dummy::R](R) reader structure"]
impl crate::Readable for DUMMY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dummy::W](W) writer structure"]
impl crate::Writable for DUMMY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUMMY to value 0"]
impl crate::Resettable for DUMMY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
