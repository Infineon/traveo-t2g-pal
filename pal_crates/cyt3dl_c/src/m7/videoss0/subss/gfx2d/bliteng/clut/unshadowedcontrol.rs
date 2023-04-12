#[doc = "Register `UNSHADOWEDCONTROL` reader"]
pub struct R(crate::R<UNSHADOWEDCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNSHADOWEDCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNSHADOWEDCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNSHADOWEDCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNSHADOWEDCONTROL` writer"]
pub struct W(crate::W<UNSHADOWEDCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNSHADOWEDCONTROL_SPEC>;
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
impl From<crate::W<UNSHADOWEDCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNSHADOWEDCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B_EN` reader - Write enable for writing the blue color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
pub type B_EN_R = crate::BitReader<B_EN_A>;
#[doc = "Write enable for writing the blue color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<B_EN_A> for bool {
    #[inline(always)]
    fn from(variant: B_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl B_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B_EN_A {
        match self.bits {
            false => B_EN_A::DISABLE,
            true => B_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == B_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == B_EN_A::ENABLE
    }
}
#[doc = "Field `B_EN` writer - Write enable for writing the blue color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
pub type B_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UNSHADOWEDCONTROL_SPEC, B_EN_A, O>;
impl<'a, const O: u8> B_EN_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(B_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(B_EN_A::ENABLE)
    }
}
#[doc = "Field `G_EN` reader - Write enable for writing the green color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
pub type G_EN_R = crate::BitReader<G_EN_A>;
#[doc = "Write enable for writing the green color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<G_EN_A> for bool {
    #[inline(always)]
    fn from(variant: G_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl G_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> G_EN_A {
        match self.bits {
            false => G_EN_A::DISABLE,
            true => G_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == G_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == G_EN_A::ENABLE
    }
}
#[doc = "Field `G_EN` writer - Write enable for writing the green color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
pub type G_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UNSHADOWEDCONTROL_SPEC, G_EN_A, O>;
impl<'a, const O: u8> G_EN_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(G_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(G_EN_A::ENABLE)
    }
}
#[doc = "Field `R_EN` reader - Write enable for writing the red color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
pub type R_EN_R = crate::BitReader<R_EN_A>;
#[doc = "Write enable for writing the red color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<R_EN_A> for bool {
    #[inline(always)]
    fn from(variant: R_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl R_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_EN_A {
        match self.bits {
            false => R_EN_A::DISABLE,
            true => R_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == R_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == R_EN_A::ENABLE
    }
}
#[doc = "Field `R_EN` writer - Write enable for writing the red color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
pub type R_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UNSHADOWEDCONTROL_SPEC, R_EN_A, O>;
impl<'a, const O: u8> R_EN_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(R_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(R_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Write enable for writing the blue color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
    #[inline(always)]
    pub fn b_en(&self) -> B_EN_R {
        B_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write enable for writing the green color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
    #[inline(always)]
    pub fn g_en(&self) -> G_EN_R {
        G_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write enable for writing the red color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
    #[inline(always)]
    pub fn r_en(&self) -> R_EN_R {
        R_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable for writing the blue color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
    #[inline(always)]
    #[must_use]
    pub fn b_en(&mut self) -> B_EN_W<0> {
        B_EN_W::new(self)
    }
    #[doc = "Bit 1 - Write enable for writing the green color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
    #[inline(always)]
    #[must_use]
    pub fn g_en(&mut self) -> G_EN_W<1> {
        G_EN_W::new(self)
    }
    #[doc = "Bit 2 - Write enable for writing the red color LUT entry from the host (allows writing a single color entry without a read-modify-write cycle)"]
    #[inline(always)]
    #[must_use]
    pub fn r_en(&mut self) -> R_EN_W<2> {
        R_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLUT unshadowed control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unshadowedcontrol](index.html) module"]
pub struct UNSHADOWEDCONTROL_SPEC;
impl crate::RegisterSpec for UNSHADOWEDCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unshadowedcontrol::R](R) reader structure"]
impl crate::Readable for UNSHADOWEDCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unshadowedcontrol::W](W) writer structure"]
impl crate::Writable for UNSHADOWEDCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNSHADOWEDCONTROL to value 0"]
impl crate::Resettable for UNSHADOWEDCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
