#[doc = "Register `EVALCONTROL7` reader"]
pub struct R(crate::R<EVALCONTROL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL7` writer"]
pub struct W(crate::W<EVALCONTROL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL7_SPEC>;
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
impl From<crate::W<EVALCONTROL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN7` reader - See EnEvalWin0."]
pub type ENEVALWIN7_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN7` writer - See EnEvalWin0."]
pub type ENEVALWIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL7_SPEC, bool, O>;
#[doc = "Field `ENCRC7` reader - See EnCRC0."]
pub type ENCRC7_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC7` writer - See EnCRC0."]
pub type ENCRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL7_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK7` reader - See AlphaMask0."]
pub type ALPHAMASK7_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK7` writer - See AlphaMask0."]
pub type ALPHAMASK7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL7_SPEC, bool, O>;
#[doc = "Field `ALPHAINV7` reader - See AlphaInv0."]
pub type ALPHAINV7_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV7` writer - See AlphaInv0."]
pub type ALPHAINV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL7_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC7` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC7_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC7` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL7_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC7` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC7_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC7` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL7_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin7(&self) -> ENEVALWIN7_R {
        ENEVALWIN7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc7(&self) -> ENCRC7_R {
        ENCRC7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask7(&self) -> ALPHAMASK7_R {
        ALPHAMASK7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv7(&self) -> ALPHAINV7_R {
        ALPHAINV7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic7(&self) -> ENLOCALPANIC7_R {
        ENLOCALPANIC7_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic7(&self) -> ENGLOBALPANIC7_R {
        ENGLOBALPANIC7_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin7(&mut self) -> ENEVALWIN7_W<0> {
        ENEVALWIN7_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc7(&mut self) -> ENCRC7_W<1> {
        ENCRC7_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask7(&mut self) -> ALPHAMASK7_W<8> {
        ALPHAMASK7_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv7(&mut self) -> ALPHAINV7_W<9> {
        ALPHAINV7_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic7(&mut self) -> ENLOCALPANIC7_W<16> {
        ENLOCALPANIC7_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic7(&mut self) -> ENGLOBALPANIC7_W<17> {
        ENGLOBALPANIC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol7](index.html) module"]
pub struct EVALCONTROL7_SPEC;
impl crate::RegisterSpec for EVALCONTROL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol7::R](R) reader structure"]
impl crate::Readable for EVALCONTROL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol7::W](W) writer structure"]
impl crate::Writable for EVALCONTROL7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL7 to value 0"]
impl crate::Resettable for EVALCONTROL7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
