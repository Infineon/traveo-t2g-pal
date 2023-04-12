#[doc = "Register `SHAD_LPMR12` reader"]
pub struct R(crate::R<SHAD_LPMR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR12_FS0_VREFCAS` reader - VREF-CA Settings for frequency set 0 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type SHAD_LPMR12_FS0_VREFCAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR12_FS0_VREFCAR` reader - VREF-CA range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type SHAD_LPMR12_FS0_VREFCAR_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR12_RSVD0` reader - N/A"]
pub type SHAD_LPMR12_RSVD0_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR12_FS1_VREFCAS` reader - VREF-CA Settings for frequency set 1 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type SHAD_LPMR12_FS1_VREFCAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR12_FS1_VREFCAR` reader - VREF-CA range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type SHAD_LPMR12_FS1_VREFCAR_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR12_RSVD1` reader - N/A"]
pub type SHAD_LPMR12_RSVD1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - VREF-CA Settings for frequency set 0 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn shad_lpmr12_fs0_vrefcas(&self) -> SHAD_LPMR12_FS0_VREFCAS_R {
        SHAD_LPMR12_FS0_VREFCAS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - VREF-CA range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn shad_lpmr12_fs0_vrefcar(&self) -> SHAD_LPMR12_FS0_VREFCAR_R {
        SHAD_LPMR12_FS0_VREFCAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr12_rsvd0(&self) -> SHAD_LPMR12_RSVD0_R {
        SHAD_LPMR12_RSVD0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - VREF-CA Settings for frequency set 1 VREF-CA Setting values according to JESD209-4: See JESD209-4B Table 13 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn shad_lpmr12_fs1_vrefcas(&self) -> SHAD_LPMR12_FS1_VREFCAS_R {
        SHAD_LPMR12_FS1_VREFCAS_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - VREF-CA range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn shad_lpmr12_fs1_vrefcar(&self) -> SHAD_LPMR12_FS1_VREFCAR_R {
        SHAD_LPMR12_FS1_VREFCAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr12_rsvd1(&self) -> SHAD_LPMR12_RSVD1_R {
        SHAD_LPMR12_RSVD1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr12](index.html) module"]
pub struct SHAD_LPMR12_SPEC;
impl crate::RegisterSpec for SHAD_LPMR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr12::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR12 to value 0x4d4d"]
impl crate::Resettable for SHAD_LPMR12_SPEC {
    const RESET_VALUE: Self::Ux = 0x4d4d;
}
