#[doc = "Register `EVALCONTROL3` reader"]
pub struct R(crate::R<EVALCONTROL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL3` writer"]
pub struct W(crate::W<EVALCONTROL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL3_SPEC>;
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
impl From<crate::W<EVALCONTROL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN3` reader - See EnEvalWin0."]
pub type ENEVALWIN3_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN3` writer - See EnEvalWin0."]
pub type ENEVALWIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL3_SPEC, bool, O>;
#[doc = "Field `ENCRC3` reader - See EnCRC0."]
pub type ENCRC3_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC3` writer - See EnCRC0."]
pub type ENCRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL3_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK3` reader - See AlphaMask0."]
pub type ALPHAMASK3_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK3` writer - See AlphaMask0."]
pub type ALPHAMASK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL3_SPEC, bool, O>;
#[doc = "Field `ALPHAINV3` reader - See AlphaInv0."]
pub type ALPHAINV3_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV3` writer - See AlphaInv0."]
pub type ALPHAINV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL3_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC3` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC3_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC3` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL3_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC3` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC3_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC3` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin3(&self) -> ENEVALWIN3_R {
        ENEVALWIN3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc3(&self) -> ENCRC3_R {
        ENCRC3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask3(&self) -> ALPHAMASK3_R {
        ALPHAMASK3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv3(&self) -> ALPHAINV3_R {
        ALPHAINV3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic3(&self) -> ENLOCALPANIC3_R {
        ENLOCALPANIC3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic3(&self) -> ENGLOBALPANIC3_R {
        ENGLOBALPANIC3_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin3(&mut self) -> ENEVALWIN3_W<0> {
        ENEVALWIN3_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc3(&mut self) -> ENCRC3_W<1> {
        ENCRC3_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask3(&mut self) -> ALPHAMASK3_W<8> {
        ALPHAMASK3_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv3(&mut self) -> ALPHAINV3_W<9> {
        ALPHAINV3_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic3(&mut self) -> ENLOCALPANIC3_W<16> {
        ENLOCALPANIC3_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic3(&mut self) -> ENGLOBALPANIC3_W<17> {
        ENGLOBALPANIC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol3](index.html) module"]
pub struct EVALCONTROL3_SPEC;
impl crate::RegisterSpec for EVALCONTROL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol3::R](R) reader structure"]
impl crate::Readable for EVALCONTROL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol3::W](W) writer structure"]
impl crate::Writable for EVALCONTROL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL3 to value 0"]
impl crate::Resettable for EVALCONTROL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
