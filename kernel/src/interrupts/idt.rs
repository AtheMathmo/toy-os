use super::HandlerFunc;
use x86::{IDTPointer, DescriptorTablePointer};
use x86::asm;
use bit_field::BitField;

pub struct Idt([Descriptor; 16]);

impl Idt {
    /// Initializes an IDT with missing descriptors.
    pub fn new() -> Self {
        Idt([Descriptor::missing(); 16])
    }

    pub fn set_handler(&mut self, descriptor: u8, handler: HandlerFunc) -> &mut DescriptorOptions {
        self.0[descriptor as usize] = Descriptor::new(asm::get_cs(), handler);
        &mut self.0[descriptor as usize].options
    }

    pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = IDTPointer(DescriptorTablePointer {
            limit: (size_of::<Self>() - 1) as u16,
            offset: self as *const _ as u64,
        });

        unsafe { asm::lidt(&ptr) }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct DescriptorOptions(BitField<u16>);

impl DescriptorOptions {
    fn minimal() -> Self {
        let mut options = BitField::new(0);
        // Set compulsory '1' bits.
        options.set_range(9..12, 0b111);
        DescriptorOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_priviledge_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_range(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_range(0..3, index);
        self
    }
}

/// Describes an entry in the IDT
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Descriptor {
    offset_low: u16,
    selector: u16,
    options: DescriptorOptions,
    offset_middle: u16,
    offset_top: u32,
    /// Must be set to zero
    reserved: u32,
}

impl Descriptor {
    fn new(selector: u16, handler_fn: HandlerFunc) -> Self {
        let ptr = handler_fn as u64;

        Descriptor {
            offset_low: ptr as u16,
            selector: selector,
            options: DescriptorOptions::new(),
            offset_middle: (ptr >> 16) as u16,
            offset_top: (ptr >> 32) as u32,
            reserved: 0,
        }
    }

    fn missing() -> Self {
        Descriptor {
            offset_low: 0,
            selector: 0,
            options: DescriptorOptions::minimal(),
            offset_middle: 0,
            offset_top: 0,
            reserved: 0,
        }
    }
}
