#[doc = "Register `TTTMK` reader"]
pub struct R(crate::R<TTTMK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTTMK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTTMK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTTMK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTTMK` writer"]
pub struct W(crate::W<TTTMK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTTMK_SPEC>;
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
impl From<crate::W<TTTMK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTTMK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TM_` reader - Time Mark 0x0000-FFFF Time Mark"]
pub type TM__R = crate::FieldReader<u16, u16>;
#[doc = "Field `TM_` writer - Time Mark 0x0000-FFFF Time Mark"]
pub type TM__W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTMK_SPEC, u16, u16, 16, O>;
#[doc = "Field `TICC` reader - Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
pub type TICC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TICC` writer - Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
pub type TICC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTMK_SPEC, u8, u8, 7, O>;
#[doc = "Field `LCKM` reader - TT Time Mark Register Locked Always set by a write access to registers TTOCN. Set by write access to register TTTMK when TTOCN.TMC != '00'. Reset when the registers have been synchronized into the CAN clock domain. 0= Write access to TTTMK enabled 1= Write access to TTTMK locked"]
pub type LCKM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Time Mark 0x0000-FFFF Time Mark"]
    #[inline(always)]
    pub fn tm_(&self) -> TM__R {
        TM__R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked Always set by a write access to registers TTOCN. Set by write access to register TTTMK when TTOCN.TMC != '00'. Reset when the registers have been synchronized into the CAN clock domain. 0= Write access to TTTMK enabled 1= Write access to TTTMK locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark 0x0000-FFFF Time Mark"]
    #[inline(always)]
    #[must_use]
    pub fn tm_(&mut self) -> TM__W<0> {
        TM__W::new(self)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
    #[inline(always)]
    #[must_use]
    pub fn ticc(&mut self) -> TICC_W<16> {
        TICC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TT Time Mark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tttmk](index.html) module"]
pub struct TTTMK_SPEC;
impl crate::RegisterSpec for TTTMK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tttmk::R](R) reader structure"]
impl crate::Readable for TTTMK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tttmk::W](W) writer structure"]
impl crate::Writable for TTTMK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTTMK to value 0"]
impl crate::Resettable for TTTMK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
