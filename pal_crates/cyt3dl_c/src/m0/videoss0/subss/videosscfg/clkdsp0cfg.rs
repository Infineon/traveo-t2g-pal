#[doc = "Register `CLKDSP0CFG` reader"]
pub struct R(crate::R<CLKDSP0CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDSP0CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDSP0CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDSP0CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDSP0CFG` writer"]
pub struct W(crate::W<CLKDSP0CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDSP0CFG_SPEC>;
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
impl From<crate::W<CLKDSP0CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDSP0CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVVAL0` reader - Clock Divider Setting (KDIV)"]
pub type DIVVAL0_R = crate::FieldReader<u8, DIVVAL0_A>;
#[doc = "Clock Divider Setting (KDIV)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVVAL0_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
}
impl From<DIVVAL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVVAL0_A) -> Self {
        variant as _
    }
}
impl DIVVAL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVVAL0_A {
        match self.bits {
            0 => DIVVAL0_A::DIV1,
            1 => DIVVAL0_A::DIV2,
            2 => DIVVAL0_A::DIV4,
            3 => DIVVAL0_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVVAL0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVVAL0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVVAL0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVVAL0_A::DIV8
    }
}
#[doc = "Field `DIVVAL0` writer - Clock Divider Setting (KDIV)"]
pub type DIVVAL0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLKDSP0CFG_SPEC, u8, DIVVAL0_A, 2, O>;
impl<'a, const O: u8> DIVVAL0_W<'a, O> {
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVVAL0_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVVAL0_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIVVAL0_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIVVAL0_A::DIV8)
    }
}
#[doc = "Field `TTLCLKINV0` reader - Enable bit for pixel clock inversion on TTL display output interface 0. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
pub type TTLCLKINV0_R = crate::BitReader<bool>;
#[doc = "Field `TTLCLKINV0` writer - Enable bit for pixel clock inversion on TTL display output interface 0. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
pub type TTLCLKINV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKDSP0CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Clock Divider Setting (KDIV)"]
    #[inline(always)]
    pub fn divval0(&self) -> DIVVAL0_R {
        DIVVAL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Enable bit for pixel clock inversion on TTL display output interface 0. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
    #[inline(always)]
    pub fn ttlclkinv0(&self) -> TTLCLKINV0_R {
        TTLCLKINV0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Divider Setting (KDIV)"]
    #[inline(always)]
    #[must_use]
    pub fn divval0(&mut self) -> DIVVAL0_W<0> {
        DIVVAL0_W::new(self)
    }
    #[doc = "Bit 4 - Enable bit for pixel clock inversion on TTL display output interface 0. If enabled, pixel data changes with falling instead of rising clock edge (180 deg phase shift)."]
    #[inline(always)]
    #[must_use]
    pub fn ttlclkinv0(&mut self) -> TTLCLKINV0_W<4> {
        TTLCLKINV0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display 0 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdsp0cfg](index.html) module"]
pub struct CLKDSP0CFG_SPEC;
impl crate::RegisterSpec for CLKDSP0CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdsp0cfg::R](R) reader structure"]
impl crate::Readable for CLKDSP0CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdsp0cfg::W](W) writer structure"]
impl crate::Writable for CLKDSP0CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDSP0CFG to value 0x10"]
impl crate::Resettable for CLKDSP0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
