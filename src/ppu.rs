use std::fmt;

use crate::mochanes::Region;

pub struct Ppu {
    ppuctrl: u8, // tempat CPU mengatur PPU
    ppumask: u8, // tempat CPU mengatur setting visual
    ppustatus: u8, // tempat PPU menuliskan statusnya yang kemudian akan dibaca oleh CPU
    ppuaddr: u16, // tempat CPU menuliskan alamat ram yang ingin di read / write
    v: u16,
    t: u16,
    x: u8,
    w: bool, // write toggle untuk menentukan cpu lagi nulis hi byte atau lo byte
                   // ke ppuaddr

    scanlines: u16, // letak titik yang sedang di render secara vertikal
    cycles: u16, // letak titik yang sedan di render secara horizontal
    frame_rendered: u8, // total frame yang sudah di render

    region: Region,
    v_blank_limit: u16,
    pre_render_scanline: u16,
    scanlines_limit: u16,
    cycles_limit: u16
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ppu")
        .field("ppuctrl", &format!("{:08b} [{}] [${:x}]", self.ppuctrl, self.ppuctrl, self.ppuctrl))
        .field("ppumask", &format!("{:08b} [{}] [${:x}]", self.ppumask, self.ppumask, self.ppumask))
        .field("ppustatus", &format!("{:08b} [{}] [${:x}]", self.ppustatus, self.ppustatus, self.ppustatus))
        .field("v", &format!("{:08b} [{}] [${:x}]", self.v, self.v, self.v))
        .field("t", &format!("{:08b} [{}] [${:x}]", self.t, self.t, self.t))
        .field("x", &format!("{:08b} [{}] [${:x}]", self.x, self.x, self.x))
        .field("w", &format!("{}", self.w))
        .field("scanlines", &format!("{:08b} [{}] [${:x}]", self.scanlines, self.scanlines, self.scanlines))
        .field("cycles", &format!("{:08b} [{}] [${:x}]", self.cycles, self.cycles, self.cycles))
        .finish()
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            ppuaddr: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            scanlines: 0,
            cycles: 0,
            frame_rendered: 0,
            v_blank_limit: 0,
            pre_render_scanline: 0,
            scanlines_limit: 0,
            cycles_limit: 0,
            region: Region::NTSC // set region default ke NTSC
        }
    }

    pub fn tick(&mut self) {
        if self.cycles >= self.cycles_limit {
            self.cycles = 0;
            self.scanlines += 1;
        }
        if self.scanlines >= self.scanlines_limit {
            self.scanlines = 0;
        }


        if self.scanlines == 240 {
            self.cycles += 1;
            return;
        } else if self.scanlines >= 241 && self.scanlines <= self.v_blank_limit && self.cycles == 0 {
            self.ppustatus = self.ppustatus | 0b10000000;
        } else if self.scanlines == self.pre_render_scanline && self.cycles == 0 {
            self.ppustatus = self.ppustatus & 0b01111111;
        }

        self.cycles += 1;
    }

    pub fn handle_write(&mut self, addr: u16, val: u8) {
        if addr == 0x2000 {
            self.ppuctrl = val;
        } else if addr == 0x2001 {
            self.ppumask = val;
        } else if addr == 0x2005 {
            if !self.w {
                self.t = self.t & !0x001F | (val as u16) >> 3;
                self.x = val & 0b111;
                self.w = true;
            } else {
                self.t = self.t & !0x73E0
                        | ((val as u16) & 0x07) << 12
                        | ((val as u16) & 0xF8) << 2;
                self.w = false;
            }
        } else if addr == 0x2006 {
            if !self.w {
                self.t = self.t & 0x00FF | ((val as u16)<< 8);
                self.w = true;
            } else {
                self.t = self.t & 0xFF00 | val as u16;
                self.v = self.t;
                self.w = false;
            }
        }
    }

    pub fn handle_read(&self, addr: u16) -> u8 {
        if addr == 0x2002 {
            self.ppustatus
        } else {
            println!("PPU address {} not implemented", addr);
            0
        }
    }

    pub fn set_region(&mut self, region: Region) {
        self.region = region;
        self.scanlines_limit = if self.region == Region::NTSC {
            262
        } else {
            312
        };
        self.cycles_limit = 341;
        self.v_blank_limit = if self.region == Region::NTSC {
            260
        } else {
            310
        };
        self.pre_render_scanline = if self.region == Region::NTSC {
            261
        } else {
            311
        };
    }
}

