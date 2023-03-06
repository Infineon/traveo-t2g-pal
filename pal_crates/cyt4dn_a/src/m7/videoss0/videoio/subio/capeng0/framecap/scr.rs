#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUS` reader - Number of consecutive frames captured until synchronisation state is reached (value of 0 is not allowed). indicated by sts signal and register field Sts.SyncStat."]
pub type FUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUS` writer - Number of consecutive frames captured until synchronisation state is reached (value of 0 is not allowed). indicated by sts signal and register field Sts.SyncStat."]
pub type FUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Number of consecutive frames captured until synchronisation state is reached (value of 0 is not allowed). indicated by sts signal and register field Sts.SyncStat."]
    #[inline(always)]
    pub fn fus(&self) -> FUS_R {
        FUS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of consecutive frames captured until synchronisation state is reached (value of 0 is not allowed). indicated by sts signal and register field Sts.SyncStat."]
    #[inline(always)]
    #[must_use]
    pub fn fus(&mut self) -> FUS_W<0> {
        FUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap Sync frame number configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0x02"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
