#[doc = "Register `BLK6_W3` reader"]
pub struct R(crate::R<BLK6_W3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK6_W3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK6_W3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK6_W3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK6_W3` reader - Otp block6 word3 data."]
pub type BLOCK6_W3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block6 word3 data."]
    #[inline(always)]
    pub fn block6_w3(&self) -> BLOCK6_W3_R {
        BLOCK6_W3_R::new(self.bits)
    }
}
#[doc = "Otp debuger block6 data register3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk6_w3](index.html) module"]
pub struct BLK6_W3_SPEC;
impl crate::RegisterSpec for BLK6_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk6_w3::R](R) reader structure"]
impl crate::Readable for BLK6_W3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK6_W3 to value 0"]
impl crate::Resettable for BLK6_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}