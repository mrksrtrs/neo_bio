#[derive(Debug)]
pub struct Sequence {
    sequence: String,
}

impl PartialEq for Sequence {
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

impl Sequence {
    
    pub fn new(sequence: &str) -> Sequence {
        Sequence {
            sequence: String::from(sequence),
        }
    }

    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sequence.is_empty()
    }

    pub fn transcribe(&self) -> Sequence {
        let mut rna = String::from("");
        for nucleotide in self.sequence.chars() {
            if nucleotide == 'T' {
                rna.push('U');
            } else {
                rna.push(nucleotide);
            }
        }
        Sequence::new(&rna)
    }

    pub fn back_transcribe(&self) -> Sequence {
        let mut dna = String::from("");
        for nucleotide in self.sequence.chars() {
            if nucleotide == 'U' {
                dna.push('T');
            } else {
                dna.push(nucleotide);
            }
        }
        Sequence::new(&dna)
    }

    pub fn complement(&self) -> Sequence {
        let mut complement = String::from("");
        for nucleotide in self.sequence.chars() {
            match nucleotide {
                'A' => complement.push('T'),
                'T' => complement.push('A'),
                'C' => complement.push('G'),
                'G' => complement.push('C'),
                _ => complement.push(nucleotide),
            }
        }
        Sequence::new(&complement)
    }

    pub fn reverse(&self) -> Sequence {
        let mut reverse = String::from("");
        for nucleotide in self.sequence.chars().rev() {
            reverse.push(nucleotide);
        }
        Sequence::new(&reverse)
    }

    pub fn reverse_complement(&self) -> Sequence
    {
        let reverse = self.reverse();
        reverse.complement()
    }

    pub fn find(&self, pattern: &str) -> usize {
        let mut index = 0;
        let mut pattern_index = 0;
        let mut found = false;
        for nucleotide in self.sequence.chars() {
            if nucleotide == pattern.chars().nth(pattern_index).unwrap() {
                pattern_index += 1;
                if pattern_index == pattern.len() {
                    found = true;
                    break;
                }
            } else {
                pattern_index = 0;
            }
            index += 1;
        }
        if found {
            index - pattern.len() + 1
        } else {
            self.sequence.len()
        }
    }

    pub fn subsequence(&self, start_position: usize, len: usize) -> Sequence
    {
        let mut subsequence = String::from("");
        for (index, nucleotide) in self.sequence.chars().enumerate() {
            if index >= start_position && index < start_position + len {
                subsequence.push(nucleotide);
            }
        }
        Sequence::new(&subsequence)
    }

    pub fn gc_content(&self) -> f64 {
        let mut gc_count = 0;
        for nucleotide in self.sequence.chars() {
            if nucleotide == 'G' || nucleotide == 'C' {
                gc_count += 1;
            }
        }
        gc_count as f64 / self.sequence.len() as f64
    }

}

    