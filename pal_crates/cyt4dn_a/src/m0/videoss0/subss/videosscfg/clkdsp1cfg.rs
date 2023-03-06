#[doc = "Register `CLKDSP1CFG` reader"]
pub struct R(crate::R<CLKDSP1CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDSP1CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDSP1CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDSP1CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDSP1CFG` writer"]
pub struct W(crate::W<CLKDSP1CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDSP1CFG_SPEC>;
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
impl From<crate::W<CLKDSP1CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDSP1CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVVAL1` reader - Clock Divider Setting (KDIV)"]
pub type DIVVAL1_R = crate::FieldReader<u8, DIVVAL1_A>;
#[doc = "Clock Divider Setting (KDIV)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVVAL1_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
}
impl From<DIVVAL1_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVVAL1_A) -> Self {
        variant as _
    }
}
impl DIVVAL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVVAL1_A {
        match self.bits {
            0 => DIVVAL1_A::DIV1,
            1 => DIVVAL1_A::DIV2,
            2 => DIVVAL1_A::DIV4,
            3 => DIVVAL1_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVVAL1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVVAL1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVVAL1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVVAL1_A::DIV8
    }
}
#[doc = "Field `DIVVAL1` writer - Clock Divider Setting (KDIV)"]
pub type DIVVAL1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLKDSP1CFG_SPEC, u8, DIVVAL1_A, 2, O>;
impl<'a, const O: u8> DIVVAL1_W<'a, O> {
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVVAL1_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVVAL1_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIVVAL1_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIVVAL1_A::DIV8)
    }
}
#[doc = "Field `TTLCLKINV1` reader - Enable bit for pixel clock inversion on TTL display output interface 1. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
pub type TTLCLKINV1_R = crate::BitReader<bool>;
#[doc = "Field `TTLCLKINV1` writer - Enable bit for pixel clock inversion on TTL display output interface 1. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
pub type TTLCLKINV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKDSP1CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Clock Divider Setting (KDIV)"]
    #[inline(always)]
    pub fn divval1(&self) -> DIVVAL1_R {
        DIVVAL1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Enable bit for pixel clock inversion on TTL display output interface 1. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
    #[inline(always)]
    pub fn ttlclkinv1(&self) -> TTLCLKINV1_R {
        TTLCLKINV1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Divider Setting (KDIV)"]
    #[inline(always)]
    #[must_use]
    pub fn divval1(&mut self) -> DIVVAL1_W<0> {
        DIVVAL1_W::new(self)
    }
    #[doc = "Bit 4 - Enable bit for pixel clock inversion on TTL display output interface 1. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
    #[inline(always)]
    #[must_use]
    pub fn ttlclkinv1(&mut self) -> TTLCLKINV1_W<4> {
        TTLCLKINV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display 1 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdsp1cfg](index.html) module"]
pub struct CLKDSP1CFG_SPEC;
impl crate::RegisterSpec for CLKDSP1CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdsp1cfg::R](R) reader structure"]
impl crate::Readable for CLKDSP1CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdsp1cfg::W](W) writer structure"]
impl crate::Writable for CLKDSP1CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDSP1CFG to value 0x10"]
impl crate::Resettable for CLKDSP1CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
