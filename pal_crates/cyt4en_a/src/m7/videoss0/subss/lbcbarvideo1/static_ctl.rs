#[doc = "Register `STATIC_CTL` reader"]
pub struct R(crate::R<STATIC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATIC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATIC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATIC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATIC_CTL` writer"]
pub struct W(crate::W<STATIC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATIC_CTL_SPEC>;
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
impl From<crate::W<STATIC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATIC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operating mode."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Operating mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: The crossbar is deactivated; all slave are directly connected to master ports."]
    NEUTRAL = 0,
    #[doc = "1: Static mapping between slave and master ports."]
    STATIC = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::NEUTRAL,
            true => MODE_A::STATIC,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == MODE_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == MODE_A::STATIC
    }
}
#[doc = "Field `MODE` writer - Operating mode."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATIC_CTL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "The crossbar is deactivated; all slave are directly connected to master ports."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(MODE_A::NEUTRAL)
    }
    #[doc = "Static mapping between slave and master ports."]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(MODE_A::STATIC)
    }
}
impl R {
    #[doc = "Bit 0 - Operating mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static control settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [static_ctl](index.html) module"]
pub struct STATIC_CTL_SPEC;
impl crate::RegisterSpec for STATIC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [static_ctl::R](R) reader structure"]
impl crate::Readable for STATIC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [static_ctl::W](W) writer structure"]
impl crate::Writable for STATIC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATIC_CTL to value 0"]
impl crate::Resettable for STATIC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
