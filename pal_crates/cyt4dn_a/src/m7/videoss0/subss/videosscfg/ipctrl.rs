#[doc = "Register `IPCTRL` reader"]
pub struct R(crate::R<IPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCTRL` writer"]
pub struct W(crate::W<IPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCTRL_SPEC>;
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
impl From<crate::W<IPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTLENABLED` reader - VIDEOSS enable/disable bit."]
pub type CTLENABLED_R = crate::BitReader<CTLENABLED_A>;
#[doc = "VIDEOSS enable/disable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTLENABLED_A {
    #[doc = "0: Disable the entire VIDEOSS. System will be held in reset until re-enabled (with the exception of the AHB interface and this register)"]
    DIS = 0,
    #[doc = "1: Enable VIDEOSS"]
    EN = 1,
}
impl From<CTLENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CTLENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl CTLENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLENABLED_A {
        match self.bits {
            false => CTLENABLED_A::DIS,
            true => CTLENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CTLENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CTLENABLED_A::EN
    }
}
#[doc = "Field `CTLENABLED` writer - VIDEOSS enable/disable bit."]
pub type CTLENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCTRL_SPEC, CTLENABLED_A, O>;
impl<'a, const O: u8> CTLENABLED_W<'a, O> {
    #[doc = "Disable the entire VIDEOSS. System will be held in reset until re-enabled (with the exception of the AHB interface and this register)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CTLENABLED_A::DIS)
    }
    #[doc = "Enable VIDEOSS"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CTLENABLED_A::EN)
    }
}
impl R {
    #[doc = "Bit 31 - VIDEOSS enable/disable bit."]
    #[inline(always)]
    pub fn ctlenabled(&self) -> CTLENABLED_R {
        CTLENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - VIDEOSS enable/disable bit."]
    #[inline(always)]
    #[must_use]
    pub fn ctlenabled(&mut self) -> CTLENABLED_W<31> {
        CTLENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipctrl](index.html) module"]
pub struct IPCTRL_SPEC;
impl crate::RegisterSpec for IPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipctrl::R](R) reader structure"]
impl crate::Readable for IPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipctrl::W](W) writer structure"]
impl crate::Writable for IPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCTRL to value 0"]
impl crate::Resettable for IPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
