use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // 0th Interrupt Stack Table entry is the double fault stack
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            // dont have memory management so just use a static mut arr
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt, 
            Selectors { 
                code_selector, 
                tss_selector, 
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;
    
    GDT.0.load();
    // reload the Code Segment register and load the TSS
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

