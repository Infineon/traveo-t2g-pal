#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Determines the size of the remap region in the XIP space. Pairs with ADDR. To program, determine the size of the region (range is 1MB to 512MB) then set the bits 'above' that value to 1 and the lower bits to 0. Some examples: 1MB: 0x1FF 2MB: 0x1FE 4MB: 0x1FC ... 256MB: 0x100 512MB: 0x000 If USE_SMIF=11 (interleaving) then MASK=0x1FF is illegal."]
pub type MASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MASK` writer - Determines the size of the remap region in the XIP space. Pairs with ADDR. To program, determine the size of the region (range is 1MB to 512MB) then set the bits 'above' that value to 1 and the lower bits to 0. Some examples: 1MB: 0x1FF 2MB: 0x1FE 4MB: 0x1FC ... 256MB: 0x100 512MB: 0x000 If USE_SMIF=11 (interleaving) then MASK=0x1FF is illegal."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 20:28 - Determines the size of the remap region in the XIP space. Pairs with ADDR. To program, determine the size of the region (range is 1MB to 512MB) then set the bits 'above' that value to 1 and the lower bits to 0. Some examples: 1MB: 0x1FF 2MB: 0x1FE 4MB: 0x1FC ... 256MB: 0x100 512MB: 0x000 If USE_SMIF=11 (interleaving) then MASK=0x1FF is illegal."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:28 - Determines the size of the remap region in the XIP space. Pairs with ADDR. To program, determine the size of the region (range is 1MB to 512MB) then set the bits 'above' that value to 1 and the lower bits to 0. Some examples: 1MB: 0x1FF 2MB: 0x1FE 4MB: 0x1FC ... 256MB: 0x100 512MB: 0x000 If USE_SMIF=11 (interleaving) then MASK=0x1FF is illegal."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<20> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask value to be paired with ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
