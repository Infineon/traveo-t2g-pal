#[doc = "Register `LPMR1` reader"]
pub struct R(crate::R<LPMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR1` writer"]
pub struct W(crate::W<LPMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR1_SPEC>;
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
impl From<crate::W<LPMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_BL` reader - N/A"]
pub type FS0_BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_BL` writer - N/A"]
pub type FS0_BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FS0_WPRE` reader - N/A"]
pub type FS0_WPRE_R = crate::BitReader<bool>;
#[doc = "Field `FS0_WPRE` writer - N/A"]
pub type FS0_WPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR1_SPEC, bool, O>;
#[doc = "Field `FS0_RPRE` reader - Read preamble for frequency set 0 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
pub type FS0_RPRE_R = crate::BitReader<bool>;
#[doc = "Field `FS0_RPRE` writer - Read preamble for frequency set 0 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
pub type FS0_RPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR1_SPEC, bool, O>;
#[doc = "Field `FS0_NWR` reader - Write recovery to pre-charge for frequency set 0 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
pub type FS0_NWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_NWR` writer - Write recovery to pre-charge for frequency set 0 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
pub type FS0_NWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS0_RPST` reader - Read postamble for frequency set 0 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
pub type FS0_RPST_R = crate::BitReader<bool>;
#[doc = "Field `FS0_RPST` writer - Read postamble for frequency set 0 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
pub type FS0_RPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR1_SPEC, bool, O>;
#[doc = "Field `FS1_BL` reader - N/A"]
pub type FS1_BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_BL` writer - N/A"]
pub type FS1_BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FS1_WPRE` reader - N/A"]
pub type FS1_WPRE_R = crate::BitReader<bool>;
#[doc = "Field `FS1_WPRE` writer - N/A"]
pub type FS1_WPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR1_SPEC, bool, O>;
#[doc = "Field `FS1_RPRE` reader - Read preamble for frequency set 1 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
pub type FS1_RPRE_R = crate::BitReader<bool>;
#[doc = "Field `FS1_RPRE` writer - Read preamble for frequency set 1 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
pub type FS1_RPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR1_SPEC, bool, O>;
#[doc = "Field `FS1_NWR` reader - Write recovery to pre-charge for frequency set 1 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
pub type FS1_NWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_NWR` writer - Write recovery to pre-charge for frequency set 1 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
pub type FS1_NWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_RPST` reader - Read postamble for frequency set 1 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
pub type FS1_RPST_R = crate::BitReader<bool>;
#[doc = "Field `FS1_RPST` writer - Read postamble for frequency set 1 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
pub type FS1_RPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn fs0_bl(&self) -> FS0_BL_R {
        FS0_BL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn fs0_wpre(&self) -> FS0_WPRE_R {
        FS0_WPRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read preamble for frequency set 0 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
    #[inline(always)]
    pub fn fs0_rpre(&self) -> FS0_RPRE_R {
        FS0_RPRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Write recovery to pre-charge for frequency set 0 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
    #[inline(always)]
    pub fn fs0_nwr(&self) -> FS0_NWR_R {
        FS0_NWR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Read postamble for frequency set 0 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
    #[inline(always)]
    pub fn fs0_rpst(&self) -> FS0_RPST_R {
        FS0_RPST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn fs1_bl(&self) -> FS1_BL_R {
        FS1_BL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn fs1_wpre(&self) -> FS1_WPRE_R {
        FS1_WPRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read preamble for frequency set 1 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
    #[inline(always)]
    pub fn fs1_rpre(&self) -> FS1_RPRE_R {
        FS1_RPRE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Write recovery to pre-charge for frequency set 1 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
    #[inline(always)]
    pub fn fs1_nwr(&self) -> FS1_NWR_R {
        FS1_NWR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Read postamble for frequency set 1 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
    #[inline(always)]
    pub fn fs1_rpst(&self) -> FS1_RPST_R {
        FS1_RPST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_bl(&mut self) -> FS0_BL_W<0> {
        FS0_BL_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_wpre(&mut self) -> FS0_WPRE_W<2> {
        FS0_WPRE_W::new(self)
    }
    #[doc = "Bit 3 - Read preamble for frequency set 0 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_rpre(&mut self) -> FS0_RPRE_W<3> {
        FS0_RPRE_W::new(self)
    }
    #[doc = "Bits 4:6 - Write recovery to pre-charge for frequency set 0 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_nwr(&mut self) -> FS0_NWR_W<4> {
        FS0_NWR_W::new(self)
    }
    #[doc = "Bit 7 - Read postamble for frequency set 0 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_rpst(&mut self) -> FS0_RPST_W<7> {
        FS0_RPST_W::new(self)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_bl(&mut self) -> FS1_BL_W<8> {
        FS1_BL_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_wpre(&mut self) -> FS1_WPRE_W<10> {
        FS1_WPRE_W::new(self)
    }
    #[doc = "Bit 11 - Read preamble for frequency set 1 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_rpre(&mut self) -> FS1_RPRE_W<11> {
        FS1_RPRE_W::new(self)
    }
    #[doc = "Bits 12:14 - Write recovery to pre-charge for frequency set 1 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_nwr(&mut self) -> FS1_NWR_W<12> {
        FS1_NWR_W::new(self)
    }
    #[doc = "Bit 15 - Read postamble for frequency set 1 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_rpst(&mut self) -> FS1_RPST_W<15> {
        FS1_RPST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr1](index.html) module"]
pub struct LPMR1_SPEC;
impl crate::RegisterSpec for LPMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr1::R](R) reader structure"]
impl crate::Readable for LPMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr1::W](W) writer structure"]
impl crate::Writable for LPMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR1 to value 0x5404"]
impl crate::Resettable for LPMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x5404;
}
