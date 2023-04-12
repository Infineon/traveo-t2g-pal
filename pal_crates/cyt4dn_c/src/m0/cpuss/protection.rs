#[doc = "Register `PROTECTION` reader"]
pub struct R(crate::R<PROTECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECTION` writer"]
pub struct W(crate::W<PROTECTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECTION_SPEC>;
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
impl From<crate::W<PROTECTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE` reader - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transistions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
#[doc = "Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transistions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: N/A"]
    UNKNOWN = 0,
    #[doc = "1: N/A"]
    VIRGIN = 1,
    #[doc = "2: N/A"]
    NORMAL = 2,
    #[doc = "3: N/A"]
    SECURE = 3,
    #[doc = "4: N/A"]
    DEAD = 4,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::UNKNOWN),
            1 => Some(STATE_A::VIRGIN),
            2 => Some(STATE_A::NORMAL),
            3 => Some(STATE_A::SECURE),
            4 => Some(STATE_A::DEAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN`"]
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        *self == STATE_A::UNKNOWN
    }
    #[doc = "Checks if the value of the field is `VIRGIN`"]
    #[inline(always)]
    pub fn is_virgin(&self) -> bool {
        *self == STATE_A::VIRGIN
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == STATE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == STATE_A::SECURE
    }
    #[doc = "Checks if the value of the field is `DEAD`"]
    #[inline(always)]
    pub fn is_dead(&self) -> bool {
        *self == STATE_A::DEAD
    }
}
#[doc = "Field `STATE` writer - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transistions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROTECTION_SPEC, u8, STATE_A, 3, O>;
impl<'a, const O: u8> STATE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn unknown(self) -> &'a mut W {
        self.variant(STATE_A::UNKNOWN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn virgin(self) -> &'a mut W {
        self.variant(STATE_A::VIRGIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(STATE_A::NORMAL)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(STATE_A::SECURE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn dead(self) -> &'a mut W {
        self.variant(STATE_A::DEAD)
    }
}
impl R {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transistions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transistions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<0> {
        STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protection status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protection](index.html) module"]
pub struct PROTECTION_SPEC;
impl crate::RegisterSpec for PROTECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protection::R](R) reader structure"]
impl crate::Readable for PROTECTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protection::W](W) writer structure"]
impl crate::Writable for PROTECTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROTECTION to value 0"]
impl crate::Resettable for PROTECTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
