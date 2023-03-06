#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operation mode for rop"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Operation mode for rop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Neutral mode"]
    NEUTRAL = 0,
    #[doc = "1: Normal Operation"]
    OPERATION = 1,
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
            true => MODE_A::OPERATION,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == MODE_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == MODE_A::OPERATION
    }
}
#[doc = "Field `MODE` writer - Operation mode for rop"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Neutral mode"]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(MODE_A::NEUTRAL)
    }
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(MODE_A::OPERATION)
    }
}
#[doc = "Field `ALPHAMODE` reader - Selects the mode for the alpha component channel, has no effect in NEUTRAL mode"]
pub type ALPHAMODE_R = crate::BitReader<ALPHAMODE_A>;
#[doc = "Selects the mode for the alpha component channel, has no effect in NEUTRAL mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALPHAMODE_A {
    #[doc = "0: Normal raster operation mode, using the operation index"]
    ROP = 0,
    #[doc = "1: Add mode, adds this component from all enabled inputs, clamps to 1"]
    ADD = 1,
}
impl From<ALPHAMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ALPHAMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALPHAMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALPHAMODE_A {
        match self.bits {
            false => ALPHAMODE_A::ROP,
            true => ALPHAMODE_A::ADD,
        }
    }
    #[doc = "Checks if the value of the field is `ROP`"]
    #[inline(always)]
    pub fn is_rop(&self) -> bool {
        *self == ALPHAMODE_A::ROP
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == ALPHAMODE_A::ADD
    }
}
#[doc = "Field `ALPHAMODE` writer - Selects the mode for the alpha component channel, has no effect in NEUTRAL mode"]
pub type ALPHAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, ALPHAMODE_A, O>;
impl<'a, const O: u8> ALPHAMODE_W<'a, O> {
    #[doc = "Normal raster operation mode, using the operation index"]
    #[inline(always)]
    pub fn rop(self) -> &'a mut W {
        self.variant(ALPHAMODE_A::ROP)
    }
    #[doc = "Add mode, adds this component from all enabled inputs, clamps to 1"]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(ALPHAMODE_A::ADD)
    }
}
#[doc = "Field `BLUEMODE` reader - Selects the mode for the blue component channel, has no effect in NEUTRAL mode"]
pub type BLUEMODE_R = crate::BitReader<BLUEMODE_A>;
#[doc = "Selects the mode for the blue component channel, has no effect in NEUTRAL mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLUEMODE_A {
    #[doc = "0: Normal raster operation mode, using the operation index"]
    ROP = 0,
    #[doc = "1: Add mode, adds this component from all enabled inputs, clamps to 1"]
    ADD = 1,
}
impl From<BLUEMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BLUEMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BLUEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLUEMODE_A {
        match self.bits {
            false => BLUEMODE_A::ROP,
            true => BLUEMODE_A::ADD,
        }
    }
    #[doc = "Checks if the value of the field is `ROP`"]
    #[inline(always)]
    pub fn is_rop(&self) -> bool {
        *self == BLUEMODE_A::ROP
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == BLUEMODE_A::ADD
    }
}
#[doc = "Field `BLUEMODE` writer - Selects the mode for the blue component channel, has no effect in NEUTRAL mode"]
pub type BLUEMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, BLUEMODE_A, O>;
impl<'a, const O: u8> BLUEMODE_W<'a, O> {
    #[doc = "Normal raster operation mode, using the operation index"]
    #[inline(always)]
    pub fn rop(self) -> &'a mut W {
        self.variant(BLUEMODE_A::ROP)
    }
    #[doc = "Add mode, adds this component from all enabled inputs, clamps to 1"]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(BLUEMODE_A::ADD)
    }
}
#[doc = "Field `GREENMODE` reader - Selects the mode for the green component channel, has no effect in NEUTRAL mode"]
pub type GREENMODE_R = crate::BitReader<GREENMODE_A>;
#[doc = "Selects the mode for the green component channel, has no effect in NEUTRAL mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GREENMODE_A {
    #[doc = "0: Normal raster operation mode, using the operation index"]
    ROP = 0,
    #[doc = "1: Add mode, adds this component from all enabled inputs, clamps to 1"]
    ADD = 1,
}
impl From<GREENMODE_A> for bool {
    #[inline(always)]
    fn from(variant: GREENMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl GREENMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GREENMODE_A {
        match self.bits {
            false => GREENMODE_A::ROP,
            true => GREENMODE_A::ADD,
        }
    }
    #[doc = "Checks if the value of the field is `ROP`"]
    #[inline(always)]
    pub fn is_rop(&self) -> bool {
        *self == GREENMODE_A::ROP
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == GREENMODE_A::ADD
    }
}
#[doc = "Field `GREENMODE` writer - Selects the mode for the green component channel, has no effect in NEUTRAL mode"]
pub type GREENMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, GREENMODE_A, O>;
impl<'a, const O: u8> GREENMODE_W<'a, O> {
    #[doc = "Normal raster operation mode, using the operation index"]
    #[inline(always)]
    pub fn rop(self) -> &'a mut W {
        self.variant(GREENMODE_A::ROP)
    }
    #[doc = "Add mode, adds this component from all enabled inputs, clamps to 1"]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(GREENMODE_A::ADD)
    }
}
#[doc = "Field `REDMODE` reader - Selects the mode for the red component channel, has no effect in NEUTRAL mode"]
pub type REDMODE_R = crate::BitReader<REDMODE_A>;
#[doc = "Selects the mode for the red component channel, has no effect in NEUTRAL mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REDMODE_A {
    #[doc = "0: Normal raster operation mode, using the operation index"]
    ROP = 0,
    #[doc = "1: Add mode, adds this component from all enabled inputs, clamps to 1"]
    ADD = 1,
}
impl From<REDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: REDMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REDMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REDMODE_A {
        match self.bits {
            false => REDMODE_A::ROP,
            true => REDMODE_A::ADD,
        }
    }
    #[doc = "Checks if the value of the field is `ROP`"]
    #[inline(always)]
    pub fn is_rop(&self) -> bool {
        *self == REDMODE_A::ROP
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == REDMODE_A::ADD
    }
}
#[doc = "Field `REDMODE` writer - Selects the mode for the red component channel, has no effect in NEUTRAL mode"]
pub type REDMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, REDMODE_A, O>;
impl<'a, const O: u8> REDMODE_W<'a, O> {
    #[doc = "Normal raster operation mode, using the operation index"]
    #[inline(always)]
    pub fn rop(self) -> &'a mut W {
        self.variant(REDMODE_A::ROP)
    }
    #[doc = "Add mode, adds this component from all enabled inputs, clamps to 1"]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(REDMODE_A::ADD)
    }
}
#[doc = "Field `PRIMDIV2` reader - Selects whether to divide the primary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
pub type PRIMDIV2_R = crate::BitReader<PRIMDIV2_A>;
#[doc = "Selects whether to divide the primary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIMDIV2_A {
    #[doc = "0: No change to input"]
    BYPASS = 0,
    #[doc = "1: Input is divided by two/shift to the right by one"]
    DIVIDEBY2 = 1,
}
impl From<PRIMDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: PRIMDIV2_A) -> Self {
        variant as u8 != 0
    }
}
impl PRIMDIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIMDIV2_A {
        match self.bits {
            false => PRIMDIV2_A::BYPASS,
            true => PRIMDIV2_A::DIVIDEBY2,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == PRIMDIV2_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY2`"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        *self == PRIMDIV2_A::DIVIDEBY2
    }
}
#[doc = "Field `PRIMDIV2` writer - Selects whether to divide the primary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
pub type PRIMDIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, PRIMDIV2_A, O>;
impl<'a, const O: u8> PRIMDIV2_W<'a, O> {
    #[doc = "No change to input"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(PRIMDIV2_A::BYPASS)
    }
    #[doc = "Input is divided by two/shift to the right by one"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut W {
        self.variant(PRIMDIV2_A::DIVIDEBY2)
    }
}
#[doc = "Field `SECDIV2` reader - Selects whether to divide the secondary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
pub type SECDIV2_R = crate::BitReader<SECDIV2_A>;
#[doc = "Selects whether to divide the secondary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECDIV2_A {
    #[doc = "0: No change to input"]
    BYPASS = 0,
    #[doc = "1: Input is divided by two/shift to the right by one"]
    DIVIDEBY2 = 1,
}
impl From<SECDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: SECDIV2_A) -> Self {
        variant as u8 != 0
    }
}
impl SECDIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECDIV2_A {
        match self.bits {
            false => SECDIV2_A::BYPASS,
            true => SECDIV2_A::DIVIDEBY2,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == SECDIV2_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY2`"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        *self == SECDIV2_A::DIVIDEBY2
    }
}
#[doc = "Field `SECDIV2` writer - Selects whether to divide the secondary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
pub type SECDIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, SECDIV2_A, O>;
impl<'a, const O: u8> SECDIV2_W<'a, O> {
    #[doc = "No change to input"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(SECDIV2_A::BYPASS)
    }
    #[doc = "Input is divided by two/shift to the right by one"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut W {
        self.variant(SECDIV2_A::DIVIDEBY2)
    }
}
#[doc = "Field `TERTDIV2` reader - Selects whether to divide the tertiary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
pub type TERTDIV2_R = crate::BitReader<TERTDIV2_A>;
#[doc = "Selects whether to divide the tertiary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERTDIV2_A {
    #[doc = "0: No change to input"]
    BYPASS = 0,
    #[doc = "1: Input is divided by two/shift to the right by one"]
    DIVIDEBY2 = 1,
}
impl From<TERTDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: TERTDIV2_A) -> Self {
        variant as u8 != 0
    }
}
impl TERTDIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERTDIV2_A {
        match self.bits {
            false => TERTDIV2_A::BYPASS,
            true => TERTDIV2_A::DIVIDEBY2,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == TERTDIV2_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY2`"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        *self == TERTDIV2_A::DIVIDEBY2
    }
}
#[doc = "Field `TERTDIV2` writer - Selects whether to divide the tertiary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
pub type TERTDIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, TERTDIV2_A, O>;
impl<'a, const O: u8> TERTDIV2_W<'a, O> {
    #[doc = "No change to input"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(TERTDIV2_A::BYPASS)
    }
    #[doc = "Input is divided by two/shift to the right by one"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut W {
        self.variant(TERTDIV2_A::DIVIDEBY2)
    }
}
impl R {
    #[doc = "Bit 0 - Operation mode for rop"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the mode for the alpha component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    pub fn alphamode(&self) -> ALPHAMODE_R {
        ALPHAMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects the mode for the blue component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    pub fn bluemode(&self) -> BLUEMODE_R {
        BLUEMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects the mode for the green component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    pub fn greenmode(&self) -> GREENMODE_R {
        GREENMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the mode for the red component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    pub fn redmode(&self) -> REDMODE_R {
        REDMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects whether to divide the primary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
    #[inline(always)]
    pub fn primdiv2(&self) -> PRIMDIV2_R {
        PRIMDIV2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects whether to divide the secondary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
    #[inline(always)]
    pub fn secdiv2(&self) -> SECDIV2_R {
        SECDIV2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects whether to divide the tertiary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
    #[inline(always)]
    pub fn tertdiv2(&self) -> TERTDIV2_R {
        TERTDIV2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation mode for rop"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Selects the mode for the alpha component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    #[must_use]
    pub fn alphamode(&mut self) -> ALPHAMODE_W<4> {
        ALPHAMODE_W::new(self)
    }
    #[doc = "Bit 5 - Selects the mode for the blue component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    #[must_use]
    pub fn bluemode(&mut self) -> BLUEMODE_W<5> {
        BLUEMODE_W::new(self)
    }
    #[doc = "Bit 6 - Selects the mode for the green component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    #[must_use]
    pub fn greenmode(&mut self) -> GREENMODE_W<6> {
        GREENMODE_W::new(self)
    }
    #[doc = "Bit 7 - Selects the mode for the red component channel, has no effect in NEUTRAL mode"]
    #[inline(always)]
    #[must_use]
    pub fn redmode(&mut self) -> REDMODE_W<7> {
        REDMODE_W::new(self)
    }
    #[doc = "Bit 8 - Selects whether to divide the primary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
    #[inline(always)]
    #[must_use]
    pub fn primdiv2(&mut self) -> PRIMDIV2_W<8> {
        PRIMDIV2_W::new(self)
    }
    #[doc = "Bit 9 - Selects whether to divide the secondary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
    #[inline(always)]
    #[must_use]
    pub fn secdiv2(&mut self) -> SECDIV2_W<9> {
        SECDIV2_W::new(self)
    }
    #[doc = "Bit 10 - Selects whether to divide the tertiary input color components by two or not for ADD mode. This field has no effect on a color component in ROP mode."]
    #[inline(always)]
    #[must_use]
    pub fn tertdiv2(&mut self) -> TERTDIV2_W<10> {
        TERTDIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raster Operation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
