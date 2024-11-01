macro_rules! generate_sort_by {
    ($varname:ident => $($variant:ident => $path:expr),* $(,)?) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
        /// Sort and filter by
        pub enum SortAndFilterBy {
            $($variant),*
        }

        impl FromStr for SortAndFilterBy {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // Convert kebab-case to PascalCase
                let pascal = s.split('-')
                    .map(|part| {
                        let mut c = part.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().chain(c).collect()
                        }
                    })
                    .collect::<String>();

                match pascal.as_str() {
                    $(stringify!($variant) => Ok(SortAndFilterBy::$variant),)*
                    _ => Err(format!("Unknown sort criterion: {}", s))
                }
            }
        }

        impl Display for SortAndFilterBy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(SortAndFilterBy::$variant => write!(f, "{}", lower!(stringify!($variant))),)*
                }
            }
        }

        impl SortAndFilterBy {
            /// Get the value of the sort and filter by
            pub fn get_value(&self, $varname: &Apparel) -> i32 {
                match self {
                    $(SortAndFilterBy::$variant => $path),*
                }
            }
        }
    }
}

pub (crate) use generate_sort_by;
