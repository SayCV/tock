//! Component for process printers.
//!
//! Usage
//! -----
//! ```rust
//! let process_printer = ProcessPrinterTextComponent::new()
//!     .finalize(components::process_printer_component_static!());
//! ```

use core::mem::MaybeUninit;
use kernel::component::Component;

#[macro_export]
macro_rules! process_printer_text_component_static {
    () => {{
        kernel::static_buf!(kernel::process::ProcessPrinterText)
    };};
}

pub struct ProcessPrinterTextComponent {}

impl ProcessPrinterTextComponent {
    pub fn new() -> ProcessPrinterTextComponent {
        ProcessPrinterTextComponent {}
    }
}

impl Component for ProcessPrinterTextComponent {
    type StaticInput = &'static mut MaybeUninit<kernel::process::ProcessPrinterText>;
    type Output = &'static kernel::process::ProcessPrinterText;

    fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        static_buffer.write(kernel::process::ProcessPrinterText::new())
    }
}
