#[doc = "Register `CAP0CFG` reader"]
pub struct R(crate::R<CAP0CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP0CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP0CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP0CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP0CFG` writer"]
pub struct W(crate::W<CAP0CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP0CFG_SPEC>;
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
impl From<crate::W<CAP0CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP0CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0SRC` reader - Capture Source Selection"]
pub type CAP0SRC_R = crate::FieldReader<u8, CAP0SRC_A>;
#[doc = "Capture Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAP0SRC_A {
    #[doc = "0: TTL0 interface selected"]
    TTL = 0,
    #[doc = "1: CSI-2 (DPHY) Interface Selected"]
    CSI = 1,
    #[doc = "2: N/A"]
    DSP0 = 2,
    #[doc = "3: N/A"]
    DSP1 = 3,
}
impl From<CAP0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CAP0SRC_A) -> Self {
        variant as _
    }
}
impl CAP0SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0SRC_A {
        match self.bits {
            0 => CAP0SRC_A::TTL,
            1 => CAP0SRC_A::CSI,
            2 => CAP0SRC_A::DSP0,
            3 => CAP0SRC_A::DSP1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TTL`"]
    #[inline(always)]
    pub fn is_ttl(&self) -> bool {
        *self == CAP0SRC_A::TTL
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == CAP0SRC_A::CSI
    }
    #[doc = "Checks if the value of the field is `DSP0`"]
    #[inline(always)]
    pub fn is_dsp0(&self) -> bool {
        *self == CAP0SRC_A::DSP0
    }
    #[doc = "Checks if the value of the field is `DSP1`"]
    #[inline(always)]
    pub fn is_dsp1(&self) -> bool {
        *self == CAP0SRC_A::DSP1
    }
}
#[doc = "Field `CAP0SRC` writer - Capture Source Selection"]
pub type CAP0SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAP0CFG_SPEC, u8, CAP0SRC_A, 2, O>;
impl<'a, const O: u8> CAP0SRC_W<'a, O> {
    #[doc = "TTL0 interface selected"]
    #[inline(always)]
    pub fn ttl(self) -> &'a mut W {
        self.variant(CAP0SRC_A::TTL)
    }
    #[doc = "CSI-2 (DPHY) Interface Selected"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(CAP0SRC_A::CSI)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn dsp0(self) -> &'a mut W {
        self.variant(CAP0SRC_A::DSP0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn dsp1(self) -> &'a mut W {
        self.variant(CAP0SRC_A::DSP1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Capture Source Selection"]
    #[inline(always)]
    pub fn cap0src(&self) -> CAP0SRC_R {
        CAP0SRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cap0src(&mut self) -> CAP0SRC_W<0> {
        CAP0SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture 0 Confifuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap0cfg](index.html) module"]
pub struct CAP0CFG_SPEC;
impl crate::RegisterSpec for CAP0CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap0cfg::R](R) reader structure"]
impl crate::Readable for CAP0CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap0cfg::W](W) writer structure"]
impl crate::Writable for CAP0CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP0CFG to value 0"]
impl crate::Resettable for CAP0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
