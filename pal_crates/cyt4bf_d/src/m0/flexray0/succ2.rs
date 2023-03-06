#[doc = "Register `SUCC2` reader"]
pub struct R(crate::R<SUCC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUCC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUCC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUCC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUCC2` writer"]
pub struct W(crate::W<SUCC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUCC2_SPEC>;
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
impl From<crate::W<SUCC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUCC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT` reader - Listen Timeout (pdListenTimeout) Configures wakeup / startup listen timeout in uT. The range for pdListenTimeout is 1284 to 1283846 uT."]
pub type LT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LT` writer - Listen Timeout (pdListenTimeout) Configures wakeup / startup listen timeout in uT. The range for pdListenTimeout is 1284 to 1283846 uT."]
pub type LT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC2_SPEC, u32, u32, 21, O>;
#[doc = "Field `LTN` reader - Listen Timeout Noise (gListenNoise - 1) Configures the upper limit for startup and wakeup listen timeout in the presence of noise expressed as a multiple of pdListenTimeout. The range for gListenNoise is 2 to 16. LTN\\[3:0\\]
must be configured identical in all nodes of a cluster."]
pub type LTN_R = crate::FieldReader<u8, LTN_A>;
#[doc = "Listen Timeout Noise (gListenNoise - 1) Configures the upper limit for startup and wakeup listen timeout in the presence of noise expressed as a multiple of pdListenTimeout. The range for gListenNoise is 2 to 16. LTN\\[3:0\\]
must be configured identical in all nodes of a cluster.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LTN_A {
    #[doc = "1: N/A"]
    MIN = 1,
}
impl From<LTN_A> for u8 {
    #[inline(always)]
    fn from(variant: LTN_A) -> Self {
        variant as _
    }
}
impl LTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LTN_A> {
        match self.bits {
            1 => Some(LTN_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == LTN_A::MIN
    }
}
#[doc = "Field `LTN` writer - Listen Timeout Noise (gListenNoise - 1) Configures the upper limit for startup and wakeup listen timeout in the presence of noise expressed as a multiple of pdListenTimeout. The range for gListenNoise is 2 to 16. LTN\\[3:0\\]
must be configured identical in all nodes of a cluster."]
pub type LTN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC2_SPEC, u8, LTN_A, 4, O>;
impl<'a, const O: u8> LTN_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(LTN_A::MIN)
    }
}
impl R {
    #[doc = "Bits 0:20 - Listen Timeout (pdListenTimeout) Configures wakeup / startup listen timeout in uT. The range for pdListenTimeout is 1284 to 1283846 uT."]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 24:27 - Listen Timeout Noise (gListenNoise - 1) Configures the upper limit for startup and wakeup listen timeout in the presence of noise expressed as a multiple of pdListenTimeout. The range for gListenNoise is 2 to 16. LTN\\[3:0\\]
must be configured identical in all nodes of a cluster."]
    #[inline(always)]
    pub fn ltn(&self) -> LTN_R {
        LTN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:20 - Listen Timeout (pdListenTimeout) Configures wakeup / startup listen timeout in uT. The range for pdListenTimeout is 1284 to 1283846 uT."]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<0> {
        LT_W::new(self)
    }
    #[doc = "Bits 24:27 - Listen Timeout Noise (gListenNoise - 1) Configures the upper limit for startup and wakeup listen timeout in the presence of noise expressed as a multiple of pdListenTimeout. The range for gListenNoise is 2 to 16. LTN\\[3:0\\]
must be configured identical in all nodes of a cluster."]
    #[inline(always)]
    #[must_use]
    pub fn ltn(&mut self) -> LTN_W<24> {
        LTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SUC Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [succ2](index.html) module"]
pub struct SUCC2_SPEC;
impl crate::RegisterSpec for SUCC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [succ2::R](R) reader structure"]
impl crate::Readable for SUCC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [succ2::W](W) writer structure"]
impl crate::Writable for SUCC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUCC2 to value 0x0100_0504"]
impl crate::Resettable for SUCC2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0504;
}
