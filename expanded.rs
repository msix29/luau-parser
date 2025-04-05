mod local_assignment {
    //! Holding all needed information for local assignments.
    use luau_lexer::prelude::Token;
    use luau_parser_derive::{Print, Range};
    use crate::types::{Expression, List, Name, Pointer};
    /// A struct holding data for local assignments.
    pub struct LocalAssignment {
        /// The `local` keyword.
        pub local_token: Token,
        /// The List of [`names`](NormalizedName) before the `=` sign.
        pub name_list: List<Name>,
        /// The `=` sign.
        pub equal_token: Option<Token>,
        /// The list of [`expressions`](Expression) after the `=` sign.
        pub expressions: List<Pointer<Expression>>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LocalAssignment {
        #[inline]
        fn clone(&self) -> LocalAssignment {
            LocalAssignment {
                local_token: ::core::clone::Clone::clone(&self.local_token),
                name_list: ::core::clone::Clone::clone(&self.name_list),
                equal_token: ::core::clone::Clone::clone(&self.equal_token),
                expressions: ::core::clone::Clone::clone(&self.expressions),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LocalAssignment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "LocalAssignment",
                "local_token",
                &self.local_token,
                "name_list",
                &self.name_list,
                "equal_token",
                &self.equal_token,
                "expressions",
                &&self.expressions,
            )
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for LocalAssignment {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.local_token, state);
            ::core::hash::Hash::hash(&self.name_list, state);
            ::core::hash::Hash::hash(&self.equal_token, state);
            ::core::hash::Hash::hash(&self.expressions, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for LocalAssignment {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LocalAssignment {
        #[inline]
        fn eq(&self, other: &LocalAssignment) -> bool {
            self.local_token == other.local_token && self.name_list == other.name_list
                && self.equal_token == other.equal_token
                && self.expressions == other.expressions
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for LocalAssignment {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Token>;
            let _: ::core::cmp::AssertParamIsEq<List<Name>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Token>>;
            let _: ::core::cmp::AssertParamIsEq<List<Pointer<Expression>>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for LocalAssignment {
        #[inline]
        fn partial_cmp(
            &self,
            other: &LocalAssignment,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(
                &self.local_token,
                &other.local_token,
            ) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.name_list,
                        &other.name_list,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.equal_token,
                                &other.equal_token,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &self.expressions,
                                        &other.expressions,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for LocalAssignment {
        #[inline]
        fn cmp(&self, other: &LocalAssignment) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.local_token, &other.local_token) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(&self.name_list, &other.name_list) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(
                                &self.equal_token,
                                &other.equal_token,
                            ) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(&self.expressions, &other.expressions)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    impl crate::types::GetRange for LocalAssignment {
        #[inline]
        fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
            Ok(
                crate::types::Range::new(
                    self.local_token.get_range()?.start,
                    self.expressions.get_range()?.end,
                ),
            )
        }
    }
    impl crate::types::Print for LocalAssignment {
        #[inline]
        fn print_with_leading(&self) -> Result<String, crate::types::PrintError> {
            let mut start = self.local_token.print_with_leading()?;
            start += &self.name_list.print_with_leading()?;
            start += &self.equal_token.print_with_leading()?;
            start += &self.expressions.print_with_leading()?;
            Ok(start)
        }
        #[inline]
        fn print(&self) -> Result<String, crate::types::PrintError> {
            let mut start = self.local_token.print()?;
            start += &self.name_list.print_with_trailing()?;
            start += &self.equal_token.print_with_trailing()?;
            start += &self.expressions.print_with_trailing()?;
            Ok(start)
        }
        #[inline]
        fn print_with_trailing(&self) -> Result<String, crate::types::PrintError> {
            let mut start = self.local_token.print_with_trailing()?;
            start += &self.name_list.print_with_trailing()?;
            start += &self.equal_token.print_with_trailing()?;
            start += &self.expressions.print_with_trailing()?;
            Ok(start)
        }
    }
}
