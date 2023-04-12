#[doc = "Register `INPUTSETUP` reader"]
pub struct R(crate::R<INPUTSETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTSETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTSETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTSETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTSETUP` writer"]
pub struct W(crate::W<INPUTSETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTSETUP_SPEC>;
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
impl From<crate::W<INPUTSETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTSETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTCAPTUREXOFFSET` reader - X offset of capture area. Note that capture windows must not overlap the last column of the input frame. Do not change during operation."]
pub type INPUTCAPTUREXOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INPUTCAPTUREXOFFSET` writer - X offset of capture area. Note that capture windows must not overlap the last column of the input frame. Do not change during operation."]
pub type INPUTCAPTUREXOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTSETUP_SPEC, u16, u16, 14, O>;
#[doc = "Field `INPUTCAPTUREYOFFSET` reader - Y offset of capture area. Note that capture windows must not overlap the last row of the input frame. Do not change during operation."]
pub type INPUTCAPTUREYOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INPUTCAPTUREYOFFSET` writer - Y offset of capture area. Note that capture windows must not overlap the last row of the input frame. Do not change during operation."]
pub type INPUTCAPTUREYOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTSETUP_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - X offset of capture area. Note that capture windows must not overlap the last column of the input frame. Do not change during operation."]
    #[inline(always)]
    pub fn inputcapturexoffset(&self) -> INPUTCAPTUREXOFFSET_R {
        INPUTCAPTUREXOFFSET_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Y offset of capture area. Note that capture windows must not overlap the last row of the input frame. Do not change during operation."]
    #[inline(always)]
    pub fn inputcaptureyoffset(&self) -> INPUTCAPTUREYOFFSET_R {
        INPUTCAPTUREYOFFSET_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - X offset of capture area. Note that capture windows must not overlap the last column of the input frame. Do not change during operation."]
    #[inline(always)]
    #[must_use]
    pub fn inputcapturexoffset(&mut self) -> INPUTCAPTUREXOFFSET_W<0> {
        INPUTCAPTUREXOFFSET_W::new(self)
    }
    #[doc = "Bits 16:29 - Y offset of capture area. Note that capture windows must not overlap the last row of the input frame. Do not change during operation."]
    #[inline(always)]
    #[must_use]
    pub fn inputcaptureyoffset(&mut self) -> INPUTCAPTUREYOFFSET_W<16> {
        INPUTCAPTUREYOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dump window position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputsetup](index.html) module"]
pub struct INPUTSETUP_SPEC;
impl crate::RegisterSpec for INPUTSETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputsetup::R](R) reader structure"]
impl crate::Readable for INPUTSETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputsetup::W](W) writer structure"]
impl crate::Writable for INPUTSETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTSETUP to value 0"]
impl crate::Resettable for INPUTSETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
