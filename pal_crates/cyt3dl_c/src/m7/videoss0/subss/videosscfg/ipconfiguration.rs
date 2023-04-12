#[doc = "Register `IPCONFIGURATION` reader"]
pub struct R(crate::R<IPCONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GFX2D` reader - Graphics 2D Core available (0 = no, 1 = yes)"]
pub type GFX2D_R = crate::BitReader<bool>;
#[doc = "Field `MIPICSI` reader - MIPI CSI-2 Video Input Interface available (0 = no, 1 = yes)"]
pub type MIPICSI_R = crate::BitReader<bool>;
#[doc = "Field `FPDLINK0` reader - 1st FPDLink Video Output Interface available (0 = no, 1 = yes)"]
pub type FPDLINK0_R = crate::BitReader<bool>;
#[doc = "Field `FPDLINK1` reader - 2nd FPDLink Video Output Interface available (0 = no, 1 = yes)"]
pub type FPDLINK1_R = crate::BitReader<bool>;
#[doc = "Field `LCDBUSIF` reader - LCD Bus Output Interface available (0 = no, 1 = yes)"]
pub type LCDBUSIF_R = crate::BitReader<bool>;
#[doc = "Field `DISPLAY1` reader - 2nd Display Engine available (0 = no, 1 = yes)"]
pub type DISPLAY1_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE0` reader - Capture Engine available (0 = no, 1 = yes)"]
pub type CAPTURE0_R = crate::BitReader<bool>;
#[doc = "Field `WARPING` reader - Warping Layer available in Composition Engine (0 = no, 1 = yes)"]
pub type WARPING_R = crate::BitReader<bool>;
#[doc = "Field `TCON` reader - Timing Controller available in Display Engine (0 = no, 1 = yes)"]
pub type TCON_R = crate::BitReader<bool>;
#[doc = "Field `VRAM` reader - Embedded Video RAM available (0 = no, 1 = yes)"]
pub type VRAM_R = crate::BitReader<bool>;
#[doc = "Field `VRAMSIZE` reader - Size of embedded Video RAM if available (in MB)."]
pub type VRAMSIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Graphics 2D Core available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn gfx2d(&self) -> GFX2D_R {
        GFX2D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MIPI CSI-2 Video Input Interface available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn mipicsi(&self) -> MIPICSI_R {
        MIPICSI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1st FPDLink Video Output Interface available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn fpdlink0(&self) -> FPDLINK0_R {
        FPDLINK0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 2nd FPDLink Video Output Interface available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn fpdlink1(&self) -> FPDLINK1_R {
        FPDLINK1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Bus Output Interface available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn lcdbusif(&self) -> LCDBUSIF_R {
        LCDBUSIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 2nd Display Engine available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn display1(&self) -> DISPLAY1_R {
        DISPLAY1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture Engine available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn capture0(&self) -> CAPTURE0_R {
        CAPTURE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Warping Layer available in Composition Engine (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn warping(&self) -> WARPING_R {
        WARPING_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timing Controller available in Display Engine (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn tcon(&self) -> TCON_R {
        TCON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Embedded Video RAM available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn vram(&self) -> VRAM_R {
        VRAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Size of embedded Video RAM if available (in MB)."]
    #[inline(always)]
    pub fn vramsize(&self) -> VRAMSIZE_R {
        VRAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "IP Design Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipconfiguration](index.html) module"]
pub struct IPCONFIGURATION_SPEC;
impl crate::RegisterSpec for IPCONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipconfiguration::R](R) reader structure"]
impl crate::Readable for IPCONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPCONFIGURATION to value 0"]
impl crate::Resettable for IPCONFIGURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
