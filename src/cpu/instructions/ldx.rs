use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct LDX;

impl LDX {
    // LDX Immideate: Ambil byte berikutnya, taruh di register X
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : LDX #$10 [A2 10]
    // Artinya : ambil angka di byte berikutnya (10), dan masukkan ke register X
    pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.x = param;
        println!("LDX #${:x}", param);
        cpu.update_zero_and_negative_flags(cpu.x);
        cpu.cycle += 2;
        2
    }

    // LDX Zeropage
    // Ambil data yang ada di alamat ram zeropage yang di specify di byte berikutnya,
    // lalu taruh di register X
    // Ukuran opcode : 2 byte
    // Jumlah cycle  : 3
    // Contoh kode assembly : LDX $01
    // Artinya : ambil angka di bagian zeropage ram di alamat $01 ($0001), lalu masukkan hasilnya
    //           ke register X
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let data = bus.read(param as u16);
        cpu.x = data;
        println!("LDX ${}", param);
        cpu.update_zero_and_negative_flags(cpu.x);
        cpu.cycle += 3;
        3
    }
}
