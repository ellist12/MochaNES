use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct ROR;

impl ROR {
    // ROR (Rotate Right) zeropage
    // Menggeser bit address zeropage ram ke kanan sejumlah 1x, lalu masukkan nilai carry yang ada di register
    // ke bit 7 dari data yang sudah di geser. bit paling kanan akan menjadi nilai
    // carry baru,
    // Ukuran opcode : 2 byte
    // Jumlah cycle  : 5
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // baca parameter address zeropage
        let param = bus.read(cpu.pc);
        println!("ROR {:x}", param);
        cpu.pc = cpu.pc.wrapping_add(1);

        // ambil data dari alamat di parameter itu
        let data = bus.read(param as u16);

        // simpan data carry yang ada di register status ke variabel old carry
        let old_carry = cpu.status & 0b00000001;

        // cari tahu apakah data yang ingin di rubah, bit pertamanya angka 1 atau bukan
        // kalau angka 1 berarti saat di geser ke kananya, akan menyebabkan carry
        let mut new_carry = 0b00000000;
        if data & 0b000000001 != 0 {
            new_carry = 0b00000001;
        } else {
            new_carry = 0b00000000;
        }

        // geser datanya, lalu masukkan data old carry ke bit 7, dan tulis balik new_datanya ke alamat
        // zeropage ram
        let new_data = (data >> 1) | (old_carry << 7);
        bus.write(param as u16, new_data);

        // update status cpu, hapus dulu status carry yang lama sebelum assign status carry yang baru
        cpu.status = cpu.status & 0b11111110;
        cpu.status = cpu.status | new_carry;

        cpu.update_zero_and_negative_flags(new_data);
        cpu.cycle += 5;
        5
    }
}
