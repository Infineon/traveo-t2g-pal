#[doc = "Register `PTSR25` reader"]
pub struct R(crate::R<PTSR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR25` writer"]
pub struct W(crate::W<PTSR25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR25_SPEC>;
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
impl From<crate::W<PTSR25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDDLY_ODT0` reader - MEM_ODT\\[0\\]
of MEM_ODT delay setting"]
pub type CMDDLY_ODT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDDLY_ODT0` writer - MEM_ODT\\[0\\]
of MEM_ODT delay setting"]
pub type CMDDLY_ODT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR25_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDDLY_ODT1` reader - MEM_ODT\\[1\\]
of MEM_ODT delay setting"]
pub type CMDDLY_ODT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDDLY_ODT1` writer - MEM_ODT\\[1\\]
of MEM_ODT delay setting"]
pub type CMDDLY_ODT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR25_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDDLY_RSTN0` reader - MEM_RESET_N\\[0\\]
delay setting"]
pub type CMDDLY_RSTN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDDLY_RSTN0` writer - MEM_RESET_N\\[0\\]
delay setting"]
pub type CMDDLY_RSTN0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR25_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDDLY_RSTN1` reader - MEM_RESET_N\\[1\\]
delay setting"]
pub type CMDDLY_RSTN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDDLY_RSTN1` writer - MEM_RESET_N\\[1\\]
delay setting"]
pub type CMDDLY_RSTN1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR25_SPEC, u8, u8, 6, O>;
#[doc = "Field `DQSLEADCK` reader - Rank 0 DQS leads CK flag. One bit for each slice. 0: CK leads DQS, 1: DQS leads CK."]
pub type DQSLEADCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSLEADCK` writer - Rank 0 DQS leads CK flag. One bit for each slice. 0: CK leads DQS, 1: DQS leads CK."]
pub type DQSLEADCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR25_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5 - MEM_ODT\\[0\\]
of MEM_ODT delay setting"]
    #[inline(always)]
    pub fn cmddly_odt0(&self) -> CMDDLY_ODT0_R {
        CMDDLY_ODT0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - MEM_ODT\\[1\\]
of MEM_ODT delay setting"]
    #[inline(always)]
    pub fn cmddly_odt1(&self) -> CMDDLY_ODT1_R {
        CMDDLY_ODT1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - MEM_RESET_N\\[0\\]
delay setting"]
    #[inline(always)]
    pub fn cmddly_rstn0(&self) -> CMDDLY_RSTN0_R {
        CMDDLY_RSTN0_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - MEM_RESET_N\\[1\\]
delay setting"]
    #[inline(always)]
    pub fn cmddly_rstn1(&self) -> CMDDLY_RSTN1_R {
        CMDDLY_RSTN1_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Rank 0 DQS leads CK flag. One bit for each slice. 0: CK leads DQS, 1: DQS leads CK."]
    #[inline(always)]
    pub fn dqsleadck(&self) -> DQSLEADCK_R {
        DQSLEADCK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - MEM_ODT\\[0\\]
of MEM_ODT delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmddly_odt0(&mut self) -> CMDDLY_ODT0_W<0> {
        CMDDLY_ODT0_W::new(self)
    }
    #[doc = "Bits 6:11 - MEM_ODT\\[1\\]
of MEM_ODT delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmddly_odt1(&mut self) -> CMDDLY_ODT1_W<6> {
        CMDDLY_ODT1_W::new(self)
    }
    #[doc = "Bits 12:17 - MEM_RESET_N\\[0\\]
delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmddly_rstn0(&mut self) -> CMDDLY_RSTN0_W<12> {
        CMDDLY_RSTN0_W::new(self)
    }
    #[doc = "Bits 18:23 - MEM_RESET_N\\[1\\]
delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmddly_rstn1(&mut self) -> CMDDLY_RSTN1_W<18> {
        CMDDLY_RSTN1_W::new(self)
    }
    #[doc = "Bits 24:27 - Rank 0 DQS leads CK flag. One bit for each slice. 0: CK leads DQS, 1: DQS leads CK."]
    #[inline(always)]
    #[must_use]
    pub fn dqsleadck(&mut self) -> DQSLEADCK_W<24> {
        DQSLEADCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr25](index.html) module"]
pub struct PTSR25_SPEC;
impl crate::RegisterSpec for PTSR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr25::R](R) reader structure"]
impl crate::Readable for PTSR25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr25::W](W) writer structure"]
impl crate::Writable for PTSR25_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR25 to value 0"]
impl crate::Resettable for PTSR25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
