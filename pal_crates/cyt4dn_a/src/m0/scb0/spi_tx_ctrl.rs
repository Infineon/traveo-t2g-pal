#[doc = "Register `SPI_TX_CTRL` reader"]
pub struct R(crate::R<SPI_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_TX_CTRL` writer"]
pub struct W(crate::W<SPI_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_TX_CTRL_SPEC>;
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
impl From<crate::W<SPI_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARITY` reader - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity."]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity."]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_TX_CTRL_SPEC, bool, O>;
#[doc = "Field `PARITY_ENABLED` reader - Parity generation enabled ('1') or not ('0')."]
pub type PARITY_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENABLED` writer - Parity generation enabled ('1') or not ('0')."]
pub type PARITY_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_TX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity generation enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<4> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 5 - Parity generation enabled ('1') or not ('0')."]
    #[inline(always)]
    #[must_use]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W<5> {
        PARITY_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI transmitter control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_tx_ctrl](index.html) module"]
pub struct SPI_TX_CTRL_SPEC;
impl crate::RegisterSpec for SPI_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_tx_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_tx_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_TX_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_TX_CTRL to value 0"]
impl crate::Resettable for SPI_TX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
