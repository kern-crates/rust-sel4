use core::mem;
use core::ops::Range;
use core::slice;

use crate::{
    newtype_methods, sys, ASIDControl, ASIDPool, CNode, CPtr, CapType, IRQControl, LocalCPtr,
    GRANULE, PGD, TCB,
};

#[derive(Clone, Debug)]
pub struct BootInfo {
    ptr: *const sys::seL4_BootInfo,
}

pub type InitCSpaceSlot = usize;

impl BootInfo {
    pub unsafe fn from_ptr(ptr: *const sys::seL4_BootInfo) -> Self {
        assert_eq!(ptr.addr() % GRANULE.bytes(), 0); // sanity check
        Self { ptr }
    }

    pub fn sys(&self) -> &sys::seL4_BootInfo {
        unsafe { &*self.ptr }
    }

    fn extra_ptr(&self) -> *const u8 {
        unsafe {
            self.ptr
                .cast::<u8>()
                .offset(GRANULE.bytes().try_into().unwrap())
        }
    }

    fn extra_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.extra_ptr(), self.extra_len()) }
    }

    pub fn extra<'a>(&'a self) -> BootInfoExtraIter<'a> {
        BootInfoExtraIter::new(self)
    }

    pub fn extra_len(&self) -> usize {
        self.sys().extraLen.try_into().unwrap()
    }

    pub fn ipc_buffer(&self) -> *mut sys::seL4_IPCBuffer {
        self.sys().ipcBuffer
    }

    pub fn empty(&self) -> Range<InitCSpaceSlot> {
        region_to_range(self.sys().empty)
    }

    pub fn user_image_frames(&self) -> Range<InitCSpaceSlot> {
        region_to_range(self.sys().userImageFrames)
    }

    pub fn untyped(&self) -> Range<InitCSpaceSlot> {
        region_to_range(self.sys().untyped)
    }

    pub fn num_untyped(&self) -> usize {
        usize::try_from(self.untyped().end - self.untyped().start).unwrap()
    }

    fn untyped_list_inner(&self) -> &[sys::seL4_UntypedDesc] {
        &self.sys().untypedList[..self.num_untyped()]
    }

    pub fn untyped_list(&self) -> &[UntypedDesc] {
        let inner = self.untyped_list_inner();
        // safe because of #[repr(trasnparent)]
        unsafe { slice::from_raw_parts(inner.as_ptr().cast(), inner.len()) }
    }

    fn untyped_list_partition_point(&self) -> usize {
        self.untyped_list().partition_point(|ut| ut.is_device())
    }

    pub fn device_untyped_list(&self) -> &[UntypedDesc] {
        &self.untyped_list()[..self.untyped_list_partition_point()]
    }

    pub fn kernel_untyped_list(&self) -> &[UntypedDesc] {
        &self.untyped_list()[self.untyped_list_partition_point()..]
    }

    pub fn footprint(&self) -> Range<usize> {
        self.ptr.addr()..self.extra_ptr().addr() + self.extra_len()
    }

    pub fn init_thread_cnode() -> CNode {
        CNode::from_bits(
            sys::seL4_RootCapSlot::seL4_CapInitThreadCNode
                .try_into()
                .unwrap(),
        )
    }

    pub fn irq_control() -> IRQControl {
        IRQControl::from_bits(
            sys::seL4_RootCapSlot::seL4_CapIRQControl
                .try_into()
                .unwrap(),
        )
    }

    pub fn asid_control() -> ASIDControl {
        ASIDControl::from_bits(
            sys::seL4_RootCapSlot::seL4_CapASIDControl
                .try_into()
                .unwrap(),
        )
    }

    pub fn init_thread_asid_pool() -> ASIDPool {
        ASIDPool::from_bits(
            sys::seL4_RootCapSlot::seL4_CapInitThreadASIDPool
                .try_into()
                .unwrap(),
        )
    }

    pub fn init_thread_vspace() -> PGD {
        PGD::from_bits(
            sys::seL4_RootCapSlot::seL4_CapInitThreadVSpace
                .try_into()
                .unwrap(),
        )
    }

    pub fn init_thread_tcb() -> TCB {
        TCB::from_bits(
            sys::seL4_RootCapSlot::seL4_CapInitThreadTCB
                .try_into()
                .unwrap(),
        )
    }

    pub fn init_cspace_cptr(slot: InitCSpaceSlot) -> CPtr {
        CPtr::from_bits(slot.try_into().unwrap())
    }

    pub fn init_cspace_local_cptr<T: CapType>(slot: InitCSpaceSlot) -> LocalCPtr<T> {
        Self::init_cspace_cptr(slot).cast()
    }
}

fn region_to_range(region: sys::seL4_SlotRegion) -> Range<InitCSpaceSlot> {
    region.start.try_into().unwrap()..region.end.try_into().unwrap()
}

#[derive(Clone, Debug)]
pub struct BootInfoExtraStructure<'a> {
    pub id: BootInfoExtraStructureId,
    pub content: &'a [u8],
}

#[derive(Clone, Debug)]
pub enum BootInfoExtraStructureId {
    Padding,
    Fdt,
}

impl BootInfoExtraStructureId {
    // precondition: id \in seL4_BootInfoID
    // SEL4_BOOTINFO_HEADER_PADDING => None
    pub fn from_sys(id: sys::seL4_BootInfoID::Type) -> Option<Self> {
        match id {
            sys::seL4_BootInfoID::SEL4_BOOTINFO_HEADER_PADDING => {
                Some(BootInfoExtraStructureId::Padding)
            }
            sys::seL4_BootInfoID::SEL4_BOOTINFO_HEADER_FDT => Some(BootInfoExtraStructureId::Fdt),
            _ => None,
        }
    }
}

pub struct BootInfoExtraIter<'a> {
    bootinfo: &'a BootInfo,
    cursor: usize,
}

impl<'a> BootInfoExtraIter<'a> {
    fn new(bootinfo: &'a BootInfo) -> Self {
        Self {
            bootinfo,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for BootInfoExtraIter<'a> {
    type Item = BootInfoExtraStructure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.bootinfo.extra_slice().len() {
            let header = unsafe {
                &*self
                    .bootinfo
                    .extra_slice()
                    .as_ptr()
                    .offset(self.cursor.try_into().unwrap())
                    .cast::<sys::seL4_BootInfoHeader>()
            };
            let content_start = self.cursor + mem::size_of::<sys::seL4_BootInfoHeader>();
            let content_end = self.cursor + usize::try_from(header.len).unwrap();
            self.cursor = content_end;
            if let Some(id) = BootInfoExtraStructureId::from_sys(header.id) {
                return Some(BootInfoExtraStructure {
                    id,
                    content: &self.bootinfo.extra_slice()[content_start..content_end],
                });
            }
        }
        None
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntypedDesc(pub sys::seL4_UntypedDesc);

impl UntypedDesc {
    newtype_methods!(sys::seL4_UntypedDesc);

    pub fn paddr(&self) -> usize {
        self.inner().paddr.try_into().unwrap()
    }

    pub fn size_bits(&self) -> usize {
        self.inner().sizeBits.try_into().unwrap()
    }

    pub fn is_device(&self) -> bool {
        self.inner().isDevice != 0
    }
}

impl From<sys::seL4_UntypedDesc> for UntypedDesc {
    fn from(desc: sys::seL4_UntypedDesc) -> Self {
        Self::from_inner(desc)
    }
}
