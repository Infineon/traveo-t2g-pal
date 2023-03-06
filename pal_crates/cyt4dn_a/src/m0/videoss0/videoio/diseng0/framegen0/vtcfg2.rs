#[doc = "Register `VTCFG2` reader"]
pub struct R(crate::R<VTCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTCFG2` writer"]
pub struct W(crate::W<VTCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTCFG2_SPEC>;
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
impl From<crate::W<VTCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSYNC` reader - Width of VSYNC pulse in lines. (Set value +1 is the width of VSYNC puls) VSYNC is of zero width if VsEn==0. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VSYNC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VSYNC` writer - Width of VSYNC pulse in lines. (Set value +1 is the width of VSYNC puls) VSYNC is of zero width if VsEn==0. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTCFG2_SPEC, u16, u16, 14, O>;
#[doc = "Field `VSBP` reader - Width of VSYNC pulse plus width of vertical back porch in lines. (Set value +1 is the width of VSYNC pulse plus width of vertical back porch) Vsbp shall be greater or equal than Vsync. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VSBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VSBP` writer - Width of VSYNC pulse plus width of vertical back porch in lines. (Set value +1 is the width of VSYNC pulse plus width of vertical back porch) Vsbp shall be greater or equal than Vsync. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VSBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTCFG2_SPEC, u16, u16, 14, O>;
#[doc = "Field `VSEN` reader - Enables generation of VSYNC pulse."]
pub type VSEN_R = crate::BitReader<bool>;
#[doc = "Field `VSEN` writer - Enables generation of VSYNC pulse."]
pub type VSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VTCFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Width of VSYNC pulse in lines. (Set value +1 is the width of VSYNC puls) VSYNC is of zero width if VsEn==0. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Width of VSYNC pulse plus width of vertical back porch in lines. (Set value +1 is the width of VSYNC pulse plus width of vertical back porch) Vsbp shall be greater or equal than Vsync. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    pub fn vsbp(&self) -> VSBP_R {
        VSBP_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables generation of VSYNC pulse."]
    #[inline(always)]
    pub fn vsen(&self) -> VSEN_R {
        VSEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width of VSYNC pulse in lines. (Set value +1 is the width of VSYNC puls) VSYNC is of zero width if VsEn==0. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<0> {
        VSYNC_W::new(self)
    }
    #[doc = "Bits 16:29 - Width of VSYNC pulse plus width of vertical back porch in lines. (Set value +1 is the width of VSYNC pulse plus width of vertical back porch) Vsbp shall be greater or equal than Vsync. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    #[must_use]
    pub fn vsbp(&mut self) -> VSBP_W<16> {
        VSBP_W::new(self)
    }
    #[doc = "Bit 31 - Enables generation of VSYNC pulse."]
    #[inline(always)]
    #[must_use]
    pub fn vsen(&mut self) -> VSEN_W<31> {
        VSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Vertical Timing Config Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtcfg2](index.html) module"]
pub struct VTCFG2_SPEC;
impl crate::RegisterSpec for VTCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vtcfg2::R](R) reader structure"]
impl crate::Readable for VTCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtcfg2::W](W) writer structure"]
impl crate::Writable for VTCFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTCFG2 to value 0x8009_0003"]
impl crate::Resettable for VTCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8009_0003;
}
