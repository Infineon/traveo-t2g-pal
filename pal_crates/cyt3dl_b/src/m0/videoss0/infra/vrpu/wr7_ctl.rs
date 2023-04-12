#[doc = "Register `WR7_CTL` reader"]
pub struct R(crate::R<WR7_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR7_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR7_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR7_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR7_CTL` writer"]
pub struct W(crate::W<WR7_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR7_CTL_SPEC>;
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
impl From<crate::W<WR7_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR7_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - See RD1_CTL.P."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - See RD1_CTL.P."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR7_CTL_SPEC, bool, O>;
#[doc = "Field `NS` reader - See RD1_CTL.NS."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `NS` writer - See RD1_CTL.NS."]
pub type NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR7_CTL_SPEC, bool, O>;
#[doc = "Field `PRIO` reader - See RD0_CTL.PRIO"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - See RD0_CTL.PRIO"]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR7_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PC_MASK_0` reader - See RD1_CTL.PC_MASK_0."]
pub type PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `PC_MASK_15_TO_1` reader - See WR1_CTL.PC_MASK_15_TO_1."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - See WR1_CTL.PC_MASK_15_TO_1."]
pub type PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WR7_CTL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - See RD1_CTL.P."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See RD1_CTL.NS."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - See RD0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See RD1_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - See WR1_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - See RD1_CTL.P."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bit 1 - See RD1_CTL.NS."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<1> {
        NS_W::new(self)
    }
    #[doc = "Bits 8:9 - See RD0_CTL.PRIO"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<8> {
        PRIO_W::new(self)
    }
    #[doc = "Bits 17:31 - See WR1_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<17> {
        PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VRAM Protection for write master with ID=7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr7_ctl](index.html) module"]
pub struct WR7_CTL_SPEC;
impl crate::RegisterSpec for WR7_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr7_ctl::R](R) reader structure"]
impl crate::Readable for WR7_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr7_ctl::W](W) writer structure"]
impl crate::Writable for WR7_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR7_CTL to value 0x0302"]
impl crate::Resettable for WR7_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302;
}