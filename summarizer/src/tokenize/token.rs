use crate::error::Serror;
use tiktoken_rs::CoreBPE;
pub struct Tokens<'a> {
    tokens: Vec<usize>,
    tokenizer: &'a CoreBPE,
    max_tokens: usize,
}

impl<'a> Tokens<'a> {
    pub fn detokenize(&self) -> Result<String, Serror> {
        Ok(self.tokenizer.decode(self.tokens.clone())?)
    }
}

pub struct MultiTokens<'a> {
    tokens: Vec<Vec<usize>>,
    tokenizer: &'a CoreBPE,
    max_tokens: usize,
}

impl<'a> MultiTokens<'a> {
    pub fn detokenize(self) -> Result<String, Serror> {
        Ok(self.detokenize_inarray()?.into_iter().collect())
    }

    pub fn detokenize_inarray(self) -> Result<Vec<String>, Serror> {
        let mapper = self.tokens.into_iter().map(|x| self.tokenizer.decode(x));
        let mut contents: Vec<String> = vec![];
        for each_string in mapper {
            let content = each_string?;
            contents.push(content)
        }
        Ok(contents)
    }
}

pub trait Tokenizer {
    const MAX_N_TOKENS: usize;
    fn bpe(&self) -> &CoreBPE;
    fn tokenize(&self, text: &str) -> Tokens {
        let tokens = self.bpe().encode_with_special_tokens(text);
        Tokens {
            tokens,
            tokenizer: self.bpe(),
            max_tokens: Self::MAX_N_TOKENS,
        }
    }
    fn tokenize_in_max_tokenlimit(&self, text: &str) -> Result<MultiTokens, Serror> {
        let encoded_text = self.bpe().encode_with_special_tokens(text);
        let mut n_tokens = self.bpe().encode_with_special_tokens(text).len();
        let mut tokens: Vec<Vec<usize>> = vec![];
        let mut start = 0;
        loop {
            if n_tokens == 0 {
                break;
            }
            let max_lim = std::cmp::min(n_tokens, Self::MAX_N_TOKENS);
            tokens.push(encoded_text[start..start + max_lim].to_vec());
            n_tokens -= max_lim;
            start += max_lim;
        }
        Ok(MultiTokens {
            tokenizer: self.bpe(),
            tokens,
            max_tokens: Self::MAX_N_TOKENS,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use tiktoken_rs::{cl100k_base, CoreBPE};
    struct DummToken(CoreBPE);
    struct BigToken(CoreBPE);

    impl Tokenizer for DummToken {
        const MAX_N_TOKENS: usize = 3;
        fn bpe(&self) -> &CoreBPE {
            &self.0
        }
    }
    impl Tokenizer for BigToken {
        const MAX_N_TOKENS: usize = 2000;
        fn bpe(&self) -> &CoreBPE {
            &self.0
        }
    }

    #[test]
    fn test_a_tokenize() {
        let dt = DummToken(cl100k_base().unwrap());
        assert_eq!(dt.tokenize("a").tokens, [64]);
    }
    #[test]
    fn test_a_multitokenize() {
        let dt = DummToken(cl100k_base().unwrap());
        assert_eq!(dt.tokenize_in_max_tokenlimit("a").unwrap().tokens, [[64]]);
    }
    #[test]
    fn test_abcd_tokenize() {
        let dt = DummToken(cl100k_base().unwrap());
        assert_eq!(dt.tokenize("abcd").tokens, [69744]);
    }

    #[test]
    fn test_complex_tokenize() {
        let dt = DummToken(cl100k_base().unwrap());
        assert_eq!(
            dt.tokenize("aurwfhdhbasdfiawifujbads").tokens,
            [4202, 44183, 16373, 71, 18275, 3013, 72, 675, 333, 9832, 65, 7819]
        );
    }
    #[test]
    fn test_max_tokenlimit_tokenize() {
        let dt = DummToken(cl100k_base().unwrap());
        let some_vec = [
            [4202, 44183, 16373],
            [71, 18275, 3013],
            [72, 675, 333],
            [9832, 65, 7819],
        ];
        assert_eq!(
            dt.tokenize_in_max_tokenlimit("aurwfhdhbasdfiawifujbads")
                .unwrap()
                .tokens,
            some_vec
        );
    }
    #[test]
    fn test_max_tokenlimit_diff_tokenize() {
        let dt = DummToken(cl100k_base().unwrap());
        let text = "asdfiawifujbads";
        assert_eq!(
            dt.tokenize_in_max_tokenlimit(text).unwrap().tokens,
            [
                [77715, 72, 675].to_vec(),
                [333, 9832, 65].to_vec(),
                [7819].to_vec()
            ]
        );
        assert_eq!(dt.tokenize(text).tokens.len(), 7);
    }
    #[test]
    fn test_detokenize() {
        let dt = cl100k_base().unwrap();
        let tokens = Tokens {
            tokenizer: &dt,
            tokens: [64].to_vec(),
            max_tokens: 10,
        };
        assert_eq!(tokens.detokenize().unwrap(), "a");
    }
    #[test]
    fn test_detokenize_sequence() {
        let dt = cl100k_base().unwrap();
        let tokens = Tokens {
            tokenizer: &dt,
            tokens: [69744].to_vec(),
            max_tokens: 10,
        };
        assert_eq!(tokens.detokenize().unwrap(), "abcd");
    }
    #[test]
    fn test_detokenize_complex() {
        let dt = cl100k_base().unwrap();
        let tokens = Tokens {
            tokenizer: &dt,
            tokens: [
                4202, 44183, 16373, 71, 18275, 3013, 72, 675, 333, 9832, 65, 7819,
            ]
            .to_vec(),
            max_tokens: 10,
        };
        assert_eq!(tokens.detokenize().unwrap(), "aurwfhdhbasdfiawifujbads");
    }
    #[test]
    fn test_detokenize_complex_multi() {
        let dt = cl100k_base().unwrap();
        let tokens = MultiTokens {
            tokenizer: &dt,
            tokens: [
                [4202, 44183, 16373].to_vec(),
                [71, 18275, 3013].to_vec(),
                [72, 675, 333].to_vec(),
                [9832, 65, 7819].to_vec(),
            ]
            .to_vec(),
            max_tokens: 10,
        };
        assert_eq!(tokens.detokenize().unwrap(), "aurwfhdhbasdfiawifujbads");
    }
    #[test]
    fn test_detokenize_complex_multi_different_lengths() {
        let dt = cl100k_base().unwrap();
        let tokens = MultiTokens {
            tokenizer: &dt,
            tokens: [
                [77715, 72, 675].to_vec(),
                [333, 9832, 65].to_vec(),
                [7819].to_vec(),
            ]
            .to_vec(),
            max_tokens: 10,
        };
        assert_eq!(tokens.detokenize().unwrap(), "asdfiawifujbads");
    }
    #[test]
    fn test_big_tokenize() {
        let content = include_str!("../../tests/data/transcript-1.txt");
        let bt = BigToken(cl100k_base().unwrap());
        assert_eq!(
            bt.tokenize_in_max_tokenlimit(content)
                .unwrap()
                .detokenize_inarray()
                .unwrap()
                .len(),
            7
        );
    }
    #[test]
    #[ignore = "Run manually to see the parts of the text."]
    fn test_big_parts() {
        let content = include_str!("../../tests/data/transcript-1.txt");
        let bt = BigToken(cl100k_base().unwrap());
        for part in bt
            .tokenize_in_max_tokenlimit(content)
            .unwrap()
            .detokenize_inarray()
            .unwrap()
        {
            println!("{part}\n")
        }
    }
}
