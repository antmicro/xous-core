use crate::api::Point;
use susres::{ManagedMem, RegManager, RegOrField, SuspendResume};
use utralib::generated::*;
use xous::MemoryRange;
use crate::api::{LINES, WIDTH};

pub const FB_WIDTH_WORDS: usize = 11;
pub const FB_WIDTH_PIXELS: usize = WIDTH;
pub const FB_LINES: usize = LINES;
pub const FB_SIZE: usize = FB_WIDTH_WORDS * FB_LINES; // 44 bytes by 536 lines
const CONFIG_CLOCK_FREQUENCY: u32 = 100_000_000;


pub struct XousDisplay {
    fb: MemoryRange,
    hwfb: MemoryRange,
    srfb: ManagedMem<{ utralib::generated::HW_MEMLCD_MEM_LEN }>,
    csr: utralib::CSR<u32>,
    susres: RegManager<{ utra::memlcd::MEMLCD_NUMREGS }>,
}

impl XousDisplay {
    pub fn new() -> XousDisplay {
        let fb = xous::syscall::map_memory(
            None,
            None,
            ((FB_WIDTH_WORDS * FB_LINES * 4) + 4096) & !4095,
            xous::MemoryFlags::R | xous::MemoryFlags::W,
        )
        .expect("couldn't map backing frame buffer");
        let temp: *mut [u32; FB_SIZE] = fb.as_mut_ptr() as *mut [u32; FB_SIZE];
        for words in 0..FB_SIZE {
            unsafe {
                (*temp)[words] = 0xFFFF_FFFF;
            }
        }

        let hwfb = xous::syscall::map_memory(
            xous::MemoryAddress::new(HW_MEMLCD_MEM),
            None,
            ((FB_WIDTH_WORDS * FB_LINES * 4) + 4096) & !4095,
            xous::MemoryFlags::R | xous::MemoryFlags::W,
        )
        .expect("couldn't map hardware frame buffer");
        let temp: *mut [u32; FB_SIZE] = hwfb.as_mut_ptr() as *mut [u32; FB_SIZE];
        for words in 0..FB_SIZE {
            unsafe {
                (*temp)[words] = 0xFFFF_FFFF;
            }
        }

        let control = xous::syscall::map_memory(
            xous::MemoryAddress::new(HW_MEMLCD_BASE),
            None,
            4096,
            xous::MemoryFlags::R | xous::MemoryFlags::W,
        )
        .expect("couldn't map control port");

        let mut display = XousDisplay {
            fb: fb,
            hwfb: hwfb,
            csr: CSR::new(control.as_mut_ptr() as *mut u32),
            susres: RegManager::new(control.as_mut_ptr() as *mut u32),
            srfb: ManagedMem::new(hwfb),
        };

        display.set_clock(CONFIG_CLOCK_FREQUENCY);
        display
            .susres
            .push(RegOrField::Field(utra::memlcd::PRESCALER_PRESCALER), None);
        display.sync_clear();

        /*
        use log::{error, info};
        info!("GFX: fb 0x{:08x} bytes at 0x{:08x}", usize::from(fb.size), usize::from(fb.addr));
        info!("GFX: hwfb 0x{:08x} bytes at 0x{:08x}", usize::from(hwfb.size), usize::from(hwfb.addr));
        info!("GFX: csr 0x{:08x} bytes at 0x{:08x}", usize::from(control.size), usize::from(control.addr));
        */

        display
    }

    pub fn suspend(&mut self, draw_note: bool) {
        while self.busy() {
            // just wait until any pending FB operations are done
        }
        self.srfb.suspend();
        self.susres.suspend();

        if draw_note {
            let note = crate::sleep_note::LOGO_MAP;
            let note_lines = note.len() / FB_WIDTH_WORDS;
            let start_line = (FB_LINES - note_lines) / 2;
            let hwfb: *mut [u32; FB_SIZE] = self.hwfb.as_mut_ptr() as *mut [u32; FB_SIZE];
            for words in 0..note.len() {
                unsafe {
                    (*hwfb)[words + start_line * FB_WIDTH_WORDS] = note[words];
                }
            }
            for lines in start_line..start_line + note_lines {
                // set the dirty bits
                unsafe {
                    (*hwfb)[lines * FB_WIDTH_WORDS + (FB_WIDTH_WORDS - 1)] |= 0x1_0000;
                }
            }
            self.update_dirty();
            while self.busy() {
                // busy wait, blocking suspend until this has happened
            }
        }
    }
    pub fn resume(&mut self, drew_note: bool) {
        self.susres.resume();
        self.srfb.resume();

        if drew_note {
            let hwfb: *mut [u32; FB_SIZE] = self.hwfb.as_mut_ptr() as *mut [u32; FB_SIZE];
            for lines in 0..FB_LINES {
                // set the dirty bits of all lines to force a redraw of the restored data
                unsafe {
                    (*hwfb)[lines * FB_WIDTH_WORDS + (FB_WIDTH_WORDS - 1)] |= 0x1_0000;
                }
            }
            self.update_dirty();
            while self.busy() {
                // busy wait, blocking resume until this has happened
            }
        }
    }

    pub fn screen_size(&self) -> Point {
        Point::new(FB_WIDTH_PIXELS as i16, FB_LINES as i16)
    }

    pub fn redraw(&mut self) {
        let mut busy_count = 0;
        let mut dirty_count = 0;
        while self.busy() {
            xous::yield_slice();
            busy_count += 1;
        }
        let fb: *mut [u32; FB_SIZE] = self.fb.as_mut_ptr() as *mut [u32; FB_SIZE];
        let hwfb: *mut [u32; FB_SIZE] = self.hwfb.as_mut_ptr() as *mut [u32; FB_SIZE];
        for words in 0..FB_SIZE {
            unsafe {
                (*hwfb)[words] = (*fb)[words];
            }
        }
        self.update_dirty();
        // clear all the dirty bits, under the theory that it's time-wise cheaper on average
        // to visit every line and clear the dirty bits than it is to do an update_all()
        for lines in 0..FB_LINES {
            if unsafe { (*fb)[lines * FB_WIDTH_WORDS + (FB_WIDTH_WORDS - 1)] & 0xFFFF_0000 } != 0x0
            {
                dirty_count += 1;
            }
            unsafe {
                (*fb)[lines * FB_WIDTH_WORDS + (FB_WIDTH_WORDS - 1)] &= 0x0000_FFFF;
            }
        }
        log::trace!("redraw {}/{}", busy_count, dirty_count);
    }

    // note: this API is used by emulation, don't remove calls to it
    pub fn update(&mut self) {}

    pub fn native_buffer(&mut self) -> &mut [u32; FB_SIZE] {
        unsafe { &mut *(self.fb.as_mut_ptr() as *mut [u32; FB_SIZE]) }
    }

    pub fn blit_screen(&mut self, bmp: &[u32]) {
        let framebuffer = self.fb.as_mut_ptr() as *mut u32;

        for words in 0..FB_SIZE {
            unsafe {
                framebuffer.add(words).write_volatile(bmp[words]);
            }
        }
        self.update_all();

        while self.busy() {}
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.fb.as_slice::<u32>()[..FB_SIZE]
    }

    /// Beneath this line are pure-HAL layer, and should not be user-visible

    ///
    fn set_clock(&mut self, clk_mhz: u32) {
        self.csr
            .wfo(utra::memlcd::PRESCALER_PRESCALER, (clk_mhz / 2_000_000) - 1);
    }

    fn update_all(&mut self) {
        self.csr.wfo(utra::memlcd::COMMAND_UPDATEALL, 1);
    }

    fn update_dirty(&mut self) {
        self.csr.wfo(utra::memlcd::COMMAND_UPDATEDIRTY, 1);
    }

    /// "synchronous clear" -- must be called on init, so that the state of the LCD
    /// internal memory is consistent with the state of the frame buffer
    fn sync_clear(&mut self) {
        let framebuffer = self.fb.as_mut_ptr() as *mut u32;
        for words in 0..FB_SIZE {
            if words % FB_WIDTH_WORDS != 10 {
                unsafe { framebuffer.add(words).write_volatile(0xFFFF_FFFF) };
            } else {
                unsafe { framebuffer.add(words).write_volatile(0x0000_FFFF) };
            }
        }
        self.update_all(); // because we force an all update here
        while self.busy() {}
    }

    fn busy(&self) -> bool {
        self.csr.rf(utra::memlcd::BUSY_BUSY) == 1
    }

    pub fn set_devboot(&mut self, ena: bool) {
        if ena {
            self.csr.wfo(utra::memlcd::DEVBOOT_DEVBOOT, 1);
        } else {
            self.csr.wfo(utra::memlcd::DEVBOOT_DEVBOOT, 0);
        }
    }
}
