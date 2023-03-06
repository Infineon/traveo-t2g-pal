#[doc = "Register `ECC_ERR_INJ` reader"]
pub struct R(crate::R<ECC_ERR_INJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_ERR_INJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_ERR_INJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_ERR_INJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_ERR_INJ` writer"]
pub struct W(crate::W<ECC_ERR_INJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_ERR_INJ_SPEC>;
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
impl From<crate::W<ECC_ERR_INJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_ERR_INJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR_ADDR` reader - Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
pub type ERR_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ERR_ADDR` writer - Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
pub type ERR_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECC_ERR_INJ_SPEC, u16, u16, 14, O>;
#[doc = "Field `ERR_EN` reader - Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
pub type ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `ERR_EN` writer - Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
pub type ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECC_ERR_INJ_SPEC, bool, O>;
#[doc = "Field `ERR_PAR` reader - ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
pub type ERR_PAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERR_PAR` writer - ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
pub type ERR_PAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECC_ERR_INJ_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 2:15 - Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
    #[inline(always)]
    pub fn err_addr(&self) -> ERR_ADDR_R {
        ERR_ADDR_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 20 - Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
    #[inline(always)]
    pub fn err_en(&self) -> ERR_EN_R {
        ERR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:30 - ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
    #[inline(always)]
    pub fn err_par(&self) -> ERR_PAR_R {
        ERR_PAR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
    #[inline(always)]
    #[must_use]
    pub fn err_addr(&mut self) -> ERR_ADDR_W<2> {
        ERR_ADDR_W::new(self)
    }
    #[doc = "Bit 20 - Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
    #[inline(always)]
    #[must_use]
    pub fn err_en(&mut self) -> ERR_EN_W<20> {
        ERR_EN_W::new(self)
    }
    #[doc = "Bits 24:30 - ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn err_par(&mut self) -> ERR_PAR_W<24> {
        ERR_PAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC error injection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_err_inj](index.html) module"]
pub struct ECC_ERR_INJ_SPEC;
impl crate::RegisterSpec for ECC_ERR_INJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_err_inj::R](R) reader structure"]
impl crate::Readable for ECC_ERR_INJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_err_inj::W](W) writer structure"]
impl crate::Writable for ECC_ERR_INJ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_ERR_INJ to value 0xfffc"]
impl crate::Resettable for ECC_ERR_INJ_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffc;
}
