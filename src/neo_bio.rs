
pub struct Sequence {
    sequence: String,
}

impl Sequence {
    pub fn new(sequence: &str) -> Sequence {
        Sequence {
            sequence: String::from(sequence),
        }
    }

    pub fn len(&self) -> usize {
        return self.sequence.len();
    }

    pub fn transcribe(&self) -> String {
        let mut rna = String::from("");
        for nucleotide in self.sequence.chars() {
            if nucleotide == 'T' {
                rna.push('U');
            } else {
                rna.push(nucleotide);
            }
        }
        return rna;
    }

    pub fn back_transcribe(&self) -> String {
        let mut dna = String::from("");
        for nucleotide in self.sequence.chars() {
            if nucleotide == 'U' {
                dna.push('T');
            } else {
                dna.push(nucleotide);
            }
        }
        return dna;
    }

    pub fn complement(&self) -> String {
        let mut complement = String::from("");
        for nucleotide in self.sequence.chars() {
            if nucleotide == 'A' {
                complement.push('T');
            } else if nucleotide == 'T' {
                complement.push('A');
            } else if nucleotide == 'C' {
                complement.push('G');
            } else if nucleotide == 'G' {
                complement.push('C');
            }
        }
        return complement;
    }

    pub fn reverse(&self) -> String {
        let mut reversed = String::from("");
        for nucleotide in self.sequence.chars().rev() {
            reversed.push(nucleotide);
        }
        return reversed;
    }

    pub fn reverse_complement(&self) -> String
    {
        let reverse = self.reverse();
        let reverse_complement = Sequence::new(&reverse).complement();
        return reverse_complement;
    }

}

    