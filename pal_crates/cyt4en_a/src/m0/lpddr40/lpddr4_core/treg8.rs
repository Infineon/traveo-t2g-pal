#[doc = "Register `TREG8` reader"]
pub struct R(crate::R<TREG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG8` writer"]
pub struct W(crate::W<TREG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG8_SPEC>;
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
impl From<crate::W<TREG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_WRITE_LOW` reader - Maximum number of cycles that a low priority write can wait before initiating the direction switch = 512"]
pub type T_WRITE_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_WRITE_LOW` writer - Maximum number of cycles that a low priority write can wait before initiating the direction switch = 512"]
pub type T_WRITE_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG8_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_WRITE_HIGH` reader - Maximum number of cycles that a high priority write can wait before initiating the direction switch = 64"]
pub type T_WRITE_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_WRITE_HIGH` writer - Maximum number of cycles that a high priority write can wait before initiating the direction switch = 64"]
pub type T_WRITE_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG8_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_MRW` reader - Mode Register Write command period. = tMRW"]
pub type T_MRW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MRW` writer - Mode Register Write command period. = tMRW"]
pub type T_MRW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG8_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_MPCWR2RD` reader - MPC write fifo to MPC read fifo = WL + BL/2 + 1 + tWTR"]
pub type T_MPCWR2RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MPCWR2RD` writer - MPC write fifo to MPC read fifo = WL + BL/2 + 1 + tWTR"]
pub type T_MPCWR2RD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG8_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:9 - Maximum number of cycles that a low priority write can wait before initiating the direction switch = 512"]
    #[inline(always)]
    pub fn t_write_low(&self) -> T_WRITE_LOW_R {
        T_WRITE_LOW_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Maximum number of cycles that a high priority write can wait before initiating the direction switch = 64"]
    #[inline(always)]
    pub fn t_write_high(&self) -> T_WRITE_HIGH_R {
        T_WRITE_HIGH_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:24 - Mode Register Write command period. = tMRW"]
    #[inline(always)]
    pub fn t_mrw(&self) -> T_MRW_R {
        T_MRW_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:31 - MPC write fifo to MPC read fifo = WL + BL/2 + 1 + tWTR"]
    #[inline(always)]
    pub fn t_mpcwr2rd(&self) -> T_MPCWR2RD_R {
        T_MPCWR2RD_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Maximum number of cycles that a low priority write can wait before initiating the direction switch = 512"]
    #[inline(always)]
    #[must_use]
    pub fn t_write_low(&mut self) -> T_WRITE_LOW_W<0> {
        T_WRITE_LOW_W::new(self)
    }
    #[doc = "Bits 10:19 - Maximum number of cycles that a high priority write can wait before initiating the direction switch = 64"]
    #[inline(always)]
    #[must_use]
    pub fn t_write_high(&mut self) -> T_WRITE_HIGH_W<10> {
        T_WRITE_HIGH_W::new(self)
    }
    #[doc = "Bits 20:24 - Mode Register Write command period. = tMRW"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrw(&mut self) -> T_MRW_W<20> {
        T_MRW_W::new(self)
    }
    #[doc = "Bits 25:31 - MPC write fifo to MPC read fifo = WL + BL/2 + 1 + tWTR"]
    #[inline(always)]
    #[must_use]
    pub fn t_mpcwr2rd(&mut self) -> T_MPCWR2RD_W<25> {
        T_MPCWR2RD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg8](index.html) module"]
pub struct TREG8_SPEC;
impl crate::RegisterSpec for TREG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg8::R](R) reader structure"]
impl crate::Readable for TREG8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg8::W](W) writer structure"]
impl crate::Writable for TREG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG8 to value 0x0002_03ff"]
impl crate::Resettable for TREG8_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_03ff;
}
