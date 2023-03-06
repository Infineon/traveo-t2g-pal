#[doc = "Register `CLOCK_CTL` reader"]
pub struct R(crate::R<CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTL` writer"]
pub struct W(crate::W<CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTL_SPEC>;
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
impl From<crate::W<CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC_SEL` reader - Escape Clock Select. '0': clk_video '1': clk_sys (clk_slow in videoss)"]
pub type ESC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ESC_SEL` writer - Escape Clock Select. '0': clk_video '1': clk_sys (clk_slow in videoss)"]
pub type ESC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CTL_SPEC, bool, O>;
#[doc = "Field `ESC_DIV` reader - Escape Clock Divider. '00': divide by 1 '01': divide by 2 '10': divide by 3 '11': divide by 4 Note: If ESC_DIV_PRESENT=0, that bits do not have an effect."]
pub type ESC_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ESC_DIV` writer - Escape Clock Divider. '00': divide by 1 '01': divide by 2 '10': divide by 3 '11': divide by 4 Note: If ESC_DIV_PRESENT=0, that bits do not have an effect."]
pub type ESC_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Escape Clock Select. '0': clk_video '1': clk_sys (clk_slow in videoss)"]
    #[inline(always)]
    pub fn esc_sel(&self) -> ESC_SEL_R {
        ESC_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Escape Clock Divider. '00': divide by 1 '01': divide by 2 '10': divide by 3 '11': divide by 4 Note: If ESC_DIV_PRESENT=0, that bits do not have an effect."]
    #[inline(always)]
    pub fn esc_div(&self) -> ESC_DIV_R {
        ESC_DIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Escape Clock Select. '0': clk_video '1': clk_sys (clk_slow in videoss)"]
    #[inline(always)]
    #[must_use]
    pub fn esc_sel(&mut self) -> ESC_SEL_W<0> {
        ESC_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Escape Clock Divider. '00': divide by 1 '01': divide by 2 '10': divide by 3 '11': divide by 4 Note: If ESC_DIV_PRESENT=0, that bits do not have an effect."]
    #[inline(always)]
    #[must_use]
    pub fn esc_div(&mut self) -> ESC_DIV_W<8> {
        ESC_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](index.html) module"]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
