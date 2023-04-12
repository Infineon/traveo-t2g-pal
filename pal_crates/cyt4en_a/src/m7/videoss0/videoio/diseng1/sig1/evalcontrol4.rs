#[doc = "Register `EVALCONTROL4` reader"]
pub struct R(crate::R<EVALCONTROL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL4` writer"]
pub struct W(crate::W<EVALCONTROL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL4_SPEC>;
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
impl From<crate::W<EVALCONTROL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN4` reader - See EnEvalWin0."]
pub type ENEVALWIN4_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN4` writer - See EnEvalWin0."]
pub type ENEVALWIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL4_SPEC, bool, O>;
#[doc = "Field `ENCRC4` reader - See EnCRC0."]
pub type ENCRC4_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC4` writer - See EnCRC0."]
pub type ENCRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL4_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK4` reader - See AlphaMask0."]
pub type ALPHAMASK4_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK4` writer - See AlphaMask0."]
pub type ALPHAMASK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL4_SPEC, bool, O>;
#[doc = "Field `ALPHAINV4` reader - See AlphaInv0."]
pub type ALPHAINV4_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV4` writer - See AlphaInv0."]
pub type ALPHAINV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL4_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC4` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC4_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC4` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL4_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC4` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC4_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC4` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin4(&self) -> ENEVALWIN4_R {
        ENEVALWIN4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc4(&self) -> ENCRC4_R {
        ENCRC4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask4(&self) -> ALPHAMASK4_R {
        ALPHAMASK4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv4(&self) -> ALPHAINV4_R {
        ALPHAINV4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic4(&self) -> ENLOCALPANIC4_R {
        ENLOCALPANIC4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic4(&self) -> ENGLOBALPANIC4_R {
        ENGLOBALPANIC4_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin4(&mut self) -> ENEVALWIN4_W<0> {
        ENEVALWIN4_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc4(&mut self) -> ENCRC4_W<1> {
        ENCRC4_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask4(&mut self) -> ALPHAMASK4_W<8> {
        ALPHAMASK4_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv4(&mut self) -> ALPHAINV4_W<9> {
        ALPHAINV4_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic4(&mut self) -> ENLOCALPANIC4_W<16> {
        ENLOCALPANIC4_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic4(&mut self) -> ENGLOBALPANIC4_W<17> {
        ENGLOBALPANIC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol4](index.html) module"]
pub struct EVALCONTROL4_SPEC;
impl crate::RegisterSpec for EVALCONTROL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol4::R](R) reader structure"]
impl crate::Readable for EVALCONTROL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol4::W](W) writer structure"]
impl crate::Writable for EVALCONTROL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL4 to value 0"]
impl crate::Resettable for EVALCONTROL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
