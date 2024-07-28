use std::str::Chars;

pub trait SplitNonAlphabetic {
    fn split_non_alphabetic(&self) -> IterSplitNonAlphabetic;
}

pub struct IterSplitNonAlphabetic<'a> {
    l: usize,
    r: usize,
    string: &'a str,
    chars: Chars<'a>,
    next_next: Option<&'a str>,
}

impl<'a> Iterator for IterSplitNonAlphabetic<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_next.is_some() {
            return self.next_next.take();
        }

        let mut l = self.l;
        let mut r = self.r;
        let mut nexts = (None, None);

        for ch in self.chars.by_ref() {
            let size = ch.len_utf8();

            if !ch.is_alphabetic() {
                if l < r {
                    nexts.0 = Some(&self.string[l..r]);
                    nexts.1 = Some(&self.string[r..r + size]);
                } else {
                    nexts.0 = Some(&self.string[r..r + size]);
                }

                l = r + size;
            }

            r += size;
            if nexts.0.is_some() {
                self.l = l;
                self.r = r;
                self.next_next = nexts.1;
                return nexts.0;
            }
        }

        (l < r).then(|| {
            self.l = r;
            self.r = r;
            &self.string[l..r]
        })
    }
}

impl SplitNonAlphabetic for str {
    fn split_non_alphabetic(&self) -> IterSplitNonAlphabetic {
        IterSplitNonAlphabetic {
            l: 0,
            r: 0,
            string: self,
            chars: self.chars(),
            next_next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let string = "hell o87. .bye,|/\nahh";
        let v: Vec<_> = string.split_non_alphabetic().collect();
        let expected = [
            "hell", " ", "o", "8", "7", ".", " ", ".", "bye", ",", "|", "/", "\n", "ahh",
        ];

        assert_eq!(expected, *v);
    }
}
