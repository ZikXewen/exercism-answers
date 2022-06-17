use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .copied()
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        NUCLEOTIDES.iter().map(|&x| (x, 0)).collect(),
        |mut cr: HashMap<char, usize>, c| {
            cr.get_mut(&c).map(|x| *x += 1).ok_or(c)?;
            Ok(cr)
        },
    )
}
