#[doc = "Register `EXTDST5_SAFETYMASK` reader"]
pub struct R(crate::R<EXTDST5_SAFETYMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST5_SAFETYMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST5_SAFETYMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST5_SAFETYMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTDST5_SAFETYMASK` writer"]
pub struct W(crate::W<EXTDST5_SAFETYMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST5_SAFETYMASK_SPEC>;
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
impl From<crate::W<EXTDST5_SAFETYMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST5_SAFETYMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST5_SAFETYMASK` reader - Each bit in this field describes whether the corresponding processing unit is allowed to be configured in a path leading to this endpoint (extdst5). 1 = allowed, 0 = prohibited."]
pub type EXTDST5_SAFETYMASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXTDST5_SAFETYMASK` writer - Each bit in this field describes whether the corresponding processing unit is allowed to be configured in a path leading to this endpoint (extdst5). 1 = allowed, 0 = prohibited."]
pub type EXTDST5_SAFETYMASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTDST5_SAFETYMASK_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Each bit in this field describes whether the corresponding processing unit is allowed to be configured in a path leading to this endpoint (extdst5). 1 = allowed, 0 = prohibited."]
    #[inline(always)]
    pub fn extdst5_safetymask(&self) -> EXTDST5_SAFETYMASK_R {
        EXTDST5_SAFETYMASK_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Each bit in this field describes whether the corresponding processing unit is allowed to be configured in a path leading to this endpoint (extdst5). 1 = allowed, 0 = prohibited."]
    #[inline(always)]
    #[must_use]
    pub fn extdst5_safetymask(&mut self) -> EXTDST5_SAFETYMASK_W<0> {
        EXTDST5_SAFETYMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Safety mask for extdst5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst5_safetymask](index.html) module"]
pub struct EXTDST5_SAFETYMASK_SPEC;
impl crate::RegisterSpec for EXTDST5_SAFETYMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst5_safetymask::R](R) reader structure"]
impl crate::Readable for EXTDST5_SAFETYMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extdst5_safetymask::W](W) writer structure"]
impl crate::Writable for EXTDST5_SAFETYMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST5_SAFETYMASK to value 0x001f_ffff"]
impl crate::Resettable for EXTDST5_SAFETYMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x001f_ffff;
}
