#[doc = "Register `TREG6` reader"]
pub struct R(crate::R<TREG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG6` writer"]
pub struct W(crate::W<TREG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG6_SPEC>;
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
impl From<crate::W<TREG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_READ_LOW` reader - Maximum number of cycles that a low priority read can wait before initiating the direction switch = 512"]
pub type T_READ_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_READ_LOW` writer - Maximum number of cycles that a low priority read can wait before initiating the direction switch = 512"]
pub type T_READ_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG6_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_READ_HIGH` reader - Maximum number of cycles that a high priority read can wait before initiating the direction switch = 64"]
pub type T_READ_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_READ_HIGH` writer - Maximum number of cycles that a high priority read can wait before initiating the direction switch = 64"]
pub type T_READ_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG6_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_CCD` reader - CAS_n to CAS_n command delay (min = BL/2) = tCCD"]
pub type T_CCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CCD` writer - CAS_n to CAS_n command delay (min = BL/2) = tCCD"]
pub type T_CCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG6_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_WLBTR` reader - Delay from start of internal write transaction to internal read command - Different BG. = WL + BL/2 + tWTR+1"]
pub type T_WLBTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_WLBTR` writer - Delay from start of internal write transaction to internal read command - Different BG. = WL + BL/2 + tWTR+1"]
pub type T_WLBTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG6_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:9 - Maximum number of cycles that a low priority read can wait before initiating the direction switch = 512"]
    #[inline(always)]
    pub fn t_read_low(&self) -> T_READ_LOW_R {
        T_READ_LOW_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Maximum number of cycles that a high priority read can wait before initiating the direction switch = 64"]
    #[inline(always)]
    pub fn t_read_high(&self) -> T_READ_HIGH_R {
        T_READ_HIGH_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:24 - CAS_n to CAS_n command delay (min = BL/2) = tCCD"]
    #[inline(always)]
    pub fn t_ccd(&self) -> T_CCD_R {
        T_CCD_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:31 - Delay from start of internal write transaction to internal read command - Different BG. = WL + BL/2 + tWTR+1"]
    #[inline(always)]
    pub fn t_wlbtr(&self) -> T_WLBTR_R {
        T_WLBTR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Maximum number of cycles that a low priority read can wait before initiating the direction switch = 512"]
    #[inline(always)]
    #[must_use]
    pub fn t_read_low(&mut self) -> T_READ_LOW_W<0> {
        T_READ_LOW_W::new(self)
    }
    #[doc = "Bits 10:19 - Maximum number of cycles that a high priority read can wait before initiating the direction switch = 64"]
    #[inline(always)]
    #[must_use]
    pub fn t_read_high(&mut self) -> T_READ_HIGH_W<10> {
        T_READ_HIGH_W::new(self)
    }
    #[doc = "Bits 20:24 - CAS_n to CAS_n command delay (min = BL/2) = tCCD"]
    #[inline(always)]
    #[must_use]
    pub fn t_ccd(&mut self) -> T_CCD_W<20> {
        T_CCD_W::new(self)
    }
    #[doc = "Bits 25:31 - Delay from start of internal write transaction to internal read command - Different BG. = WL + BL/2 + tWTR+1"]
    #[inline(always)]
    #[must_use]
    pub fn t_wlbtr(&mut self) -> T_WLBTR_W<25> {
        T_WLBTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg6](index.html) module"]
pub struct TREG6_SPEC;
impl crate::RegisterSpec for TREG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg6::R](R) reader structure"]
impl crate::Readable for TREG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg6::W](W) writer structure"]
impl crate::Writable for TREG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG6 to value 0x0001_0200"]
impl crate::Resettable for TREG6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0200;
}
