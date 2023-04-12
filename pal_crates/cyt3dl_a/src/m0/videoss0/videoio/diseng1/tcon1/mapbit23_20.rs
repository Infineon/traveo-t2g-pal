#[doc = "Register `MAPBIT23_20` reader"]
pub struct R(crate::R<MAPBIT23_20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT23_20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT23_20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT23_20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT23_20` writer"]
pub struct W(crate::W<MAPBIT23_20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT23_20_SPEC>;
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
impl From<crate::W<MAPBIT23_20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT23_20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT20` reader - map bit\\[ 20 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit20 in \\[ 23..0 \\]
=> bit\\[ 20 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit20 \\]; if MapBit20 in \\[29 .. 24\\]
=> bit\\[ 20 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit20 = 31 => bit\\[ 20 \\]
= 0 ; if MapBit20= 30 => bit\\[ 20 \\]
= 1"]
pub type MAPBIT20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT20` writer - map bit\\[ 20 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit20 in \\[ 23..0 \\]
=> bit\\[ 20 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit20 \\]; if MapBit20 in \\[29 .. 24\\]
=> bit\\[ 20 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit20 = 31 => bit\\[ 20 \\]
= 0 ; if MapBit20= 30 => bit\\[ 20 \\]
= 1"]
pub type MAPBIT20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT23_20_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT21` reader - map bit\\[ 21 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit21 in \\[ 23..0 \\]
=> bit\\[ 21 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit21 \\]; if MapBit21 in \\[29 .. 24\\]
=> bit\\[ 21 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit21 = 31 => bit\\[ 21 \\]
= 0 ; if MapBit21= 30 => bit\\[ 21 \\]
= 1"]
pub type MAPBIT21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT21` writer - map bit\\[ 21 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit21 in \\[ 23..0 \\]
=> bit\\[ 21 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit21 \\]; if MapBit21 in \\[29 .. 24\\]
=> bit\\[ 21 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit21 = 31 => bit\\[ 21 \\]
= 0 ; if MapBit21= 30 => bit\\[ 21 \\]
= 1"]
pub type MAPBIT21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT23_20_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT22` reader - map bit\\[ 22 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit22 in \\[ 23..0 \\]
=> bit\\[ 22 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit22 \\]; if MapBit22 in \\[29 .. 24\\]
=> bit\\[ 22 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit22 = 31 => bit\\[ 22 \\]
= 0 ; if MapBit22= 30 => bit\\[ 22 \\]
= 1"]
pub type MAPBIT22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT22` writer - map bit\\[ 22 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit22 in \\[ 23..0 \\]
=> bit\\[ 22 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit22 \\]; if MapBit22 in \\[29 .. 24\\]
=> bit\\[ 22 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit22 = 31 => bit\\[ 22 \\]
= 0 ; if MapBit22= 30 => bit\\[ 22 \\]
= 1"]
pub type MAPBIT22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT23_20_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT23` reader - map bit\\[ 23 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit23 in \\[ 23..0 \\]
=> bit\\[ 23 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit23 \\]; if MapBit23 in \\[29 .. 24\\]
=> bit\\[ 23 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit23 = 31 => bit\\[ 23 \\]
= 0 ; if MapBit23= 30 => bit\\[ 23 \\]
= 1"]
pub type MAPBIT23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT23` writer - map bit\\[ 23 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit23 in \\[ 23..0 \\]
=> bit\\[ 23 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit23 \\]; if MapBit23 in \\[29 .. 24\\]
=> bit\\[ 23 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit23 = 31 => bit\\[ 23 \\]
= 0 ; if MapBit23= 30 => bit\\[ 23 \\]
= 1"]
pub type MAPBIT23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT23_20_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[ 20 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit20 in \\[ 23..0 \\]
=> bit\\[ 20 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit20 \\]; if MapBit20 in \\[29 .. 24\\]
=> bit\\[ 20 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit20 = 31 => bit\\[ 20 \\]
= 0 ; if MapBit20= 30 => bit\\[ 20 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit20(&self) -> MAPBIT20_R {
        MAPBIT20_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[ 21 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit21 in \\[ 23..0 \\]
=> bit\\[ 21 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit21 \\]; if MapBit21 in \\[29 .. 24\\]
=> bit\\[ 21 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit21 = 31 => bit\\[ 21 \\]
= 0 ; if MapBit21= 30 => bit\\[ 21 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit21(&self) -> MAPBIT21_R {
        MAPBIT21_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[ 22 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit22 in \\[ 23..0 \\]
=> bit\\[ 22 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit22 \\]; if MapBit22 in \\[29 .. 24\\]
=> bit\\[ 22 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit22 = 31 => bit\\[ 22 \\]
= 0 ; if MapBit22= 30 => bit\\[ 22 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit22(&self) -> MAPBIT22_R {
        MAPBIT22_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[ 23 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit23 in \\[ 23..0 \\]
=> bit\\[ 23 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit23 \\]; if MapBit23 in \\[29 .. 24\\]
=> bit\\[ 23 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit23 = 31 => bit\\[ 23 \\]
= 0 ; if MapBit23= 30 => bit\\[ 23 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit23(&self) -> MAPBIT23_R {
        MAPBIT23_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[ 20 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit20 in \\[ 23..0 \\]
=> bit\\[ 20 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit20 \\]; if MapBit20 in \\[29 .. 24\\]
=> bit\\[ 20 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit20 = 31 => bit\\[ 20 \\]
= 0 ; if MapBit20= 30 => bit\\[ 20 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit20(&mut self) -> MAPBIT20_W<0> {
        MAPBIT20_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[ 21 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit21 in \\[ 23..0 \\]
=> bit\\[ 21 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit21 \\]; if MapBit21 in \\[29 .. 24\\]
=> bit\\[ 21 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit21 = 31 => bit\\[ 21 \\]
= 0 ; if MapBit21= 30 => bit\\[ 21 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit21(&mut self) -> MAPBIT21_W<8> {
        MAPBIT21_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[ 22 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit22 in \\[ 23..0 \\]
=> bit\\[ 22 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit22 \\]; if MapBit22 in \\[29 .. 24\\]
=> bit\\[ 22 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit22 = 31 => bit\\[ 22 \\]
= 0 ; if MapBit22= 30 => bit\\[ 22 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit22(&mut self) -> MAPBIT22_W<16> {
        MAPBIT22_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[ 23 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit23 in \\[ 23..0 \\]
=> bit\\[ 23 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit23 \\]; if MapBit23 in \\[29 .. 24\\]
=> bit\\[ 23 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit23 = 31 => bit\\[ 23 \\]
= 0 ; if MapBit23= 30 => bit\\[ 23 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit23(&mut self) -> MAPBIT23_W<24> {
        MAPBIT23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 20 .. 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit23_20](index.html) module"]
pub struct MAPBIT23_20_SPEC;
impl crate::RegisterSpec for MAPBIT23_20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit23_20::R](R) reader structure"]
impl crate::Readable for MAPBIT23_20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit23_20::W](W) writer structure"]
impl crate::Writable for MAPBIT23_20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT23_20 to value 0x1716_1514"]
impl crate::Resettable for MAPBIT23_20_SPEC {
    const RESET_VALUE: Self::Ux = 0x1716_1514;
}
