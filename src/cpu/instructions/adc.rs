use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct ADC;

impl ADC {
    // ADC zeropage
    // Ambil data dari ram di bagian address zeropage, lalu tambahkan value data itu dengan
    // value di register A, dan value carry (kalau ada). Lalu masukkan hasilnya ke register A.
    // Jangan lupa cek apakah hasil penjumlahan punya carry dan overflow apa tidak
    // Ukuran opcode : 2 byte
    // Jumlah cycle  : 3
    // Contoh kode assembly : ADC $10
    // Artinya  : Ambil data dari ram di bagian address zeropage di alamat $10, lalu tambahkan
    //            value itu dengan value di register A, hasilnya simpan di dalam register A
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let addr = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let data = bus.read(addr as u16);
        let carry = cpu.status & 0b00000001;
        let new_data = cpu.a as u16 + data as u16 + carry as u16;
        // cek carry
        if new_data > 0xFF {
            // set carry flag ke 1
            cpu.status = cpu.status | 0b00000001;
        } else {
            cpu.status = cpu.status & 0b11111110;
        }

        // cek overflow
        if ((cpu.a & 0b10000000) == (data & 0b10000000)) &&
            (cpu.a & 0b10000000) != (new_data as u8 & 0b10000000) {
            cpu.status = cpu.status | 0b01000000;
        } else {
            cpu.status = cpu.status & 0b10111111;
        }

        cpu.a = new_data as u8;

        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 3;
        3
    }

    // ADC Immediate
    // Ambil data dari angka yang di specify di byte berikutnya, lalu tambahkan angka itu dengan
    // value yang ada di register A, tambahkan juga bit carry jika operasi penjumlahan sebelumnya
    // menghasilkan carry. Hasilnya nanti disimpan di register A. Jangan lupa cek juga untuk status
    // overflow dan carry setelah menjumlahkan
    // Ukuran opcode : 2 byte
    // Jumlah cycle  : 3
    // Contoh kode assembly : ADC #$10
    // Artinya : Ambil angka 10, lalu tambahkan dengan value di register A, hasilnya simpan di register A
    pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let carry = cpu.status & 0b00000001;
        let result = cpu.a as u16 + param as u16 + carry as u16;

        // Cek carry
        if result > 0xFF {
            cpu.status = cpu.status | 0b00000001;
        } else {
            cpu.status = cpu.status & 0b11111110;
        }

        // Cek overflow
        if ((cpu.a & 0b10000000) == (param & 0b10000000)) &&
                (result as u8 & 0b10000000) != (cpu.a & 0b10000000) {
            cpu.status = cpu.status | 0b01000000;
        } else {
            cpu.status = cpu.status & 0b10111111;
        }

        cpu.a = result as u8;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 3;
        3
    }
}
