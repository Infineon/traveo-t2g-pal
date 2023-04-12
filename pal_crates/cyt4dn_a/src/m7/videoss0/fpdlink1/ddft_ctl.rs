#[doc = "Register `DDFT_CTL` reader"]
pub struct R(crate::R<DDFT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDFT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDFT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDFT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDFT_CTL` writer"]
pub struct W(crate::W<DDFT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDFT_CTL_SPEC>;
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
impl From<crate::W<DDFT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDFT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDFT_SEL0` reader - Spare DDFT_SEL bits to be used for analog signal checking, by default ddft_out port is kept at constant 0"]
pub type DDFT_SEL0_R = crate::FieldReader<u8, DDFT_SEL0_A>;
#[doc = "Spare DDFT_SEL bits to be used for analog signal checking, by default ddft_out port is kept at constant 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DDFT_SEL0_A {
    #[doc = "0: By default ddft_out port is kept at constant 0"]
    DEFAULT = 0,
    #[doc = "1: ROUTE the output ddft_out of the hardIP to the DDFT_OUT port of our wrapper"]
    ROUTE_DDFT_OUT = 1,
    #[doc = "2: ROUTE the PLL_LOCK output of the hardIP to the DDFT_OUT port of our wrapper"]
    ROUTE_PLL_LOCK = 2,
    #[doc = "3: ROUTE the HS_PXL_CLKS output of the hardIP to the DDFT_OUT port of our wrapper"]
    ROUTE_HS_PXL_CLKS = 3,
}
impl From<DDFT_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT_SEL0_A) -> Self {
        variant as _
    }
}
impl DDFT_SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DDFT_SEL0_A> {
        match self.bits {
            0 => Some(DDFT_SEL0_A::DEFAULT),
            1 => Some(DDFT_SEL0_A::ROUTE_DDFT_OUT),
            2 => Some(DDFT_SEL0_A::ROUTE_PLL_LOCK),
            3 => Some(DDFT_SEL0_A::ROUTE_HS_PXL_CLKS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == DDFT_SEL0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ROUTE_DDFT_OUT`"]
    #[inline(always)]
    pub fn is_route_ddft_out(&self) -> bool {
        *self == DDFT_SEL0_A::ROUTE_DDFT_OUT
    }
    #[doc = "Checks if the value of the field is `ROUTE_PLL_LOCK`"]
    #[inline(always)]
    pub fn is_route_pll_lock(&self) -> bool {
        *self == DDFT_SEL0_A::ROUTE_PLL_LOCK
    }
    #[doc = "Checks if the value of the field is `ROUTE_HS_PXL_CLKS`"]
    #[inline(always)]
    pub fn is_route_hs_pxl_clks(&self) -> bool {
        *self == DDFT_SEL0_A::ROUTE_HS_PXL_CLKS
    }
}
#[doc = "Field `DDFT_SEL0` writer - Spare DDFT_SEL bits to be used for analog signal checking, by default ddft_out port is kept at constant 0"]
pub type DDFT_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDFT_CTL_SPEC, u8, DDFT_SEL0_A, 4, O>;
impl<'a, const O: u8> DDFT_SEL0_W<'a, O> {
    #[doc = "By default ddft_out port is kept at constant 0"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(DDFT_SEL0_A::DEFAULT)
    }
    #[doc = "ROUTE the output ddft_out of the hardIP to the DDFT_OUT port of our wrapper"]
    #[inline(always)]
    pub fn route_ddft_out(self) -> &'a mut W {
        self.variant(DDFT_SEL0_A::ROUTE_DDFT_OUT)
    }
    #[doc = "ROUTE the PLL_LOCK output of the hardIP to the DDFT_OUT port of our wrapper"]
    #[inline(always)]
    pub fn route_pll_lock(self) -> &'a mut W {
        self.variant(DDFT_SEL0_A::ROUTE_PLL_LOCK)
    }
    #[doc = "ROUTE the HS_PXL_CLKS output of the hardIP to the DDFT_OUT port of our wrapper"]
    #[inline(always)]
    pub fn route_hs_pxl_clks(self) -> &'a mut W {
        self.variant(DDFT_SEL0_A::ROUTE_HS_PXL_CLKS)
    }
}
#[doc = "Field `DDFT_SEL1` reader - Spare DDFT_SEL bits to be used later for analog signal checking"]
pub type DDFT_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDFT_SEL1` writer - Spare DDFT_SEL bits to be used later for analog signal checking"]
pub type DDFT_SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDFT_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Spare DDFT_SEL bits to be used for analog signal checking, by default ddft_out port is kept at constant 0"]
    #[inline(always)]
    pub fn ddft_sel0(&self) -> DDFT_SEL0_R {
        DDFT_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Spare DDFT_SEL bits to be used later for analog signal checking"]
    #[inline(always)]
    pub fn ddft_sel1(&self) -> DDFT_SEL1_R {
        DDFT_SEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Spare DDFT_SEL bits to be used for analog signal checking, by default ddft_out port is kept at constant 0"]
    #[inline(always)]
    #[must_use]
    pub fn ddft_sel0(&mut self) -> DDFT_SEL0_W<0> {
        DDFT_SEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Spare DDFT_SEL bits to be used later for analog signal checking"]
    #[inline(always)]
    #[must_use]
    pub fn ddft_sel1(&mut self) -> DDFT_SEL1_W<4> {
        DDFT_SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital DFT control register (select signals for Muxes used for analog signal checking purposes)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_ctl](index.html) module"]
pub struct DDFT_CTL_SPEC;
impl crate::RegisterSpec for DDFT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddft_ctl::R](R) reader structure"]
impl crate::Readable for DDFT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddft_ctl::W](W) writer structure"]
impl crate::Writable for DDFT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDFT_CTL to value 0"]
impl crate::Resettable for DDFT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
