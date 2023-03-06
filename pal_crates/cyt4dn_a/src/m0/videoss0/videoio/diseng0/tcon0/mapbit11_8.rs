#[doc = "Register `MAPBIT11_8` reader"]
pub struct R(crate::R<MAPBIT11_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT11_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT11_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT11_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT11_8` writer"]
pub struct W(crate::W<MAPBIT11_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT11_8_SPEC>;
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
impl From<crate::W<MAPBIT11_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT11_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT8` reader - map bit\\[ 8 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit8 in \\[ 23..0 \\]
=> bit\\[ 8 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit8 \\]; if MapBit8 in \\[29 .. 24\\]
=> bit\\[ 8 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit8 = 31 => bit\\[ 8 \\]
= 0 ; if MapBit8= 30 => bit\\[ 8 \\]
= 1"]
pub type MAPBIT8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT8` writer - map bit\\[ 8 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit8 in \\[ 23..0 \\]
=> bit\\[ 8 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit8 \\]; if MapBit8 in \\[29 .. 24\\]
=> bit\\[ 8 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit8 = 31 => bit\\[ 8 \\]
= 0 ; if MapBit8= 30 => bit\\[ 8 \\]
= 1"]
pub type MAPBIT8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT11_8_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT9` reader - map bit\\[ 9 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit9 in \\[ 23..0 \\]
=> bit\\[ 9 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit9 \\]; if MapBit9 in \\[29 .. 24\\]
=> bit\\[ 9 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit9 = 31 => bit\\[ 9 \\]
= 0 ; if MapBit9= 30 => bit\\[ 9 \\]
= 1"]
pub type MAPBIT9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT9` writer - map bit\\[ 9 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit9 in \\[ 23..0 \\]
=> bit\\[ 9 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit9 \\]; if MapBit9 in \\[29 .. 24\\]
=> bit\\[ 9 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit9 = 31 => bit\\[ 9 \\]
= 0 ; if MapBit9= 30 => bit\\[ 9 \\]
= 1"]
pub type MAPBIT9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT11_8_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT10` reader - map bit\\[ 10 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit10 in \\[ 23..0 \\]
=> bit\\[ 10 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit10 \\]; if MapBit10 in \\[29 .. 24\\]
=> bit\\[ 10 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit10 = 31 => bit\\[ 10 \\]
= 0 ; if MapBit10= 30 => bit\\[ 10 \\]
= 1"]
pub type MAPBIT10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT10` writer - map bit\\[ 10 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit10 in \\[ 23..0 \\]
=> bit\\[ 10 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit10 \\]; if MapBit10 in \\[29 .. 24\\]
=> bit\\[ 10 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit10 = 31 => bit\\[ 10 \\]
= 0 ; if MapBit10= 30 => bit\\[ 10 \\]
= 1"]
pub type MAPBIT10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT11_8_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT11` reader - map bit\\[ 11 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit11 in \\[ 23..0 \\]
=> bit\\[ 11 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit11 \\]; if MapBit11 in \\[29 .. 24\\]
=> bit\\[ 11 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit11 = 31 => bit\\[ 11 \\]
= 0 ; if MapBit11= 30 => bit\\[ 11 \\]
= 1"]
pub type MAPBIT11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT11` writer - map bit\\[ 11 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit11 in \\[ 23..0 \\]
=> bit\\[ 11 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit11 \\]; if MapBit11 in \\[29 .. 24\\]
=> bit\\[ 11 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit11 = 31 => bit\\[ 11 \\]
= 0 ; if MapBit11= 30 => bit\\[ 11 \\]
= 1"]
pub type MAPBIT11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT11_8_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[ 8 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit8 in \\[ 23..0 \\]
=> bit\\[ 8 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit8 \\]; if MapBit8 in \\[29 .. 24\\]
=> bit\\[ 8 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit8 = 31 => bit\\[ 8 \\]
= 0 ; if MapBit8= 30 => bit\\[ 8 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit8(&self) -> MAPBIT8_R {
        MAPBIT8_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[ 9 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit9 in \\[ 23..0 \\]
=> bit\\[ 9 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit9 \\]; if MapBit9 in \\[29 .. 24\\]
=> bit\\[ 9 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit9 = 31 => bit\\[ 9 \\]
= 0 ; if MapBit9= 30 => bit\\[ 9 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit9(&self) -> MAPBIT9_R {
        MAPBIT9_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[ 10 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit10 in \\[ 23..0 \\]
=> bit\\[ 10 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit10 \\]; if MapBit10 in \\[29 .. 24\\]
=> bit\\[ 10 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit10 = 31 => bit\\[ 10 \\]
= 0 ; if MapBit10= 30 => bit\\[ 10 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit10(&self) -> MAPBIT10_R {
        MAPBIT10_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[ 11 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit11 in \\[ 23..0 \\]
=> bit\\[ 11 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit11 \\]; if MapBit11 in \\[29 .. 24\\]
=> bit\\[ 11 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit11 = 31 => bit\\[ 11 \\]
= 0 ; if MapBit11= 30 => bit\\[ 11 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit11(&self) -> MAPBIT11_R {
        MAPBIT11_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[ 8 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit8 in \\[ 23..0 \\]
=> bit\\[ 8 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit8 \\]; if MapBit8 in \\[29 .. 24\\]
=> bit\\[ 8 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit8 = 31 => bit\\[ 8 \\]
= 0 ; if MapBit8= 30 => bit\\[ 8 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit8(&mut self) -> MAPBIT8_W<0> {
        MAPBIT8_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[ 9 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. if MapBit9 in \\[ 23..0 \\]
=> bit\\[ 9 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit9 \\]; if MapBit9 in \\[29 .. 24\\]
=> bit\\[ 9 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit9 = 31 => bit\\[ 9 \\]
= 0 ; if MapBit9= 30 => bit\\[ 9 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit9(&mut self) -> MAPBIT9_W<8> {
        MAPBIT9_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[ 10 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit10 in \\[ 23..0 \\]
=> bit\\[ 10 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit10 \\]; if MapBit10 in \\[29 .. 24\\]
=> bit\\[ 10 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit10 = 31 => bit\\[ 10 \\]
= 0 ; if MapBit10= 30 => bit\\[ 10 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit10(&mut self) -> MAPBIT10_W<16> {
        MAPBIT10_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[ 11 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit11 in \\[ 23..0 \\]
=> bit\\[ 11 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit11 \\]; if MapBit11 in \\[29 .. 24\\]
=> bit\\[ 11 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit11 = 31 => bit\\[ 11 \\]
= 0 ; if MapBit11= 30 => bit\\[ 11 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit11(&mut self) -> MAPBIT11_W<24> {
        MAPBIT11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 8 .. 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit11_8](index.html) module"]
pub struct MAPBIT11_8_SPEC;
impl crate::RegisterSpec for MAPBIT11_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit11_8::R](R) reader structure"]
impl crate::Readable for MAPBIT11_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit11_8::W](W) writer structure"]
impl crate::Writable for MAPBIT11_8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT11_8 to value 0x0b0a_0908"]
impl crate::Resettable for MAPBIT11_8_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b0a_0908;
}
