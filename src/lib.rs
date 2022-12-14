pub mod neo_bio;

#[cfg(test)]
mod tests {

    use crate::neo_bio::Sequence;

    #[test]
    fn transcribe_dna_into_rna() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let transcribed_rna = dna_seq.transcribe();
        let reference_rna =
            Sequence::new("AUUAAAGGUUUAUACCUUCCCAGGUAACAAACCAACCAACUUUCGAUCUCUUGUAGAUCUGUUCUCUAAA");
        assert_eq!(transcribed_rna, reference_rna);
    }

    #[test]
    fn back_transcribe_rna_into_dna() {
        let rna_seq =
            Sequence::new("AUUAAAGGUUUAUACCUUCCCAGGUAACAAACCAACCAACUUUCGAUCUCUUGUAGAUCUGUUCUCUAAA");
        let back_transcribed_rna = rna_seq.back_transcribe();
        let reference_dna =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        assert_eq!(back_transcribed_rna, reference_dna);
    }

    #[test]
    fn get_len_of_sequence() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let len = dna_seq.len();
        let reference_len = 70;
        assert_eq!(len, reference_len);
    }

    #[test]
    fn check_empty_sequence() {
        let dna_seq = Sequence::new("");
        let is_empty = dna_seq.is_empty();
        assert_eq!(is_empty, true);
    }

    #[test]
    fn check_non_empty_sequence() {
        let dna_seq = Sequence::new("ATTG");
        let is_empty = dna_seq.is_empty();
        assert_eq!(is_empty, false);
    }

    #[test]
    fn reverse_complement_sequence() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let reverse_complement = dna_seq.reverse_complement();
        let reference_reverse_complement =
            Sequence::new("TTTAGAGAACAGATCTACAAGAGATCGAAAGTTGGTTGGTTTGTTACCTGGGAAGGTATAAACCTTTAAT");
        assert_eq!(reverse_complement, reference_reverse_complement);
    }

    #[test]
    fn complement_sequence() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let complement = dna_seq.complement();
        let reference_complement =
            Sequence::new("TAATTTCCAAATATGGAAGGGTCCATTGTTTGGTTGGTTGAAAGCTAGAGAACATCTAGACAAGAGATTT");
        assert_eq!(complement, reference_complement);
    }

    #[test]
    fn find_pattern_in_sequence() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let pattern = "TCT";
        let pattern_positions = dna_seq.find(pattern);
        let reference_pattern_positions: usize = 46;
        assert_eq!(pattern_positions, reference_pattern_positions);
    }

    #[test]
    fn get_subsequence() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let subsequence = dna_seq.subsequence(0, 10);
        let reference_subsequence = Sequence::new("ATTAAAGGTT");
        assert_eq!(subsequence, reference_subsequence);
    }

    #[test]
    fn find_pattern_and_get_subsequence() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let pattern = "TCT";
        let pattern_positions = dna_seq.find(pattern);
        let subsequence = dna_seq.subsequence(pattern_positions, pattern.len());
        let reference_subsequence = Sequence::new("TCT");
        assert_eq!(subsequence, reference_subsequence);
    }

    #[test]
    fn gc_content() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let gc_content = dna_seq.gc_content();
        let reference_gc_content = 0.35714285714285715;
        assert_eq!(gc_content, reference_gc_content);
    }

    #[test]
    fn get_number_of_non_overlapping_subsequence_pattern() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let pattern = "TCT";
        let number_of_subsequence_pattern = dna_seq.non_overlapping_count(pattern, None, None);
        let reference_number_of_subsequence_pattern = 3;
        assert_eq!(
            number_of_subsequence_pattern,
            reference_number_of_subsequence_pattern
        );
    }

    #[test]
    fn get_number_of_non_overlapping_subsequence_pattern_with_stat_and_end() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let pattern = "TCT";
        let number_of_subsequence_pattern = dna_seq.non_overlapping_count(pattern, Some(46), Some(60));
        let reference_number_of_subsequence_pattern = 2;
        assert_eq!(
            number_of_subsequence_pattern,
            reference_number_of_subsequence_pattern
        );
    }

    #[test]
    fn get_number_of_overlapping_subsequence_pattern() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let pattern = "TCT";
        let number_of_subsequence_pattern = dna_seq.overlapping_count(pattern, None, None);
        let reference_number_of_subsequence_pattern = 5;
        assert_eq!(
            number_of_subsequence_pattern,
            reference_number_of_subsequence_pattern
        );
    }

    #[test]
    fn get_number_of_overlapping_subsequence_pattern_with_start_and_end() {
        let dna_seq =
            Sequence::new("ATTAAAGGTTTATACCTTCCCAGGTAACAAACCAACCAACTTTCGATCTCTTGTAGATCTGTTCTCTAAA");
        let pattern = "TCT";
        let number_of_subsequence_pattern = dna_seq.overlapping_count(pattern, Some(46), Some(60));
        let reference_number_of_subsequence_pattern = 3;
        assert_eq!(
            number_of_subsequence_pattern,
            reference_number_of_subsequence_pattern
        );
    }


}
