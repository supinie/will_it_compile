use core::num::TryFromIntError;

//buf must be of even length
fn byte_ops(buffer: &[u8]) -> Result<Vec<i16>, TryFromIntError> {
    let output_vec: Vec<i16> = buffer
        .iter()
        .flat_map(|&byte| {
            (0..2).map(move |i| {
                if i == 0 {
                    (usize::from(byte & 15) * 3329 + 8) >> 4
                } else {
                    (usize::from(byte >> 4) * 3329 + 8) >> 4
                }
            })
        })
        .map(i16::try_from)
        .collect::<Result<Vec<i16>, TryFromIntError>>()?;

    Ok(output_vec)
}

// buf must be length multiple 8
pub fn another_byte_operation(buffer: &[u8]) -> Vec<i16> {
    let output_vec: Vec<i16> = buffer
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| ((i16::from(byte) >> i) & 1).wrapping_neg()))
        .map(|mask| mask & (3329 + 1)/2)
        .collect();
    output_vec
}


// buf must be even length of multiple 3
pub fn last_one_i_promise(buffer: &[u8]) -> Vec<i16> {
    let output_vec: Vec<i16> = buffer
    .chunks_exact(3)
    .flat_map(|chunk| chunk.windows(2).enumerate())
    .map(|(index, pair)| {
        if index % 2 == 0 {
            i16::from(pair[0]) | ((i16::from(pair[1]) << 8) & 0xfff)
        } else {
            i16::from(pair[0] >> 4) | ((i16::from(pair[1]) << 4) & 0xfff)
        }
    })
    .collect();

    output_vec
}
