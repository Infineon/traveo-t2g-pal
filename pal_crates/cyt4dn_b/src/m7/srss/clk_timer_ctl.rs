#[doc = "Register `CLK_TIMER_CTL` reader"]
pub struct R(crate::R<CLK_TIMER_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TIMER_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TIMER_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TIMER_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TIMER_CTL` writer"]
pub struct W(crate::W<CLK_TIMER_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TIMER_CTL_SPEC>;
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
impl From<crate::W<CLK_TIMER_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TIMER_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL` reader - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type TIMER_SEL_R = crate::BitReader<TIMER_SEL_A>;
#[doc = "Obsolete. Do not use in new designs. Keep default value in new designs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_SEL_A {
    #[doc = "0: Obsolete. Do not use in new designs. Keep default value in new designs."]
    IMO = 0,
    #[doc = "1: Obsolete. Do not use in new designs."]
    HF0_DIV = 1,
}
impl From<TIMER_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SEL_A {
        match self.bits {
            false => TIMER_SEL_A::IMO,
            true => TIMER_SEL_A::HF0_DIV,
        }
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == TIMER_SEL_A::IMO
    }
    #[doc = "Checks if the value of the field is `HF0_DIV`"]
    #[inline(always)]
    pub fn is_hf0_div(&self) -> bool {
        *self == TIMER_SEL_A::HF0_DIV
    }
}
#[doc = "Field `TIMER_SEL` writer - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type TIMER_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_TIMER_CTL_SPEC, TIMER_SEL_A, O>;
impl<'a, const O: u8> TIMER_SEL_W<'a, O> {
    #[doc = "Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(TIMER_SEL_A::IMO)
    }
    #[doc = "Obsolete. Do not use in new designs."]
    #[inline(always)]
    pub fn hf0_div(self) -> &'a mut W {
        self.variant(TIMER_SEL_A::HF0_DIV)
    }
}
#[doc = "Field `TIMER_HF0_DIV` reader - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type TIMER_HF0_DIV_R = crate::FieldReader<u8, TIMER_HF0_DIV_A>;
#[doc = "Obsolete. Do not use in new designs. Keep default value in new designs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_HF0_DIV_A {
    #[doc = "0: Obsolete. Do not use in new designs. Keep default value in new designs."]
    NO_DIV = 0,
    #[doc = "1: Obsolete. Do not use in new designs."]
    DIV_BY_2 = 1,
    #[doc = "2: Obsolete. Do not use in new designs."]
    DIV_BY_4 = 2,
    #[doc = "3: Obsolete. Do not use in new designs."]
    DIV_BY_8 = 3,
}
impl From<TIMER_HF0_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_HF0_DIV_A) -> Self {
        variant as _
    }
}
impl TIMER_HF0_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_HF0_DIV_A {
        match self.bits {
            0 => TIMER_HF0_DIV_A::NO_DIV,
            1 => TIMER_HF0_DIV_A::DIV_BY_2,
            2 => TIMER_HF0_DIV_A::DIV_BY_4,
            3 => TIMER_HF0_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == TIMER_HF0_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_8
    }
}
#[doc = "Field `TIMER_HF0_DIV` writer - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type TIMER_HF0_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLK_TIMER_CTL_SPEC, u8, TIMER_HF0_DIV_A, 2, O>;
impl<'a, const O: u8> TIMER_HF0_DIV_W<'a, O> {
    #[doc = "Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::NO_DIV)
    }
    #[doc = "Obsolete. Do not use in new designs."]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_2)
    }
    #[doc = "Obsolete. Do not use in new designs."]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_4)
    }
    #[doc = "Obsolete. Do not use in new designs."]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_8)
    }
}
#[doc = "Field `TIMER_DIV` reader - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type TIMER_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_DIV` writer - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type TIMER_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TIMER_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ENABLE` reader - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Obsolete. Do not use in new designs. Keep default value in new designs."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_TIMER_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    pub fn timer_hf0_div(&self) -> TIMER_HF0_DIV_R {
        TIMER_HF0_DIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    pub fn timer_div(&self) -> TIMER_DIV_R {
        TIMER_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<0> {
        TIMER_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    #[must_use]
    pub fn timer_hf0_div(&mut self) -> TIMER_HF0_DIV_W<8> {
        TIMER_HF0_DIV_W::new(self)
    }
    #[doc = "Bits 16:23 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    #[must_use]
    pub fn timer_div(&mut self) -> TIMER_DIV_W<16> {
        TIMER_DIV_W::new(self)
    }
    #[doc = "Bit 31 - Obsolete. Do not use in new designs. Keep default value in new designs."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_timer_ctl](index.html) module"]
pub struct CLK_TIMER_CTL_SPEC;
impl crate::RegisterSpec for CLK_TIMER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_timer_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TIMER_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_timer_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TIMER_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TIMER_CTL to value 0x8007_0000"]
impl crate::Resettable for CLK_TIMER_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8007_0000;
}
