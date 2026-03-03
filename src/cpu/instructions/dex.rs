use crate::cpu::cpu::Cpu;

pub struct DEX;

impl DEX {
    // DEX (Decrement Register X)
    // Kurangi nilai register X dengan 1, lalu simpan hasilnya ke register X
    // Ukuran opcode : 1 byte
    // Jumlah cycle  : 2 cycle
    // Contoh kode assembly : DEX [ca]
    // Artinya : Kurangi nilai di register X sebanyak 1 buah
    pub fn decrement(cpu: &mut Cpu) -> u16 {
        println!("DEX");
        cpu.x = cpu.x.wrapping_sub(1);
        cpu.update_zero_and_negative_flags(cpu.x);
        cpu.cycle += 2;
        2
    }
}
