use crate::cpu::{self, cpu::Cpu};

pub struct INY;
impl INY {
    // INY (Increment Y)
    // Jumlahkan value yang ada di register Y dengan angka 1, masukkan hasilnya ke register Y
    // Ukuran opcode : 1 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : INY
    // Artinya : naikkan angka di register Y sejumlah 1 buah
    pub fn increment(cpu: &mut Cpu) -> u16 {
        cpu.y = cpu.y.wrapping_add(1);
        cpu.update_zero_and_negative_flags(cpu.y);
        cpu.cycle += 2;
        2
    }
}
