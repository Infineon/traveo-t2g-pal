#[doc = "Register `TREG4` reader"]
pub struct R(crate::R<TREG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG4` writer"]
pub struct W(crate::W<TREG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG4_SPEC>;
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
impl From<crate::W<TREG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_WLBR` reader - = WL + BL/2 - 3 + tWR"]
pub type T_WLBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_WLBR` writer - = WL + BL/2 - 3 + tWR"]
pub type T_WLBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG4_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_WRAPDEN` reader - Timing of WRA command to Power Down entry = WL + BL/2 +tWR +2"]
pub type T_WRAPDEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_WRAPDEN` writer - Timing of WRA command to Power Down entry = WL + BL/2 +tWR +2"]
pub type T_WRAPDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG4_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_XP` reader - Exit Power Down with DLL on to any valid command; Exit Precharge Power Down with DLL frozen to commands not requiring a locked DLL = tXP"]
pub type T_XP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_XP` writer - Exit Power Down with DLL on to any valid command; Exit Precharge Power Down with DLL frozen to commands not requiring a locked DLL = tXP"]
pub type T_XP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG4_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_ZQCAL` reader - ZQ calibration time = tZQCAL"]
pub type T_ZQCAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_ZQCAL` writer - ZQ calibration time = tZQCAL"]
pub type T_ZQCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG4_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:7 - = WL + BL/2 - 3 + tWR"]
    #[inline(always)]
    pub fn t_wlbr(&self) -> T_WLBR_R {
        T_WLBR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timing of WRA command to Power Down entry = WL + BL/2 +tWR +2"]
    #[inline(always)]
    pub fn t_wrapden(&self) -> T_WRAPDEN_R {
        T_WRAPDEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Exit Power Down with DLL on to any valid command; Exit Precharge Power Down with DLL frozen to commands not requiring a locked DLL = tXP"]
    #[inline(always)]
    pub fn t_xp(&self) -> T_XP_R {
        T_XP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:31 - ZQ calibration time = tZQCAL"]
    #[inline(always)]
    pub fn t_zqcal(&self) -> T_ZQCAL_R {
        T_ZQCAL_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - = WL + BL/2 - 3 + tWR"]
    #[inline(always)]
    #[must_use]
    pub fn t_wlbr(&mut self) -> T_WLBR_W<0> {
        T_WLBR_W::new(self)
    }
    #[doc = "Bits 8:15 - Timing of WRA command to Power Down entry = WL + BL/2 +tWR +2"]
    #[inline(always)]
    #[must_use]
    pub fn t_wrapden(&mut self) -> T_WRAPDEN_W<8> {
        T_WRAPDEN_W::new(self)
    }
    #[doc = "Bits 16:20 - Exit Power Down with DLL on to any valid command; Exit Precharge Power Down with DLL frozen to commands not requiring a locked DLL = tXP"]
    #[inline(always)]
    #[must_use]
    pub fn t_xp(&mut self) -> T_XP_W<16> {
        T_XP_W::new(self)
    }
    #[doc = "Bits 21:31 - ZQ calibration time = tZQCAL"]
    #[inline(always)]
    #[must_use]
    pub fn t_zqcal(&mut self) -> T_ZQCAL_W<21> {
        T_ZQCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg4](index.html) module"]
pub struct TREG4_SPEC;
impl crate::RegisterSpec for TREG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg4::R](R) reader structure"]
impl crate::Readable for TREG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg4::W](W) writer structure"]
impl crate::Writable for TREG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG4 to value 0xc800_0000"]
impl crate::Resettable for TREG4_SPEC {
    const RESET_VALUE: Self::Ux = 0xc800_0000;
}
