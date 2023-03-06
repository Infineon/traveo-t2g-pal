#[doc = "Register `TASKSETUP` reader"]
pub struct R(crate::R<TASKSETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKSETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKSETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKSETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKSETUP` writer"]
pub struct W(crate::W<TASKSETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKSETUP_SPEC>;
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
impl From<crate::W<TASKSETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKSETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASKSELECT` reader - Select task bank for operation status registers (ypos, ...)."]
pub type TASKSELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TASKSELECT` writer - Select task bank for operation status registers (ypos, ...)."]
pub type TASKSELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TASKSETUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `RENDERMODE` reader - Blit engine operation mode."]
pub type RENDERMODE_R = crate::BitReader<RENDERMODE_A>;
#[doc = "Blit engine operation mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RENDERMODE_A {
    #[doc = "0: Frame rendering mode (framebuffer)."]
    FRAMERENDERING = 0,
    #[doc = "1: Line rendering mode (linebuffer)."]
    LINERENDERING = 1,
}
impl From<RENDERMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RENDERMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RENDERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RENDERMODE_A {
        match self.bits {
            false => RENDERMODE_A::FRAMERENDERING,
            true => RENDERMODE_A::LINERENDERING,
        }
    }
    #[doc = "Checks if the value of the field is `FRAMERENDERING`"]
    #[inline(always)]
    pub fn is_framerendering(&self) -> bool {
        *self == RENDERMODE_A::FRAMERENDERING
    }
    #[doc = "Checks if the value of the field is `LINERENDERING`"]
    #[inline(always)]
    pub fn is_linerendering(&self) -> bool {
        *self == RENDERMODE_A::LINERENDERING
    }
}
#[doc = "Field `RENDERMODE` writer - Blit engine operation mode."]
pub type RENDERMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TASKSETUP_SPEC, RENDERMODE_A, O>;
impl<'a, const O: u8> RENDERMODE_W<'a, O> {
    #[doc = "Frame rendering mode (framebuffer)."]
    #[inline(always)]
    pub fn framerendering(self) -> &'a mut W {
        self.variant(RENDERMODE_A::FRAMERENDERING)
    }
    #[doc = "Line rendering mode (linebuffer)."]
    #[inline(always)]
    pub fn linerendering(self) -> &'a mut W {
        self.variant(RENDERMODE_A::LINERENDERING)
    }
}
#[doc = "Field `PROXYSELECT` reader - Proxy to use for the next operation"]
pub type PROXYSELECT_R = crate::FieldReader<u8, PROXYSELECT_A>;
#[doc = "Proxy to use for the next operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROXYSELECT_A {
    #[doc = "0: Proxy disabled."]
    OFF = 0,
    #[doc = "1: Proxy connected to FETCHLBH0."]
    FETCHLBH0 = 1,
    #[doc = "2: Proxy connected to FETCHLBH1."]
    FETCHLBH1 = 2,
    #[doc = "3: Proxy connected to FETCHLBH2."]
    FETCHLBH2 = 3,
    #[doc = "4: Proxy connected to FETCHLBH3."]
    FETCHLBH3 = 4,
    #[doc = "5: Proxy connected to FETCHLBH4."]
    FETCHLBH4 = 5,
}
impl From<PROXYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROXYSELECT_A) -> Self {
        variant as _
    }
}
impl PROXYSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROXYSELECT_A> {
        match self.bits {
            0 => Some(PROXYSELECT_A::OFF),
            1 => Some(PROXYSELECT_A::FETCHLBH0),
            2 => Some(PROXYSELECT_A::FETCHLBH1),
            3 => Some(PROXYSELECT_A::FETCHLBH2),
            4 => Some(PROXYSELECT_A::FETCHLBH3),
            5 => Some(PROXYSELECT_A::FETCHLBH4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PROXYSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `FETCHLBH0`"]
    #[inline(always)]
    pub fn is_fetchlbh0(&self) -> bool {
        *self == PROXYSELECT_A::FETCHLBH0
    }
    #[doc = "Checks if the value of the field is `FETCHLBH1`"]
    #[inline(always)]
    pub fn is_fetchlbh1(&self) -> bool {
        *self == PROXYSELECT_A::FETCHLBH1
    }
    #[doc = "Checks if the value of the field is `FETCHLBH2`"]
    #[inline(always)]
    pub fn is_fetchlbh2(&self) -> bool {
        *self == PROXYSELECT_A::FETCHLBH2
    }
    #[doc = "Checks if the value of the field is `FETCHLBH3`"]
    #[inline(always)]
    pub fn is_fetchlbh3(&self) -> bool {
        *self == PROXYSELECT_A::FETCHLBH3
    }
    #[doc = "Checks if the value of the field is `FETCHLBH4`"]
    #[inline(always)]
    pub fn is_fetchlbh4(&self) -> bool {
        *self == PROXYSELECT_A::FETCHLBH4
    }
}
#[doc = "Field `PROXYSELECT` writer - Proxy to use for the next operation"]
pub type PROXYSELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKSETUP_SPEC, u8, PROXYSELECT_A, 3, O>;
impl<'a, const O: u8> PROXYSELECT_W<'a, O> {
    #[doc = "Proxy disabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PROXYSELECT_A::OFF)
    }
    #[doc = "Proxy connected to FETCHLBH0."]
    #[inline(always)]
    pub fn fetchlbh0(self) -> &'a mut W {
        self.variant(PROXYSELECT_A::FETCHLBH0)
    }
    #[doc = "Proxy connected to FETCHLBH1."]
    #[inline(always)]
    pub fn fetchlbh1(self) -> &'a mut W {
        self.variant(PROXYSELECT_A::FETCHLBH1)
    }
    #[doc = "Proxy connected to FETCHLBH2."]
    #[inline(always)]
    pub fn fetchlbh2(self) -> &'a mut W {
        self.variant(PROXYSELECT_A::FETCHLBH2)
    }
    #[doc = "Proxy connected to FETCHLBH3."]
    #[inline(always)]
    pub fn fetchlbh3(self) -> &'a mut W {
        self.variant(PROXYSELECT_A::FETCHLBH3)
    }
    #[doc = "Proxy connected to FETCHLBH4."]
    #[inline(always)]
    pub fn fetchlbh4(self) -> &'a mut W {
        self.variant(PROXYSELECT_A::FETCHLBH4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select task bank for operation status registers (ypos, ...)."]
    #[inline(always)]
    pub fn taskselect(&self) -> TASKSELECT_R {
        TASKSELECT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Blit engine operation mode."]
    #[inline(always)]
    pub fn rendermode(&self) -> RENDERMODE_R {
        RENDERMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Proxy to use for the next operation"]
    #[inline(always)]
    pub fn proxyselect(&self) -> PROXYSELECT_R {
        PROXYSELECT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select task bank for operation status registers (ypos, ...)."]
    #[inline(always)]
    #[must_use]
    pub fn taskselect(&mut self) -> TASKSELECT_W<0> {
        TASKSELECT_W::new(self)
    }
    #[doc = "Bit 4 - Blit engine operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn rendermode(&mut self) -> RENDERMODE_W<4> {
        RENDERMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Proxy to use for the next operation"]
    #[inline(always)]
    #[must_use]
    pub fn proxyselect(&mut self) -> PROXYSELECT_W<8> {
        PROXYSELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasksetup](index.html) module"]
pub struct TASKSETUP_SPEC;
impl crate::RegisterSpec for TASKSETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tasksetup::R](R) reader structure"]
impl crate::Readable for TASKSETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tasksetup::W](W) writer structure"]
impl crate::Writable for TASKSETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKSETUP to value 0"]
impl crate::Resettable for TASKSETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
