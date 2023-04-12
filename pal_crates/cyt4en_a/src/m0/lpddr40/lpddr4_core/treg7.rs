#[doc = "Register `TREG7` reader"]
pub struct R(crate::R<TREG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG7` writer"]
pub struct W(crate::W<TREG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG7_SPEC>;
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
impl From<crate::W<TREG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_INIT1` reader - Minimum RESET_n LOW time after completion of voltage ramp = tINIT1"]
pub type T_INIT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_INIT1` writer - Minimum RESET_n LOW time after completion of voltage ramp = tINIT1"]
pub type T_INIT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG7_SPEC, u16, u16, 14, O>;
#[doc = "Field `T_MPCWR` reader - Additional time after tXP has expired until MPC \\[WRITE FIFO\\]
command may be issued = tRCD + 3"]
pub type T_MPCWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MPCWR` writer - Additional time after tXP has expired until MPC \\[WRITE FIFO\\]
command may be issued = tRCD + 3"]
pub type T_MPCWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG7_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_CKFSPE` reader - Valid clock requirement after entering FSP change = tCKFSPE"]
pub type T_CKFSPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKFSPE` writer - Valid clock requirement after entering FSP change = tCKFSPE"]
pub type T_CKFSPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG7_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_CKFSPX` reader - Valid Clock Requirement before 1st Valid Command after FSP change (tCK: 625ps) = tCKFSPX"]
pub type T_CKFSPX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKFSPX` writer - Valid Clock Requirement before 1st Valid Command after FSP change (tCK: 625ps) = tCKFSPX"]
pub type T_CKFSPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG7_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_CKELCK` reader - CKE low to CK = tCKELCK"]
pub type T_CKELCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKELCK` writer - CKE low to CK = tCKELCK"]
pub type T_CKELCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG7_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:13 - Minimum RESET_n LOW time after completion of voltage ramp = tINIT1"]
    #[inline(always)]
    pub fn t_init1(&self) -> T_INIT1_R {
        T_INIT1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:19 - Additional time after tXP has expired until MPC \\[WRITE FIFO\\]
command may be issued = tRCD + 3"]
    #[inline(always)]
    pub fn t_mpcwr(&self) -> T_MPCWR_R {
        T_MPCWR_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:23 - Valid clock requirement after entering FSP change = tCKFSPE"]
    #[inline(always)]
    pub fn t_ckfspe(&self) -> T_CKFSPE_R {
        T_CKFSPE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Valid Clock Requirement before 1st Valid Command after FSP change (tCK: 625ps) = tCKFSPX"]
    #[inline(always)]
    pub fn t_ckfspx(&self) -> T_CKFSPX_R {
        T_CKFSPX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CKE low to CK = tCKELCK"]
    #[inline(always)]
    pub fn t_ckelck(&self) -> T_CKELCK_R {
        T_CKELCK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Minimum RESET_n LOW time after completion of voltage ramp = tINIT1"]
    #[inline(always)]
    #[must_use]
    pub fn t_init1(&mut self) -> T_INIT1_W<0> {
        T_INIT1_W::new(self)
    }
    #[doc = "Bits 14:19 - Additional time after tXP has expired until MPC \\[WRITE FIFO\\]
command may be issued = tRCD + 3"]
    #[inline(always)]
    #[must_use]
    pub fn t_mpcwr(&mut self) -> T_MPCWR_W<14> {
        T_MPCWR_W::new(self)
    }
    #[doc = "Bits 20:23 - Valid clock requirement after entering FSP change = tCKFSPE"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckfspe(&mut self) -> T_CKFSPE_W<20> {
        T_CKFSPE_W::new(self)
    }
    #[doc = "Bits 24:27 - Valid Clock Requirement before 1st Valid Command after FSP change (tCK: 625ps) = tCKFSPX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckfspx(&mut self) -> T_CKFSPX_W<24> {
        T_CKFSPX_W::new(self)
    }
    #[doc = "Bits 28:31 - CKE low to CK = tCKELCK"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckelck(&mut self) -> T_CKELCK_W<28> {
        T_CKELCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg7](index.html) module"]
pub struct TREG7_SPEC;
impl crate::RegisterSpec for TREG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg7::R](R) reader structure"]
impl crate::Readable for TREG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg7::W](W) writer structure"]
impl crate::Writable for TREG7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG7 to value 0x5000_2b68"]
impl crate::Resettable for TREG7_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000_2b68;
}
