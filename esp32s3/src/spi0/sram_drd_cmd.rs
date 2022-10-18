#[doc = "Register `SRAM_DRD_CMD` reader"]
pub struct R(crate::R<SRAM_DRD_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_DRD_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_DRD_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_DRD_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_DRD_CMD` writer"]
pub struct W(crate::W<SRAM_DRD_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_DRD_CMD_SPEC>;
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
impl From<crate::W<SRAM_DRD_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_DRD_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_VALUE` reader - When SPI0 reads Ext_RAM, it is the command value of CMD phase."]
pub type CACHE_SRAM_USR_RD_CMD_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_VALUE` writer - When SPI0 reads Ext_RAM, it is the command value of CMD phase."]
pub type CACHE_SRAM_USR_RD_CMD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_DRD_CMD_SPEC, u16, u16, 16, O>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_BITLEN` reader - When SPI0 reads Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
pub type CACHE_SRAM_USR_RD_CMD_BITLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_BITLEN` writer - When SPI0 reads Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
pub type CACHE_SRAM_USR_RD_CMD_BITLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_DRD_CMD_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:15 - When SPI0 reads Ext_RAM, it is the command value of CMD phase."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_value(&self) -> CACHE_SRAM_USR_RD_CMD_VALUE_R {
        CACHE_SRAM_USR_RD_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - When SPI0 reads Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_bitlen(&self) -> CACHE_SRAM_USR_RD_CMD_BITLEN_R {
        CACHE_SRAM_USR_RD_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - When SPI0 reads Ext_RAM, it is the command value of CMD phase."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_value(&mut self) -> CACHE_SRAM_USR_RD_CMD_VALUE_W<0> {
        CACHE_SRAM_USR_RD_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 28:31 - When SPI0 reads Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_bitlen(&mut self) -> CACHE_SRAM_USR_RD_CMD_BITLEN_W<28> {
        CACHE_SRAM_USR_RD_CMD_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM DDR read command control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_drd_cmd](index.html) module"]
pub struct SRAM_DRD_CMD_SPEC;
impl crate::RegisterSpec for SRAM_DRD_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_drd_cmd::R](R) reader structure"]
impl crate::Readable for SRAM_DRD_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_drd_cmd::W](W) writer structure"]
impl crate::Writable for SRAM_DRD_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_DRD_CMD to value 0"]
impl crate::Resettable for SRAM_DRD_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}