#[doc = "Register `LPMR12` reader"]
pub struct R(crate::R<LPMR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR12` writer"]
pub struct W(crate::W<LPMR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR12_SPEC>;
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
impl From<crate::W<LPMR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_VREFCAS` reader - VREF-CA Settings for frequency set 0 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS0_VREFCAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_VREFCAS` writer - VREF-CA Settings for frequency set 0 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS0_VREFCAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR12_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS0_VREFCAR` reader - VREF-CA range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS0_VREFCAR_R = crate::BitReader<bool>;
#[doc = "Field `FS0_VREFCAR` writer - VREF-CA range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS0_VREFCAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR12_SPEC, bool, O>;
#[doc = "Field `FS1_VREFCAS` reader - VREF-CA Settings for frequency set 1 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS1_VREFCAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_VREFCAS` writer - VREF-CA Settings for frequency set 1 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS1_VREFCAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR12_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS1_VREFCAR` reader - VREF-CA range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS1_VREFCAR_R = crate::BitReader<bool>;
#[doc = "Field `FS1_VREFCAR` writer - VREF-CA range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS1_VREFCAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR12_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - VREF-CA Settings for frequency set 0 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn fs0_vrefcas(&self) -> FS0_VREFCAS_R {
        FS0_VREFCAS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - VREF-CA range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn fs0_vrefcar(&self) -> FS0_VREFCAR_R {
        FS0_VREFCAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - VREF-CA Settings for frequency set 1 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn fs1_vrefcas(&self) -> FS1_VREFCAS_R {
        FS1_VREFCAS_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - VREF-CA range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn fs1_vrefcar(&self) -> FS1_VREFCAR_R {
        FS1_VREFCAR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - VREF-CA Settings for frequency set 0 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_vrefcas(&mut self) -> FS0_VREFCAS_W<0> {
        FS0_VREFCAS_W::new(self)
    }
    #[doc = "Bit 6 - VREF-CA range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_vrefcar(&mut self) -> FS0_VREFCAR_W<6> {
        FS0_VREFCAR_W::new(self)
    }
    #[doc = "Bits 7:12 - VREF-CA Settings for frequency set 1 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_vrefcas(&mut self) -> FS1_VREFCAS_W<7> {
        FS1_VREFCAS_W::new(self)
    }
    #[doc = "Bit 13 - VREF-CA range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_vrefcar(&mut self) -> FS1_VREFCAR_W<13> {
        FS1_VREFCAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr12](index.html) module"]
pub struct LPMR12_SPEC;
impl crate::RegisterSpec for LPMR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr12::R](R) reader structure"]
impl crate::Readable for LPMR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr12::W](W) writer structure"]
impl crate::Writable for LPMR12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR12 to value 0x26cd"]
impl crate::Resettable for LPMR12_SPEC {
    const RESET_VALUE: Self::Ux = 0x26cd;
}
