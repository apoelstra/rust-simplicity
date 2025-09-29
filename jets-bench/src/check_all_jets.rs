use std::collections::BTreeMap;

use simplicity::jet::Jet;
use simplicity::BitIter;

/// A structure which creates a map of all jets to verify that every one
/// was covered by a benchmark.
pub struct JetChecker<J: Jet> {
    map: BTreeMap<J, usize>,
}

impl<J: Jet> JetChecker<J> {
    pub fn initialize() -> Self {
        // This is insanely inefficient but will find all jets with encodings <= 24 bits
        // and empirically it takes less than a second to run against the multi-hour
        // benchmark run, so whatever.
        //
        // In our generated Rust code we should add an integer<>jet map, a count of all
        // jets, an iterator over all jets, etc., but for now we will make do.
        let mut map = BTreeMap::new();
        for u0 in 0..=255 {
            for u1 in 0..=255 {
                for u2 in 0..=255 {
                    let arr = [u0, u1, u2];
                    let mut iter = BitIter::new(arr.iter().copied());
                    if let Ok(jet) = J::decode(&mut iter) {
                        map.insert(jet, 0);
                    }
                }
            }
        }

        JetChecker { map }
    }

    pub fn n_jets(&self) -> usize {
        self.map.len()
    }

    pub fn record(&mut self, j: J) {
        *self.map.get_mut(&j).expect("all jets to be preinitialized") += 1;
    }

    pub fn check_all_covered(&self) {
        let mut missed_any = false;
        for (entry, count) in &self.map {
            if *count == 0 {
                println!("Didn't cover jet: {entry}");
                missed_any = true;
            }
            if *count > 1 {
                println!("Double-covered jet: {entry}");
                missed_any = true;
            }
        }

        if missed_any {
            panic!("Failed to cover jets.");
        }
    }
}
