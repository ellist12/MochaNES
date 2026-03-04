use crate::{bus::Bus, cpu::cpu::Cpu};


pub struct LDA;

impl LDA {
    pub fn immedeate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // LDA Immediate: Ambil byte berikutnya, taruh di register A
        // Ukuran Opcode : 2 byte
        // Jumlah cycle : 2
        // Contoh kode assembly : LDA #$30 [A9 30]
        // Artinya : Ambil angka di byte berikutnya (30) lalu masukkan ke register A
        let param = bus.read(cpu.pc);
        // println!("param a9: {}", param);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = param;
        println!("LDA #${:x}", param);
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }

    // LDA Zeropage: Ambil data di alamat ram bagian ZEROPAGE yang di specify di 1 byte berikutnya
    //               bagian ZEROPAGE di ram punya rentang dari $0000 - $00FF
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 3
    // Contoh kode assembly : LDA $10 [A5 10]
    // Artinya : ambil angka di ram dengan address $10 ($0010), dan masukkan ke register A
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let addr = bus.read(cpu.pc);
        let param = bus.read(addr as u16);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = param;
        println!("LDA ${:?}", param);
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 3;
        3
    }

    // LDA Absolute: Ambil data di alamat spesifik yang ditunjuk oleh 2 byte berikutnya
    // Ukuran Opcode : 3 byte
    // Jumlah cycle : 4
    // Contoh kode assembly : LDA $1000 [AD 00 10]
    // Artinya : Ambil angka di dua byte berikutnya ($1000), lalu masukkan ke register A
    pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let lo = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let hi = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let addr = (hi << 8) | lo;
        let data = bus.read(addr);
        println!("LDA ${:x}", addr);
        cpu.a = data;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 4;
        4
    }

    // LDA Indirect, Y
    // Ambil 2 byte data ram zeropage di alamat yang ditunjuk di byte berikutnya (ptr dan ptr + 1).
    // Kedua data itu akan digabung jadi satu alamat baru, lalu tambahkan value register Y ke alamat itu.
    // Lalu ambil data dari alamat hasil penjumlahan dengan register Y itu, dan masukkan nilainya ke
    // register A
    // Ukuran opcode : 2 byte
    // Jumlah cycle : 5 / 6 jika page cross
    // Contoh kode assembly : LDA ($00), Y
    // Artinya :
    //      1. Ambil 2 byte data dari alamat $00 di dalam bagian zeropage ram ($00 lo dan $01 hi)
    //      2. Gabung value kedua byte tersebut jadi satu alamat baru (misal : $2001)
    //      3. Tambahkan alamat baru tersebut dengan value yang ada di register Y (misal : $05, $2001 + $04 = $2004)
    //      4. Ambil data dari alamat $2004
    pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let data_lo = bus.read(param as u16) as u16;
        let data_hi = bus.read(param.wrapping_add(1) as u16) as u16;
        let data_addr = (data_hi << 8) | data_lo;
        let data_addr_append_y = data_addr + cpu.y as u16;
        let data = bus.read(data_addr_append_y as u16);
        cpu.a = data;
        let mut cycle = 5;
        if (data_addr & 0xFF00) != (data_addr_append_y & 0xFF00) {
            cycle += 1;
        }
        cpu.update_zero_and_negative_flags(cpu.a);
        cycle
    }
}
