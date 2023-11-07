use std::{
    fs,
    io::{self, BufWriter, Write},
    mem::size_of,
    path::Path,
};

use rand::RngCore;

pub fn write_random_u16(path: impl AsRef<Path>, count: usize) -> io::Result<()> {
    const BUF_SIZE: usize = 0x80000;

    let mut bw = BufWriter::new(fs::File::create(path)?);

    let mut rng = rand::thread_rng();

    let mut buffer = Box::new([0; BUF_SIZE]);

    let mut total = count * size_of::<u16>();

    while total > 0 {
        let data = if total > BUF_SIZE {
            &mut *buffer
        } else {
            &mut buffer[0..total]
        };

        rng.fill_bytes(data);
        bw.write_all(data)?;
        bw.flush()?;

        total = total.saturating_sub(BUF_SIZE);
    }

    Ok(())
}
