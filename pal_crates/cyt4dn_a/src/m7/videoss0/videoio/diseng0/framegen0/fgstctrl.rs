#[doc = "Register `FGSTCTRL` reader"]
pub struct R(crate::R<FGSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSTCTRL` writer"]
pub struct W(crate::W<FGSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSTCTRL_SPEC>;
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
impl From<crate::W<FGSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDEN` reader - Enables shadowing for RWS type configuration fields. Otherwise shadow registers are immediately updated when written."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enables shadowing for RWS type configuration fields. Otherwise shadow registers are immediately updated when written."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSTCTRL_SPEC, bool, O>;
#[doc = "Field `FGSYNCMODE` reader - Determines the operating mode of the framegen unit for side-by-side synchronization. The framegen unit programmed as master generates synchronization pulses. The framegen units programmed as slaves are synchronized to the master unit. There must be only one framgen unit programmed acting as master."]
pub type FGSYNCMODE_R = crate::FieldReader<u8, FGSYNCMODE_A>;
#[doc = "Determines the operating mode of the framegen unit for side-by-side synchronization. The framegen unit programmed as master generates synchronization pulses. The framegen units programmed as slaves are synchronized to the master unit. There must be only one framgen unit programmed acting as master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FGSYNCMODE_A {
    #[doc = "0: No side-by-side synchronization."]
    OFF = 0,
    #[doc = "1: Framegen is master."]
    MASTER = 1,
    #[doc = "2: Framegen is slave. Runs in cyclic synchronization mode."]
    SLAVE_CYC = 2,
    #[doc = "3: Framegen is slave. Runs in one time synchronization mode."]
    SLAVE_ONCE = 3,
}
impl From<FGSYNCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FGSYNCMODE_A) -> Self {
        variant as _
    }
}
impl FGSYNCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGSYNCMODE_A {
        match self.bits {
            0 => FGSYNCMODE_A::OFF,
            1 => FGSYNCMODE_A::MASTER,
            2 => FGSYNCMODE_A::SLAVE_CYC,
            3 => FGSYNCMODE_A::SLAVE_ONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FGSYNCMODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == FGSYNCMODE_A::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE_CYC`"]
    #[inline(always)]
    pub fn is_slave_cyc(&self) -> bool {
        *self == FGSYNCMODE_A::SLAVE_CYC
    }
    #[doc = "Checks if the value of the field is `SLAVE_ONCE`"]
    #[inline(always)]
    pub fn is_slave_once(&self) -> bool {
        *self == FGSYNCMODE_A::SLAVE_ONCE
    }
}
#[doc = "Field `FGSYNCMODE` writer - Determines the operating mode of the framegen unit for side-by-side synchronization. The framegen unit programmed as master generates synchronization pulses. The framegen units programmed as slaves are synchronized to the master unit. There must be only one framgen unit programmed acting as master."]
pub type FGSYNCMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FGSTCTRL_SPEC, u8, FGSYNCMODE_A, 2, O>;
impl<'a, const O: u8> FGSYNCMODE_W<'a, O> {
    #[doc = "No side-by-side synchronization."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FGSYNCMODE_A::OFF)
    }
    #[doc = "Framegen is master."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(FGSYNCMODE_A::MASTER)
    }
    #[doc = "Framegen is slave. Runs in cyclic synchronization mode."]
    #[inline(always)]
    pub fn slave_cyc(self) -> &'a mut W {
        self.variant(FGSYNCMODE_A::SLAVE_CYC)
    }
    #[doc = "Framegen is slave. Runs in one time synchronization mode."]
    #[inline(always)]
    pub fn slave_once(self) -> &'a mut W {
        self.variant(FGSYNCMODE_A::SLAVE_ONCE)
    }
}
impl R {
    #[doc = "Bit 0 - Enables shadowing for RWS type configuration fields. Otherwise shadow registers are immediately updated when written."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Determines the operating mode of the framegen unit for side-by-side synchronization. The framegen unit programmed as master generates synchronization pulses. The framegen units programmed as slaves are synchronized to the master unit. There must be only one framgen unit programmed acting as master."]
    #[inline(always)]
    pub fn fgsyncmode(&self) -> FGSYNCMODE_R {
        FGSYNCMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing for RWS type configuration fields. Otherwise shadow registers are immediately updated when written."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Determines the operating mode of the framegen unit for side-by-side synchronization. The framegen unit programmed as master generates synchronization pulses. The framegen units programmed as slaves are synchronized to the master unit. There must be only one framgen unit programmed acting as master."]
    #[inline(always)]
    #[must_use]
    pub fn fgsyncmode(&mut self) -> FGSYNCMODE_W<1> {
        FGSYNCMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Static Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgstctrl](index.html) module"]
pub struct FGSTCTRL_SPEC;
impl crate::RegisterSpec for FGSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgstctrl::R](R) reader structure"]
impl crate::Readable for FGSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgstctrl::W](W) writer structure"]
impl crate::Writable for FGSTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSTCTRL to value 0"]
impl crate::Resettable for FGSTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
