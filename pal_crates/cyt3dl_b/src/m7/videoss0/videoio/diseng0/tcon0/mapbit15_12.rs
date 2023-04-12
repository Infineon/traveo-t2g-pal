#[doc = "Register `MAPBIT15_12` reader"]
pub struct R(crate::R<MAPBIT15_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT15_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT15_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT15_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT15_12` writer"]
pub struct W(crate::W<MAPBIT15_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT15_12_SPEC>;
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
impl From<crate::W<MAPBIT15_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT15_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT12` reader - map bit\\[ 12 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit12 in \\[ 23..0 \\]
=> bit\\[ 12 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit12 \\]; if MapBit12 in \\[29 .. 24\\]
=> bit\\[ 12 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit12 = 31 => bit\\[ 12 \\]
= 0 ; if MapBit12= 30 => bit\\[ 12 \\]
= 1"]
pub type MAPBIT12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT12` writer - map bit\\[ 12 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit12 in \\[ 23..0 \\]
=> bit\\[ 12 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit12 \\]; if MapBit12 in \\[29 .. 24\\]
=> bit\\[ 12 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit12 = 31 => bit\\[ 12 \\]
= 0 ; if MapBit12= 30 => bit\\[ 12 \\]
= 1"]
pub type MAPBIT12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT15_12_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT13` reader - map bit\\[ 13 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit13 in \\[ 23..0 \\]
=> bit\\[ 13 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit13 \\]; if MapBit13 in \\[29 .. 24\\]
=> bit\\[ 13 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit13 = 31 => bit\\[ 13 \\]
= 0 ; if MapBit13= 30 => bit\\[ 13 \\]
= 1"]
pub type MAPBIT13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT13` writer - map bit\\[ 13 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit13 in \\[ 23..0 \\]
=> bit\\[ 13 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit13 \\]; if MapBit13 in \\[29 .. 24\\]
=> bit\\[ 13 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit13 = 31 => bit\\[ 13 \\]
= 0 ; if MapBit13= 30 => bit\\[ 13 \\]
= 1"]
pub type MAPBIT13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT15_12_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT14` reader - map bit\\[ 14 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit14 in \\[ 23..0 \\]
=> bit\\[ 14 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit14 \\]; if MapBit14 in \\[29 .. 24\\]
=> bit\\[ 14 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit14 = 31 => bit\\[ 14 \\]
= 0 ; if MapBit14= 30 => bit\\[ 14 \\]
= 1"]
pub type MAPBIT14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT14` writer - map bit\\[ 14 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit14 in \\[ 23..0 \\]
=> bit\\[ 14 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit14 \\]; if MapBit14 in \\[29 .. 24\\]
=> bit\\[ 14 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit14 = 31 => bit\\[ 14 \\]
= 0 ; if MapBit14= 30 => bit\\[ 14 \\]
= 1"]
pub type MAPBIT14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT15_12_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT15` reader - map bit\\[ 15 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit15 in \\[ 23..0 \\]
=> bit\\[ 15 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit15 \\]; if MapBit15 in \\[29 .. 24\\]
=> bit\\[ 15 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit15 = 31 => bit\\[ 15 \\]
= 0 ; if MapBit15= 30 => bit\\[ 15 \\]
= 1"]
pub type MAPBIT15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT15` writer - map bit\\[ 15 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit15 in \\[ 23..0 \\]
=> bit\\[ 15 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit15 \\]; if MapBit15 in \\[29 .. 24\\]
=> bit\\[ 15 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit15 = 31 => bit\\[ 15 \\]
= 0 ; if MapBit15= 30 => bit\\[ 15 \\]
= 1"]
pub type MAPBIT15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT15_12_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[ 12 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit12 in \\[ 23..0 \\]
=> bit\\[ 12 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit12 \\]; if MapBit12 in \\[29 .. 24\\]
=> bit\\[ 12 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit12 = 31 => bit\\[ 12 \\]
= 0 ; if MapBit12= 30 => bit\\[ 12 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit12(&self) -> MAPBIT12_R {
        MAPBIT12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[ 13 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit13 in \\[ 23..0 \\]
=> bit\\[ 13 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit13 \\]; if MapBit13 in \\[29 .. 24\\]
=> bit\\[ 13 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit13 = 31 => bit\\[ 13 \\]
= 0 ; if MapBit13= 30 => bit\\[ 13 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit13(&self) -> MAPBIT13_R {
        MAPBIT13_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[ 14 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit14 in \\[ 23..0 \\]
=> bit\\[ 14 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit14 \\]; if MapBit14 in \\[29 .. 24\\]
=> bit\\[ 14 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit14 = 31 => bit\\[ 14 \\]
= 0 ; if MapBit14= 30 => bit\\[ 14 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit14(&self) -> MAPBIT14_R {
        MAPBIT14_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[ 15 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit15 in \\[ 23..0 \\]
=> bit\\[ 15 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit15 \\]; if MapBit15 in \\[29 .. 24\\]
=> bit\\[ 15 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit15 = 31 => bit\\[ 15 \\]
= 0 ; if MapBit15= 30 => bit\\[ 15 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit15(&self) -> MAPBIT15_R {
        MAPBIT15_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[ 12 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit12 in \\[ 23..0 \\]
=> bit\\[ 12 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit12 \\]; if MapBit12 in \\[29 .. 24\\]
=> bit\\[ 12 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit12 = 31 => bit\\[ 12 \\]
= 0 ; if MapBit12= 30 => bit\\[ 12 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit12(&mut self) -> MAPBIT12_W<0> {
        MAPBIT12_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[ 13 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit13 in \\[ 23..0 \\]
=> bit\\[ 13 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit13 \\]; if MapBit13 in \\[29 .. 24\\]
=> bit\\[ 13 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit13 = 31 => bit\\[ 13 \\]
= 0 ; if MapBit13= 30 => bit\\[ 13 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit13(&mut self) -> MAPBIT13_W<8> {
        MAPBIT13_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[ 14 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit14 in \\[ 23..0 \\]
=> bit\\[ 14 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit14 \\]; if MapBit14 in \\[29 .. 24\\]
=> bit\\[ 14 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit14 = 31 => bit\\[ 14 \\]
= 0 ; if MapBit14= 30 => bit\\[ 14 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit14(&mut self) -> MAPBIT14_W<16> {
        MAPBIT14_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[ 15 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit15 in \\[ 23..0 \\]
=> bit\\[ 15 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit15 \\]; if MapBit15 in \\[29 .. 24\\]
=> bit\\[ 15 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit15 = 31 => bit\\[ 15 \\]
= 0 ; if MapBit15= 30 => bit\\[ 15 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit15(&mut self) -> MAPBIT15_W<24> {
        MAPBIT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 12 .. 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit15_12](index.html) module"]
pub struct MAPBIT15_12_SPEC;
impl crate::RegisterSpec for MAPBIT15_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit15_12::R](R) reader structure"]
impl crate::Readable for MAPBIT15_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit15_12::W](W) writer structure"]
impl crate::Writable for MAPBIT15_12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT15_12 to value 0x0f0e_0d0c"]
impl crate::Resettable for MAPBIT15_12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0e_0d0c;
}
