#[doc = "Register `LPMR14` reader"]
pub struct R(crate::R<LPMR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR14` writer"]
pub struct W(crate::W<LPMR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR14_SPEC>;
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
impl From<crate::W<LPMR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_VREFDQS` reader - VREF-DQ Settings for frequency set 0 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS0_VREFDQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_VREFDQS` writer - VREF-DQ Settings for frequency set 0 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS0_VREFDQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR14_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS0_VREFDQR` reader - VREF-DQ range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS0_VREFDQR_R = crate::BitReader<bool>;
#[doc = "Field `FS0_VREFDQR` writer - VREF-DQ range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS0_VREFDQR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR14_SPEC, bool, O>;
#[doc = "Field `FS1_VREFDQS` reader - VREF-DQ Settings for frequency set 1 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS1_VREFDQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_VREFDQS` writer - VREF-DQ Settings for frequency set 1 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type FS1_VREFDQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR14_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS1_VREFDQR` reader - VREF-DQ range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS1_VREFDQR_R = crate::BitReader<bool>;
#[doc = "Field `FS1_VREFDQR` writer - VREF-DQ range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type FS1_VREFDQR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR14_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - VREF-DQ Settings for frequency set 0 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn fs0_vrefdqs(&self) -> FS0_VREFDQS_R {
        FS0_VREFDQS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - VREF-DQ range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn fs0_vrefdqr(&self) -> FS0_VREFDQR_R {
        FS0_VREFDQR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - VREF-DQ Settings for frequency set 1 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn fs1_vrefdqs(&self) -> FS1_VREFDQS_R {
        FS1_VREFDQS_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - VREF-DQ range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn fs1_vrefdqr(&self) -> FS1_VREFDQR_R {
        FS1_VREFDQR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - VREF-DQ Settings for frequency set 0 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_vrefdqs(&mut self) -> FS0_VREFDQS_W<0> {
        FS0_VREFDQS_W::new(self)
    }
    #[doc = "Bit 6 - VREF-DQ range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_vrefdqr(&mut self) -> FS0_VREFDQR_W<6> {
        FS0_VREFDQR_W::new(self)
    }
    #[doc = "Bits 7:12 - VREF-DQ Settings for frequency set 1 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_vrefdqs(&mut self) -> FS1_VREFDQS_W<7> {
        FS1_VREFDQS_W::new(self)
    }
    #[doc = "Bit 13 - VREF-DQ range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_vrefdqr(&mut self) -> FS1_VREFDQR_W<13> {
        FS1_VREFDQR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr14](index.html) module"]
pub struct LPMR14_SPEC;
impl crate::RegisterSpec for LPMR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr14::R](R) reader structure"]
impl crate::Readable for LPMR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr14::W](W) writer structure"]
impl crate::Writable for LPMR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR14 to value 0x26cd"]
impl crate::Resettable for LPMR14_SPEC {
    const RESET_VALUE: Self::Ux = 0x26cd;
}
