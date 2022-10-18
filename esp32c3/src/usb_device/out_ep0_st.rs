#[doc = "Register `OUT_EP0_ST` reader"]
pub struct R(crate::R<OUT_EP0_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_EP0_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_EP0_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_EP0_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_EP0_STATE` reader - State of OUT Endpoint 0."]
pub type OUT_EP0_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_EP0_WR_ADDR` reader - Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0."]
pub type OUT_EP0_WR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_EP0_RD_ADDR` reader - Read data address of OUT endpoint 0."]
pub type OUT_EP0_RD_ADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 0."]
    #[inline(always)]
    pub fn out_ep0_state(&self) -> OUT_EP0_STATE_R {
        OUT_EP0_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0."]
    #[inline(always)]
    pub fn out_ep0_wr_addr(&self) -> OUT_EP0_WR_ADDR_R {
        OUT_EP0_WR_ADDR_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of OUT endpoint 0."]
    #[inline(always)]
    pub fn out_ep0_rd_addr(&self) -> OUT_EP0_RD_ADDR_R {
        OUT_EP0_RD_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "USB_DEVICE_OUT_EP0_ST_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ep0_st](index.html) module"]
pub struct OUT_EP0_ST_SPEC;
impl crate::RegisterSpec for OUT_EP0_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_ep0_st::R](R) reader structure"]
impl crate::Readable for OUT_EP0_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_EP0_ST to value 0"]
impl crate::Resettable for OUT_EP0_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}