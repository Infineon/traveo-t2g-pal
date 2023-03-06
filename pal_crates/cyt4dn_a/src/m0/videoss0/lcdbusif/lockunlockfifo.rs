#[doc = "Register `LOCKUNLOCKFIFO` reader"]
pub struct R(crate::R<LOCKUNLOCKFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKUNLOCKFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKUNLOCKFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKUNLOCKFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKUNLOCKFIFO` writer"]
pub struct W(crate::W<LOCKUNLOCKFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKUNLOCKFIFO_SPEC>;
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
impl From<crate::W<LOCKUNLOCKFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKUNLOCKFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUNLOCKFIFO` reader - The protection status is changed by writing one of the following key values to this field:"]
pub type LOCKUNLOCKFIFO_R = crate::FieldReader<u32, LOCKUNLOCKFIFO_A>;
#[doc = "The protection status is changed by writing one of the following key values to this field:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum LOCKUNLOCKFIFO_A {
    #[doc = "1448212323: Decrements the unlock counter. When the counter value is null, lock protection is active. Reset counter value is 1."]
    LOCK_KEY = 1448212323,
    #[doc = "1763555638: Increments the unlock counter. Max allowed value is 15."]
    UNLOCK_KEY = 1763555638,
    #[doc = "2934529244: Enables privilege protection. Disabled after reset."]
    PRIVILEGE_KEY = 2934529244,
    #[doc = "3051505262: Disables privilege protection."]
    UNPRIVILEGE_KEY = 3051505262,
    #[doc = "4226331110: Freezes current protection status. Writing keys to this register has no more effect until reset."]
    FREEZE_KEY = 4226331110,
}
impl From<LOCKUNLOCKFIFO_A> for u32 {
    #[inline(always)]
    fn from(variant: LOCKUNLOCKFIFO_A) -> Self {
        variant as _
    }
}
impl LOCKUNLOCKFIFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCKUNLOCKFIFO_A> {
        match self.bits {
            1448212323 => Some(LOCKUNLOCKFIFO_A::LOCK_KEY),
            1763555638 => Some(LOCKUNLOCKFIFO_A::UNLOCK_KEY),
            2934529244 => Some(LOCKUNLOCKFIFO_A::PRIVILEGE_KEY),
            3051505262 => Some(LOCKUNLOCKFIFO_A::UNPRIVILEGE_KEY),
            4226331110 => Some(LOCKUNLOCKFIFO_A::FREEZE_KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_KEY`"]
    #[inline(always)]
    pub fn is_lock_key(&self) -> bool {
        *self == LOCKUNLOCKFIFO_A::LOCK_KEY
    }
    #[doc = "Checks if the value of the field is `UNLOCK_KEY`"]
    #[inline(always)]
    pub fn is_unlock_key(&self) -> bool {
        *self == LOCKUNLOCKFIFO_A::UNLOCK_KEY
    }
    #[doc = "Checks if the value of the field is `PRIVILEGE_KEY`"]
    #[inline(always)]
    pub fn is_privilege_key(&self) -> bool {
        *self == LOCKUNLOCKFIFO_A::PRIVILEGE_KEY
    }
    #[doc = "Checks if the value of the field is `UNPRIVILEGE_KEY`"]
    #[inline(always)]
    pub fn is_unprivilege_key(&self) -> bool {
        *self == LOCKUNLOCKFIFO_A::UNPRIVILEGE_KEY
    }
    #[doc = "Checks if the value of the field is `FREEZE_KEY`"]
    #[inline(always)]
    pub fn is_freeze_key(&self) -> bool {
        *self == LOCKUNLOCKFIFO_A::FREEZE_KEY
    }
}
#[doc = "Field `LOCKUNLOCKFIFO` writer - The protection status is changed by writing one of the following key values to this field:"]
pub type LOCKUNLOCKFIFO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOCKUNLOCKFIFO_SPEC, u32, LOCKUNLOCKFIFO_A, 32, O>;
impl<'a, const O: u8> LOCKUNLOCKFIFO_W<'a, O> {
    #[doc = "Decrements the unlock counter. When the counter value is null, lock protection is active. Reset counter value is 1."]
    #[inline(always)]
    pub fn lock_key(self) -> &'a mut W {
        self.variant(LOCKUNLOCKFIFO_A::LOCK_KEY)
    }
    #[doc = "Increments the unlock counter. Max allowed value is 15."]
    #[inline(always)]
    pub fn unlock_key(self) -> &'a mut W {
        self.variant(LOCKUNLOCKFIFO_A::UNLOCK_KEY)
    }
    #[doc = "Enables privilege protection. Disabled after reset."]
    #[inline(always)]
    pub fn privilege_key(self) -> &'a mut W {
        self.variant(LOCKUNLOCKFIFO_A::PRIVILEGE_KEY)
    }
    #[doc = "Disables privilege protection."]
    #[inline(always)]
    pub fn unprivilege_key(self) -> &'a mut W {
        self.variant(LOCKUNLOCKFIFO_A::UNPRIVILEGE_KEY)
    }
    #[doc = "Freezes current protection status. Writing keys to this register has no more effect until reset."]
    #[inline(always)]
    pub fn freeze_key(self) -> &'a mut W {
        self.variant(LOCKUNLOCKFIFO_A::FREEZE_KEY)
    }
}
impl R {
    #[doc = "Bits 0:31 - The protection status is changed by writing one of the following key values to this field:"]
    #[inline(always)]
    pub fn lockunlockfifo(&self) -> LOCKUNLOCKFIFO_R {
        LOCKUNLOCKFIFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The protection status is changed by writing one of the following key values to this field:"]
    #[inline(always)]
    #[must_use]
    pub fn lockunlockfifo(&mut self) -> LOCKUNLOCKFIFO_W<0> {
        LOCKUNLOCKFIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to change the protection status of the fifo address blocks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockunlockfifo](index.html) module"]
pub struct LOCKUNLOCKFIFO_SPEC;
impl crate::RegisterSpec for LOCKUNLOCKFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockunlockfifo::R](R) reader structure"]
impl crate::Readable for LOCKUNLOCKFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockunlockfifo::W](W) writer structure"]
impl crate::Writable for LOCKUNLOCKFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKUNLOCKFIFO to value 0"]
impl crate::Resettable for LOCKUNLOCKFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
