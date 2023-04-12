#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USE_SMIF` reader - Determines whether the remap region is active or not, and if active whether it is a simple remap or interleaving remap: 0: remap region is inactive (all other registers associated with this remap region are ignored) 1: remap accesses to SMIF0 2: remap accesses to SMIF1 3: remap accesses to SMIF0 and SMIF1 on an interleaving basis This and all other remap region registers are relevant only if the bridge's CTL.ENABLED=1."]
pub type USE_SMIF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USE_SMIF` writer - Determines whether the remap region is active or not, and if active whether it is a simple remap or interleaving remap: 0: remap region is inactive (all other registers associated with this remap region are ignored) 1: remap accesses to SMIF0 2: remap accesses to SMIF1 3: remap accesses to SMIF0 and SMIF1 on an interleaving basis This and all other remap region registers are relevant only if the bridge's CTL.ENABLED=1."]
pub type USE_SMIF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INTLV_STEP_SIZE` reader - Interleaving step size (only relevant when USE_SMIF=3): 0: 8 bytes 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type INTLV_STEP_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTLV_STEP_SIZE` writer - Interleaving step size (only relevant when USE_SMIF=3): 0: 8 bytes 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type INTLV_STEP_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMIF_SPACE` reader - Determines which SMIF XIP space the ADDR register applies to: 0: SMIF0 1: SMIF1"]
pub type SMIF_SPACE_R = crate::BitReader<bool>;
#[doc = "Field `SMIF_SPACE` writer - Determines which SMIF XIP space the ADDR register applies to: 0: SMIF0 1: SMIF1"]
pub type SMIF_SPACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Determines whether the remap region is active or not, and if active whether it is a simple remap or interleaving remap: 0: remap region is inactive (all other registers associated with this remap region are ignored) 1: remap accesses to SMIF0 2: remap accesses to SMIF1 3: remap accesses to SMIF0 and SMIF1 on an interleaving basis This and all other remap region registers are relevant only if the bridge's CTL.ENABLED=1."]
    #[inline(always)]
    pub fn use_smif(&self) -> USE_SMIF_R {
        USE_SMIF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:18 - Interleaving step size (only relevant when USE_SMIF=3): 0: 8 bytes 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn intlv_step_size(&self) -> INTLV_STEP_SIZE_R {
        INTLV_STEP_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 31 - Determines which SMIF XIP space the ADDR register applies to: 0: SMIF0 1: SMIF1"]
    #[inline(always)]
    pub fn smif_space(&self) -> SMIF_SPACE_R {
        SMIF_SPACE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines whether the remap region is active or not, and if active whether it is a simple remap or interleaving remap: 0: remap region is inactive (all other registers associated with this remap region are ignored) 1: remap accesses to SMIF0 2: remap accesses to SMIF1 3: remap accesses to SMIF0 and SMIF1 on an interleaving basis This and all other remap region registers are relevant only if the bridge's CTL.ENABLED=1."]
    #[inline(always)]
    #[must_use]
    pub fn use_smif(&mut self) -> USE_SMIF_W<0> {
        USE_SMIF_W::new(self)
    }
    #[doc = "Bits 16:18 - Interleaving step size (only relevant when USE_SMIF=3): 0: 8 bytes 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn intlv_step_size(&mut self) -> INTLV_STEP_SIZE_W<16> {
        INTLV_STEP_SIZE_W::new(self)
    }
    #[doc = "Bit 31 - Determines which SMIF XIP space the ADDR register applies to: 0: SMIF0 1: SMIF1"]
    #[inline(always)]
    #[must_use]
    pub fn smif_space(&mut self) -> SMIF_SPACE_W<31> {
        SMIF_SPACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control bits for remap region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0003_0000"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0000;
}
