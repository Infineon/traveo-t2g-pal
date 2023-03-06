#[doc = "Register `SRC_FADE_CTL` reader"]
pub struct R(crate::R<SRC_FADE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_FADE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_FADE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_FADE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC_FADE_CTL` writer"]
pub struct W(crate::W<SRC_FADE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_FADE_CTL_SPEC>;
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
impl From<crate::W<SRC_FADE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_FADE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Gain code in the range \\[0, 115\\]. See SRC_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'. For 'fade in' HW will gradually increment the value until it reaches '115'. For 'fade out' HW will gradually decrement the value until it reaches '0'. Note: SW is typically required to reprogram the CODE field in between successive 'fade in' operations or in between successive 'fade out' operations."]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - Gain code in the range \\[0, 115\\]. See SRC_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'. For 'fade in' HW will gradually increment the value until it reaches '115'. For 'fade out' HW will gradually decrement the value until it reaches '0'. Note: SW is typically required to reprogram the CODE field in between successive 'fade in' operations or in between successive 'fade out' operations."]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRC_FADE_CTL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Gain code in the range \\[0, 115\\]. See SRC_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'. For 'fade in' HW will gradually increment the value until it reaches '115'. For 'fade out' HW will gradually decrement the value until it reaches '0'. Note: SW is typically required to reprogram the CODE field in between successive 'fade in' operations or in between successive 'fade out' operations."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Gain code in the range \\[0, 115\\]. See SRC_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'. For 'fade in' HW will gradually increment the value until it reaches '115'. For 'fade out' HW will gradually decrement the value until it reaches '0'. Note: SW is typically required to reprogram the CODE field in between successive 'fade in' operations or in between successive 'fade out' operations."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source fade control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_fade_ctl](index.html) module"]
pub struct SRC_FADE_CTL_SPEC;
impl crate::RegisterSpec for SRC_FADE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_fade_ctl::R](R) reader structure"]
impl crate::Readable for SRC_FADE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src_fade_ctl::W](W) writer structure"]
impl crate::Writable for SRC_FADE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC_FADE_CTL to value 0x73"]
impl crate::Resettable for SRC_FADE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x73;
}
