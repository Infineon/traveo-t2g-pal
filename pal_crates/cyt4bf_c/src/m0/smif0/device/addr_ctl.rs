#[doc = "Register `ADDR_CTL` reader"]
pub struct R(crate::R<ADDR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR_CTL` writer"]
pub struct W(crate::W<ADDR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_CTL_SPEC>;
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
impl From<crate::W<ADDR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE3` reader - N/A"]
pub type SIZE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE3` writer - N/A"]
pub type SIZE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIV2` reader - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If DIV2 is set to '1', the memory does not support write masking (WR_DUMMY_CTL.RWDS_EN = '0'), and in this configuration a write transfer is requested and the write transfer request address is NOT a multiple of 2 or the requested number of Bytes to be written is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type DIV2_R = crate::BitReader<bool>;
#[doc = "Field `DIV2` writer - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If DIV2 is set to '1', the memory does not support write masking (WR_DUMMY_CTL.RWDS_EN = '0'), and in this configuration a write transfer is requested and the write transfer request address is NOT a multiple of 2 or the requested number of Bytes to be written is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type DIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDR_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn size3(&self) -> SIZE3_R {
        SIZE3_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If DIV2 is set to '1', the memory does not support write masking (WR_DUMMY_CTL.RWDS_EN = '0'), and in this configuration a write transfer is requested and the write transfer request address is NOT a multiple of 2 or the requested number of Bytes to be written is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn size3(&mut self) -> SIZE3_W<0> {
        SIZE3_W::new(self)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If DIV2 is set to '1', the memory does not support write masking (WR_DUMMY_CTL.RWDS_EN = '0'), and in this configuration a write transfer is requested and the write transfer request address is NOT a multiple of 2 or the requested number of Bytes to be written is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    #[must_use]
    pub fn div2(&mut self) -> DIV2_W<8> {
        DIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_ctl](index.html) module"]
pub struct ADDR_CTL_SPEC;
impl crate::RegisterSpec for ADDR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_ctl::R](R) reader structure"]
impl crate::Readable for ADDR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_ctl::W](W) writer structure"]
impl crate::Writable for ADDR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR_CTL to value 0"]
impl crate::Resettable for ADDR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
