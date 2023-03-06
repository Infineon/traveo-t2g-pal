#[doc = "Register `MAPBIT27_24` reader"]
pub struct R(crate::R<MAPBIT27_24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT27_24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT27_24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT27_24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT27_24` writer"]
pub struct W(crate::W<MAPBIT27_24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT27_24_SPEC>;
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
impl From<crate::W<MAPBIT27_24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT27_24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT24` reader - map bit\\[24\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit24 in \\[ 23..0 \\]
=> bit\\[24\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit24 \\]; if MapBit24 in \\[29 .. 24\\]
=> bit\\[24\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit24 = 31 => bit\\[24\\]
= 0 ; if MapBit24= 30 => bit\\[24\\]
= 1"]
pub type MAPBIT24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT24` writer - map bit\\[24\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit24 in \\[ 23..0 \\]
=> bit\\[24\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit24 \\]; if MapBit24 in \\[29 .. 24\\]
=> bit\\[24\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit24 = 31 => bit\\[24\\]
= 0 ; if MapBit24= 30 => bit\\[24\\]
= 1"]
pub type MAPBIT24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT27_24_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT25` reader - map bit\\[25\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit25 in \\[ 23..0 \\]
=> bit\\[25\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit25 \\]; if MapBit25 in \\[29 .. 24\\]
=> bit\\[25\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit25 = 31 => bit\\[25\\]
= 0 ; if MapBit25= 30 => bit\\[25\\]
= 1"]
pub type MAPBIT25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT25` writer - map bit\\[25\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit25 in \\[ 23..0 \\]
=> bit\\[25\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit25 \\]; if MapBit25 in \\[29 .. 24\\]
=> bit\\[25\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit25 = 31 => bit\\[25\\]
= 0 ; if MapBit25= 30 => bit\\[25\\]
= 1"]
pub type MAPBIT25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT27_24_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT26` reader - map bit\\[26\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit26 in \\[ 23..0 \\]
=> bit\\[26\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit26 \\]; if MapBit26 in \\[29 .. 24\\]
=> bit\\[26\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit26 = 31 => bit\\[26\\]
= 0 ; if MapBit26= 30 => bit\\[26\\]
= 1"]
pub type MAPBIT26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT26` writer - map bit\\[26\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit26 in \\[ 23..0 \\]
=> bit\\[26\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit26 \\]; if MapBit26 in \\[29 .. 24\\]
=> bit\\[26\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit26 = 31 => bit\\[26\\]
= 0 ; if MapBit26= 30 => bit\\[26\\]
= 1"]
pub type MAPBIT26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT27_24_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT27` reader - map bit\\[27\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit27 in \\[ 23..0 \\]
=> bit\\[27\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit27 \\]; if MapBit27 in \\[29 .. 24\\]
=> bit\\[27\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit27 = 31 => bit\\[27\\]
= 0 ; if MapBit27= 30 => bit\\[27\\]
= 1"]
pub type MAPBIT27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT27` writer - map bit\\[27\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit27 in \\[ 23..0 \\]
=> bit\\[27\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit27 \\]; if MapBit27 in \\[29 .. 24\\]
=> bit\\[27\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit27 = 31 => bit\\[27\\]
= 0 ; if MapBit27= 30 => bit\\[27\\]
= 1"]
pub type MAPBIT27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT27_24_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[24\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit24 in \\[ 23..0 \\]
=> bit\\[24\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit24 \\]; if MapBit24 in \\[29 .. 24\\]
=> bit\\[24\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit24 = 31 => bit\\[24\\]
= 0 ; if MapBit24= 30 => bit\\[24\\]
= 1"]
    #[inline(always)]
    pub fn mapbit24(&self) -> MAPBIT24_R {
        MAPBIT24_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[25\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit25 in \\[ 23..0 \\]
=> bit\\[25\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit25 \\]; if MapBit25 in \\[29 .. 24\\]
=> bit\\[25\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit25 = 31 => bit\\[25\\]
= 0 ; if MapBit25= 30 => bit\\[25\\]
= 1"]
    #[inline(always)]
    pub fn mapbit25(&self) -> MAPBIT25_R {
        MAPBIT25_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[26\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit26 in \\[ 23..0 \\]
=> bit\\[26\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit26 \\]; if MapBit26 in \\[29 .. 24\\]
=> bit\\[26\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit26 = 31 => bit\\[26\\]
= 0 ; if MapBit26= 30 => bit\\[26\\]
= 1"]
    #[inline(always)]
    pub fn mapbit26(&self) -> MAPBIT26_R {
        MAPBIT26_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[27\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit27 in \\[ 23..0 \\]
=> bit\\[27\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit27 \\]; if MapBit27 in \\[29 .. 24\\]
=> bit\\[27\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit27 = 31 => bit\\[27\\]
= 0 ; if MapBit27= 30 => bit\\[27\\]
= 1"]
    #[inline(always)]
    pub fn mapbit27(&self) -> MAPBIT27_R {
        MAPBIT27_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[24\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit24 in \\[ 23..0 \\]
=> bit\\[24\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit24 \\]; if MapBit24 in \\[29 .. 24\\]
=> bit\\[24\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit24 = 31 => bit\\[24\\]
= 0 ; if MapBit24= 30 => bit\\[24\\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit24(&mut self) -> MAPBIT24_W<0> {
        MAPBIT24_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[25\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit25 in \\[ 23..0 \\]
=> bit\\[25\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit25 \\]; if MapBit25 in \\[29 .. 24\\]
=> bit\\[25\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit25 = 31 => bit\\[25\\]
= 0 ; if MapBit25= 30 => bit\\[25\\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit25(&mut self) -> MAPBIT25_W<8> {
        MAPBIT25_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[26\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit26 in \\[ 23..0 \\]
=> bit\\[26\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit26 \\]; if MapBit26 in \\[29 .. 24\\]
=> bit\\[26\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit26 = 31 => bit\\[26\\]
= 0 ; if MapBit26= 30 => bit\\[26\\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit26(&mut self) -> MAPBIT26_W<16> {
        MAPBIT26_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[27\\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit27 in \\[ 23..0 \\]
=> bit\\[27\\]
= \\[ Blue, Green, Red \\]
\\[ MapBit27 \\]; if MapBit27 in \\[29 .. 24\\]
=> bit\\[27\\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit27 = 31 => bit\\[27\\]
= 0 ; if MapBit27= 30 => bit\\[27\\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit27(&mut self) -> MAPBIT27_W<24> {
        MAPBIT27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 24 .. 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit27_24](index.html) module"]
pub struct MAPBIT27_24_SPEC;
impl crate::RegisterSpec for MAPBIT27_24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit27_24::R](R) reader structure"]
impl crate::Readable for MAPBIT27_24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit27_24::W](W) writer structure"]
impl crate::Writable for MAPBIT27_24_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT27_24 to value 0x1b1a_1918"]
impl crate::Resettable for MAPBIT27_24_SPEC {
    const RESET_VALUE: Self::Ux = 0x1b1a_1918;
}
