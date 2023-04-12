#[doc = "Register `BINPROPERTIES` reader"]
pub struct R(crate::R<BINPROPERTIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINPROPERTIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINPROPERTIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINPROPERTIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BINPROPERTIES` writer"]
pub struct W(crate::W<BINPROPERTIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BINPROPERTIES_SPEC>;
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
impl From<crate::W<BINPROPERTIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BINPROPERTIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINNUM_WIDTH` reader - 2**binnum_width is the number of bins in a histogram."]
pub type BINNUM_WIDTH_R = crate::FieldReader<u8, BINNUM_WIDTH_A>;
#[doc = "2**binnum_width is the number of bins in a histogram.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BINNUM_WIDTH_A {
    #[doc = "3: There are 8 bins, addressable with 3bit."]
    BINS_8 = 3,
    #[doc = "4: There are 16 bins, addressable with 4bit."]
    BINS_16 = 4,
    #[doc = "5: There are 32 bins, addressable with 5bit."]
    BINS_32 = 5,
    #[doc = "6: There are 64 bins, addressable with 6bit."]
    BINS_64 = 6,
}
impl From<BINNUM_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: BINNUM_WIDTH_A) -> Self {
        variant as _
    }
}
impl BINNUM_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BINNUM_WIDTH_A> {
        match self.bits {
            3 => Some(BINNUM_WIDTH_A::BINS_8),
            4 => Some(BINNUM_WIDTH_A::BINS_16),
            5 => Some(BINNUM_WIDTH_A::BINS_32),
            6 => Some(BINNUM_WIDTH_A::BINS_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BINS_8`"]
    #[inline(always)]
    pub fn is_bins_8(&self) -> bool {
        *self == BINNUM_WIDTH_A::BINS_8
    }
    #[doc = "Checks if the value of the field is `BINS_16`"]
    #[inline(always)]
    pub fn is_bins_16(&self) -> bool {
        *self == BINNUM_WIDTH_A::BINS_16
    }
    #[doc = "Checks if the value of the field is `BINS_32`"]
    #[inline(always)]
    pub fn is_bins_32(&self) -> bool {
        *self == BINNUM_WIDTH_A::BINS_32
    }
    #[doc = "Checks if the value of the field is `BINS_64`"]
    #[inline(always)]
    pub fn is_bins_64(&self) -> bool {
        *self == BINNUM_WIDTH_A::BINS_64
    }
}
#[doc = "Field `BINNUM_WIDTH` writer - 2**binnum_width is the number of bins in a histogram."]
pub type BINNUM_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BINPROPERTIES_SPEC, u8, BINNUM_WIDTH_A, 3, O>;
impl<'a, const O: u8> BINNUM_WIDTH_W<'a, O> {
    #[doc = "There are 8 bins, addressable with 3bit."]
    #[inline(always)]
    pub fn bins_8(self) -> &'a mut W {
        self.variant(BINNUM_WIDTH_A::BINS_8)
    }
    #[doc = "There are 16 bins, addressable with 4bit."]
    #[inline(always)]
    pub fn bins_16(self) -> &'a mut W {
        self.variant(BINNUM_WIDTH_A::BINS_16)
    }
    #[doc = "There are 32 bins, addressable with 5bit."]
    #[inline(always)]
    pub fn bins_32(self) -> &'a mut W {
        self.variant(BINNUM_WIDTH_A::BINS_32)
    }
    #[doc = "There are 64 bins, addressable with 6bit."]
    #[inline(always)]
    pub fn bins_64(self) -> &'a mut W {
        self.variant(BINNUM_WIDTH_A::BINS_64)
    }
}
#[doc = "Field `CNT_MODE` reader - Counter mode, which controls the way the bin values are accumulated."]
pub type CNT_MODE_R = crate::BitReader<CNT_MODE_A>;
#[doc = "Counter mode, which controls the way the bin values are accumulated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNT_MODE_A {
    #[doc = "0: Counter of nearest bin is incremented."]
    NEAREST = 0,
    #[doc = "1: Counters of the 2 nearest bins are increased by one minus the distance to bin center (normalized between 0 and 1), it works only for first (component0) histogram, reading of component1/2 histogram results in undefined data."]
    LINEAR = 1,
}
impl From<CNT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CNT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CNT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNT_MODE_A {
        match self.bits {
            false => CNT_MODE_A::NEAREST,
            true => CNT_MODE_A::LINEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NEAREST`"]
    #[inline(always)]
    pub fn is_nearest(&self) -> bool {
        *self == CNT_MODE_A::NEAREST
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == CNT_MODE_A::LINEAR
    }
}
#[doc = "Field `CNT_MODE` writer - Counter mode, which controls the way the bin values are accumulated."]
pub type CNT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BINPROPERTIES_SPEC, CNT_MODE_A, O>;
impl<'a, const O: u8> CNT_MODE_W<'a, O> {
    #[doc = "Counter of nearest bin is incremented."]
    #[inline(always)]
    pub fn nearest(self) -> &'a mut W {
        self.variant(CNT_MODE_A::NEAREST)
    }
    #[doc = "Counters of the 2 nearest bins are increased by one minus the distance to bin center (normalized between 0 and 1), it works only for first (component0) histogram, reading of component1/2 histogram results in undefined data."]
    #[inline(always)]
    pub fn linear(self) -> &'a mut W {
        self.variant(CNT_MODE_A::LINEAR)
    }
}
#[doc = "Field `BINCNT_RD_MODE` reader - The field controls if the actual bin count or the sum of all bin counts up to actual bin count is provided in registers RsltComp0Bincnt, RsltComp1Bincnt and RsltComp2Bincnt."]
pub type BINCNT_RD_MODE_R = crate::BitReader<BINCNT_RD_MODE_A>;
#[doc = "The field controls if the actual bin count or the sum of all bin counts up to actual bin count is provided in registers RsltComp0Bincnt, RsltComp1Bincnt and RsltComp2Bincnt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BINCNT_RD_MODE_A {
    #[doc = "0: Actual bin cnt is provided."]
    BINCNT = 0,
    #[doc = "1: Bin count sum is provided."]
    BINCNT_ACCU = 1,
}
impl From<BINCNT_RD_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BINCNT_RD_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BINCNT_RD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BINCNT_RD_MODE_A {
        match self.bits {
            false => BINCNT_RD_MODE_A::BINCNT,
            true => BINCNT_RD_MODE_A::BINCNT_ACCU,
        }
    }
    #[doc = "Checks if the value of the field is `BINCNT`"]
    #[inline(always)]
    pub fn is_bincnt(&self) -> bool {
        *self == BINCNT_RD_MODE_A::BINCNT
    }
    #[doc = "Checks if the value of the field is `BINCNT_ACCU`"]
    #[inline(always)]
    pub fn is_bincnt_accu(&self) -> bool {
        *self == BINCNT_RD_MODE_A::BINCNT_ACCU
    }
}
#[doc = "Field `BINCNT_RD_MODE` writer - The field controls if the actual bin count or the sum of all bin counts up to actual bin count is provided in registers RsltComp0Bincnt, RsltComp1Bincnt and RsltComp2Bincnt."]
pub type BINCNT_RD_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BINPROPERTIES_SPEC, BINCNT_RD_MODE_A, O>;
impl<'a, const O: u8> BINCNT_RD_MODE_W<'a, O> {
    #[doc = "Actual bin cnt is provided."]
    #[inline(always)]
    pub fn bincnt(self) -> &'a mut W {
        self.variant(BINCNT_RD_MODE_A::BINCNT)
    }
    #[doc = "Bin count sum is provided."]
    #[inline(always)]
    pub fn bincnt_accu(self) -> &'a mut W {
        self.variant(BINCNT_RD_MODE_A::BINCNT_ACCU)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2**binnum_width is the number of bins in a histogram."]
    #[inline(always)]
    pub fn binnum_width(&self) -> BINNUM_WIDTH_R {
        BINNUM_WIDTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Counter mode, which controls the way the bin values are accumulated."]
    #[inline(always)]
    pub fn cnt_mode(&self) -> CNT_MODE_R {
        CNT_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - The field controls if the actual bin count or the sum of all bin counts up to actual bin count is provided in registers RsltComp0Bincnt, RsltComp1Bincnt and RsltComp2Bincnt."]
    #[inline(always)]
    pub fn bincnt_rd_mode(&self) -> BINCNT_RD_MODE_R {
        BINCNT_RD_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2**binnum_width is the number of bins in a histogram."]
    #[inline(always)]
    #[must_use]
    pub fn binnum_width(&mut self) -> BINNUM_WIDTH_W<0> {
        BINNUM_WIDTH_W::new(self)
    }
    #[doc = "Bit 4 - Counter mode, which controls the way the bin values are accumulated."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_mode(&mut self) -> CNT_MODE_W<4> {
        CNT_MODE_W::new(self)
    }
    #[doc = "Bit 8 - The field controls if the actual bin count or the sum of all bin counts up to actual bin count is provided in registers RsltComp0Bincnt, RsltComp1Bincnt and RsltComp2Bincnt."]
    #[inline(always)]
    #[must_use]
    pub fn bincnt_rd_mode(&mut self) -> BINCNT_RD_MODE_W<8> {
        BINCNT_RD_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the histogram bins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [binproperties](index.html) module"]
pub struct BINPROPERTIES_SPEC;
impl crate::RegisterSpec for BINPROPERTIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [binproperties::R](R) reader structure"]
impl crate::Readable for BINPROPERTIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [binproperties::W](W) writer structure"]
impl crate::Writable for BINPROPERTIES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BINPROPERTIES to value 0x03"]
impl crate::Resettable for BINPROPERTIES_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
