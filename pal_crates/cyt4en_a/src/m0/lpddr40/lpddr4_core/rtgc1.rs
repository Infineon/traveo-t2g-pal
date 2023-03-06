#[doc = "Register `RTGC1` reader"]
pub struct R(crate::R<RTGC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTGC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTGC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTGC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTGC1` writer"]
pub struct W(crate::W<RTGC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTGC1_SPEC>;
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
impl From<crate::W<RTGC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTGC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS1_TWREN` reader - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
pub type FS1_TWREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_TWREN` writer - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
pub type FS1_TWREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTGC1_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS1_TRDEN` reader - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
pub type FS1_TRDEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_TRDEN` writer - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
pub type FS1_TRDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTGC1_SPEC, u8, u8, 6, O>;
#[doc = "Field `FS1_TRDENDBI` reader - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
pub type FS1_TRDENDBI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_TRDENDBI` writer - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
pub type FS1_TRDENDBI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTGC1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:5 - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
    #[inline(always)]
    pub fn fs1_twren(&self) -> FS1_TWREN_R {
        FS1_TWREN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
    #[inline(always)]
    pub fn fs1_trden(&self) -> FS1_TRDEN_R {
        FS1_TRDEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:18 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
    #[inline(always)]
    pub fn fs1_trdendbi(&self) -> FS1_TRDENDBI_R {
        FS1_TRDENDBI_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Specifies the number of DFI PHY clock cycles between when a write command is sent on the DFI control interface and when the dfi_wrdata_en signal is asserted. LPDDR4: WL-1"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_twren(&mut self) -> FS1_TWREN_W<0> {
        FS1_TWREN_W::new(self)
    }
    #[doc = "Bits 6:11 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal LPDDR4: RL-1"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_trden(&mut self) -> FS1_TRDEN_W<6> {
        FS1_TRDEN_W::new(self)
    }
    #[doc = "Bits 12:18 - Specifies the number of DFI PHY clock cycles from the assertion of a read command on the DFI to the assertion of the dfi_rddata_en signal in DBI mode"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_trdendbi(&mut self) -> FS1_TRDENDBI_W<12> {
        FS1_TRDENDBI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Read Training General Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtgc1](index.html) module"]
pub struct RTGC1_SPEC;
impl crate::RegisterSpec for RTGC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtgc1::R](R) reader structure"]
impl crate::Readable for RTGC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtgc1::W](W) writer structure"]
impl crate::Writable for RTGC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTGC1 to value 0"]
impl crate::Resettable for RTGC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
