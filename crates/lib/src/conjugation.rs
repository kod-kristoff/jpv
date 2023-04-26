use std::collections::BTreeMap;
use std::fmt;

use fixed_map::{Key, Set};
use musli::{Decode, Encode};

use crate::{
    kana::{Pair, Word},
    Concat,
};

/// Helper to construct a particular [`Conjugation`].
///
/// # Examples
///
/// ```rust
/// lib::conjugation!(Present + Past);
/// lib::conjugation!(Present + Past + *Polite);
/// lib::conjugation!(Present + Past + *Alternate);
/// ```
#[macro_export]
macro_rules! conjugation {
    ($kind:ident $(+ $kind2:ident)* $(+ *$flag:ident)*) => {{
        let mut form = $crate::macro_support::fixed_map::Set::new();
        form.insert($crate::Form::$kind);
        $(form.insert($crate::Form::$kind2);)*
        #[allow(unused_mut)]
        let mut flag = $crate::macro_support::fixed_map::Set::new();
        $(flag.insert($crate::Flag::$flag);)*
        $crate::Conjugation::new(form, flag)
    }}
}

/// Helper macro to build a kana pair.
macro_rules! pair {
    ($k:expr, $r:expr, $last:expr) => {
        $crate::kana::Pair::new([$k], [$r], $last)
    };

    ($k:expr, $r:expr, $a:expr, $last:expr) => {
        $crate::kana::Pair::new([$k, $a], [$r, $a], $last)
    };

    ($k:expr, $r:expr, $a:expr, $b:expr, $last:expr) => {
        $crate::kana::Pair::new([$k, $a], [$r, $b], $last)
    };
}

/// Setup a collection of conjugations.
macro_rules! conjugations {
    ($k:expr, $r:expr, $(
        $kind:ident $(+ $kind2:ident)* $(+ *$flag:ident)* ( $($tt:tt)* )
    ),* $(,)?) => {{
        let mut tree = ::std::collections::BTreeMap::new();
        $(tree.insert($crate::conjugation!($kind $(+ $kind2)* $(+ *$flag)*), pair!($k, $r, $($tt)*));)*
        tree
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode, Key)]
pub enum Form {
    Te,
    Present,
    Negative,
    Past,
    Command,
    Hypothetical,
    Conditional,
    Passive,
    Potential,
    /// Volitional / Presumptive
    Volitional,
    Causative,
    Tai,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode, Key)]
pub enum Flag {
    Polite,
    Alternate,
    Conversation,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode)]
pub struct Conjugation {
    #[musli(with = crate::musli::set::<_>)]
    pub form: Set<Form>,
    #[musli(with = crate::musli::set::<_>)]
    pub flag: Set<Flag>,
}

impl Conjugation {
    // Macro support.
    #[doc(hidden)]
    pub fn new(form: Set<Form>, flag: Set<Flag>) -> Self {
        Self { form, flag }
    }
}

impl fmt::Debug for Conjugation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.flag.is_empty() {
            self.form.fmt(f)
        } else {
            write!(f, "{:?} / {:?}", self.form, self.flag)
        }
    }
}

/// A collection of conjugations.
#[non_exhaustive]
pub struct Conjugations<'a> {
    pub dictionary: Word<'a>,
    pub conjugations: BTreeMap<Conjugation, Pair<'a, 2>>,
}

impl<'a> Conjugations<'a> {
    /// Test if any polite conjugations exist.
    pub fn has_polite(&self) -> bool {
        for c in self.conjugations.keys() {
            if c.flag.contains(Flag::Polite) {
                return true;
            }
        }

        false
    }

    /// Get a conjugation.
    pub fn get(&self, conjugation: Conjugation) -> Option<&Pair<'a, 2>> {
        self.conjugations.get(&conjugation)
    }

    /// Iterate over all conjugations.
    pub fn iter(&self) -> impl Iterator<Item = (Conjugation, Concat<'a, 3>)> + '_ {
        self.conjugations
            .iter()
            .flat_map(|(k, p)| p.clone().into_iter().map(|p| (*k, p)))
    }
}