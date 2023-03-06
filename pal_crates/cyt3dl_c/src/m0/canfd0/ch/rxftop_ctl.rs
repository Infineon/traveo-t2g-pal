#[doc = "Register `RXFTOP_CTL` reader"]
pub struct R(crate::R<RXFTOP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTOP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTOP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTOP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFTOP_CTL` writer"]
pub struct W(crate::W<RXFTOP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFTOP_CTL_SPEC>;
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
impl From<crate::W<RXFTOP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFTOP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0TPE` reader - FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
pub type F0TPE_R = crate::BitReader<bool>;
#[doc = "Field `F0TPE` writer - FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
pub type F0TPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFTOP_CTL_SPEC, bool, O>;
#[doc = "Field `F1TPE` reader - FIFO 1 Top Pointer Enable."]
pub type F1TPE_R = crate::BitReader<bool>;
#[doc = "Field `F1TPE` writer - FIFO 1 Top Pointer Enable."]
pub type F1TPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFTOP_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
    #[inline(always)]
    pub fn f0tpe(&self) -> F0TPE_R {
        F0TPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO 1 Top Pointer Enable."]
    #[inline(always)]
    pub fn f1tpe(&self) -> F1TPE_R {
        F1TPE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
    #[inline(always)]
    #[must_use]
    pub fn f0tpe(&mut self) -> F0TPE_W<0> {
        F0TPE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO 1 Top Pointer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn f1tpe(&mut self) -> F1TPE_W<1> {
        F1TPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Top control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop_ctl](index.html) module"]
pub struct RXFTOP_CTL_SPEC;
impl crate::RegisterSpec for RXFTOP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftop_ctl::R](R) reader structure"]
impl crate::Readable for RXFTOP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxftop_ctl::W](W) writer structure"]
impl crate::Writable for RXFTOP_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFTOP_CTL to value 0"]
impl crate::Resettable for RXFTOP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
