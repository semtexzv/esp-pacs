#[doc = "Register `DATA_8` reader"]
pub struct R(crate::R<DATA_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_8` writer"]
pub struct W(crate::W<DATA_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_8_SPEC>;
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
impl From<crate::W<DATA_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_8` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
pub type DATA_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_8` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
pub type DATA_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
    #[inline(always)]
    pub fn data_8(&self) -> DATA_8_R {
        DATA_8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
    #[inline(always)]
    #[must_use]
    pub fn data_8(&mut self) -> DATA_8_W<0> {
        DATA_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_8](index.html) module"]
pub struct DATA_8_SPEC;
impl crate::RegisterSpec for DATA_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_8::R](R) reader structure"]
impl crate::Readable for DATA_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_8::W](W) writer structure"]
impl crate::Writable for DATA_8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_8 to value 0"]
impl crate::Resettable for DATA_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}