#[doc = "Register `SMIF0_REMAP` reader"]
pub struct R(crate::R<SMIF0_REMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMIF0_REMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMIF0_REMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMIF0_REMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMIF0_REMAP` writer"]
pub struct W(crate::W<SMIF0_REMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMIF0_REMAP_SPEC>;
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
impl From<crate::W<SMIF0_REMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMIF0_REMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMIF0_REMAP` reader - Determines the base address within the SMIF0 physical space where remaps to SMIF0 are to be located. The size of the remap region is equal to MASK, unless interleaving is true (CTL.USE_SMIF=3) then it is equal to half the size of MASK. SMIF0_REMAP must be situated on a boundardy that equals that resulting granularity."]
pub type SMIF0_REMAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SMIF0_REMAP` writer - Determines the base address within the SMIF0 physical space where remaps to SMIF0 are to be located. The size of the remap region is equal to MASK, unless interleaving is true (CTL.USE_SMIF=3) then it is equal to half the size of MASK. SMIF0_REMAP must be situated on a boundardy that equals that resulting granularity."]
pub type SMIF0_REMAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMIF0_REMAP_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 20:28 - Determines the base address within the SMIF0 physical space where remaps to SMIF0 are to be located. The size of the remap region is equal to MASK, unless interleaving is true (CTL.USE_SMIF=3) then it is equal to half the size of MASK. SMIF0_REMAP must be situated on a boundardy that equals that resulting granularity."]
    #[inline(always)]
    pub fn smif0_remap(&self) -> SMIF0_REMAP_R {
        SMIF0_REMAP_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:28 - Determines the base address within the SMIF0 physical space where remaps to SMIF0 are to be located. The size of the remap region is equal to MASK, unless interleaving is true (CTL.USE_SMIF=3) then it is equal to half the size of MASK. SMIF0_REMAP must be situated on a boundardy that equals that resulting granularity."]
    #[inline(always)]
    #[must_use]
    pub fn smif0_remap(&mut self) -> SMIF0_REMAP_W<20> {
        SMIF0_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base address for remaps into SMIF0 physical memory space\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smif0_remap](index.html) module"]
pub struct SMIF0_REMAP_SPEC;
impl crate::RegisterSpec for SMIF0_REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smif0_remap::R](R) reader structure"]
impl crate::Readable for SMIF0_REMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smif0_remap::W](W) writer structure"]
impl crate::Writable for SMIF0_REMAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMIF0_REMAP to value 0"]
impl crate::Resettable for SMIF0_REMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
