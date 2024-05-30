#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x84],
    dma_in_status: DMA_IN_STATUS,
    dma_inlink: DMA_INLINK,
    _reserved2: [u8; 0x0bb0],
    dma_int_status: DMA_INT_STATUS,
    dma_int_clear: DMA_INT_CLEAR,
    _reserved4: [u8; 0x6c],
    status: STATUS,
    _reserved5: [u8; 0x54],
    dma_outlink: DMA_OUTLINK,
    _reserved6: [u8; 0x08],
    dma_out_status: DMA_OUT_STATUS,
}
impl RegisterBlock {
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn dma_in_status(&self) -> &DMA_IN_STATUS {
        &self.dma_in_status
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn dma_inlink(&self) -> &DMA_INLINK {
        &self.dma_inlink
    }
    #[doc = "0xc3c - "]
    #[inline(always)]
    pub const fn dma_int_status(&self) -> &DMA_INT_STATUS {
        &self.dma_int_status
    }
    #[doc = "0xc40 - "]
    #[inline(always)]
    pub const fn dma_int_clear(&self) -> &DMA_INT_CLEAR {
        &self.dma_int_clear
    }
    #[doc = "0xcb0 - "]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0xd08 - "]
    #[inline(always)]
    pub const fn dma_outlink(&self) -> &DMA_OUTLINK {
        &self.dma_outlink
    }
    #[doc = "0xd14 - "]
    #[inline(always)]
    pub const fn dma_out_status(&self) -> &DMA_OUT_STATUS {
        &self.dma_out_status
    }
}
#[doc = "DMA_IN_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_status`] module"]
pub type DMA_IN_STATUS = crate::Reg<dma_in_status::DMA_IN_STATUS_SPEC>;
#[doc = ""]
pub mod dma_in_status;
#[doc = "DMA_INLINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_inlink::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_inlink::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_inlink`] module"]
pub type DMA_INLINK = crate::Reg<dma_inlink::DMA_INLINK_SPEC>;
#[doc = ""]
pub mod dma_inlink;
#[doc = "DMA_INT_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_status`] module"]
pub type DMA_INT_STATUS = crate::Reg<dma_int_status::DMA_INT_STATUS_SPEC>;
#[doc = ""]
pub mod dma_int_status;
#[doc = "DMA_INT_CLEAR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_clear`] module"]
pub type DMA_INT_CLEAR = crate::Reg<dma_int_clear::DMA_INT_CLEAR_SPEC>;
#[doc = ""]
pub mod dma_int_clear;
#[doc = "STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "DMA_OUTLINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_outlink::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_outlink::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_outlink`] module"]
pub type DMA_OUTLINK = crate::Reg<dma_outlink::DMA_OUTLINK_SPEC>;
#[doc = ""]
pub mod dma_outlink;
#[doc = "DMA_OUT_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_status`] module"]
pub type DMA_OUT_STATUS = crate::Reg<dma_out_status::DMA_OUT_STATUS_SPEC>;
#[doc = ""]
pub mod dma_out_status;
