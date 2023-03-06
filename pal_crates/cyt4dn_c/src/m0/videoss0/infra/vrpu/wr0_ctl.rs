#[doc = "Register `WR0_CTL` reader"]
pub struct R(crate::R<WR0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR0_CTL` writer"]
pub struct W(crate::W<WR0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR0_CTL_SPEC>;
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
impl From<crate::W<WR0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIO` reader - See RD0_CTL.PRIO"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - See RD0_CTL.PRIO"]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR0_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 8:9 - See RD0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - See RD0_CTL.PRIO"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<8> {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VRAM Protection for write master with ID=0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr0_ctl](index.html) module"]
pub struct WR0_CTL_SPEC;
impl crate::RegisterSpec for WR0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr0_ctl::R](R) reader structure"]
impl crate::Readable for WR0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr0_ctl::W](W) writer structure"]
impl crate::Writable for WR0_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR0_CTL to value 0x0300"]
impl crate::Resettable for WR0_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
