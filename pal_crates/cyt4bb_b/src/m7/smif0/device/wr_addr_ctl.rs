#[doc = "Register `WR_ADDR_CTL` reader"]
pub struct R(crate::R<WR_ADDR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_ADDR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_ADDR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_ADDR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_ADDR_CTL` writer"]
pub struct W(crate::W<WR_ADDR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_ADDR_CTL_SPEC>;
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
impl From<crate::W<WR_ADDR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_ADDR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_ADDR_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DDR_MODE` reader - Mode of transfer rate."]
pub type DDR_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DDR_MODE` writer - Mode of transfer rate."]
pub type DDR_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR_ADDR_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Mode of transfer rate."]
    #[inline(always)]
    pub fn ddr_mode(&self) -> DDR_MODE_R {
        DDR_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<16> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 18 - Mode of transfer rate."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_mode(&mut self) -> DDR_MODE_W<18> {
        DDR_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_addr_ctl](index.html) module"]
pub struct WR_ADDR_CTL_SPEC;
impl crate::RegisterSpec for WR_ADDR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_addr_ctl::R](R) reader structure"]
impl crate::Readable for WR_ADDR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_addr_ctl::W](W) writer structure"]
impl crate::Writable for WR_ADDR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_ADDR_CTL to value 0"]
impl crate::Resettable for WR_ADDR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
