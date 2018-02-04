use std::{fmt, iter, slice, str};
use self::replacements::replacements_for;

const MOLECULE_TARGET: &[u8] = b"CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";

pub fn part2() {
    /*
    let m = Molecule(b"CCaRnFYFYFAr".to_vec());
    for a in m.atoms() {
        match a {
            Atom::OneLetter(letter) => println!("{}", letter as char),
            Atom::TwoLetters((a, b)) => println!("{}{}", a as char, b as char)
        };
    }
    let test = m.replace(10, 2, b"ZZZ");
    println!("{}", test);
    */

    let m = Molecule(b"e".to_vec());
    do_replacements(m, 0);
}

fn do_replacements(molecule: Molecule, step_count: u32) {
    for (index, atom) in molecule.atoms().enumerate() {
        if let (atom_letter_count, Some(replacements)) = replacements_for(atom) {
            for replacement in replacements {
                if molecule.len() + replacement.len() - atom_letter_count <= MOLECULE_TARGET.len() {
                    let replaced = molecule.replace(index, atom_letter_count, replacement);
                    if replaced.equals(MOLECULE_TARGET) {
                        println!("Hit Target in {} steps!", step_count + 1);
                    } else {
                        do_replacements(replaced, step_count + 1);
                    }
                }
            }
        }
    }
}

pub enum Atom {
    OneLetter(u8),
    TwoLetters((u8, u8))
}

struct Molecule(Vec<u8>);

impl Molecule {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn atoms(&self) -> MoleculeIter {
        MoleculeIter {
            it: self.0.iter().peekable()
        }
    }

    fn replace(&self, index: usize, count: usize, replacement: &[u8]) -> Molecule {
        let mut letters = Vec::<u8>::with_capacity(self.0.len() + replacement.len() - 1);
        letters.extend_from_slice(&self.0[0..index]);
        letters.extend_from_slice(replacement);
        letters.extend_from_slice(&self.0[(index + count)..]);
        Molecule(letters)
    }

    fn equals(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", str::from_utf8(&self.0[0..]).unwrap())?;
        Ok(()) 
	}
}

struct MoleculeIter<'a> {
    it: iter::Peekable<slice::Iter<'a, u8>>
}

impl <'a> Iterator for MoleculeIter<'a> {
    type Item = Atom;

    fn next(&mut self) -> Option<Atom> {
        if let Some(&letter) = self.it.next() {
            if let Some(&&next) = self.it.peek() {
                if is_lower_case(next) {
                    self.it.next();
                    Some(Atom::TwoLetters((letter, next)))
                } else {
                    Some(Atom::OneLetter(letter))
                }
            } else {
                Some(Atom::OneLetter(letter))
            }
        } else {
            None
        }
    }
}

fn is_lower_case(letter: u8) -> bool {
    return b'a' <= letter && letter <= b'z'
}

mod replacements {
    use super::Atom;
    use std::slice;

    const AL_REPLACE: [&[u8]; 2]  = [b"ThF", b"ThRnFAr"];
    const B_REPLACE:  [&[u8]; 3]  = [b"BCa", b"TiB", b"TiRnFAr"];
    const CA_REPLACE: [&[u8]; 6]  = [b"CaCa", b"PB", b"PRnFAr", b"SiRnFYFAr", b"SiRnMgAr", b"SiTh"];
    const F_REPLACE:  [&[u8]; 3]  = [b"CaF", b"PMg", b"SiAl"];
    const H_REPLACE:  [&[u8]; 10] = [b"CRnAlAr", b"CRnFYFYFAr", b"CRnFYMgAr", b"CRnMgYFAr", b"HCa", b"NRnFYFAr", b"NRnMgAr", b"NTh", b"OB", b"ORnFAr"];
    const MG_REPLACE: [&[u8]; 2]  = [b"BF", b"TiMg"];
    const N_REPLACE:  [&[u8]; 2]  = [b"CRnFAr", b"HSi"];
    const O_REPLACE:  [&[u8]; 5]  = [b"CRnFYFAr", b"CRnMgAr", b"HP", b"NRnFAr", b"OTi"];
    const P_REPLACE:  [&[u8]; 3]  = [b"CaP", b"PTi", b"SiRnFAr"];
    const SI_REPLACE: [&[u8]; 1]  = [b"CaSi"];
    const TH_REPLACE: [&[u8]; 1]  = [b"ThCa"];
    const TI_REPLACE: [&[u8]; 2]  = [b"BP", b"TiTi"];
    const E_REPLACE:  [&[u8]; 3]  = [b"HF", b"NAl", b"OMg"];

    pub fn replacements_for(atom: Atom) -> (usize, Option<slice::Iter<'static, &'static [u8]>>) {
        match atom {
            Atom::OneLetter(letter)   => (1, one_letter_replacements(letter)),
            Atom::TwoLetters(letters) => (2, two_letter_replacements(letters))
        }
    }

    fn one_letter_replacements(letter: u8) -> Option<slice::Iter<'static, &'static [u8]>> {
        match letter {
            b'B' => Some(B_REPLACE.iter()),
            b'F' => Some(F_REPLACE.iter()),
            b'H' => Some(H_REPLACE.iter()),
            b'N' => Some(N_REPLACE.iter()),
            b'O' => Some(O_REPLACE.iter()),
            b'P' => Some(P_REPLACE.iter()),
            b'e' => Some(E_REPLACE.iter()),
            _    => None
        }
    }

    fn two_letter_replacements(letters: (u8, u8)) -> Option<slice::Iter<'static, &'static [u8]>> {
        match letters {
            (b'A', b'l') => Some(AL_REPLACE.iter()),
            (b'C', b'a') => Some(CA_REPLACE.iter()),
            (b'M', b'g') => Some(MG_REPLACE.iter()),
            (b'S', b'i') => Some(SI_REPLACE.iter()),
            (b'T', b'h') => Some(TH_REPLACE.iter()),
            (b'T', b'i') => Some(TI_REPLACE.iter()),
            _            => None
        }
    }

    /*
    pub fn print_replacements() {
        //let dummy: i8 = AL_REPLACE.iter();

        use std::str;

        for r in AL_REPLACE.iter() {
            println!("Al => {}", str::from_utf8(r).unwrap());
        }
        for r in B_REPLACE.iter() {
            println!("B => {}", str::from_utf8(r).unwrap());
        }
        for r in CA_REPLACE.iter() {
            println!("Ca => {}", str::from_utf8(r).unwrap());
        }
        for r in F_REPLACE.iter() {
            println!("F => {}", str::from_utf8(r).unwrap());
        }
        for r in H_REPLACE.iter() {
            println!("H => {}", str::from_utf8(r).unwrap());
        }
        for r in MG_REPLACE.iter() {
            println!("Mg => {}", str::from_utf8(r).unwrap());
        }
        for r in N_REPLACE.iter() {
            println!("N => {}", str::from_utf8(r).unwrap());
        }
        for r in O_REPLACE.iter() {
            println!("O => {}", str::from_utf8(r).unwrap());
        }
        for r in P_REPLACE.iter() {
            println!("P => {}", str::from_utf8(r).unwrap());
        }
        for r in SI_REPLACE.iter() {
            println!("Si => {}", str::from_utf8(r).unwrap());
        }
        for r in TH_REPLACE.iter() {
            println!("Th => {}", str::from_utf8(r).unwrap());
        }
        for r in TI_REPLACE.iter() {
            println!("Ti => {}", str::from_utf8(r).unwrap());
        }
        for r in E_REPLACE.iter() {
            println!("e => {}", str::from_utf8(r).unwrap());
        }
    }
    */

    /*
    const E_REPLACE:  [&[u8]; 2]  = [b"H", b"O"];
    const H_REPLACE:  [&[u8]; 2]  = [b"HO", b"OH"];
    const O_REPLACE:  [&[u8]; 1]  = [b"HH"];

    fn one_letter_replacements(letter: u8) -> Option<slice::Iter<'static, &'static [u8]>> {
        match letter {
            b'e' => Some(E_REPLACE.iter()),
            b'H' => Some(H_REPLACE.iter()),
            b'O' => Some(O_REPLACE.iter()),
            _    => None
        }
    }

    fn two_letter_replacements(_: (u8, u8)) -> Option<slice::Iter<'static, &'static [u8]>> {
        None
    }
    */
}
