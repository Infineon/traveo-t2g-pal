#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARG_READ` reader - Margin Read"]
pub type MARG_READ_R = crate::FieldReader<u8, MARG_READ_A>;
#[doc = "Margin Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MARG_READ_A {
    #[doc = "0: Low Resistance: -50 percent from nominal"]
    LOWR = 0,
    #[doc = "1: Nominal resistance (Default read condition)"]
    DEFAULTR = 1,
    #[doc = "2: High Resistance: +50 percent from nominal"]
    HIGHR = 2,
    #[doc = "3: Higher Resistance: +100 percent from nominal"]
    HIGHERR = 3,
}
impl From<MARG_READ_A> for u8 {
    #[inline(always)]
    fn from(variant: MARG_READ_A) -> Self {
        variant as _
    }
}
impl MARG_READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MARG_READ_A {
        match self.bits {
            0 => MARG_READ_A::LOWR,
            1 => MARG_READ_A::DEFAULTR,
            2 => MARG_READ_A::HIGHR,
            3 => MARG_READ_A::HIGHERR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOWR`"]
    #[inline(always)]
    pub fn is_lowr(&self) -> bool {
        *self == MARG_READ_A::LOWR
    }
    #[doc = "Checks if the value of the field is `DEFAULTR`"]
    #[inline(always)]
    pub fn is_defaultr(&self) -> bool {
        *self == MARG_READ_A::DEFAULTR
    }
    #[doc = "Checks if the value of the field is `HIGHR`"]
    #[inline(always)]
    pub fn is_highr(&self) -> bool {
        *self == MARG_READ_A::HIGHR
    }
    #[doc = "Checks if the value of the field is `HIGHERR`"]
    #[inline(always)]
    pub fn is_higherr(&self) -> bool {
        *self == MARG_READ_A::HIGHERR
    }
}
#[doc = "Field `MARG_READ` writer - Margin Read"]
pub type MARG_READ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TEST_SPEC, u8, MARG_READ_A, 2, O>;
impl<'a, const O: u8> MARG_READ_W<'a, O> {
    #[doc = "Low Resistance: -50 percent from nominal"]
    #[inline(always)]
    pub fn lowr(self) -> &'a mut W {
        self.variant(MARG_READ_A::LOWR)
    }
    #[doc = "Nominal resistance (Default read condition)"]
    #[inline(always)]
    pub fn defaultr(self) -> &'a mut W {
        self.variant(MARG_READ_A::DEFAULTR)
    }
    #[doc = "High Resistance: +50 percent from nominal"]
    #[inline(always)]
    pub fn highr(self) -> &'a mut W {
        self.variant(MARG_READ_A::HIGHR)
    }
    #[doc = "Higher Resistance: +100 percent from nominal"]
    #[inline(always)]
    pub fn higherr(self) -> &'a mut W {
        self.variant(MARG_READ_A::HIGHERR)
    }
}
impl R {
    #[doc = "Bits 0:1 - Margin Read"]
    #[inline(always)]
    pub fn marg_read(&self) -> MARG_READ_R {
        MARG_READ_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Margin Read"]
    #[inline(always)]
    #[must_use]
    pub fn marg_read(&mut self) -> MARG_READ_W<0> {
        MARG_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0x01"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
