#[doc = "Register `HTCFG2` reader"]
pub struct R(crate::R<HTCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTCFG2` writer"]
pub struct W(crate::W<HTCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTCFG2_SPEC>;
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
impl From<crate::W<HTCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSYNC` reader - Width of HSYNC pulse in pixels. (Set value +1 is the width of HSYNC pulse) HSYNC is of zero width if HsEn==0. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
pub type HSYNC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSYNC` writer - Width of HSYNC pulse in pixels. (Set value +1 is the width of HSYNC pulse) HSYNC is of zero width if HsEn==0. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
pub type HSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTCFG2_SPEC, u16, u16, 14, O>;
#[doc = "Field `HSBP` reader - Width of HSYNC pulse plus width of horizontal back porch in pixels. (Set value +1 is the width of HSYNC pulse plus width of horizontal back porch) Hsbp shall be greater or equal than Hsync. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
pub type HSBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSBP` writer - Width of HSYNC pulse plus width of horizontal back porch in pixels. (Set value +1 is the width of HSYNC pulse plus width of horizontal back porch) Hsbp shall be greater or equal than Hsync. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
pub type HSBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTCFG2_SPEC, u16, u16, 14, O>;
#[doc = "Field `HSEN` reader - Enables generation of HSYNC pulse."]
pub type HSEN_R = crate::BitReader<bool>;
#[doc = "Field `HSEN` writer - Enables generation of HSYNC pulse."]
pub type HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTCFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Width of HSYNC pulse in pixels. (Set value +1 is the width of HSYNC pulse) HSYNC is of zero width if HsEn==0. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Width of HSYNC pulse plus width of horizontal back porch in pixels. (Set value +1 is the width of HSYNC pulse plus width of horizontal back porch) Hsbp shall be greater or equal than Hsync. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
    #[inline(always)]
    pub fn hsbp(&self) -> HSBP_R {
        HSBP_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables generation of HSYNC pulse."]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width of HSYNC pulse in pixels. (Set value +1 is the width of HSYNC pulse) HSYNC is of zero width if HsEn==0. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
    #[inline(always)]
    #[must_use]
    pub fn hsync(&mut self) -> HSYNC_W<0> {
        HSYNC_W::new(self)
    }
    #[doc = "Bits 16:29 - Width of HSYNC pulse plus width of horizontal back porch in pixels. (Set value +1 is the width of HSYNC pulse plus width of horizontal back porch) Hsbp shall be greater or equal than Hsync. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
    #[inline(always)]
    #[must_use]
    pub fn hsbp(&mut self) -> HSBP_W<16> {
        HSBP_W::new(self)
    }
    #[doc = "Bit 31 - Enables generation of HSYNC pulse."]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HSEN_W<31> {
        HSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Horizontal Timing Config Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcfg2](index.html) module"]
pub struct HTCFG2_SPEC;
impl crate::RegisterSpec for HTCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcfg2::R](R) reader structure"]
impl crate::Readable for HTCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htcfg2::W](W) writer structure"]
impl crate::Writable for HTCFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTCFG2 to value 0x8047_001f"]
impl crate::Resettable for HTCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8047_001f;
}
