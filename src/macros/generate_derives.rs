//! The [`generate_derives`] macro.

#[macro_export]
/// Generates all derives for `item`. To include `Default` (and other custom ones), pass
/// them at the start of the invocation.
macro_rules! generate_derives {
    ($($extras: ident)+, $item: item) => {
        #[derive(Clone, Debug $(, $extras)*)]
        #[cfg_attr(not(feature = "references"), derive(Hash, PartialEq, Eq, PartialOrd, Ord))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $item
    };

    ($item: item) => {
        #[derive(Clone, Debug)]
        #[cfg_attr(not(feature = "references"), derive(Hash, PartialEq, Eq, PartialOrd, Ord))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $item
    };
}
