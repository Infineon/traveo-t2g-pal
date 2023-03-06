#[doc = "Register `BLK_LUT` reader"]
pub struct R(crate::R<BLK_LUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_LUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_LUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_LUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK_LUT` writer"]
pub struct W(crate::W<BLK_LUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK_LUT_SPEC>;
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
impl From<crate::W<BLK_LUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK_LUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR_NS0` reader - NS bit for block 0 based on BLK_IDX"]
pub type ATTR_NS0_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS0` writer - NS bit for block 0 based on BLK_IDX"]
pub type ATTR_NS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS1` reader - NS bit for block 1 based on BLK_IDX"]
pub type ATTR_NS1_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS1` writer - NS bit for block 1 based on BLK_IDX"]
pub type ATTR_NS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS2` reader - NS bit for block 2 based on BLK_IDX"]
pub type ATTR_NS2_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS2` writer - NS bit for block 2 based on BLK_IDX"]
pub type ATTR_NS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS3` reader - NS bit for block 3 based on BLK_IDX"]
pub type ATTR_NS3_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS3` writer - NS bit for block 3 based on BLK_IDX"]
pub type ATTR_NS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS4` reader - NS bit for block 4 based on BLK_IDX"]
pub type ATTR_NS4_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS4` writer - NS bit for block 4 based on BLK_IDX"]
pub type ATTR_NS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS5` reader - NS bit for block 5 based on BLK_IDX"]
pub type ATTR_NS5_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS5` writer - NS bit for block 5 based on BLK_IDX"]
pub type ATTR_NS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS6` reader - NS bit for block 6 based on BLK_IDX"]
pub type ATTR_NS6_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS6` writer - NS bit for block 6 based on BLK_IDX"]
pub type ATTR_NS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS7` reader - NS bit for block 7 based on BLK_IDX"]
pub type ATTR_NS7_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS7` writer - NS bit for block 7 based on BLK_IDX"]
pub type ATTR_NS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS8` reader - NS bit for block 8 based on BLK_IDX"]
pub type ATTR_NS8_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS8` writer - NS bit for block 8 based on BLK_IDX"]
pub type ATTR_NS8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS9` reader - NS bit for block 9 based on BLK_IDX"]
pub type ATTR_NS9_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS9` writer - NS bit for block 9 based on BLK_IDX"]
pub type ATTR_NS9_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS10` reader - NS bit for block 10 based on BLK_IDX"]
pub type ATTR_NS10_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS10` writer - NS bit for block 10 based on BLK_IDX"]
pub type ATTR_NS10_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS11` reader - NS bit for block 11 based on BLK_IDX"]
pub type ATTR_NS11_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS11` writer - NS bit for block 11 based on BLK_IDX"]
pub type ATTR_NS11_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS12` reader - NS bit for block 12 based on BLK_IDX"]
pub type ATTR_NS12_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS12` writer - NS bit for block 12 based on BLK_IDX"]
pub type ATTR_NS12_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS13` reader - NS bit for block 13 based on BLK_IDX"]
pub type ATTR_NS13_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS13` writer - NS bit for block 13 based on BLK_IDX"]
pub type ATTR_NS13_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS14` reader - NS bit for block 14 based on BLK_IDX"]
pub type ATTR_NS14_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS14` writer - NS bit for block 14 based on BLK_IDX"]
pub type ATTR_NS14_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS15` reader - NS bit for block 15 based on BLK_IDX"]
pub type ATTR_NS15_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS15` writer - NS bit for block 15 based on BLK_IDX"]
pub type ATTR_NS15_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS16` reader - NS bit for block 16 based on BLK_IDX"]
pub type ATTR_NS16_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS16` writer - NS bit for block 16 based on BLK_IDX"]
pub type ATTR_NS16_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS17` reader - NS bit for block 17 based on BLK_IDX"]
pub type ATTR_NS17_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS17` writer - NS bit for block 17 based on BLK_IDX"]
pub type ATTR_NS17_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS18` reader - NS bit for block 18 based on BLK_IDX"]
pub type ATTR_NS18_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS18` writer - NS bit for block 18 based on BLK_IDX"]
pub type ATTR_NS18_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS19` reader - NS bit for block 19 based on BLK_IDX"]
pub type ATTR_NS19_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS19` writer - NS bit for block 19 based on BLK_IDX"]
pub type ATTR_NS19_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS20` reader - NS bit for block 20 based on BLK_IDX"]
pub type ATTR_NS20_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS20` writer - NS bit for block 20 based on BLK_IDX"]
pub type ATTR_NS20_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS21` reader - NS bit for block 21 based on BLK_IDX"]
pub type ATTR_NS21_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS21` writer - NS bit for block 21 based on BLK_IDX"]
pub type ATTR_NS21_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS22` reader - NS bit for block 22 based on BLK_IDX"]
pub type ATTR_NS22_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS22` writer - NS bit for block 22 based on BLK_IDX"]
pub type ATTR_NS22_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS23` reader - NS bit for block 23 based on BLK_IDX"]
pub type ATTR_NS23_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS23` writer - NS bit for block 23 based on BLK_IDX"]
pub type ATTR_NS23_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS24` reader - NS bit for block 24 based on BLK_IDX"]
pub type ATTR_NS24_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS24` writer - NS bit for block 24 based on BLK_IDX"]
pub type ATTR_NS24_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS25` reader - NS bit for block 25 based on BLK_IDX"]
pub type ATTR_NS25_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS25` writer - NS bit for block 25 based on BLK_IDX"]
pub type ATTR_NS25_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS26` reader - NS bit for block 26 based on BLK_IDX"]
pub type ATTR_NS26_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS26` writer - NS bit for block 26 based on BLK_IDX"]
pub type ATTR_NS26_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS27` reader - NS bit for block 27 based on BLK_IDX"]
pub type ATTR_NS27_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS27` writer - NS bit for block 27 based on BLK_IDX"]
pub type ATTR_NS27_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS28` reader - NS bit for block 28 based on BLK_IDX"]
pub type ATTR_NS28_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS28` writer - NS bit for block 28 based on BLK_IDX"]
pub type ATTR_NS28_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS29` reader - NS bit for block 29 based on BLK_IDX"]
pub type ATTR_NS29_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS29` writer - NS bit for block 29 based on BLK_IDX"]
pub type ATTR_NS29_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS30` reader - NS bit for block 30 based on BLK_IDX"]
pub type ATTR_NS30_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS30` writer - NS bit for block 30 based on BLK_IDX"]
pub type ATTR_NS30_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
#[doc = "Field `ATTR_NS31` reader - NS bit for block 31 based on BLK_IDX"]
pub type ATTR_NS31_R = crate::BitReader<bool>;
#[doc = "Field `ATTR_NS31` writer - NS bit for block 31 based on BLK_IDX"]
pub type ATTR_NS31_W<'a, const O: u8> = crate::BitWriter<'a, u32, BLK_LUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NS bit for block 0 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns0(&self) -> ATTR_NS0_R {
        ATTR_NS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NS bit for block 1 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns1(&self) -> ATTR_NS1_R {
        ATTR_NS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NS bit for block 2 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns2(&self) -> ATTR_NS2_R {
        ATTR_NS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NS bit for block 3 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns3(&self) -> ATTR_NS3_R {
        ATTR_NS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NS bit for block 4 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns4(&self) -> ATTR_NS4_R {
        ATTR_NS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NS bit for block 5 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns5(&self) -> ATTR_NS5_R {
        ATTR_NS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NS bit for block 6 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns6(&self) -> ATTR_NS6_R {
        ATTR_NS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NS bit for block 7 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns7(&self) -> ATTR_NS7_R {
        ATTR_NS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NS bit for block 8 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns8(&self) -> ATTR_NS8_R {
        ATTR_NS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NS bit for block 9 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns9(&self) -> ATTR_NS9_R {
        ATTR_NS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NS bit for block 10 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns10(&self) -> ATTR_NS10_R {
        ATTR_NS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NS bit for block 11 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns11(&self) -> ATTR_NS11_R {
        ATTR_NS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NS bit for block 12 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns12(&self) -> ATTR_NS12_R {
        ATTR_NS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NS bit for block 13 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns13(&self) -> ATTR_NS13_R {
        ATTR_NS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NS bit for block 14 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns14(&self) -> ATTR_NS14_R {
        ATTR_NS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NS bit for block 15 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns15(&self) -> ATTR_NS15_R {
        ATTR_NS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NS bit for block 16 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns16(&self) -> ATTR_NS16_R {
        ATTR_NS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NS bit for block 17 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns17(&self) -> ATTR_NS17_R {
        ATTR_NS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NS bit for block 18 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns18(&self) -> ATTR_NS18_R {
        ATTR_NS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NS bit for block 19 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns19(&self) -> ATTR_NS19_R {
        ATTR_NS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NS bit for block 20 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns20(&self) -> ATTR_NS20_R {
        ATTR_NS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - NS bit for block 21 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns21(&self) -> ATTR_NS21_R {
        ATTR_NS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - NS bit for block 22 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns22(&self) -> ATTR_NS22_R {
        ATTR_NS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NS bit for block 23 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns23(&self) -> ATTR_NS23_R {
        ATTR_NS23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - NS bit for block 24 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns24(&self) -> ATTR_NS24_R {
        ATTR_NS24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NS bit for block 25 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns25(&self) -> ATTR_NS25_R {
        ATTR_NS25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NS bit for block 26 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns26(&self) -> ATTR_NS26_R {
        ATTR_NS26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - NS bit for block 27 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns27(&self) -> ATTR_NS27_R {
        ATTR_NS27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - NS bit for block 28 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns28(&self) -> ATTR_NS28_R {
        ATTR_NS28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - NS bit for block 29 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns29(&self) -> ATTR_NS29_R {
        ATTR_NS29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - NS bit for block 30 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns30(&self) -> ATTR_NS30_R {
        ATTR_NS30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NS bit for block 31 based on BLK_IDX"]
    #[inline(always)]
    pub fn attr_ns31(&self) -> ATTR_NS31_R {
        ATTR_NS31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NS bit for block 0 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns0(&mut self) -> ATTR_NS0_W<0> {
        ATTR_NS0_W::new(self)
    }
    #[doc = "Bit 1 - NS bit for block 1 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns1(&mut self) -> ATTR_NS1_W<1> {
        ATTR_NS1_W::new(self)
    }
    #[doc = "Bit 2 - NS bit for block 2 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns2(&mut self) -> ATTR_NS2_W<2> {
        ATTR_NS2_W::new(self)
    }
    #[doc = "Bit 3 - NS bit for block 3 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns3(&mut self) -> ATTR_NS3_W<3> {
        ATTR_NS3_W::new(self)
    }
    #[doc = "Bit 4 - NS bit for block 4 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns4(&mut self) -> ATTR_NS4_W<4> {
        ATTR_NS4_W::new(self)
    }
    #[doc = "Bit 5 - NS bit for block 5 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns5(&mut self) -> ATTR_NS5_W<5> {
        ATTR_NS5_W::new(self)
    }
    #[doc = "Bit 6 - NS bit for block 6 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns6(&mut self) -> ATTR_NS6_W<6> {
        ATTR_NS6_W::new(self)
    }
    #[doc = "Bit 7 - NS bit for block 7 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns7(&mut self) -> ATTR_NS7_W<7> {
        ATTR_NS7_W::new(self)
    }
    #[doc = "Bit 8 - NS bit for block 8 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns8(&mut self) -> ATTR_NS8_W<8> {
        ATTR_NS8_W::new(self)
    }
    #[doc = "Bit 9 - NS bit for block 9 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns9(&mut self) -> ATTR_NS9_W<9> {
        ATTR_NS9_W::new(self)
    }
    #[doc = "Bit 10 - NS bit for block 10 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns10(&mut self) -> ATTR_NS10_W<10> {
        ATTR_NS10_W::new(self)
    }
    #[doc = "Bit 11 - NS bit for block 11 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns11(&mut self) -> ATTR_NS11_W<11> {
        ATTR_NS11_W::new(self)
    }
    #[doc = "Bit 12 - NS bit for block 12 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns12(&mut self) -> ATTR_NS12_W<12> {
        ATTR_NS12_W::new(self)
    }
    #[doc = "Bit 13 - NS bit for block 13 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns13(&mut self) -> ATTR_NS13_W<13> {
        ATTR_NS13_W::new(self)
    }
    #[doc = "Bit 14 - NS bit for block 14 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns14(&mut self) -> ATTR_NS14_W<14> {
        ATTR_NS14_W::new(self)
    }
    #[doc = "Bit 15 - NS bit for block 15 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns15(&mut self) -> ATTR_NS15_W<15> {
        ATTR_NS15_W::new(self)
    }
    #[doc = "Bit 16 - NS bit for block 16 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns16(&mut self) -> ATTR_NS16_W<16> {
        ATTR_NS16_W::new(self)
    }
    #[doc = "Bit 17 - NS bit for block 17 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns17(&mut self) -> ATTR_NS17_W<17> {
        ATTR_NS17_W::new(self)
    }
    #[doc = "Bit 18 - NS bit for block 18 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns18(&mut self) -> ATTR_NS18_W<18> {
        ATTR_NS18_W::new(self)
    }
    #[doc = "Bit 19 - NS bit for block 19 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns19(&mut self) -> ATTR_NS19_W<19> {
        ATTR_NS19_W::new(self)
    }
    #[doc = "Bit 20 - NS bit for block 20 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns20(&mut self) -> ATTR_NS20_W<20> {
        ATTR_NS20_W::new(self)
    }
    #[doc = "Bit 21 - NS bit for block 21 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns21(&mut self) -> ATTR_NS21_W<21> {
        ATTR_NS21_W::new(self)
    }
    #[doc = "Bit 22 - NS bit for block 22 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns22(&mut self) -> ATTR_NS22_W<22> {
        ATTR_NS22_W::new(self)
    }
    #[doc = "Bit 23 - NS bit for block 23 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns23(&mut self) -> ATTR_NS23_W<23> {
        ATTR_NS23_W::new(self)
    }
    #[doc = "Bit 24 - NS bit for block 24 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns24(&mut self) -> ATTR_NS24_W<24> {
        ATTR_NS24_W::new(self)
    }
    #[doc = "Bit 25 - NS bit for block 25 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns25(&mut self) -> ATTR_NS25_W<25> {
        ATTR_NS25_W::new(self)
    }
    #[doc = "Bit 26 - NS bit for block 26 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns26(&mut self) -> ATTR_NS26_W<26> {
        ATTR_NS26_W::new(self)
    }
    #[doc = "Bit 27 - NS bit for block 27 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns27(&mut self) -> ATTR_NS27_W<27> {
        ATTR_NS27_W::new(self)
    }
    #[doc = "Bit 28 - NS bit for block 28 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns28(&mut self) -> ATTR_NS28_W<28> {
        ATTR_NS28_W::new(self)
    }
    #[doc = "Bit 29 - NS bit for block 29 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns29(&mut self) -> ATTR_NS29_W<29> {
        ATTR_NS29_W::new(self)
    }
    #[doc = "Bit 30 - NS bit for block 30 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns30(&mut self) -> ATTR_NS30_W<30> {
        ATTR_NS30_W::new(self)
    }
    #[doc = "Bit 31 - NS bit for block 31 based on BLK_IDX"]
    #[inline(always)]
    #[must_use]
    pub fn attr_ns31(&mut self) -> ATTR_NS31_W<31> {
        ATTR_NS31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NS status for 32 blocks at BLK_IDX with PC=&lt;access_pc>\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_lut](index.html) module"]
pub struct BLK_LUT_SPEC;
impl crate::RegisterSpec for BLK_LUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_lut::R](R) reader structure"]
impl crate::Readable for BLK_LUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk_lut::W](W) writer structure"]
impl crate::Writable for BLK_LUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK_LUT to value 0"]
impl crate::Resettable for BLK_LUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
