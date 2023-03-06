#[doc = "Register `RTGC0` reader"]
pub struct R(crate::R<RTGC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTGC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTGC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTGC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTGC0` writer"]
pub struct W(crate::W<RTGC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTGC0_SPEC>;
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
impl From<crate::W<RTGC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTGC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GT_UPDT` reader - Gate training setting update Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type GT_UPDT_R = crate::BitReader<bool>;
#[doc = "Field `GT_UPDT` writer - Gate training setting update Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type GT_UPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTGC0_SPEC, bool, O>;
#[doc = "Field `GT_DIS` reader - Auto gate training disable. This will disable automatic read gate enable. Enable/disable coding: ENABLE = 0 Enable this feature DISABLE = 1 Disable this feature"]
pub type GT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `GT_DIS` writer - Auto gate training disable. This will disable automatic read gate enable. Enable/disable coding: ENABLE = 0 Enable this feature DISABLE = 1 Disable this feature"]
pub type GT_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTGC0_SPEC, bool, O>;
#[doc = "Field `FS0_TWREN` reader - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
pub type FS0_TWREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_TWREN` writer - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
pub type FS0_TWREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTGC0_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS0_TRDEN` reader - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
pub type FS0_TRDEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_TRDEN` writer - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
pub type FS0_TRDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTGC0_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS0_TRDENDBI` reader - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
pub type FS0_TRDENDBI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_TRDENDBI` writer - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
pub type FS0_TRDENDBI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTGC0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Gate training setting update Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn gt_updt(&self) -> GT_UPDT_R {
        GT_UPDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto gate training disable. This will disable automatic read gate enable. Enable/disable coding: ENABLE = 0 Enable this feature DISABLE = 1 Disable this feature"]
    #[inline(always)]
    pub fn gt_dis(&self) -> GT_DIS_R {
        GT_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
    #[inline(always)]
    pub fn fs0_twren(&self) -> FS0_TWREN_R {
        FS0_TWREN_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
    #[inline(always)]
    pub fn fs0_trden(&self) -> FS0_TRDEN_R {
        FS0_TRDEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:20 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
    #[inline(always)]
    pub fn fs0_trdendbi(&self) -> FS0_TRDENDBI_R {
        FS0_TRDENDBI_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Gate training setting update Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn gt_updt(&mut self) -> GT_UPDT_W<0> {
        GT_UPDT_W::new(self)
    }
    #[doc = "Bit 1 - Auto gate training disable. This will disable automatic read gate enable. Enable/disable coding: ENABLE = 0 Enable this feature DISABLE = 1 Disable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn gt_dis(&mut self) -> GT_DIS_W<1> {
        GT_DIS_W::new(self)
    }
    #[doc = "Bits 2:7 - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_twren(&mut self) -> FS0_TWREN_W<2> {
        FS0_TWREN_W::new(self)
    }
    #[doc = "Bits 8:13 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_trden(&mut self) -> FS0_TRDEN_W<8> {
        FS0_TRDEN_W::new(self)
    }
    #[doc = "Bits 14:20 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_trdendbi(&mut self) -> FS0_TRDENDBI_W<14> {
        FS0_TRDENDBI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Read Training General Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtgc0](index.html) module"]
pub struct RTGC0_SPEC;
impl crate::RegisterSpec for RTGC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtgc0::R](R) reader structure"]
impl crate::Readable for RTGC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtgc0::W](W) writer structure"]
impl crate::Writable for RTGC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTGC0 to value 0"]
impl crate::Resettable for RTGC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
