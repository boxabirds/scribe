    fn required_ext(&self) -> Option<String> {
        if self.opts.case_insensitive {
            return None;
        }
        // We don't care at all about the beginning of this pattern. All we
        // need to check for is if it ends with a literal of the form `.ext`.
        let mut ext: Vec<char> = vec![]; // built in reverse
        for t in self.tokens.iter().rev() {
            match *t {
                Token::Literal('/') => return None,
                Token::Literal(c) => {
                    ext.push(c);
                    if c == '.' {
                        break;
                    }
                }
                _ => return None,
            }
        }
        if ext.last() != Some(&'.') {
            None
        } else {
            ext.reverse();
            Some(ext.into_iter().collect())
        }
    }
