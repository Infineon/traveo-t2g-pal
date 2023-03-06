#[doc = "Register `DMA_CTL` reader"]
pub struct R(crate::R<DMA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CTL` writer"]
pub struct W(crate::W<DMA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CTL_SPEC>;
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
impl From<crate::W<DMA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMATOE` reader - IBF DMA trigger output enable '0' : Disable: tr_ibf_out = '0'. '1' : Enable: tr_ibf_out is asserted after the detection of tr_ibf_in, postponed until IBF is not busy."]
pub type IDMATOE_R = crate::BitReader<IDMATOE_A>;
#[doc = "IBF DMA trigger output enable '0' : Disable: tr_ibf_out = '0'. '1' : Enable: tr_ibf_out is asserted after the detection of tr_ibf_in, postponed until IBF is not busy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMATOE_A {
    #[doc = "0: N/A"]
    DISABLE = 0,
    #[doc = "1: N/A"]
    ENABLE = 1,
}
impl From<IDMATOE_A> for bool {
    #[inline(always)]
    fn from(variant: IDMATOE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDMATOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDMATOE_A {
        match self.bits {
            false => IDMATOE_A::DISABLE,
            true => IDMATOE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDMATOE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IDMATOE_A::ENABLE
    }
}
#[doc = "Field `IDMATOE` writer - IBF DMA trigger output enable '0' : Disable: tr_ibf_out = '0'. '1' : Enable: tr_ibf_out is asserted after the detection of tr_ibf_in, postponed until IBF is not busy."]
pub type IDMATOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, IDMATOE_A, O>;
impl<'a, const O: u8> IDMATOE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDMATOE_A::DISABLE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IDMATOE_A::ENABLE)
    }
}
#[doc = "Field `ODMATOE` reader - OBF DMA trigger output enable '0' : Disable: tr_obf_out = '0'. '1' : Enable: tr_obf_out is asserted after detection of tr_obf_in according to the setting of ODMAFFE."]
pub type ODMATOE_R = crate::BitReader<ODMATOE_A>;
#[doc = "OBF DMA trigger output enable '0' : Disable: tr_obf_out = '0'. '1' : Enable: tr_obf_out is asserted after detection of tr_obf_in according to the setting of ODMAFFE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODMATOE_A {
    #[doc = "0: N/A"]
    DISABLE = 0,
    #[doc = "1: N/A"]
    ENABLE = 1,
}
impl From<ODMATOE_A> for bool {
    #[inline(always)]
    fn from(variant: ODMATOE_A) -> Self {
        variant as u8 != 0
    }
}
impl ODMATOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODMATOE_A {
        match self.bits {
            false => ODMATOE_A::DISABLE,
            true => ODMATOE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ODMATOE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ODMATOE_A::ENABLE
    }
}
#[doc = "Field `ODMATOE` writer - OBF DMA trigger output enable '0' : Disable: tr_obf_out = '0'. '1' : Enable: tr_obf_out is asserted after detection of tr_obf_in according to the setting of ODMAFFE."]
pub type ODMATOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, ODMATOE_A, O>;
impl<'a, const O: u8> ODMATOE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ODMATOE_A::DISABLE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ODMATOE_A::ENABLE)
    }
}
#[doc = "Field `ODMAFFE` reader - OBF DMA FIFO enable '0' : Disable: tr_obf_out is postponed until OBF is not busy. '1' : Enable: tr_obf_out is postponed until OBF is not busy AND the receive FIFO is not empty."]
pub type ODMAFFE_R = crate::BitReader<ODMAFFE_A>;
#[doc = "OBF DMA FIFO enable '0' : Disable: tr_obf_out is postponed until OBF is not busy. '1' : Enable: tr_obf_out is postponed until OBF is not busy AND the receive FIFO is not empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODMAFFE_A {
    #[doc = "0: N/A"]
    DISABLE = 0,
    #[doc = "1: N/A"]
    ENABLE = 1,
}
impl From<ODMAFFE_A> for bool {
    #[inline(always)]
    fn from(variant: ODMAFFE_A) -> Self {
        variant as u8 != 0
    }
}
impl ODMAFFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODMAFFE_A {
        match self.bits {
            false => ODMAFFE_A::DISABLE,
            true => ODMAFFE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ODMAFFE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ODMAFFE_A::ENABLE
    }
}
#[doc = "Field `ODMAFFE` writer - OBF DMA FIFO enable '0' : Disable: tr_obf_out is postponed until OBF is not busy. '1' : Enable: tr_obf_out is postponed until OBF is not busy AND the receive FIFO is not empty."]
pub type ODMAFFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, ODMAFFE_A, O>;
impl<'a, const O: u8> ODMAFFE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ODMAFFE_A::DISABLE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ODMAFFE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - IBF DMA trigger output enable '0' : Disable: tr_ibf_out = '0'. '1' : Enable: tr_ibf_out is asserted after the detection of tr_ibf_in, postponed until IBF is not busy."]
    #[inline(always)]
    pub fn idmatoe(&self) -> IDMATOE_R {
        IDMATOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OBF DMA trigger output enable '0' : Disable: tr_obf_out = '0'. '1' : Enable: tr_obf_out is asserted after detection of tr_obf_in according to the setting of ODMAFFE."]
    #[inline(always)]
    pub fn odmatoe(&self) -> ODMATOE_R {
        ODMATOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OBF DMA FIFO enable '0' : Disable: tr_obf_out is postponed until OBF is not busy. '1' : Enable: tr_obf_out is postponed until OBF is not busy AND the receive FIFO is not empty."]
    #[inline(always)]
    pub fn odmaffe(&self) -> ODMAFFE_R {
        ODMAFFE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IBF DMA trigger output enable '0' : Disable: tr_ibf_out = '0'. '1' : Enable: tr_ibf_out is asserted after the detection of tr_ibf_in, postponed until IBF is not busy."]
    #[inline(always)]
    #[must_use]
    pub fn idmatoe(&mut self) -> IDMATOE_W<0> {
        IDMATOE_W::new(self)
    }
    #[doc = "Bit 1 - OBF DMA trigger output enable '0' : Disable: tr_obf_out = '0'. '1' : Enable: tr_obf_out is asserted after detection of tr_obf_in according to the setting of ODMAFFE."]
    #[inline(always)]
    #[must_use]
    pub fn odmatoe(&mut self) -> ODMATOE_W<1> {
        ODMATOE_W::new(self)
    }
    #[doc = "Bit 2 - OBF DMA FIFO enable '0' : Disable: tr_obf_out is postponed until OBF is not busy. '1' : Enable: tr_obf_out is postponed until OBF is not busy AND the receive FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn odmaffe(&mut self) -> ODMAFFE_W<2> {
        ODMAFFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctl](index.html) module"]
pub struct DMA_CTL_SPEC;
impl crate::RegisterSpec for DMA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ctl::R](R) reader structure"]
impl crate::Readable for DMA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_ctl::W](W) writer structure"]
impl crate::Writable for DMA_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CTL to value 0"]
impl crate::Resettable for DMA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
