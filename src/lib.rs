#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::fmt::{Debug, Formatter};

use rand::Rng;

/// A default instance of `GoofyAnimals` initialized with the built-in English word lists.
///
/// This constant provides convenient access to a pre-configured `GoofyAnimals` instance
/// that uses the included animal and adjective lists.
pub const DEFAULT_GOOFY_ANIMALS: GoofyAnimals<'static> = GoofyAnimals::new(
    &const_str::split!(include_str!("data/en_animals.txt"), "\n"),
    &const_str::split!(include_str!("data/en_adjectives.txt"), "\n"),
);

/// A struct that manages lists of adjectives and animals for generating goofy names.
///
/// `GoofyAnimals` allows you to generate random names in the format
/// `adjective-adjective-animal` using custom word lists or the default ones.
pub struct GoofyAnimals<'a> {
    animals: &'a [&'a str],
    adjectives: &'a [&'a str],
}

impl<'a> GoofyAnimals<'a> {
    /// Creates a new `GoofyAnimals` instance with the given animal and adjective lists.
    ///
    /// This constructor performs several checks at compile time to ensure the
    /// provided lists are valid:
    /// - Verifies that the animals list is not empty
    /// - Ensures there are at least two adjectives
    /// - Checks that there are no trailing newlines in either list
    ///
    /// # Arguments
    ///
    /// * `animals` - A slice of string slices containing animal names
    /// * `adjectives` - A slice of string slices containing adjectives
    ///
    /// # Returns
    ///
    /// A new `GoofyAnimals` instance.
    ///
    /// # Panics
    ///
    /// This function will panic at compile time if:
    /// - The animals list is empty
    /// - The adjectives list has fewer than 2 entries
    /// - Either list has trailing newlines
    pub const fn new(animals: &'a [&'a str], adjectives: &'a [&'a str]) -> Self {
        let total_animals = animals.len();
        let total_adjectives = adjectives.len();

        if total_animals < 1 {
            panic!("empty animals");
        }

        if total_adjectives < 2 {
            panic!("must have at least two adjectives");
        }

        if const_str::equal!(*animals.last().unwrap(), "") {
            panic!("trailing newline in animals");
        }

        if const_str::equal!(*adjectives.last().unwrap(), "") {
            panic!("trailing newline in adjectives");
        }

        Self::new_unchecked(animals, adjectives)
    }

    /// Creates a new `GoofyAnimals` instance without performing any validity checks.
    ///
    /// This constructor is useful when you're certain that your word lists are valid
    /// or when you want to defer validation to runtime.
    ///
    /// # Arguments
    ///
    /// * `animals` - A slice of string slices containing animal names
    /// * `adjectives` - A slice of string slices containing adjectives
    ///
    /// # Returns
    ///
    /// A new `GoofyAnimals` instance.
    ///
    /// # Safety
    ///
    /// This function does not check if:
    /// - The animals list is empty
    /// - The adjectives list has at least 2 entries
    /// - Either list has trailing newlines
    ///
    /// Using invalid inputs may result in panics or unexpected behavior when
    /// generating names.
    pub const fn new_unchecked(animals: &'a [&'a str], adjectives: &'a [&'a str]) -> Self {
        Self {
            animals,
            adjectives,
        }
    }

    /// Returns a reference to the list of animal names.
    ///
    /// This can be useful for inspecting or using the animal names directly.
    ///
    /// # Returns
    ///
    /// A slice of string slices containing the animal names.
    pub fn get_animals(&self) -> &'a [&'a str] {
        self.animals
    }

    /// Returns a reference to the list of adjectives.
    ///
    /// This can be useful for inspecting or using the adjectives directly.
    ///
    /// # Returns
    ///
    /// A slice of string slices containing the adjectives.
    pub fn get_adjectives(&self) -> &'a [&'a str] {
        self.adjectives
    }

    /// Generates the individual parts of a goofy name: two adjectives and an animal.
    ///
    /// This function selects two different adjectives and one animal randomly using the
    /// provided random number generator. It ensures the two adjectives are not the same.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to any random number generator that implements the `Rng` trait.
    ///
    /// # Returns
    ///
    /// A tuple containing three string slices: `(adjective1, adjective2, animal)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rand::SeedableRng;
    /// use rand_chacha::ChaCha20Rng;
    /// use goofy_animals::DEFAULT_GOOFY_ANIMALS;
    ///
    /// // Use a seeded RNG for deterministic output
    /// let mut rng = ChaCha20Rng::seed_from_u64(0x1337);
    /// let (adj1, adj2, animal) = DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng);
    /// assert_eq!(adj1, "dismal");
    /// assert_eq!(adj2, "outlying");
    /// assert_eq!(animal, "moth");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(rng), level = tracing::Level::TRACE))]
    pub fn generate_name_parts(&self, rng: &mut impl Rng) -> (&'a str, &'a str, &'a str) {
        let (adjective_one, adjective_two) = loop {
            let one = rng.random_range(0..self.adjectives.len());
            let two = rng.random_range(0..self.adjectives.len());

            if one == two {
                continue;
            }

            break (one, two);
        };

        let animal = rng.random_range(0..self.animals.len());

        #[cfg(feature = "tracing")]
        tracing::trace!(adjective_one, adjective_two, animal, "generated name");

        (
            self.adjectives[adjective_one],
            self.adjectives[adjective_two],
            self.animals[animal],
        )
    }

    /// Generates a complete goofy name as a string in the format `adjective-adjective-animal`.
    ///
    /// This function combines two randomly selected adjectives with a randomly selected animal name,
    /// joining them with hyphens to form a single string.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to any random number generator that implements the `Rng` trait.
    ///
    /// # Returns
    ///
    /// A `String` containing the generated name in the format `adjective-adjective-animal`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rand::SeedableRng;
    /// use rand_chacha::ChaCha20Rng;
    /// use goofy_animals::DEFAULT_GOOFY_ANIMALS;
    ///
    /// // Use a seeded RNG for deterministic output
    /// let mut rng = ChaCha20Rng::seed_from_u64(0x1337);
    /// let name = DEFAULT_GOOFY_ANIMALS.generate_name(&mut rng);
    /// assert_eq!(name, "dismal-outlying-moth");
    /// ```
    ///
    /// # Feature Flag
    ///
    /// This function is only available when the `alloc` feature is enabled.
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn generate_name(&self, rng: &mut impl Rng) -> ::alloc::string::String {
        let (adjective_one, adjective_two, animal) = self.generate_name_parts(rng);

        ::alloc::format!("{adjective_one}-{adjective_two}-{animal}")
    }
}

impl Debug for GoofyAnimals<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GoofyAnimals")
            .field("total_adjectives", &self.adjectives.len())
            .field("total_animals", &self.animals.len())
            .finish()
    }
}

/// Generates the individual parts of a goofy name using the default word lists.
///
/// This is a convenience function that calls `generate_name_parts` on the
/// `DEFAULT_GOOFY_ANIMALS` instance.
///
/// # Arguments
///
/// * `rng` - A mutable reference to any random number generator that implements the `Rng` trait.
///
/// # Returns
///
/// A tuple containing three string slices: `(adjective1, adjective2, animal)`.
///
/// # Examples
///
/// ```rust
/// use rand::SeedableRng;
/// use rand_chacha::ChaCha20Rng;
/// use goofy_animals::generate_name_parts;
///
/// // Use a seeded RNG for deterministic output
/// let mut rng = ChaCha20Rng::seed_from_u64(0x1337);
/// let (adj1, adj2, animal) = generate_name_parts(&mut rng);
/// assert_eq!(adj1, "dismal");
/// assert_eq!(adj2, "outlying");
/// assert_eq!(animal, "moth");
/// ```
///
/// See [`GoofyAnimals::generate_name_parts`] for more details.
#[inline]
pub fn generate_name_parts(rng: &mut impl Rng) -> (&'static str, &'static str, &'static str) {
    DEFAULT_GOOFY_ANIMALS.generate_name_parts(rng)
}

/// Generates a complete goofy name as a string using the default word lists.
///
/// This is a convenience function that calls `generate_name` on the
/// `DEFAULT_GOOFY_ANIMALS` instance.
///
/// # Arguments
///
/// * `rng` - A mutable reference to any random number generator that implements the `Rng` trait.
///
/// # Returns
///
/// A `String` containing the generated name in the format `adjective-adjective-animal`.
///
/// # Examples
///
/// ```rust
/// use rand::SeedableRng;
/// use rand_chacha::ChaCha20Rng;
/// use goofy_animals::generate_name;
///
/// // Use a seeded RNG for deterministic output
/// let mut rng = ChaCha20Rng::seed_from_u64(0x1337);
/// let name = generate_name(&mut rng);
/// assert_eq!(name, "dismal-outlying-moth");
/// ```
///
/// # Feature Flag
///
/// This function is only available when the `alloc` feature is enabled.
///
/// See [`GoofyAnimals::generate_name`] for more details.
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn generate_name(rng: &mut impl Rng) -> ::alloc::string::String {
    DEFAULT_GOOFY_ANIMALS.generate_name(rng)
}

#[cfg(test)]
mod test {
    use super::DEFAULT_GOOFY_ANIMALS;

    use pretty_assertions::assert_eq;

    #[test]
    fn animals() {
        assert_eq!(DEFAULT_GOOFY_ANIMALS.get_animals().len(), 355);
    }

    #[test]
    fn adjectives() {
        assert_eq!(DEFAULT_GOOFY_ANIMALS.get_adjectives().len(), 1300);
    }

    #[test]
    #[cfg_attr(feature = "tracing", tracing_test::traced_test)]
    fn name_generation() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha20Rng;

        let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("dismal", "outlying", "moth"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("healthy", "yellowish", "firefly"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("flat", "faint", "squirrel"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("glorious", "educated", "louse"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("big", "glittering", "perch"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("relieved", "shadowy", "booby"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("simplistic", "thankful", "panther"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("black", "serene", "marten"),
        );

        #[cfg(all(feature = "tracing", feature = "alloc"))]
        {
            logs_assert(|lines: &[&str]| {
                const EXPECTED: usize = 8;

                match lines
                    .iter()
                    .filter(|line| line.contains("generated name"))
                    .count()
                {
                    EXPECTED => Ok(()),
                    n => Err(::alloc::format!("Expected {EXPECTED} logs, but got {n}")),
                }
            });
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn name_generation_alloc() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha20Rng;

        let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name(&mut rng),
            "dismal-outlying-moth",
        );
    }
}
