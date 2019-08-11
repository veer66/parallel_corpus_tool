use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RTok {
    pub s: usize,
    pub e: usize,
    pub text: String,
}

impl RTok {
    #[allow(dead_code)]
    pub fn is_overlap(&self, another: &RTok) -> bool {
        // NOT OVERLAP
        // =SELF=     =ANOTHER=
        //      E0  <  S1
        //
        // =ANOTHER=  =SELF=
        //       E1  <  S0
        !(self.e < another.s || another.e < self.s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_at_edge() {
        assert!(RTok {
            s: 0,
            e: 10,
            text: "".to_string()
        }
        .is_overlap(&RTok {
            s: 10,
            e: 20,
            text: "".to_string()
        }));
    }
}
