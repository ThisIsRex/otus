trait Chess {
    ///король
    fn king(&self) -> u64;

    ///конь
    fn horse(&self) -> u64;

    ///ладья
    fn rook(&self) -> u64;

    ///слон
    fn bishop(&self) -> u64;

    ///ферзь
    fn queen(&self) -> u64;
}

impl Chess for u64 {
    fn king(&self) -> u64 {
        self.wrapping_shl(7)
            | self.wrapping_shl(8)
            | self.wrapping_shl(9)
            | self.wrapping_shr(1)
            | self.wrapping_shl(1)
            | self.wrapping_shr(9)
            | self.wrapping_shr(8)
            | self.wrapping_shr(7)
    }

    fn horse(&self) -> u64 {
        todo!()
    }

    fn rook(&self) -> u64 {
        const VERT: u64 = 0x101010101010101;
        const HORZ: u64 = 0xff;

        let shift_h = self % 8;
        let shift_v = shift_h * 8;

        dbg!(shift_h);
        dbg!(shift_v);

        VERT.wrapping_shr(shift_v as u32) ^ HORZ.wrapping_shr(shift_h as u32)
    }

    fn bishop(&self) -> u64 {
        todo!()
    }

    fn queen(&self) -> u64 {
        todo!()
    }
    //
}

fn main() {
    println!("{}", 34359738368.king());
    println!("{}", 131072.rook());
}

#[cfg(test)]
mod test {}
