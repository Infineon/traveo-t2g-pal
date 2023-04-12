#[doc = "Register `SHAD_LPMR14` reader"]
pub struct R(crate::R<SHAD_LPMR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR14_FS0_VREFDQS` reader - VREF-DQ Settings for frequency set 0 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type SHAD_LPMR14_FS0_VREFDQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR14_FS0_VREFDQR` reader - VREF-DQ range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type SHAD_LPMR14_FS0_VREFDQR_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR14_RSVD0` reader - N/A"]
pub type SHAD_LPMR14_RSVD0_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR14_FS1_VREFDQS` reader - VREF-DQ Settings for frequency set 1 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
pub type SHAD_LPMR14_FS1_VREFDQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR14_FS1_VREFDQR` reader - VREF-DQ range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type SHAD_LPMR14_FS1_VREFDQR_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR14_RSVD1` reader - N/A"]
pub type SHAD_LPMR14_RSVD1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - VREF-DQ Settings for frequency set 0 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn shad_lpmr14_fs0_vrefdqs(&self) -> SHAD_LPMR14_FS0_VREFDQS_R {
        SHAD_LPMR14_FS0_VREFDQS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - VREF-DQ range for frequency set 0 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn shad_lpmr14_fs0_vrefdqr(&self) -> SHAD_LPMR14_FS0_VREFDQR_R {
        SHAD_LPMR14_FS0_VREFDQR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr14_rsvd0(&self) -> SHAD_LPMR14_RSVD0_R {
        SHAD_LPMR14_RSVD0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - VREF-DQ Settings for frequency set 1 VREF-DQ Setting values according to JESD209-4: See JESD209-4B Table 14 - VREF Settings for Range\\[0\\]
and Range\\[1\\]"]
    #[inline(always)]
    pub fn shad_lpmr14_fs1_vrefdqs(&self) -> SHAD_LPMR14_FS1_VREFDQS_R {
        SHAD_LPMR14_FS1_VREFDQS_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - VREF-DQ range for frequency set 1 VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn shad_lpmr14_fs1_vrefdqr(&self) -> SHAD_LPMR14_FS1_VREFDQR_R {
        SHAD_LPMR14_FS1_VREFDQR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr14_rsvd1(&self) -> SHAD_LPMR14_RSVD1_R {
        SHAD_LPMR14_RSVD1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 14\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr14](index.html) module"]
pub struct SHAD_LPMR14_SPEC;
impl crate::RegisterSpec for SHAD_LPMR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr14::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR14 to value 0x4d4d"]
impl crate::Resettable for SHAD_LPMR14_SPEC {
    const RESET_VALUE: Self::Ux = 0x4d4d;
}
