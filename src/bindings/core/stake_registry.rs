#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]
use StakeRegistry::*;

///Module containing a contract's types and functions.
/**

```solidity
library IStakeRegistryTypes {
    type StakeType is uint8;
    struct StakeUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint96 stake; }
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IStakeRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<StakeType> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl StakeType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for StakeType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<StakeType> for u8 {
            fn from(value: StakeType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StakeType {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeType {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct StakeUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint96 stake; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeUpdate {
        #[allow(missing_docs)]
        pub updateBlockNumber: u32,
        #[allow(missing_docs)]
        pub nextUpdateBlockNumber: u32,
        #[allow(missing_docs)]
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StakeUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: StakeUpdate) -> Self {
                (value.updateBlockNumber, value.nextUpdateBlockNumber, value.stake)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StakeUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    updateBlockNumber: tuple.0,
                    nextUpdateBlockNumber: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StakeUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StakeUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.updateBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StakeUpdate {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StakeUpdate {
            const NAME: &'static str = "StakeUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StakeUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint96 stake)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.updateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nextUpdateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.updateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nextUpdateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.updateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nextUpdateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct StrategyParams { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeRegistryTypesInstance<P, N> {
        IStakeRegistryTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IStakeRegistryTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IStakeRegistryTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeRegistryTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IStakeRegistryTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeRegistryTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IStakeRegistryTypesInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IStakeRegistryTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeRegistryTypesInstance<P, N> {
            IStakeRegistryTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IStakeRegistryTypesInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IStakeRegistryTypesInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IStakeRegistryTypes {
    type StakeType is uint8;
    struct StakeUpdate {
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
        uint96 stake;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface StakeRegistry {
    error BelowMinimumStakeRequirement();
    error EmptyStakeHistory();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InputDuplicateStrategy();
    error InputMultiplierZero();
    error InvalidBlockNumber();
    error LookAheadPeriodTooLong();
    error OnlySlashingRegistryCoordinator();
    error OnlySlashingRegistryCoordinatorOwner();
    error QuorumAlreadyExists();
    error QuorumDoesNotExist();
    error QuorumNotSlashable();

    event LookAheadPeriodChanged(uint32 oldLookAheadBlocks, uint32 newLookAheadBlocks);
    event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
    event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
    event QuorumCreated(uint8 indexed quorumNumber);
    event StakeTypeSet(IStakeRegistryTypes.StakeType newStakeType);
    event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
    event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
    event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);

    constructor(address _slashingRegistryCoordinator, address _delegationManager, address _avsDirectory, address _allocationManager);

    function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
    function WEIGHTING_DIVISOR() external view returns (uint256);
    function addStrategies(uint8 quorumNumber, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function delegation() external view returns (address);
    function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
    function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
    function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);
    function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint96);
    function getStakeAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, bytes32 operatorId, uint256 index) external view returns (uint96);
    function getStakeHistory(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate[] memory);
    function getStakeHistoryLength(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
    function getStakeUpdateAtIndex(uint8 quorumNumber, bytes32 operatorId, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    function getStakeUpdateIndexAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
    function getTotalStakeAtBlockNumberFromIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (uint96);
    function getTotalStakeHistoryLength(uint8 quorumNumber) external view returns (uint256);
    function getTotalStakeIndicesAtBlockNumber(uint32 blockNumber, bytes memory quorumNumbers) external view returns (uint32[] memory);
    function getTotalStakeUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    function initializeDelegatedStakeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    function initializeSlashableStakeQuorum(uint8 quorumNumber, uint96 minimumStake, uint32 lookAheadPeriod, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    function minimumStakeForQuorum(uint8) external view returns (uint96);
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
    function registryCoordinator() external view returns (address);
    function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
    function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
    function setSlashableStakeLookahead(uint8 quorumNumber, uint32 _lookAheadBlocks) external;
    function slashableStakeLookAheadPerQuorum(uint8 quorumNumber) external view returns (uint32);
    function stakeTypePerQuorum(uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeType);
    function strategiesPerQuorum(uint8 quorumNumber, uint256) external view returns (address);
    function strategyParams(uint8 quorumNumber, uint256) external view returns (address strategy, uint96 multiplier);
    function strategyParamsByIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StrategyParams memory);
    function strategyParamsLength(uint8 quorumNumber) external view returns (uint256);
    function updateOperatorsStake(address[] memory operators, bytes32[] memory operatorIds, uint8 quorumNumber) external returns (bool[] memory);
    function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_slashingRegistryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      },
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "MAX_WEIGHING_FUNCTION_LENGTH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "WEIGHTING_DIVISOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addStrategies",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCurrentStake",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentTotalStake",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestStakeUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeAtBlockNumber",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeAtBlockNumberAndIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeHistory",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate[]",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeHistoryLength",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeUpdateIndexAtBlockNumber",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeAtBlockNumberFromIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeHistoryLength",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializeDelegatedStakeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initializeSlashableStakeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "minimumStakeForQuorum",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "modifyStrategyParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "strategyIndices",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "newMultipliers",
        "type": "uint96[]",
        "internalType": "uint96[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96[]",
        "internalType": "uint96[]"
      },
      {
        "name": "",
        "type": "uint96[]",
        "internalType": "uint96[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeStrategies",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "indicesToRemove",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMinimumStakeForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setSlashableStakeLookahead",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_lookAheadBlocks",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashableStakeLookAheadPerQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeTypePerQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IStakeRegistryTypes.StakeType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategiesPerQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "multiplier",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParamsByIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StrategyParams",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParamsLength",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateOperatorsStake",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool[]",
        "internalType": "bool[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "weightOfOperatorForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "LookAheadPeriodChanged",
    "inputs": [
      {
        "name": "oldLookAheadBlocks",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "newLookAheadBlocks",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinimumStakeForQuorumUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorStakeUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "stake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumCreated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakeTypeSet",
    "inputs": [
      {
        "name": "newStakeType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IStakeRegistryTypes.StakeType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyAddedToQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyMultiplierUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "multiplier",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyRemovedFromQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "BelowMinimumStakeRequirement",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyStakeHistory",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputDuplicateStrategy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputMultiplierZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LookAheadPeriodTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlySlashingRegistryCoordinator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlySlashingRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumNotSlashable",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b50604051613e58380380613e5883398101604081905261002f91610068565b6001600160a01b0393841660e052918316608052821660a0521660c0526100c4565b6001600160a01b0381168114610065575f5ffd5b50565b5f5f5f5f6080858703121561007b575f5ffd5b845161008681610051565b602086015190945061009781610051565b60408601519093506100a881610051565b60608601519092506100b981610051565b939692955090935050565b60805160a05160c05160e051613d0d61014b5f395f818161041201528181610e590152818161166501528181611d5f01528181611e1b0152612d9301525f81816105ba01528181610f0701528181610f97015281816117130152818161184201528181612a6d0152612d5901525f6103b301525f818161061a01526123eb0152613d0d5ff3fe608060405234801561000f575f5ffd5b506004361061021e575f3560e01c80639ab4d6ff1161012a578063c8294c56116100b4578063df5cf72311610079578063df5cf72314610615578063e086adb31461063c578063f2be94ae1461064f578063f851e19814610662578063fa28c62714610675575f5ffd5b8063c8294c56146105a2578063ca8aa7c7146105b5578063cc5a7c20146105dc578063d5eccc05146105ef578063dd9846b914610602575f5ffd5b8063b6904b78116100fa578063b6904b781461052e578063bc9a40c314610541578063bd29b8cd14610554578063c46778a514610567578063c601527d1461058f575f5ffd5b80639ab4d6ff146104815780639f3ccf65146104bb578063ac6bfb03146104ce578063adc804da146104ee575f5ffd5b80635e5a6775116101ab5780636c3fb4bf1161017b5780636c3fb4bf146103ed5780636d14a9871461040d57806375d4173a146104345780637c1723471461044757806381c0750214610461575f5ffd5b80635e5a67751461035d5780635f1f2d771461036c578063697fbd931461037f5780636b3aa72e146103ae575f5ffd5b806325504777116101f157806325504777146102b85780632cd95940146102d95780633ca5a5f5146102f95780634bd26e091461031b5780635401ed271461034a575f5ffd5b80630491b41c1461022257806308732461146102575780631f9b74e01461027857806320b66298146102a3575b5f5ffd5b610244610230366004612eb1565b60ff165f9081526001602052604090205490565b6040519081526020015b60405180910390f35b61026a610265366004612eca565b610688565b60405161024e929190612ef2565b61028b610286366004612f28565b6106cd565b6040516001600160601b03909116815260200161024e565b6102b66102b1366004612f9d565b6106ef565b005b6102cb6102c6366004613058565b61088d565b60405161024e9291906130f2565b6102ec6102e7366004613116565b610a15565b60405161024e9190613140565b610244610307366004612eb1565b60ff165f9081526003602052604090205490565b610244610329366004613116565b5f91825260026020908152604080842060ff93909316845291905290205490565b61028b610358366004613116565b610ab2565b610244670de0b6b3a764000081565b6102b661037a366004613247565b610aca565b6103a161038d366004612eb1565b60056020525f908152604090205460ff1681565b60405161024e9190613303565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161024e565b6104006103fb36600461338e565b611008565b60405161024e9190613460565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6102b6610442366004613544565b611163565b61044f602081565b60405160ff909116815260200161024e565b61047461046f3660046135ae565b611249565b60405161024e91906135fe565b6104a661048f366004612eb1565b60066020525f908152604090205463ffffffff1681565b60405163ffffffff909116815260200161024e565b6103d56104c9366004612eca565b6113ef565b6104e16104dc36600461363b565b611423565b60405161024e919061366b565b6105016104fc366004612eca565b6114b9565b6040805182516001600160a01b031681526020928301516001600160601b0316928101929092520161024e565b6104e161053c366004612eca565b611530565b6102b661054f3660046136a5565b6115bd565b6102b66105623660046136cd565b6115de565b61028b610575366004612eb1565b5f602081905290815260409020546001600160601b031681565b6102b661059d3660046136fb565b611641565b61028b6105b0366004613745565b6118b1565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6102b66105ea366004613781565b61192d565b61028b6105fd366004612eb1565b611a1f565b6104a66106103660046137ed565b611a70565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6102b661064a36600461382a565b611a84565b61028b61065d366004613854565b611aa0565b6104e1610670366004613116565b611b33565b61028b6106833660046137ed565b611c21565b6003602052815f5260405f2081815481106106a1575f80fd5b5f918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b5f826106d881611c80565b5f6106e38585611cb2565b509250505b5092915050565b6106f7611d5d565b8461070181611c80565b83806107205760405163796cc52560e01b815260040160405180910390fd5b828114610740576040516343714afd60e01b815260040160405180910390fd5b60ff87165f908152600360205260408120905b828110156108825785858281811061076d5761076d613895565b905060200201602081019061078291906138a9565b8289898481811061079557610795613895565b90506020020135815481106107ac576107ac613895565b905f5260205f20015f0160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a8581811061081257610812613895565b905060200201358154811061082957610829613895565b5f918252602090912001546001600160a01b031688888581811061084f5761084f613895565b905060200201602081019061086491906138a9565b604051610872929190612ef2565b60405180910390a2600101610753565b505050505050505050565b606080610898611e10565b5f836001600160401b038111156108b1576108b16131b9565b6040519080825280602002602001820160405280156108da578160200160208202803683370190505b5090505f846001600160401b038111156108f6576108f66131b9565b60405190808252806020026020018201604052801561091f578160200160208202803683370190505b5090505f5b85811015610a07575f87878381811061093f5761093f613895565b919091013560f81c9150610954905081611c80565b5f5f610960838d611cb2565b91509150806109825760405163207f13e360e11b815260040160405180910390fd5b5f61098e8c8585611e59565b9050828786815181106109a3576109a3613895565b60200260200101906001600160601b031690816001600160601b0316815250506109cd84826120d2565b8686815181106109df576109df613895565b6001600160601b03909216602092830291909101909101525050600190920191506109249050565b509097909650945050505050565b5f82815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610aa5575f848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610a4d565b5050505090505b92915050565b5f5f610abe8484611b33565b60400151949350505050565b610ad2611d5d565b81610adc81611c80565b815180610afc5760405163796cc52560e01b815260040160405180910390fd5b60ff84165f908152600360209081526040808320600490925282209091836001600160401b03811115610b3157610b316131b9565b604051908082528060200260200182016040528015610b5a578160200160208202803683370190505b5090505f5b84811015610e555783878281518110610b7a57610b7a613895565b602002602001015181548110610b9257610b92613895565b5f9182526020909120015482516001600160a01b0390911690839083908110610bbd57610bbd613895565b60200260200101906001600160a01b031690816001600160a01b0316815250508760ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f785898481518110610c1557610c15613895565b602002602001015181548110610c2d57610c2d613895565b5f91825260209182902001546040516001600160a01b0390911681520160405180910390a28760ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7585898481518110610c8a57610c8a613895565b602002602001015181548110610ca257610ca2613895565b5f91825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a283548490610ce1906001906138d6565b81548110610cf157610cf1613895565b905f5260205f200184888381518110610d0c57610d0c613895565b602002602001015181548110610d2457610d24613895565b5f91825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558354849080610d7657610d766138e9565b5f8281526020812082015f199081019190915501905582548390610d9c906001906138d6565b81548110610dac57610dac613895565b905f5260205f20015f9054906101000a90046001600160a01b031683888381518110610dda57610dda613895565b602002602001015181548110610df257610df2613895565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555082805480610e2d57610e2d6138e9565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610b5f565b505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610eb3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ed791906138fd565b6040805180820182526001600160a01b03838116825260ff8c16602083015291516304c1b8eb60e31b81529293507f00000000000000000000000000000000000000000000000000000000000000009091169163260dc75891610f3c91600401613918565b602060405180830381865afa158015610f57573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f7b919061393e565b15610ffe5760405163b66bd98960e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063b66bd98990610fd09084908c90879060040161395d565b5f604051808303815f87803b158015610fe7575f5ffd5b505af1158015610ff9573d5f5f3e3d5ffd5b505050505b5050505050505050565b6060611012611e10565b5f84516001600160401b0381111561102c5761102c6131b9565b604051908082528060200260200182016040528015611055578160200160208202803683370190505b50905061106183611c80565b5f5f61106d8588612243565b90925090505f805b88518110156111495782818151811061109057611090613895565b60200260200101516110f4575f8482815181106110af576110af613895565b60200260200101906001600160601b031690816001600160601b03168152505060018582815181106110e3576110e3613895565b911515602092830291909101909101525b5f61113289838151811061110a5761110a613895565b60200260200101518987858151811061112557611125613895565b6020026020010151611e59565b905061113e81846139c7565b925050600101611075565b5061115486826120d2565b509293505050505b9392505050565b61116b611e10565b60ff83165f908152600160205260409020541561119b576040516310cda51760e21b815260040160405180910390fd5b6111a583826125e9565b6111af83836128da565b6111b9835f612942565b505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60605f826001600160401b03811115611264576112646131b9565b60405190808252806020026020018201604052801561128d578160200160208202803683370190505b5090505f5b838110156113e6575f8585838181106112ad576112ad613895565b919091013560f81c91506112c2905081611c80565b60ff81165f908152600160205260408120805463ffffffff8a1692906112ea576112ea613895565b5f9182526020909120015463ffffffff16111561131a5760405163cc64657360e01b815260040160405180910390fd5b60ff81165f90815260016020526040812054905b818110156113db5760ff83165f90815260016020819052604090912063ffffffff8b169161135c84866138d6565b61136691906138d6565b8154811061137657611376613895565b5f9182526020909120015463ffffffff16116113d357600161139882846138d6565b6113a291906138d6565b8585815181106113b4576113b4613895565b602002602001019063ffffffff16908163ffffffff16815250506113db565b60010161132e565b505050600101611292565b50949350505050565b6004602052815f5260405f208181548110611408575f80fd5b5f918252602090912001546001600160a01b03169150829050565b604080516060810182525f80825260208083018290528284018290528582526002815283822060ff8816835290529190912080548390811061146757611467613895565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091525f808252602082015260ff83165f9081526003602052604090208054839081106114ef576114ef613895565b5f918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182525f808252602080830182905282840182905260ff86168252600190529190912080548390811061156c5761156c613895565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b6115c5611d5d565b816115cf81611c80565b6115d983836128da565b505050565b6115e6611e10565b5f5b8181101561163b575f83838381811061160357611603613895565b919091013560f81c9150611618905081611c80565b5f61162486835f611e59565b905061163082826120d2565b5050506001016115e8565b50505050565b611649611d5d565b8161165381611c80565b61165d83836125e9565b5f825190505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116e391906138fd565b6040805180820182526001600160a01b03838116825260ff8916602083015291516304c1b8eb60e31b81529293507f00000000000000000000000000000000000000000000000000000000000000009091169163260dc7589161174891600401613918565b602060405180830381865afa158015611763573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611787919061393e565b156118aa575f826001600160401b038111156117a5576117a56131b9565b6040519080825280602002602001820160405280156117ce578160200160208202803683370190505b5090505f5b8381101561182a578581815181106117ed576117ed613895565b60200260200101515f015182828151811061180a5761180a613895565b6001600160a01b03909216602092830291909101909101526001016117d3565b50604051630287f75160e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906350feea209061187b9085908a90869060040161395d565b5f604051808303815f87803b158015611892575f5ffd5b505af11580156118a4573d5f5f3e3d5ffd5b50505050505b5050505050565b60ff83165f9081526001602052604081208054829190849081106118d7576118d7613895565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610abe81856129af565b611935611e10565b60ff84165f9081526001602052604090205415611965576040516310cda51760e21b815260040160405180910390fd5b61196f84826125e9565b61197984846128da565b611984846001612942565b61198e8483612a26565b50505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff81165f908152600160208190526040822080549091611a3f916138d6565b81548110611a4f57611a4f613895565b5f91825260209091200154600160401b90046001600160601b031692915050565b5f611a7c848484612b83565b949350505050565b611a8c611d5d565b81611a9681611c80565b6115d98383612a26565b5f82815260026020908152604080832060ff881684529091528120805482919084908110611ad057611ad0613895565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611b2681866129af565b6040015195945050505050565b60408051606080820183525f80835260208084018290528385018290528682526002815284822060ff8716835281528482205485519384018652828452908301829052938201819052919291829003611b8f579150610aac9050565b5f85815260026020908152604080832060ff881684529091529020611bb56001846138d6565b81548110611bc557611bc5613895565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610aac915050565b505092915050565b5f83815260026020908152604080832060ff861684529091528120611c47858585612b83565b63ffffffff1681548110611c5d57611c5d613895565b5f91825260209091200154600160401b90046001600160601b0316949350505050565b60ff81165f90815260016020526040902054611caf57604051637310cff560e11b815260040160405180910390fd5b50565b6040805160018082528183019092525f91829182916020808301908036833701905050905083815f81518110611cea57611cea613895565b60200260200101906001600160a01b031690816001600160a01b0316815250505f5f611d168784612243565b91509150815f81518110611d2c57611d2c613895565b6020026020010151815f81518110611d4657611d46613895565b6020026020010151945094505050505b9250929050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611db9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ddd91906138fd565b6001600160a01b0316336001600160a01b031614611e0e5760405163ce98c24b60e01b815260040160405180910390fd5b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611e0e57604051632c01b20560e21b815260040160405180910390fd5b5f83815260026020908152604080832060ff861684529091528120548190808203611f1d575f86815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055612078565b5f86815260026020908152604080832060ff891684529091528120611f436001846138d6565b81548110611f5357611f53613895565b5f91825260209091200180546001600160601b03600160401b9091048116945090915085168303611f89575f935050505061115c565b805463ffffffff438116911603611fc1578054600160401b600160a01b031916600160401b6001600160601b03871602178155612076565b805467ffffffff000000001916600160201b4363ffffffff9081168281029390931784555f8a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a26120c88285612cea565b9695505050505050565b60ff82165f90815260016020819052604082208054918391906120f590846138d6565b8154811061210557612105613895565b905f5260205f20019050835f036121305754600160401b90046001600160601b03169150610aac9050565b80545f9061214e90600160401b90046001600160601b031686612d01565b825490915063ffffffff438116911603612189578154600160401b600160a01b031916600160401b6001600160601b0383160217825561223a565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff89165f90815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b6060805f83516001600160401b03811115612260576122606131b9565b604051908082528060200260200182016040528015612289578160200160208202803683370190505b5090505f84516001600160401b038111156122a6576122a66131b9565b6040519080825280602002602001820160405280156122cf578160200160208202803683370190505b5090505f6122eb8760ff165f9081526003602052604090205490565b60ff88165f90815260036020908152604080832080548251818502810185019093528083529495509293909291849084015b8282101561236b575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b03168183015282526001909201910161231d565b5050505090506060600180811115612385576123856132ef565b60ff808b165f908152600560205260409020541660018111156123aa576123aa6132ef565b036123c0576123b98989612d2e565b9050612463565b60ff89165f908152600460208190526040918290209151637870733b60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169263f0e0e6769261241f928d929101613a5b565b5f60405180830381865afa158015612439573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124609190810190613a7f565b90505b5f5b88518110156125d9575f5b84811015612569575f84828151811061248b5761248b613895565b602002602001015190505f8484815181106124a8576124a8613895565b602002602001015183815181106124c1576124c1613895565b6020026020010151111561256057670de0b6b3a764000081602001516001600160601b03168585815181106124f8576124f8613895565b6020026020010151848151811061251157612511613895565b60200260200101516125239190613b8b565b61252d9190613ba2565b88848151811061253f5761253f613895565b602002602001018181516125539190613bc1565b6001600160601b03169052505b50600101612470565b5060ff8a165f9081526020819052604090205486516001600160601b039091169087908390811061259c5761259c613895565b60200260200101516001600160601b031610158582815181106125c1576125c1613895565b91151560209283029190910190910152600101612465565b5093989297509195505050505050565b5f81511161260a5760405163796cc52560e01b815260040160405180910390fd5b805160ff83165f908152600360209081526040909120549061262c8383613be0565b111561264b576040516343714afd60e01b815260040160405180910390fd5b5f5b828110156118aa575f5b6126618284613be0565b8110156126f05784828151811061267a5761267a613895565b60200260200101515f01516001600160a01b031660035f8860ff1660ff1681526020019081526020015f2082815481106126b6576126b6613895565b5f918252602090912001546001600160a01b0316036126e857604051637b74340b60e01b815260040160405180910390fd5b600101612657565b505f84828151811061270457612704613895565b6020026020010151602001516001600160601b03161161273757604051637257125160e01b815260040160405180910390fd5b60ff85165f908152600360205260409020845185908390811061275c5761275c613895565b60209081029190910181015182546001810184555f9384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106127c0576127c0613895565b6020908102919091018101515182546001810184555f938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f54049086908490811061283657612836613895565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7585838151811061289357612893613895565b60200260200101515f01518684815181106128b0576128b0613895565b6020026020010151602001516040516128ca929190612ef2565b60405180910390a260010161264d565b60ff82165f818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60ff82165f908152600560205260409020805482919060ff19166001838181111561296f5761296f6132ef565b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516129a39190613303565b60405180910390a15050565b815f015163ffffffff168163ffffffff1610156129df57604051631391e11b60e21b815260040160405180910390fd5b602082015163ffffffff161580612a055750816020015163ffffffff168163ffffffff16105b612a2257604051631391e11b60e21b815260040160405180910390fd5b5050565b600160ff8084165f90815260056020526040902054166001811115612a4d57612a4d6132ef565b14612a6b5760405163a3be258360e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632981eb776040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ac7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aeb9190613bf3565b63ffffffff168163ffffffff161115612b1757604051630bd441b960e21b815260040160405180910390fd5b60ff82165f90815260066020908152604091829020805463ffffffff19811663ffffffff8681169182179093558451929091168083529282015290917f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7910160405180910390a1505050565b5f83815260026020908152604080832060ff86168452909152812054805b8015612c21575f86815260026020908152604080832060ff89168452909152902063ffffffff851690612bd56001846138d6565b81548110612be557612be5613895565b5f9182526020909120015463ffffffff1611612c0f57612c066001826138d6565b9250505061115c565b80612c1981613c0e565b915050612ba1565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e40160405180910390fd5b5f61115c6001600160601b03808516908416613c23565b5f5f821215612d2457612d1382613c42565b612d1d9084613c5c565b9050610aac565b612d1d8284613bc1565b60ff82165f9081526006602052604081205460609190612d549063ffffffff1643613be0565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632bab2c4a60405180604001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ded573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e1191906138fd565b6001600160a01b0316815260ff891660209182018190525f90815260049182905260409081902090516001600160e01b031960e086901b168152612e5b93928a9291899101613c7b565b5f60405180830381865afa158015612e75573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261223a9190810190613a7f565b803560ff81168114612eac575f5ffd5b919050565b5f60208284031215612ec1575f5ffd5b61115c82612e9c565b5f5f60408385031215612edb575f5ffd5b612ee483612e9c565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114611caf575f5ffd5b5f5f60408385031215612f39575f5ffd5b612f4283612e9c565b91506020830135612f5281612f14565b809150509250929050565b5f5f83601f840112612f6d575f5ffd5b5081356001600160401b03811115612f83575f5ffd5b6020830191508360208260051b8501011115611d56575f5ffd5b5f5f5f5f5f60608688031215612fb1575f5ffd5b612fba86612e9c565b945060208601356001600160401b03811115612fd4575f5ffd5b612fe088828901612f5d565b90955093505060408601356001600160401b03811115612ffe575f5ffd5b61300a88828901612f5d565b969995985093965092949392505050565b5f5f83601f84011261302b575f5ffd5b5081356001600160401b03811115613041575f5ffd5b602083019150836020828501011115611d56575f5ffd5b5f5f5f5f6060858703121561306b575f5ffd5b843561307681612f14565b93506020850135925060408501356001600160401b03811115613097575f5ffd5b6130a38782880161301b565b95989497509550505050565b5f8151808452602084019350602083015f5b828110156130e85781516001600160601b03168652602095860195909101906001016130c1565b5093949350505050565b604081525f61310460408301856130af565b828103602084015261223a81856130af565b5f5f60408385031215613127575f5ffd5b8235915061313760208401612e9c565b90509250929050565b602080825282518282018190525f918401906040840190835b818110156131ae5761319883855163ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b6020939093019260609290920191600101613159565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b03811182821017156131ef576131ef6131b9565b60405290565b604051601f8201601f191681016001600160401b038111828210171561321d5761321d6131b9565b604052919050565b5f6001600160401b0382111561323d5761323d6131b9565b5060051b60200190565b5f5f60408385031215613258575f5ffd5b61326183612e9c565b915060208301356001600160401b0381111561327b575f5ffd5b8301601f8101851361328b575f5ffd5b803561329e61329982613225565b6131f5565b8082825260208201915060208360051b8501019250878311156132bf575f5ffd5b6020840193505b828410156132e15783358252602093840193909101906132c6565b809450505050509250929050565b634e487b7160e01b5f52602160045260245ffd5b602081016002831061332357634e487b7160e01b5f52602160045260245ffd5b91905290565b5f82601f830112613338575f5ffd5b813561334661329982613225565b8082825260208201915060208360051b860101925085831115613367575f5ffd5b602085015b8381101561338457803583526020928301920161336c565b5095945050505050565b5f5f5f606084860312156133a0575f5ffd5b83356001600160401b038111156133b5575f5ffd5b8401601f810186136133c5575f5ffd5b80356133d361329982613225565b8082825260208201915060208360051b8501019250888311156133f4575f5ffd5b6020840193505b8284101561341f57833561340e81612f14565b8252602093840193909101906133fb565b955050505060208401356001600160401b0381111561343c575f5ffd5b61344886828701613329565b92505061345760408501612e9c565b90509250925092565b602080825282518282018190525f918401906040840190835b818110156131ae5783511515835260209384019390920191600101613479565b80356001600160601b0381168114612eac575f5ffd5b5f82601f8301126134be575f5ffd5b81356134cc61329982613225565b8082825260208201915060208360061b8601019250858311156134ed575f5ffd5b602085015b838110156133845760408188031215613509575f5ffd5b6135116131cd565b813561351c81612f14565b815261352a60208301613499565b6020820152808452506020830192506040810190506134f2565b5f5f5f60608486031215613556575f5ffd5b61355f84612e9c565b925061356d60208501613499565b915060408401356001600160401b03811115613587575f5ffd5b613593868287016134af565b9150509250925092565b63ffffffff81168114611caf575f5ffd5b5f5f5f604084860312156135c0575f5ffd5b83356135cb8161359d565b925060208401356001600160401b038111156135e5575f5ffd5b6135f18682870161301b565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b818110156131ae57835163ffffffff16835260209384019390920191600101613617565b5f5f5f6060848603121561364d575f5ffd5b61365684612e9c565b95602085013595506040909401359392505050565b60608101610aac828463ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b5f5f604083850312156136b6575f5ffd5b6136bf83612e9c565b915061313760208401613499565b5f5f5f604084860312156136df575f5ffd5b8335925060208401356001600160401b038111156135e5575f5ffd5b5f5f6040838503121561370c575f5ffd5b61371583612e9c565b915060208301356001600160401b0381111561372f575f5ffd5b61373b858286016134af565b9150509250929050565b5f5f5f60608486031215613757575f5ffd5b61376084612e9c565b925060208401356137708161359d565b929592945050506040919091013590565b5f5f5f5f60808587031215613794575f5ffd5b61379d85612e9c565b93506137ab60208601613499565b925060408501356137bb8161359d565b915060608501356001600160401b038111156137d5575f5ffd5b6137e1878288016134af565b91505092959194509250565b5f5f5f606084860312156137ff575f5ffd5b8335925061380f60208501612e9c565b9150604084013561381f8161359d565b809150509250925092565b5f5f6040838503121561383b575f5ffd5b61384483612e9c565b91506020830135612f528161359d565b5f5f5f5f60808587031215613867575f5ffd5b61387085612e9c565b935060208501356138808161359d565b93969395505050506040820135916060013590565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156138b9575f5ffd5b61115c82613499565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610aac57610aac6138c2565b634e487b7160e01b5f52603160045260245ffd5b5f6020828403121561390d575f5ffd5b815161115c81612f14565b81516001600160a01b0316815260208083015163ffffffff169082015260408101610aac565b5f6020828403121561394e575f5ffd5b8151801515811461115c575f5ffd5b6001600160a01b038416815260ff831660208083019190915260606040830181905283519083018190525f918401906080840190835b818110156139ba5783516001600160a01b0316835260209384019390920191600101613993565b5090979650505050505050565b8082018281125f831280158216821582161715611c1957611c196138c2565b5f8151808452602084019350602083015f5b828110156130e85781516001600160a01b03168652602095860195909101906001016139f8565b5f8154808452602084019350825f5260205f205f5b828110156130e85781546001600160a01b0316865260209095019460019182019101613a34565b604081525f613a6d60408301856139e6565b828103602084015261223a8185613a1f565b5f60208284031215613a8f575f5ffd5b81516001600160401b03811115613aa4575f5ffd5b8201601f81018413613ab4575f5ffd5b8051613ac261329982613225565b8082825260208201915060208360051b850101925086831115613ae3575f5ffd5b602084015b83811015613b805780516001600160401b03811115613b05575f5ffd5b8501603f81018913613b15575f5ffd5b6020810151613b2661329982613225565b808282526020820191506020808460051b8601010192508b831115613b49575f5ffd5b6040840193505b82841015613b6b578351825260209384019390910190613b50565b86525050602093840193919091019050613ae8565b509695505050505050565b8082028115828204841417610aac57610aac6138c2565b5f82613bbc57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160601b038181168382160190811115610aac57610aac6138c2565b80820180821115610aac57610aac6138c2565b5f60208284031215613c03575f5ffd5b815161115c8161359d565b5f81613c1c57613c1c6138c2565b505f190190565b8181035f8312801583831316838312821617156106e8576106e86138c2565b5f600160ff1b8201613c5657613c566138c2565b505f0390565b6001600160601b038281168282160390811115610aac57610aac6138c2565b84516001600160a01b0316815260208086015163ffffffff169082015260a060408201525f613cad60a08301866139e6565b8281036060840152613cbf8186613a1f565b91505063ffffffff831660808301529594505050505056fea2646970667358221220182ad3dfe4084d4f02ad981a1771312883d4fe913befb36d1c251992d7847fa864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa>X8\x03\x80a>X\x839\x81\x01`@\x81\x90Ra\0/\x91a\0hV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xE0R\x91\x83\x16`\x80R\x82\x16`\xA0R\x16`\xC0Ra\0\xC4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0eW__\xFD[PV[____`\x80\x85\x87\x03\x12\x15a\0{W__\xFD[\x84Qa\0\x86\x81a\0QV[` \x86\x01Q\x90\x94Pa\0\x97\x81a\0QV[`@\x86\x01Q\x90\x93Pa\0\xA8\x81a\0QV[``\x86\x01Q\x90\x92Pa\0\xB9\x81a\0QV[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa=\ra\x01K_9_\x81\x81a\x04\x12\x01R\x81\x81a\x0EY\x01R\x81\x81a\x16e\x01R\x81\x81a\x1D_\x01R\x81\x81a\x1E\x1B\x01Ra-\x93\x01R_\x81\x81a\x05\xBA\x01R\x81\x81a\x0F\x07\x01R\x81\x81a\x0F\x97\x01R\x81\x81a\x17\x13\x01R\x81\x81a\x18B\x01R\x81\x81a*m\x01Ra-Y\x01R_a\x03\xB3\x01R_\x81\x81a\x06\x1A\x01Ra#\xEB\x01Ra=\r_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x1EW_5`\xE0\x1C\x80c\x9A\xB4\xD6\xFF\x11a\x01*W\x80c\xC8)LV\x11a\0\xB4W\x80c\xDF\\\xF7#\x11a\0yW\x80c\xDF\\\xF7#\x14a\x06\x15W\x80c\xE0\x86\xAD\xB3\x14a\x06<W\x80c\xF2\xBE\x94\xAE\x14a\x06OW\x80c\xF8Q\xE1\x98\x14a\x06bW\x80c\xFA(\xC6'\x14a\x06uW__\xFD[\x80c\xC8)LV\x14a\x05\xA2W\x80c\xCA\x8A\xA7\xC7\x14a\x05\xB5W\x80c\xCCZ| \x14a\x05\xDCW\x80c\xD5\xEC\xCC\x05\x14a\x05\xEFW\x80c\xDD\x98F\xB9\x14a\x06\x02W__\xFD[\x80c\xB6\x90Kx\x11a\0\xFAW\x80c\xB6\x90Kx\x14a\x05.W\x80c\xBC\x9A@\xC3\x14a\x05AW\x80c\xBD)\xB8\xCD\x14a\x05TW\x80c\xC4gx\xA5\x14a\x05gW\x80c\xC6\x01R}\x14a\x05\x8FW__\xFD[\x80c\x9A\xB4\xD6\xFF\x14a\x04\x81W\x80c\x9F<\xCFe\x14a\x04\xBBW\x80c\xACk\xFB\x03\x14a\x04\xCEW\x80c\xAD\xC8\x04\xDA\x14a\x04\xEEW__\xFD[\x80c^Zgu\x11a\x01\xABW\x80cl?\xB4\xBF\x11a\x01{W\x80cl?\xB4\xBF\x14a\x03\xEDW\x80cm\x14\xA9\x87\x14a\x04\rW\x80cu\xD4\x17:\x14a\x044W\x80c|\x17#G\x14a\x04GW\x80c\x81\xC0u\x02\x14a\x04aW__\xFD[\x80c^Zgu\x14a\x03]W\x80c_\x1F-w\x14a\x03lW\x80ci\x7F\xBD\x93\x14a\x03\x7FW\x80ck:\xA7.\x14a\x03\xAEW__\xFD[\x80c%PGw\x11a\x01\xF1W\x80c%PGw\x14a\x02\xB8W\x80c,\xD9Y@\x14a\x02\xD9W\x80c<\xA5\xA5\xF5\x14a\x02\xF9W\x80cK\xD2n\t\x14a\x03\x1BW\x80cT\x01\xED'\x14a\x03JW__\xFD[\x80c\x04\x91\xB4\x1C\x14a\x02\"W\x80c\x08s$a\x14a\x02WW\x80c\x1F\x9Bt\xE0\x14a\x02xW\x80c \xB6b\x98\x14a\x02\xA3W[__\xFD[a\x02Da\x0206`\x04a.\xB1V[`\xFF\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ja\x02e6`\x04a.\xCAV[a\x06\x88V[`@Qa\x02N\x92\x91\x90a.\xF2V[a\x02\x8Ba\x02\x866`\x04a/(V[a\x06\xCDV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x02\xB6a\x02\xB16`\x04a/\x9DV[a\x06\xEFV[\0[a\x02\xCBa\x02\xC66`\x04a0XV[a\x08\x8DV[`@Qa\x02N\x92\x91\x90a0\xF2V[a\x02\xECa\x02\xE76`\x04a1\x16V[a\n\x15V[`@Qa\x02N\x91\x90a1@V[a\x02Da\x03\x076`\x04a.\xB1V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Da\x03)6`\x04a1\x16V[_\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x8Ba\x03X6`\x04a1\x16V[a\n\xB2V[a\x02Dg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xB6a\x03z6`\x04a2GV[a\n\xCAV[a\x03\xA1a\x03\x8D6`\x04a.\xB1V[`\x05` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02N\x91\x90a3\x03V[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x04\0a\x03\xFB6`\x04a3\x8EV[a\x10\x08V[`@Qa\x02N\x91\x90a4`V[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x04B6`\x04a5DV[a\x11cV[a\x04O` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02NV[a\x04ta\x04o6`\x04a5\xAEV[a\x12IV[`@Qa\x02N\x91\x90a5\xFEV[a\x04\xA6a\x04\x8F6`\x04a.\xB1V[`\x06` R_\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02NV[a\x03\xD5a\x04\xC96`\x04a.\xCAV[a\x13\xEFV[a\x04\xE1a\x04\xDC6`\x04a6;V[a\x14#V[`@Qa\x02N\x91\x90a6kV[a\x05\x01a\x04\xFC6`\x04a.\xCAV[a\x14\xB9V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02NV[a\x04\xE1a\x05<6`\x04a.\xCAV[a\x150V[a\x02\xB6a\x05O6`\x04a6\xA5V[a\x15\xBDV[a\x02\xB6a\x05b6`\x04a6\xCDV[a\x15\xDEV[a\x02\x8Ba\x05u6`\x04a.\xB1V[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\xB6a\x05\x9D6`\x04a6\xFBV[a\x16AV[a\x02\x8Ba\x05\xB06`\x04a7EV[a\x18\xB1V[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x05\xEA6`\x04a7\x81V[a\x19-V[a\x02\x8Ba\x05\xFD6`\x04a.\xB1V[a\x1A\x1FV[a\x04\xA6a\x06\x106`\x04a7\xEDV[a\x1ApV[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x06J6`\x04a8*V[a\x1A\x84V[a\x02\x8Ba\x06]6`\x04a8TV[a\x1A\xA0V[a\x04\xE1a\x06p6`\x04a1\x16V[a\x1B3V[a\x02\x8Ba\x06\x836`\x04a7\xEDV[a\x1C!V[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x06\xA1W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[_\x82a\x06\xD8\x81a\x1C\x80V[_a\x06\xE3\x85\x85a\x1C\xB2V[P\x92PP[P\x92\x91PPV[a\x06\xF7a\x1D]V[\x84a\x07\x01\x81a\x1C\x80V[\x83\x80a\x07 W`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x07@W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x87\x16_\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\x08\x82W\x85\x85\x82\x81\x81\x10a\x07mWa\x07ma8\x95V[\x90P` \x02\x01` \x81\x01\x90a\x07\x82\x91\x90a8\xA9V[\x82\x89\x89\x84\x81\x81\x10a\x07\x95Wa\x07\x95a8\x95V[\x90P` \x02\x015\x81T\x81\x10a\x07\xACWa\x07\xACa8\x95V[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08\x12Wa\x08\x12a8\x95V[\x90P` \x02\x015\x81T\x81\x10a\x08)Wa\x08)a8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\x08OWa\x08Oa8\x95V[\x90P` \x02\x01` \x81\x01\x90a\x08d\x91\x90a8\xA9V[`@Qa\x08r\x92\x91\x90a.\xF2V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x07SV[PPPPPPPPPV[``\x80a\x08\x98a\x1E\x10V[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xB1Wa\x08\xB1a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xDAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xF6Wa\x08\xF6a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a\n\x07W_\x87\x87\x83\x81\x81\x10a\t?Wa\t?a8\x95V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\tT\x90P\x81a\x1C\x80V[__a\t`\x83\x8Da\x1C\xB2V[\x91P\x91P\x80a\t\x82W`@Qc \x7F\x13\xE3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\x8E\x8C\x85\x85a\x1EYV[\x90P\x82\x87\x86\x81Q\x81\x10a\t\xA3Wa\t\xA3a8\x95V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\t\xCD\x84\x82a \xD2V[\x86\x86\x81Q\x81\x10a\t\xDFWa\t\xDFa8\x95V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x90\x92\x01\x91Pa\t$\x90PV[P\x90\x97\x90\x96P\x94PPPPPV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\n\xA5W_\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\nMV[PPPP\x90P[\x92\x91PPV[__a\n\xBE\x84\x84a\x1B3V[`@\x01Q\x94\x93PPPPV[a\n\xD2a\x1D]V[\x81a\n\xDC\x81a\x1C\x80V[\x81Q\x80a\n\xFCW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x84\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B1Wa\x0B1a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x0EUW\x83\x87\x82\x81Q\x81\x10a\x0BzWa\x0Bza8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\x0B\x92Wa\x0B\x92a8\x95V[_\x91\x82R` \x90\x91 \x01T\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a\x0B\xBDWa\x0B\xBDa8\x95V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x87`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x85\x89\x84\x81Q\x81\x10a\x0C\x15Wa\x0C\x15a8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\x0C-Wa\x0C-a8\x95V[_\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x87`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x89\x84\x81Q\x81\x10a\x0C\x8AWa\x0C\x8Aa8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\x0C\xA2Wa\x0C\xA2a8\x95V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x83T\x84\x90a\x0C\xE1\x90`\x01\x90a8\xD6V[\x81T\x81\x10a\x0C\xF1Wa\x0C\xF1a8\x95V[\x90_R` _ \x01\x84\x88\x83\x81Q\x81\x10a\r\x0CWa\r\x0Ca8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\r$Wa\r$a8\x95V[_\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x83T\x84\x90\x80a\rvWa\rva8\xE9V[_\x82\x81R` \x81 \x82\x01_\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x82T\x83\x90a\r\x9C\x90`\x01\x90a8\xD6V[\x81T\x81\x10a\r\xACWa\r\xACa8\x95V[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x83\x81Q\x81\x10a\r\xDAWa\r\xDAa8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\r\xF2Wa\r\xF2a8\x95V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x82\x80T\x80a\x0E-Wa\x0E-a8\xE9V[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\x0B_V[P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD7\x91\x90a8\xFDV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xFF\x8C\x16` \x83\x01R\x91Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x92\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91c&\r\xC7X\x91a\x0F<\x91`\x04\x01a9\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FWW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F{\x91\x90a9>V[\x15a\x0F\xFEW`@Qc\xB6k\xD9\x89`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB6k\xD9\x89\x90a\x0F\xD0\x90\x84\x90\x8C\x90\x87\x90`\x04\x01a9]V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xE7W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xF9W=__>=_\xFD[PPPP[PPPPPPPPV[``a\x10\x12a\x1E\x10V[_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10,Wa\x10,a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10UW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90Pa\x10a\x83a\x1C\x80V[__a\x10m\x85\x88a\"CV[\x90\x92P\x90P_\x80[\x88Q\x81\x10\x15a\x11IW\x82\x81\x81Q\x81\x10a\x10\x90Wa\x10\x90a8\x95V[` \x02` \x01\x01Qa\x10\xF4W_\x84\x82\x81Q\x81\x10a\x10\xAFWa\x10\xAFa8\x95V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\x01\x85\x82\x81Q\x81\x10a\x10\xE3Wa\x10\xE3a8\x95V[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R[_a\x112\x89\x83\x81Q\x81\x10a\x11\nWa\x11\na8\x95V[` \x02` \x01\x01Q\x89\x87\x85\x81Q\x81\x10a\x11%Wa\x11%a8\x95V[` \x02` \x01\x01Qa\x1EYV[\x90Pa\x11>\x81\x84a9\xC7V[\x92PP`\x01\x01a\x10uV[Pa\x11T\x86\x82a \xD2V[P\x92\x93PPPP[\x93\x92PPPV[a\x11ka\x1E\x10V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x11\x9BW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xA5\x83\x82a%\xE9V[a\x11\xAF\x83\x83a(\xDAV[a\x11\xB9\x83_a)BV[PP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12dWa\x12da1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x13\xE6W_\x85\x85\x83\x81\x81\x10a\x12\xADWa\x12\xADa8\x95V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x12\xC2\x90P\x81a\x1C\x80V[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x12\xEAWa\x12\xEAa8\x95V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13\x1AW`@Qc\xCCdes`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x13\xDBW`\xFF\x83\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x13\\\x84\x86a8\xD6V[a\x13f\x91\x90a8\xD6V[\x81T\x81\x10a\x13vWa\x13va8\x95V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x13\xD3W`\x01a\x13\x98\x82\x84a8\xD6V[a\x13\xA2\x91\x90a8\xD6V[\x85\x85\x81Q\x81\x10a\x13\xB4Wa\x13\xB4a8\x95V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x13\xDBV[`\x01\x01a\x13.V[PPP`\x01\x01a\x12\x92V[P\x94\x93PPPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x14\x08W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14gWa\x14ga8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x14\xEFWa\x14\xEFa8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15lWa\x15la8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x15\xC5a\x1D]V[\x81a\x15\xCF\x81a\x1C\x80V[a\x15\xD9\x83\x83a(\xDAV[PPPV[a\x15\xE6a\x1E\x10V[_[\x81\x81\x10\x15a\x16;W_\x83\x83\x83\x81\x81\x10a\x16\x03Wa\x16\x03a8\x95V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x16\x18\x90P\x81a\x1C\x80V[_a\x16$\x86\x83_a\x1EYV[\x90Pa\x160\x82\x82a \xD2V[PPP`\x01\x01a\x15\xE8V[PPPPV[a\x16Ia\x1D]V[\x81a\x16S\x81a\x1C\x80V[a\x16]\x83\x83a%\xE9V[_\x82Q\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE3\x91\x90a8\xFDV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xFF\x89\x16` \x83\x01R\x91Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x92\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91c&\r\xC7X\x91a\x17H\x91`\x04\x01a9\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17cW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x87\x91\x90a9>V[\x15a\x18\xAAW_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA5Wa\x17\xA5a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x18*W\x85\x81\x81Q\x81\x10a\x17\xEDWa\x17\xEDa8\x95V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x18\nWa\x18\na8\x95V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x17\xD3V[P`@Qc\x02\x87\xF7Q`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cP\xFE\xEA \x90a\x18{\x90\x85\x90\x8A\x90\x86\x90`\x04\x01a9]V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\x92W__\xFD[PZ\xF1\x15\x80\x15a\x18\xA4W=__>=_\xFD[PPPPP[PPPPPV[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x18\xD7Wa\x18\xD7a8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\n\xBE\x81\x85a)\xAFV[a\x195a\x1E\x10V[`\xFF\x84\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x19eW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19o\x84\x82a%\xE9V[a\x19y\x84\x84a(\xDAV[a\x19\x84\x84`\x01a)BV[a\x19\x8E\x84\x83a*&V[PPP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x81\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1A?\x91a8\xD6V[\x81T\x81\x10a\x1AOWa\x1AOa8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[_a\x1A|\x84\x84\x84a+\x83V[\x94\x93PPPPV[a\x1A\x8Ca\x1D]V[\x81a\x1A\x96\x81a\x1C\x80V[a\x15\xD9\x83\x83a*&V[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\xD0Wa\x1A\xD0a8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1B&\x81\x86a)\xAFV[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x91\x92\x91\x82\x90\x03a\x1B\x8FW\x91Pa\n\xAC\x90PV[_\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1B\xB5`\x01\x84a8\xD6V[\x81T\x81\x10a\x1B\xC5Wa\x1B\xC5a8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\n\xAC\x91PPV[PP\x92\x91PPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1CG\x85\x85\x85a+\x83V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C]Wa\x1C]a8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x90 Ta\x1C\xAFW`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x82\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81_\x81Q\x81\x10a\x1C\xEAWa\x1C\xEAa8\x95V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP__a\x1D\x16\x87\x84a\"CV[\x91P\x91P\x81_\x81Q\x81\x10a\x1D,Wa\x1D,a8\x95V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a\x1DFWa\x1DFa8\x95V[` \x02` \x01\x01Q\x94P\x94PPPP[\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xDD\x91\x90a8\xFDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x0EW`@Qc\xCE\x98\xC2K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1E\x0EW`@Qc,\x01\xB2\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x82\x03a\x1F\x1DW_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua xV[_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a\x1FC`\x01\x84a8\xD6V[\x81T\x81\x10a\x1FSWa\x1FSa8\x95V[_\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x03a\x1F\x89W_\x93PPPPa\x11\\V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a\x1F\xC1W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua vV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U_\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a \xC8\x82\x85a,\xEAV[\x96\x95PPPPPPV[`\xFF\x82\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a \xF5\x90\x84a8\xD6V[\x81T\x81\x10a!\x05Wa!\x05a8\x95V[\x90_R` _ \x01\x90P\x83_\x03a!0WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\n\xAC\x90PV[\x80T_\x90a!N\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a-\x01V[\x82T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a!\x89W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua\":V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[``\x80_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"`Wa\"`a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x89W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xA6Wa\"\xA6a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_a\"\xEB\x87`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[`\xFF\x88\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x94\x95P\x92\x93\x90\x92\x91\x84\x90\x84\x01[\x82\x82\x10\x15a#kW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a#\x1DV[PPPP\x90P```\x01\x80\x81\x11\x15a#\x85Wa#\x85a2\xEFV[`\xFF\x80\x8B\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a#\xAAWa#\xAAa2\xEFV[\x03a#\xC0Wa#\xB9\x89\x89a-.V[\x90Pa$cV[`\xFF\x89\x16_\x90\x81R`\x04` \x81\x90R`@\x91\x82\x90 \x91Qcxps;`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\xF0\xE0\xE6v\x92a$\x1F\x92\x8D\x92\x91\x01a:[V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$`\x91\x90\x81\x01\x90a:\x7FV[\x90P[_[\x88Q\x81\x10\x15a%\xD9W_[\x84\x81\x10\x15a%iW_\x84\x82\x81Q\x81\x10a$\x8BWa$\x8Ba8\x95V[` \x02` \x01\x01Q\x90P_\x84\x84\x81Q\x81\x10a$\xA8Wa$\xA8a8\x95V[` \x02` \x01\x01Q\x83\x81Q\x81\x10a$\xC1Wa$\xC1a8\x95V[` \x02` \x01\x01Q\x11\x15a%`Wg\r\xE0\xB6\xB3\xA7d\0\0\x81` \x01Q`\x01`\x01``\x1B\x03\x16\x85\x85\x81Q\x81\x10a$\xF8Wa$\xF8a8\x95V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a%\x11Wa%\x11a8\x95V[` \x02` \x01\x01Qa%#\x91\x90a;\x8BV[a%-\x91\x90a;\xA2V[\x88\x84\x81Q\x81\x10a%?Wa%?a8\x95V[` \x02` \x01\x01\x81\x81Qa%S\x91\x90a;\xC1V[`\x01`\x01``\x1B\x03\x16\x90RP[P`\x01\x01a$pV[P`\xFF\x8A\x16_\x90\x81R` \x81\x90R`@\x90 T\x86Q`\x01`\x01``\x1B\x03\x90\x91\x16\x90\x87\x90\x83\x90\x81\x10a%\x9CWa%\x9Ca8\x95V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16\x10\x15\x85\x82\x81Q\x81\x10a%\xC1Wa%\xC1a8\x95V[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a$eV[P\x93\x98\x92\x97P\x91\x95PPPPPPV[_\x81Q\x11a&\nW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a&,\x83\x83a;\xE0V[\x11\x15a&KW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x18\xAAW_[a&a\x82\x84a;\xE0V[\x81\x10\x15a&\xF0W\x84\x82\x81Q\x81\x10a&zWa&za8\x95V[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a&\xB6Wa&\xB6a8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a&\xE8W`@Qc{t4\x0B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a&WV[P_\x84\x82\x81Q\x81\x10a'\x04Wa'\x04a8\x95V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a'7W`@QcrW\x12Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x85\x16_\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\\Wa'\\a8\x95V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\xC0Wa'\xC0a8\x95V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U_\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a(6Wa(6a8\x95V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a(\x93Wa(\x93a8\x95V[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a(\xB0Wa(\xB0a8\x95V[` \x02` \x01\x01Q` \x01Q`@Qa(\xCA\x92\x91\x90a.\xF2V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a&MV[`\xFF\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\xFF\x82\x16_\x90\x81R`\x05` R`@\x90 \x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a)oWa)oa2\xEFV[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa)\xA3\x91\x90a3\x03V[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\xDFW`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a*\x05WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*\"W`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\x01`\xFF\x80\x84\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a*MWa*Ma2\xEFV[\x14a*kW`@Qc\xA3\xBE%\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c)\x81\xEBw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xEB\x91\x90a;\xF3V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a+\x17W`@Qc\x0B\xD4A\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x81\x16c\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x17\x90\x93U\x84Q\x92\x90\x91\x16\x80\x83R\x92\x82\x01R\x90\x91\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a,!W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a+\xD5`\x01\x84a8\xD6V[\x81T\x81\x10a+\xE5Wa+\xE5a8\x95V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a,\x0FWa,\x06`\x01\x82a8\xD6V[\x92PPPa\x11\\V[\x80a,\x19\x81a<\x0EV[\x91PPa+\xA1V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\\`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a<#V[__\x82\x12\x15a-$Wa-\x13\x82a<BV[a-\x1D\x90\x84a<\\V[\x90Pa\n\xACV[a-\x1D\x82\x84a;\xC1V[`\xFF\x82\x16_\x90\x81R`\x06` R`@\x81 T``\x91\x90a-T\x90c\xFF\xFF\xFF\xFF\x16Ca;\xE0V[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x11\x91\x90a8\xFDV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xFF\x89\x16` \x91\x82\x01\x81\x90R_\x90\x81R`\x04\x91\x82\x90R`@\x90\x81\x90 \x90Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra.[\x93\x92\x8A\x92\x91\x89\x91\x01a<{V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.uW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\":\x91\x90\x81\x01\x90a:\x7FV[\x805`\xFF\x81\x16\x81\x14a.\xACW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a.\xC1W__\xFD[a\x11\\\x82a.\x9CV[__`@\x83\x85\x03\x12\x15a.\xDBW__\xFD[a.\xE4\x83a.\x9CV[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xAFW__\xFD[__`@\x83\x85\x03\x12\x15a/9W__\xFD[a/B\x83a.\x9CV[\x91P` \x83\x015a/R\x81a/\x14V[\x80\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a/mW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x83W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1DVW__\xFD[_____``\x86\x88\x03\x12\x15a/\xB1W__\xFD[a/\xBA\x86a.\x9CV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xD4W__\xFD[a/\xE0\x88\x82\x89\x01a/]V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xFEW__\xFD[a0\n\x88\x82\x89\x01a/]V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a0+W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a0AW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1DVW__\xFD[____``\x85\x87\x03\x12\x15a0kW__\xFD[\x845a0v\x81a/\x14V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x97W__\xFD[a0\xA3\x87\x82\x88\x01a0\x1BV[\x95\x98\x94\x97P\x95PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a0\xE8W\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a0\xC1V[P\x93\x94\x93PPPPV[`@\x81R_a1\x04`@\x83\x01\x85a0\xAFV[\x82\x81\x03` \x84\x01Ra\":\x81\x85a0\xAFV[__`@\x83\x85\x03\x12\x15a1'W__\xFD[\x825\x91Pa17` \x84\x01a.\x9CV[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a1\xAEWa1\x98\x83\x85Qc\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[` \x93\x90\x93\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a1YV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\xEFWa1\xEFa1\xB9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a2\x1DWa2\x1Da1\xB9V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a2=Wa2=a1\xB9V[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a2XW__\xFD[a2a\x83a.\x9CV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2{W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a2\x8BW__\xFD[\x805a2\x9Ea2\x99\x82a2%V[a1\xF5V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a2\xBFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a2\xE1W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a2\xC6V[\x80\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x02\x83\x10a3#WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[_\x82`\x1F\x83\x01\x12a38W__\xFD[\x815a3Fa2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3gW__\xFD[` \x85\x01[\x83\x81\x10\x15a3\x84W\x805\x83R` \x92\x83\x01\x92\x01a3lV[P\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a3\xA0W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xB5W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a3\xC5W__\xFD[\x805a3\xD3a2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a3\xF4W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a4\x1FW\x835a4\x0E\x81a/\x14V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a3\xFBV[\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4<W__\xFD[a4H\x86\x82\x87\x01a3)V[\x92PPa4W`@\x85\x01a.\x9CV[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a1\xAEW\x83Q\x15\x15\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4yV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a.\xACW__\xFD[_\x82`\x1F\x83\x01\x12a4\xBEW__\xFD[\x815a4\xCCa2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a4\xEDW__\xFD[` \x85\x01[\x83\x81\x10\x15a3\x84W`@\x81\x88\x03\x12\x15a5\tW__\xFD[a5\x11a1\xCDV[\x815a5\x1C\x81a/\x14V[\x81Ra5*` \x83\x01a4\x99V[` \x82\x01R\x80\x84RP` \x83\x01\x92P`@\x81\x01\x90Pa4\xF2V[___``\x84\x86\x03\x12\x15a5VW__\xFD[a5_\x84a.\x9CV[\x92Pa5m` \x85\x01a4\x99V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x87W__\xFD[a5\x93\x86\x82\x87\x01a4\xAFV[\x91PP\x92P\x92P\x92V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xAFW__\xFD[___`@\x84\x86\x03\x12\x15a5\xC0W__\xFD[\x835a5\xCB\x81a5\x9DV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xE5W__\xFD[a5\xF1\x86\x82\x87\x01a0\x1BV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a1\xAEW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\x17V[___``\x84\x86\x03\x12\x15a6MW__\xFD[a6V\x84a.\x9CV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[``\x81\x01a\n\xAC\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[__`@\x83\x85\x03\x12\x15a6\xB6W__\xFD[a6\xBF\x83a.\x9CV[\x91Pa17` \x84\x01a4\x99V[___`@\x84\x86\x03\x12\x15a6\xDFW__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xE5W__\xFD[__`@\x83\x85\x03\x12\x15a7\x0CW__\xFD[a7\x15\x83a.\x9CV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7/W__\xFD[a7;\x85\x82\x86\x01a4\xAFV[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a7WW__\xFD[a7`\x84a.\x9CV[\x92P` \x84\x015a7p\x81a5\x9DV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[____`\x80\x85\x87\x03\x12\x15a7\x94W__\xFD[a7\x9D\x85a.\x9CV[\x93Pa7\xAB` \x86\x01a4\x99V[\x92P`@\x85\x015a7\xBB\x81a5\x9DV[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xD5W__\xFD[a7\xE1\x87\x82\x88\x01a4\xAFV[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a7\xFFW__\xFD[\x835\x92Pa8\x0F` \x85\x01a.\x9CV[\x91P`@\x84\x015a8\x1F\x81a5\x9DV[\x80\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a8;W__\xFD[a8D\x83a.\x9CV[\x91P` \x83\x015a/R\x81a5\x9DV[____`\x80\x85\x87\x03\x12\x15a8gW__\xFD[a8p\x85a.\x9CV[\x93P` \x85\x015a8\x80\x81a5\x9DV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a8\xB9W__\xFD[a\x11\\\x82a4\x99V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\xACWa\n\xACa8\xC2V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a9\rW__\xFD[\x81Qa\x11\\\x81a/\x14V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x81\x01a\n\xACV[_` \x82\x84\x03\x12\x15a9NW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\\W__\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\xFF\x83\x16` \x80\x83\x01\x91\x90\x91R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R_\x91\x84\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15a9\xBAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9\x93V[P\x90\x97\x96PPPPPPPV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1C\x19Wa\x1C\x19a8\xC2V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a0\xE8W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a9\xF8V[_\x81T\x80\x84R` \x84\x01\x93P\x82_R` _ _[\x82\x81\x10\x15a0\xE8W\x81T`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x95\x01\x94`\x01\x91\x82\x01\x91\x01a:4V[`@\x81R_a:m`@\x83\x01\x85a9\xE6V[\x82\x81\x03` \x84\x01Ra\":\x81\x85a:\x1FV[_` \x82\x84\x03\x12\x15a:\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a:\xA4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a:\xB4W__\xFD[\x80Qa:\xC2a2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a:\xE3W__\xFD[` \x84\x01[\x83\x81\x10\x15a;\x80W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a;\x05W__\xFD[\x85\x01`?\x81\x01\x89\x13a;\x15W__\xFD[` \x81\x01Qa;&a2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15a;IW__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15a;kW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a;PV[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa:\xE8V[P\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xACWa\n\xACa8\xC2V[_\x82a;\xBCWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\n\xACWa\n\xACa8\xC2V[\x80\x82\x01\x80\x82\x11\x15a\n\xACWa\n\xACa8\xC2V[_` \x82\x84\x03\x12\x15a<\x03W__\xFD[\x81Qa\x11\\\x81a5\x9DV[_\x81a<\x1CWa<\x1Ca8\xC2V[P_\x19\x01\x90V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x06\xE8Wa\x06\xE8a8\xC2V[_`\x01`\xFF\x1B\x82\x01a<VWa<Va8\xC2V[P_\x03\x90V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\n\xACWa\n\xACa8\xC2V[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`\xA0`@\x82\x01R_a<\xAD`\xA0\x83\x01\x86a9\xE6V[\x82\x81\x03``\x84\x01Ra<\xBF\x81\x86a:\x1FV[\x91PPc\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x18*\xD3\xDF\xE4\x08MO\x02\xAD\x98\x1A\x17q1(\x83\xD4\xFE\x91;\xEF\xB3m\x1C%\x19\x92\xD7\x84\x7F\xA8dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061021e575f3560e01c80639ab4d6ff1161012a578063c8294c56116100b4578063df5cf72311610079578063df5cf72314610615578063e086adb31461063c578063f2be94ae1461064f578063f851e19814610662578063fa28c62714610675575f5ffd5b8063c8294c56146105a2578063ca8aa7c7146105b5578063cc5a7c20146105dc578063d5eccc05146105ef578063dd9846b914610602575f5ffd5b8063b6904b78116100fa578063b6904b781461052e578063bc9a40c314610541578063bd29b8cd14610554578063c46778a514610567578063c601527d1461058f575f5ffd5b80639ab4d6ff146104815780639f3ccf65146104bb578063ac6bfb03146104ce578063adc804da146104ee575f5ffd5b80635e5a6775116101ab5780636c3fb4bf1161017b5780636c3fb4bf146103ed5780636d14a9871461040d57806375d4173a146104345780637c1723471461044757806381c0750214610461575f5ffd5b80635e5a67751461035d5780635f1f2d771461036c578063697fbd931461037f5780636b3aa72e146103ae575f5ffd5b806325504777116101f157806325504777146102b85780632cd95940146102d95780633ca5a5f5146102f95780634bd26e091461031b5780635401ed271461034a575f5ffd5b80630491b41c1461022257806308732461146102575780631f9b74e01461027857806320b66298146102a3575b5f5ffd5b610244610230366004612eb1565b60ff165f9081526001602052604090205490565b6040519081526020015b60405180910390f35b61026a610265366004612eca565b610688565b60405161024e929190612ef2565b61028b610286366004612f28565b6106cd565b6040516001600160601b03909116815260200161024e565b6102b66102b1366004612f9d565b6106ef565b005b6102cb6102c6366004613058565b61088d565b60405161024e9291906130f2565b6102ec6102e7366004613116565b610a15565b60405161024e9190613140565b610244610307366004612eb1565b60ff165f9081526003602052604090205490565b610244610329366004613116565b5f91825260026020908152604080842060ff93909316845291905290205490565b61028b610358366004613116565b610ab2565b610244670de0b6b3a764000081565b6102b661037a366004613247565b610aca565b6103a161038d366004612eb1565b60056020525f908152604090205460ff1681565b60405161024e9190613303565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161024e565b6104006103fb36600461338e565b611008565b60405161024e9190613460565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6102b6610442366004613544565b611163565b61044f602081565b60405160ff909116815260200161024e565b61047461046f3660046135ae565b611249565b60405161024e91906135fe565b6104a661048f366004612eb1565b60066020525f908152604090205463ffffffff1681565b60405163ffffffff909116815260200161024e565b6103d56104c9366004612eca565b6113ef565b6104e16104dc36600461363b565b611423565b60405161024e919061366b565b6105016104fc366004612eca565b6114b9565b6040805182516001600160a01b031681526020928301516001600160601b0316928101929092520161024e565b6104e161053c366004612eca565b611530565b6102b661054f3660046136a5565b6115bd565b6102b66105623660046136cd565b6115de565b61028b610575366004612eb1565b5f602081905290815260409020546001600160601b031681565b6102b661059d3660046136fb565b611641565b61028b6105b0366004613745565b6118b1565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6102b66105ea366004613781565b61192d565b61028b6105fd366004612eb1565b611a1f565b6104a66106103660046137ed565b611a70565b6103d57f000000000000000000000000000000000000000000000000000000000000000081565b6102b661064a36600461382a565b611a84565b61028b61065d366004613854565b611aa0565b6104e1610670366004613116565b611b33565b61028b6106833660046137ed565b611c21565b6003602052815f5260405f2081815481106106a1575f80fd5b5f918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b5f826106d881611c80565b5f6106e38585611cb2565b509250505b5092915050565b6106f7611d5d565b8461070181611c80565b83806107205760405163796cc52560e01b815260040160405180910390fd5b828114610740576040516343714afd60e01b815260040160405180910390fd5b60ff87165f908152600360205260408120905b828110156108825785858281811061076d5761076d613895565b905060200201602081019061078291906138a9565b8289898481811061079557610795613895565b90506020020135815481106107ac576107ac613895565b905f5260205f20015f0160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a8581811061081257610812613895565b905060200201358154811061082957610829613895565b5f918252602090912001546001600160a01b031688888581811061084f5761084f613895565b905060200201602081019061086491906138a9565b604051610872929190612ef2565b60405180910390a2600101610753565b505050505050505050565b606080610898611e10565b5f836001600160401b038111156108b1576108b16131b9565b6040519080825280602002602001820160405280156108da578160200160208202803683370190505b5090505f846001600160401b038111156108f6576108f66131b9565b60405190808252806020026020018201604052801561091f578160200160208202803683370190505b5090505f5b85811015610a07575f87878381811061093f5761093f613895565b919091013560f81c9150610954905081611c80565b5f5f610960838d611cb2565b91509150806109825760405163207f13e360e11b815260040160405180910390fd5b5f61098e8c8585611e59565b9050828786815181106109a3576109a3613895565b60200260200101906001600160601b031690816001600160601b0316815250506109cd84826120d2565b8686815181106109df576109df613895565b6001600160601b03909216602092830291909101909101525050600190920191506109249050565b509097909650945050505050565b5f82815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610aa5575f848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610a4d565b5050505090505b92915050565b5f5f610abe8484611b33565b60400151949350505050565b610ad2611d5d565b81610adc81611c80565b815180610afc5760405163796cc52560e01b815260040160405180910390fd5b60ff84165f908152600360209081526040808320600490925282209091836001600160401b03811115610b3157610b316131b9565b604051908082528060200260200182016040528015610b5a578160200160208202803683370190505b5090505f5b84811015610e555783878281518110610b7a57610b7a613895565b602002602001015181548110610b9257610b92613895565b5f9182526020909120015482516001600160a01b0390911690839083908110610bbd57610bbd613895565b60200260200101906001600160a01b031690816001600160a01b0316815250508760ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f785898481518110610c1557610c15613895565b602002602001015181548110610c2d57610c2d613895565b5f91825260209182902001546040516001600160a01b0390911681520160405180910390a28760ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7585898481518110610c8a57610c8a613895565b602002602001015181548110610ca257610ca2613895565b5f91825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a283548490610ce1906001906138d6565b81548110610cf157610cf1613895565b905f5260205f200184888381518110610d0c57610d0c613895565b602002602001015181548110610d2457610d24613895565b5f91825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558354849080610d7657610d766138e9565b5f8281526020812082015f199081019190915501905582548390610d9c906001906138d6565b81548110610dac57610dac613895565b905f5260205f20015f9054906101000a90046001600160a01b031683888381518110610dda57610dda613895565b602002602001015181548110610df257610df2613895565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555082805480610e2d57610e2d6138e9565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610b5f565b505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610eb3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ed791906138fd565b6040805180820182526001600160a01b03838116825260ff8c16602083015291516304c1b8eb60e31b81529293507f00000000000000000000000000000000000000000000000000000000000000009091169163260dc75891610f3c91600401613918565b602060405180830381865afa158015610f57573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f7b919061393e565b15610ffe5760405163b66bd98960e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063b66bd98990610fd09084908c90879060040161395d565b5f604051808303815f87803b158015610fe7575f5ffd5b505af1158015610ff9573d5f5f3e3d5ffd5b505050505b5050505050505050565b6060611012611e10565b5f84516001600160401b0381111561102c5761102c6131b9565b604051908082528060200260200182016040528015611055578160200160208202803683370190505b50905061106183611c80565b5f5f61106d8588612243565b90925090505f805b88518110156111495782818151811061109057611090613895565b60200260200101516110f4575f8482815181106110af576110af613895565b60200260200101906001600160601b031690816001600160601b03168152505060018582815181106110e3576110e3613895565b911515602092830291909101909101525b5f61113289838151811061110a5761110a613895565b60200260200101518987858151811061112557611125613895565b6020026020010151611e59565b905061113e81846139c7565b925050600101611075565b5061115486826120d2565b509293505050505b9392505050565b61116b611e10565b60ff83165f908152600160205260409020541561119b576040516310cda51760e21b815260040160405180910390fd5b6111a583826125e9565b6111af83836128da565b6111b9835f612942565b505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60605f826001600160401b03811115611264576112646131b9565b60405190808252806020026020018201604052801561128d578160200160208202803683370190505b5090505f5b838110156113e6575f8585838181106112ad576112ad613895565b919091013560f81c91506112c2905081611c80565b60ff81165f908152600160205260408120805463ffffffff8a1692906112ea576112ea613895565b5f9182526020909120015463ffffffff16111561131a5760405163cc64657360e01b815260040160405180910390fd5b60ff81165f90815260016020526040812054905b818110156113db5760ff83165f90815260016020819052604090912063ffffffff8b169161135c84866138d6565b61136691906138d6565b8154811061137657611376613895565b5f9182526020909120015463ffffffff16116113d357600161139882846138d6565b6113a291906138d6565b8585815181106113b4576113b4613895565b602002602001019063ffffffff16908163ffffffff16815250506113db565b60010161132e565b505050600101611292565b50949350505050565b6004602052815f5260405f208181548110611408575f80fd5b5f918252602090912001546001600160a01b03169150829050565b604080516060810182525f80825260208083018290528284018290528582526002815283822060ff8816835290529190912080548390811061146757611467613895565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091525f808252602082015260ff83165f9081526003602052604090208054839081106114ef576114ef613895565b5f918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182525f808252602080830182905282840182905260ff86168252600190529190912080548390811061156c5761156c613895565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b6115c5611d5d565b816115cf81611c80565b6115d983836128da565b505050565b6115e6611e10565b5f5b8181101561163b575f83838381811061160357611603613895565b919091013560f81c9150611618905081611c80565b5f61162486835f611e59565b905061163082826120d2565b5050506001016115e8565b50505050565b611649611d5d565b8161165381611c80565b61165d83836125e9565b5f825190505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116e391906138fd565b6040805180820182526001600160a01b03838116825260ff8916602083015291516304c1b8eb60e31b81529293507f00000000000000000000000000000000000000000000000000000000000000009091169163260dc7589161174891600401613918565b602060405180830381865afa158015611763573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611787919061393e565b156118aa575f826001600160401b038111156117a5576117a56131b9565b6040519080825280602002602001820160405280156117ce578160200160208202803683370190505b5090505f5b8381101561182a578581815181106117ed576117ed613895565b60200260200101515f015182828151811061180a5761180a613895565b6001600160a01b03909216602092830291909101909101526001016117d3565b50604051630287f75160e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906350feea209061187b9085908a90869060040161395d565b5f604051808303815f87803b158015611892575f5ffd5b505af11580156118a4573d5f5f3e3d5ffd5b50505050505b5050505050565b60ff83165f9081526001602052604081208054829190849081106118d7576118d7613895565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610abe81856129af565b611935611e10565b60ff84165f9081526001602052604090205415611965576040516310cda51760e21b815260040160405180910390fd5b61196f84826125e9565b61197984846128da565b611984846001612942565b61198e8483612a26565b50505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff81165f908152600160208190526040822080549091611a3f916138d6565b81548110611a4f57611a4f613895565b5f91825260209091200154600160401b90046001600160601b031692915050565b5f611a7c848484612b83565b949350505050565b611a8c611d5d565b81611a9681611c80565b6115d98383612a26565b5f82815260026020908152604080832060ff881684529091528120805482919084908110611ad057611ad0613895565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611b2681866129af565b6040015195945050505050565b60408051606080820183525f80835260208084018290528385018290528682526002815284822060ff8716835281528482205485519384018652828452908301829052938201819052919291829003611b8f579150610aac9050565b5f85815260026020908152604080832060ff881684529091529020611bb56001846138d6565b81548110611bc557611bc5613895565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610aac915050565b505092915050565b5f83815260026020908152604080832060ff861684529091528120611c47858585612b83565b63ffffffff1681548110611c5d57611c5d613895565b5f91825260209091200154600160401b90046001600160601b0316949350505050565b60ff81165f90815260016020526040902054611caf57604051637310cff560e11b815260040160405180910390fd5b50565b6040805160018082528183019092525f91829182916020808301908036833701905050905083815f81518110611cea57611cea613895565b60200260200101906001600160a01b031690816001600160a01b0316815250505f5f611d168784612243565b91509150815f81518110611d2c57611d2c613895565b6020026020010151815f81518110611d4657611d46613895565b6020026020010151945094505050505b9250929050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611db9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ddd91906138fd565b6001600160a01b0316336001600160a01b031614611e0e5760405163ce98c24b60e01b815260040160405180910390fd5b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611e0e57604051632c01b20560e21b815260040160405180910390fd5b5f83815260026020908152604080832060ff861684529091528120548190808203611f1d575f86815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055612078565b5f86815260026020908152604080832060ff891684529091528120611f436001846138d6565b81548110611f5357611f53613895565b5f91825260209091200180546001600160601b03600160401b9091048116945090915085168303611f89575f935050505061115c565b805463ffffffff438116911603611fc1578054600160401b600160a01b031916600160401b6001600160601b03871602178155612076565b805467ffffffff000000001916600160201b4363ffffffff9081168281029390931784555f8a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a26120c88285612cea565b9695505050505050565b60ff82165f90815260016020819052604082208054918391906120f590846138d6565b8154811061210557612105613895565b905f5260205f20019050835f036121305754600160401b90046001600160601b03169150610aac9050565b80545f9061214e90600160401b90046001600160601b031686612d01565b825490915063ffffffff438116911603612189578154600160401b600160a01b031916600160401b6001600160601b0383160217825561223a565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff89165f90815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b6060805f83516001600160401b03811115612260576122606131b9565b604051908082528060200260200182016040528015612289578160200160208202803683370190505b5090505f84516001600160401b038111156122a6576122a66131b9565b6040519080825280602002602001820160405280156122cf578160200160208202803683370190505b5090505f6122eb8760ff165f9081526003602052604090205490565b60ff88165f90815260036020908152604080832080548251818502810185019093528083529495509293909291849084015b8282101561236b575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b03168183015282526001909201910161231d565b5050505090506060600180811115612385576123856132ef565b60ff808b165f908152600560205260409020541660018111156123aa576123aa6132ef565b036123c0576123b98989612d2e565b9050612463565b60ff89165f908152600460208190526040918290209151637870733b60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169263f0e0e6769261241f928d929101613a5b565b5f60405180830381865afa158015612439573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124609190810190613a7f565b90505b5f5b88518110156125d9575f5b84811015612569575f84828151811061248b5761248b613895565b602002602001015190505f8484815181106124a8576124a8613895565b602002602001015183815181106124c1576124c1613895565b6020026020010151111561256057670de0b6b3a764000081602001516001600160601b03168585815181106124f8576124f8613895565b6020026020010151848151811061251157612511613895565b60200260200101516125239190613b8b565b61252d9190613ba2565b88848151811061253f5761253f613895565b602002602001018181516125539190613bc1565b6001600160601b03169052505b50600101612470565b5060ff8a165f9081526020819052604090205486516001600160601b039091169087908390811061259c5761259c613895565b60200260200101516001600160601b031610158582815181106125c1576125c1613895565b91151560209283029190910190910152600101612465565b5093989297509195505050505050565b5f81511161260a5760405163796cc52560e01b815260040160405180910390fd5b805160ff83165f908152600360209081526040909120549061262c8383613be0565b111561264b576040516343714afd60e01b815260040160405180910390fd5b5f5b828110156118aa575f5b6126618284613be0565b8110156126f05784828151811061267a5761267a613895565b60200260200101515f01516001600160a01b031660035f8860ff1660ff1681526020019081526020015f2082815481106126b6576126b6613895565b5f918252602090912001546001600160a01b0316036126e857604051637b74340b60e01b815260040160405180910390fd5b600101612657565b505f84828151811061270457612704613895565b6020026020010151602001516001600160601b03161161273757604051637257125160e01b815260040160405180910390fd5b60ff85165f908152600360205260409020845185908390811061275c5761275c613895565b60209081029190910181015182546001810184555f9384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106127c0576127c0613895565b6020908102919091018101515182546001810184555f938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f54049086908490811061283657612836613895565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7585838151811061289357612893613895565b60200260200101515f01518684815181106128b0576128b0613895565b6020026020010151602001516040516128ca929190612ef2565b60405180910390a260010161264d565b60ff82165f818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60ff82165f908152600560205260409020805482919060ff19166001838181111561296f5761296f6132ef565b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516129a39190613303565b60405180910390a15050565b815f015163ffffffff168163ffffffff1610156129df57604051631391e11b60e21b815260040160405180910390fd5b602082015163ffffffff161580612a055750816020015163ffffffff168163ffffffff16105b612a2257604051631391e11b60e21b815260040160405180910390fd5b5050565b600160ff8084165f90815260056020526040902054166001811115612a4d57612a4d6132ef565b14612a6b5760405163a3be258360e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632981eb776040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ac7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aeb9190613bf3565b63ffffffff168163ffffffff161115612b1757604051630bd441b960e21b815260040160405180910390fd5b60ff82165f90815260066020908152604091829020805463ffffffff19811663ffffffff8681169182179093558451929091168083529282015290917f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7910160405180910390a1505050565b5f83815260026020908152604080832060ff86168452909152812054805b8015612c21575f86815260026020908152604080832060ff89168452909152902063ffffffff851690612bd56001846138d6565b81548110612be557612be5613895565b5f9182526020909120015463ffffffff1611612c0f57612c066001826138d6565b9250505061115c565b80612c1981613c0e565b915050612ba1565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e40160405180910390fd5b5f61115c6001600160601b03808516908416613c23565b5f5f821215612d2457612d1382613c42565b612d1d9084613c5c565b9050610aac565b612d1d8284613bc1565b60ff82165f9081526006602052604081205460609190612d549063ffffffff1643613be0565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632bab2c4a60405180604001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ded573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e1191906138fd565b6001600160a01b0316815260ff891660209182018190525f90815260049182905260409081902090516001600160e01b031960e086901b168152612e5b93928a9291899101613c7b565b5f60405180830381865afa158015612e75573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261223a9190810190613a7f565b803560ff81168114612eac575f5ffd5b919050565b5f60208284031215612ec1575f5ffd5b61115c82612e9c565b5f5f60408385031215612edb575f5ffd5b612ee483612e9c565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114611caf575f5ffd5b5f5f60408385031215612f39575f5ffd5b612f4283612e9c565b91506020830135612f5281612f14565b809150509250929050565b5f5f83601f840112612f6d575f5ffd5b5081356001600160401b03811115612f83575f5ffd5b6020830191508360208260051b8501011115611d56575f5ffd5b5f5f5f5f5f60608688031215612fb1575f5ffd5b612fba86612e9c565b945060208601356001600160401b03811115612fd4575f5ffd5b612fe088828901612f5d565b90955093505060408601356001600160401b03811115612ffe575f5ffd5b61300a88828901612f5d565b969995985093965092949392505050565b5f5f83601f84011261302b575f5ffd5b5081356001600160401b03811115613041575f5ffd5b602083019150836020828501011115611d56575f5ffd5b5f5f5f5f6060858703121561306b575f5ffd5b843561307681612f14565b93506020850135925060408501356001600160401b03811115613097575f5ffd5b6130a38782880161301b565b95989497509550505050565b5f8151808452602084019350602083015f5b828110156130e85781516001600160601b03168652602095860195909101906001016130c1565b5093949350505050565b604081525f61310460408301856130af565b828103602084015261223a81856130af565b5f5f60408385031215613127575f5ffd5b8235915061313760208401612e9c565b90509250929050565b602080825282518282018190525f918401906040840190835b818110156131ae5761319883855163ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b6020939093019260609290920191600101613159565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b03811182821017156131ef576131ef6131b9565b60405290565b604051601f8201601f191681016001600160401b038111828210171561321d5761321d6131b9565b604052919050565b5f6001600160401b0382111561323d5761323d6131b9565b5060051b60200190565b5f5f60408385031215613258575f5ffd5b61326183612e9c565b915060208301356001600160401b0381111561327b575f5ffd5b8301601f8101851361328b575f5ffd5b803561329e61329982613225565b6131f5565b8082825260208201915060208360051b8501019250878311156132bf575f5ffd5b6020840193505b828410156132e15783358252602093840193909101906132c6565b809450505050509250929050565b634e487b7160e01b5f52602160045260245ffd5b602081016002831061332357634e487b7160e01b5f52602160045260245ffd5b91905290565b5f82601f830112613338575f5ffd5b813561334661329982613225565b8082825260208201915060208360051b860101925085831115613367575f5ffd5b602085015b8381101561338457803583526020928301920161336c565b5095945050505050565b5f5f5f606084860312156133a0575f5ffd5b83356001600160401b038111156133b5575f5ffd5b8401601f810186136133c5575f5ffd5b80356133d361329982613225565b8082825260208201915060208360051b8501019250888311156133f4575f5ffd5b6020840193505b8284101561341f57833561340e81612f14565b8252602093840193909101906133fb565b955050505060208401356001600160401b0381111561343c575f5ffd5b61344886828701613329565b92505061345760408501612e9c565b90509250925092565b602080825282518282018190525f918401906040840190835b818110156131ae5783511515835260209384019390920191600101613479565b80356001600160601b0381168114612eac575f5ffd5b5f82601f8301126134be575f5ffd5b81356134cc61329982613225565b8082825260208201915060208360061b8601019250858311156134ed575f5ffd5b602085015b838110156133845760408188031215613509575f5ffd5b6135116131cd565b813561351c81612f14565b815261352a60208301613499565b6020820152808452506020830192506040810190506134f2565b5f5f5f60608486031215613556575f5ffd5b61355f84612e9c565b925061356d60208501613499565b915060408401356001600160401b03811115613587575f5ffd5b613593868287016134af565b9150509250925092565b63ffffffff81168114611caf575f5ffd5b5f5f5f604084860312156135c0575f5ffd5b83356135cb8161359d565b925060208401356001600160401b038111156135e5575f5ffd5b6135f18682870161301b565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b818110156131ae57835163ffffffff16835260209384019390920191600101613617565b5f5f5f6060848603121561364d575f5ffd5b61365684612e9c565b95602085013595506040909401359392505050565b60608101610aac828463ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b5f5f604083850312156136b6575f5ffd5b6136bf83612e9c565b915061313760208401613499565b5f5f5f604084860312156136df575f5ffd5b8335925060208401356001600160401b038111156135e5575f5ffd5b5f5f6040838503121561370c575f5ffd5b61371583612e9c565b915060208301356001600160401b0381111561372f575f5ffd5b61373b858286016134af565b9150509250929050565b5f5f5f60608486031215613757575f5ffd5b61376084612e9c565b925060208401356137708161359d565b929592945050506040919091013590565b5f5f5f5f60808587031215613794575f5ffd5b61379d85612e9c565b93506137ab60208601613499565b925060408501356137bb8161359d565b915060608501356001600160401b038111156137d5575f5ffd5b6137e1878288016134af565b91505092959194509250565b5f5f5f606084860312156137ff575f5ffd5b8335925061380f60208501612e9c565b9150604084013561381f8161359d565b809150509250925092565b5f5f6040838503121561383b575f5ffd5b61384483612e9c565b91506020830135612f528161359d565b5f5f5f5f60808587031215613867575f5ffd5b61387085612e9c565b935060208501356138808161359d565b93969395505050506040820135916060013590565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156138b9575f5ffd5b61115c82613499565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610aac57610aac6138c2565b634e487b7160e01b5f52603160045260245ffd5b5f6020828403121561390d575f5ffd5b815161115c81612f14565b81516001600160a01b0316815260208083015163ffffffff169082015260408101610aac565b5f6020828403121561394e575f5ffd5b8151801515811461115c575f5ffd5b6001600160a01b038416815260ff831660208083019190915260606040830181905283519083018190525f918401906080840190835b818110156139ba5783516001600160a01b0316835260209384019390920191600101613993565b5090979650505050505050565b8082018281125f831280158216821582161715611c1957611c196138c2565b5f8151808452602084019350602083015f5b828110156130e85781516001600160a01b03168652602095860195909101906001016139f8565b5f8154808452602084019350825f5260205f205f5b828110156130e85781546001600160a01b0316865260209095019460019182019101613a34565b604081525f613a6d60408301856139e6565b828103602084015261223a8185613a1f565b5f60208284031215613a8f575f5ffd5b81516001600160401b03811115613aa4575f5ffd5b8201601f81018413613ab4575f5ffd5b8051613ac261329982613225565b8082825260208201915060208360051b850101925086831115613ae3575f5ffd5b602084015b83811015613b805780516001600160401b03811115613b05575f5ffd5b8501603f81018913613b15575f5ffd5b6020810151613b2661329982613225565b808282526020820191506020808460051b8601010192508b831115613b49575f5ffd5b6040840193505b82841015613b6b578351825260209384019390910190613b50565b86525050602093840193919091019050613ae8565b509695505050505050565b8082028115828204841417610aac57610aac6138c2565b5f82613bbc57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160601b038181168382160190811115610aac57610aac6138c2565b80820180821115610aac57610aac6138c2565b5f60208284031215613c03575f5ffd5b815161115c8161359d565b5f81613c1c57613c1c6138c2565b505f190190565b8181035f8312801583831316838312821617156106e8576106e86138c2565b5f600160ff1b8201613c5657613c566138c2565b505f0390565b6001600160601b038281168282160390811115610aac57610aac6138c2565b84516001600160a01b0316815260208086015163ffffffff169082015260a060408201525f613cad60a08301866139e6565b8281036060840152613cbf8186613a1f565b91505063ffffffff831660808301529594505050505056fea2646970667358221220182ad3dfe4084d4f02ad981a1771312883d4fe913befb36d1c251992d7847fa864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x1EW_5`\xE0\x1C\x80c\x9A\xB4\xD6\xFF\x11a\x01*W\x80c\xC8)LV\x11a\0\xB4W\x80c\xDF\\\xF7#\x11a\0yW\x80c\xDF\\\xF7#\x14a\x06\x15W\x80c\xE0\x86\xAD\xB3\x14a\x06<W\x80c\xF2\xBE\x94\xAE\x14a\x06OW\x80c\xF8Q\xE1\x98\x14a\x06bW\x80c\xFA(\xC6'\x14a\x06uW__\xFD[\x80c\xC8)LV\x14a\x05\xA2W\x80c\xCA\x8A\xA7\xC7\x14a\x05\xB5W\x80c\xCCZ| \x14a\x05\xDCW\x80c\xD5\xEC\xCC\x05\x14a\x05\xEFW\x80c\xDD\x98F\xB9\x14a\x06\x02W__\xFD[\x80c\xB6\x90Kx\x11a\0\xFAW\x80c\xB6\x90Kx\x14a\x05.W\x80c\xBC\x9A@\xC3\x14a\x05AW\x80c\xBD)\xB8\xCD\x14a\x05TW\x80c\xC4gx\xA5\x14a\x05gW\x80c\xC6\x01R}\x14a\x05\x8FW__\xFD[\x80c\x9A\xB4\xD6\xFF\x14a\x04\x81W\x80c\x9F<\xCFe\x14a\x04\xBBW\x80c\xACk\xFB\x03\x14a\x04\xCEW\x80c\xAD\xC8\x04\xDA\x14a\x04\xEEW__\xFD[\x80c^Zgu\x11a\x01\xABW\x80cl?\xB4\xBF\x11a\x01{W\x80cl?\xB4\xBF\x14a\x03\xEDW\x80cm\x14\xA9\x87\x14a\x04\rW\x80cu\xD4\x17:\x14a\x044W\x80c|\x17#G\x14a\x04GW\x80c\x81\xC0u\x02\x14a\x04aW__\xFD[\x80c^Zgu\x14a\x03]W\x80c_\x1F-w\x14a\x03lW\x80ci\x7F\xBD\x93\x14a\x03\x7FW\x80ck:\xA7.\x14a\x03\xAEW__\xFD[\x80c%PGw\x11a\x01\xF1W\x80c%PGw\x14a\x02\xB8W\x80c,\xD9Y@\x14a\x02\xD9W\x80c<\xA5\xA5\xF5\x14a\x02\xF9W\x80cK\xD2n\t\x14a\x03\x1BW\x80cT\x01\xED'\x14a\x03JW__\xFD[\x80c\x04\x91\xB4\x1C\x14a\x02\"W\x80c\x08s$a\x14a\x02WW\x80c\x1F\x9Bt\xE0\x14a\x02xW\x80c \xB6b\x98\x14a\x02\xA3W[__\xFD[a\x02Da\x0206`\x04a.\xB1V[`\xFF\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ja\x02e6`\x04a.\xCAV[a\x06\x88V[`@Qa\x02N\x92\x91\x90a.\xF2V[a\x02\x8Ba\x02\x866`\x04a/(V[a\x06\xCDV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x02\xB6a\x02\xB16`\x04a/\x9DV[a\x06\xEFV[\0[a\x02\xCBa\x02\xC66`\x04a0XV[a\x08\x8DV[`@Qa\x02N\x92\x91\x90a0\xF2V[a\x02\xECa\x02\xE76`\x04a1\x16V[a\n\x15V[`@Qa\x02N\x91\x90a1@V[a\x02Da\x03\x076`\x04a.\xB1V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Da\x03)6`\x04a1\x16V[_\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x8Ba\x03X6`\x04a1\x16V[a\n\xB2V[a\x02Dg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xB6a\x03z6`\x04a2GV[a\n\xCAV[a\x03\xA1a\x03\x8D6`\x04a.\xB1V[`\x05` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02N\x91\x90a3\x03V[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x04\0a\x03\xFB6`\x04a3\x8EV[a\x10\x08V[`@Qa\x02N\x91\x90a4`V[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x04B6`\x04a5DV[a\x11cV[a\x04O` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02NV[a\x04ta\x04o6`\x04a5\xAEV[a\x12IV[`@Qa\x02N\x91\x90a5\xFEV[a\x04\xA6a\x04\x8F6`\x04a.\xB1V[`\x06` R_\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02NV[a\x03\xD5a\x04\xC96`\x04a.\xCAV[a\x13\xEFV[a\x04\xE1a\x04\xDC6`\x04a6;V[a\x14#V[`@Qa\x02N\x91\x90a6kV[a\x05\x01a\x04\xFC6`\x04a.\xCAV[a\x14\xB9V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02NV[a\x04\xE1a\x05<6`\x04a.\xCAV[a\x150V[a\x02\xB6a\x05O6`\x04a6\xA5V[a\x15\xBDV[a\x02\xB6a\x05b6`\x04a6\xCDV[a\x15\xDEV[a\x02\x8Ba\x05u6`\x04a.\xB1V[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\xB6a\x05\x9D6`\x04a6\xFBV[a\x16AV[a\x02\x8Ba\x05\xB06`\x04a7EV[a\x18\xB1V[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x05\xEA6`\x04a7\x81V[a\x19-V[a\x02\x8Ba\x05\xFD6`\x04a.\xB1V[a\x1A\x1FV[a\x04\xA6a\x06\x106`\x04a7\xEDV[a\x1ApV[a\x03\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x06J6`\x04a8*V[a\x1A\x84V[a\x02\x8Ba\x06]6`\x04a8TV[a\x1A\xA0V[a\x04\xE1a\x06p6`\x04a1\x16V[a\x1B3V[a\x02\x8Ba\x06\x836`\x04a7\xEDV[a\x1C!V[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x06\xA1W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[_\x82a\x06\xD8\x81a\x1C\x80V[_a\x06\xE3\x85\x85a\x1C\xB2V[P\x92PP[P\x92\x91PPV[a\x06\xF7a\x1D]V[\x84a\x07\x01\x81a\x1C\x80V[\x83\x80a\x07 W`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x07@W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x87\x16_\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\x08\x82W\x85\x85\x82\x81\x81\x10a\x07mWa\x07ma8\x95V[\x90P` \x02\x01` \x81\x01\x90a\x07\x82\x91\x90a8\xA9V[\x82\x89\x89\x84\x81\x81\x10a\x07\x95Wa\x07\x95a8\x95V[\x90P` \x02\x015\x81T\x81\x10a\x07\xACWa\x07\xACa8\x95V[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08\x12Wa\x08\x12a8\x95V[\x90P` \x02\x015\x81T\x81\x10a\x08)Wa\x08)a8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\x08OWa\x08Oa8\x95V[\x90P` \x02\x01` \x81\x01\x90a\x08d\x91\x90a8\xA9V[`@Qa\x08r\x92\x91\x90a.\xF2V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x07SV[PPPPPPPPPV[``\x80a\x08\x98a\x1E\x10V[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xB1Wa\x08\xB1a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xDAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xF6Wa\x08\xF6a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a\n\x07W_\x87\x87\x83\x81\x81\x10a\t?Wa\t?a8\x95V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\tT\x90P\x81a\x1C\x80V[__a\t`\x83\x8Da\x1C\xB2V[\x91P\x91P\x80a\t\x82W`@Qc \x7F\x13\xE3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\x8E\x8C\x85\x85a\x1EYV[\x90P\x82\x87\x86\x81Q\x81\x10a\t\xA3Wa\t\xA3a8\x95V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\t\xCD\x84\x82a \xD2V[\x86\x86\x81Q\x81\x10a\t\xDFWa\t\xDFa8\x95V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x90\x92\x01\x91Pa\t$\x90PV[P\x90\x97\x90\x96P\x94PPPPPV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\n\xA5W_\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\nMV[PPPP\x90P[\x92\x91PPV[__a\n\xBE\x84\x84a\x1B3V[`@\x01Q\x94\x93PPPPV[a\n\xD2a\x1D]V[\x81a\n\xDC\x81a\x1C\x80V[\x81Q\x80a\n\xFCW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x84\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B1Wa\x0B1a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x0EUW\x83\x87\x82\x81Q\x81\x10a\x0BzWa\x0Bza8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\x0B\x92Wa\x0B\x92a8\x95V[_\x91\x82R` \x90\x91 \x01T\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a\x0B\xBDWa\x0B\xBDa8\x95V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x87`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x85\x89\x84\x81Q\x81\x10a\x0C\x15Wa\x0C\x15a8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\x0C-Wa\x0C-a8\x95V[_\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x87`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x89\x84\x81Q\x81\x10a\x0C\x8AWa\x0C\x8Aa8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\x0C\xA2Wa\x0C\xA2a8\x95V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x83T\x84\x90a\x0C\xE1\x90`\x01\x90a8\xD6V[\x81T\x81\x10a\x0C\xF1Wa\x0C\xF1a8\x95V[\x90_R` _ \x01\x84\x88\x83\x81Q\x81\x10a\r\x0CWa\r\x0Ca8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\r$Wa\r$a8\x95V[_\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x83T\x84\x90\x80a\rvWa\rva8\xE9V[_\x82\x81R` \x81 \x82\x01_\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x82T\x83\x90a\r\x9C\x90`\x01\x90a8\xD6V[\x81T\x81\x10a\r\xACWa\r\xACa8\x95V[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x83\x81Q\x81\x10a\r\xDAWa\r\xDAa8\x95V[` \x02` \x01\x01Q\x81T\x81\x10a\r\xF2Wa\r\xF2a8\x95V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x82\x80T\x80a\x0E-Wa\x0E-a8\xE9V[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\x0B_V[P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD7\x91\x90a8\xFDV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xFF\x8C\x16` \x83\x01R\x91Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x92\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91c&\r\xC7X\x91a\x0F<\x91`\x04\x01a9\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FWW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F{\x91\x90a9>V[\x15a\x0F\xFEW`@Qc\xB6k\xD9\x89`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB6k\xD9\x89\x90a\x0F\xD0\x90\x84\x90\x8C\x90\x87\x90`\x04\x01a9]V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xE7W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xF9W=__>=_\xFD[PPPP[PPPPPPPPV[``a\x10\x12a\x1E\x10V[_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10,Wa\x10,a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10UW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90Pa\x10a\x83a\x1C\x80V[__a\x10m\x85\x88a\"CV[\x90\x92P\x90P_\x80[\x88Q\x81\x10\x15a\x11IW\x82\x81\x81Q\x81\x10a\x10\x90Wa\x10\x90a8\x95V[` \x02` \x01\x01Qa\x10\xF4W_\x84\x82\x81Q\x81\x10a\x10\xAFWa\x10\xAFa8\x95V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\x01\x85\x82\x81Q\x81\x10a\x10\xE3Wa\x10\xE3a8\x95V[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R[_a\x112\x89\x83\x81Q\x81\x10a\x11\nWa\x11\na8\x95V[` \x02` \x01\x01Q\x89\x87\x85\x81Q\x81\x10a\x11%Wa\x11%a8\x95V[` \x02` \x01\x01Qa\x1EYV[\x90Pa\x11>\x81\x84a9\xC7V[\x92PP`\x01\x01a\x10uV[Pa\x11T\x86\x82a \xD2V[P\x92\x93PPPP[\x93\x92PPPV[a\x11ka\x1E\x10V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x11\x9BW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xA5\x83\x82a%\xE9V[a\x11\xAF\x83\x83a(\xDAV[a\x11\xB9\x83_a)BV[PP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12dWa\x12da1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x13\xE6W_\x85\x85\x83\x81\x81\x10a\x12\xADWa\x12\xADa8\x95V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x12\xC2\x90P\x81a\x1C\x80V[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x12\xEAWa\x12\xEAa8\x95V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13\x1AW`@Qc\xCCdes`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x13\xDBW`\xFF\x83\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x13\\\x84\x86a8\xD6V[a\x13f\x91\x90a8\xD6V[\x81T\x81\x10a\x13vWa\x13va8\x95V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x13\xD3W`\x01a\x13\x98\x82\x84a8\xD6V[a\x13\xA2\x91\x90a8\xD6V[\x85\x85\x81Q\x81\x10a\x13\xB4Wa\x13\xB4a8\x95V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x13\xDBV[`\x01\x01a\x13.V[PPP`\x01\x01a\x12\x92V[P\x94\x93PPPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x14\x08W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14gWa\x14ga8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x14\xEFWa\x14\xEFa8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15lWa\x15la8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x15\xC5a\x1D]V[\x81a\x15\xCF\x81a\x1C\x80V[a\x15\xD9\x83\x83a(\xDAV[PPPV[a\x15\xE6a\x1E\x10V[_[\x81\x81\x10\x15a\x16;W_\x83\x83\x83\x81\x81\x10a\x16\x03Wa\x16\x03a8\x95V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x16\x18\x90P\x81a\x1C\x80V[_a\x16$\x86\x83_a\x1EYV[\x90Pa\x160\x82\x82a \xD2V[PPP`\x01\x01a\x15\xE8V[PPPPV[a\x16Ia\x1D]V[\x81a\x16S\x81a\x1C\x80V[a\x16]\x83\x83a%\xE9V[_\x82Q\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE3\x91\x90a8\xFDV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xFF\x89\x16` \x83\x01R\x91Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x92\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91c&\r\xC7X\x91a\x17H\x91`\x04\x01a9\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17cW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x87\x91\x90a9>V[\x15a\x18\xAAW_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA5Wa\x17\xA5a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x18*W\x85\x81\x81Q\x81\x10a\x17\xEDWa\x17\xEDa8\x95V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x18\nWa\x18\na8\x95V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x17\xD3V[P`@Qc\x02\x87\xF7Q`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cP\xFE\xEA \x90a\x18{\x90\x85\x90\x8A\x90\x86\x90`\x04\x01a9]V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\x92W__\xFD[PZ\xF1\x15\x80\x15a\x18\xA4W=__>=_\xFD[PPPPP[PPPPPV[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x18\xD7Wa\x18\xD7a8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\n\xBE\x81\x85a)\xAFV[a\x195a\x1E\x10V[`\xFF\x84\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x19eW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19o\x84\x82a%\xE9V[a\x19y\x84\x84a(\xDAV[a\x19\x84\x84`\x01a)BV[a\x19\x8E\x84\x83a*&V[PPP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x81\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1A?\x91a8\xD6V[\x81T\x81\x10a\x1AOWa\x1AOa8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[_a\x1A|\x84\x84\x84a+\x83V[\x94\x93PPPPV[a\x1A\x8Ca\x1D]V[\x81a\x1A\x96\x81a\x1C\x80V[a\x15\xD9\x83\x83a*&V[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\xD0Wa\x1A\xD0a8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1B&\x81\x86a)\xAFV[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x91\x92\x91\x82\x90\x03a\x1B\x8FW\x91Pa\n\xAC\x90PV[_\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1B\xB5`\x01\x84a8\xD6V[\x81T\x81\x10a\x1B\xC5Wa\x1B\xC5a8\x95V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\n\xAC\x91PPV[PP\x92\x91PPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1CG\x85\x85\x85a+\x83V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C]Wa\x1C]a8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x90 Ta\x1C\xAFW`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x82\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81_\x81Q\x81\x10a\x1C\xEAWa\x1C\xEAa8\x95V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP__a\x1D\x16\x87\x84a\"CV[\x91P\x91P\x81_\x81Q\x81\x10a\x1D,Wa\x1D,a8\x95V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a\x1DFWa\x1DFa8\x95V[` \x02` \x01\x01Q\x94P\x94PPPP[\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xDD\x91\x90a8\xFDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x0EW`@Qc\xCE\x98\xC2K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1E\x0EW`@Qc,\x01\xB2\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x82\x03a\x1F\x1DW_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua xV[_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a\x1FC`\x01\x84a8\xD6V[\x81T\x81\x10a\x1FSWa\x1FSa8\x95V[_\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x03a\x1F\x89W_\x93PPPPa\x11\\V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a\x1F\xC1W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua vV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U_\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a \xC8\x82\x85a,\xEAV[\x96\x95PPPPPPV[`\xFF\x82\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a \xF5\x90\x84a8\xD6V[\x81T\x81\x10a!\x05Wa!\x05a8\x95V[\x90_R` _ \x01\x90P\x83_\x03a!0WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\n\xAC\x90PV[\x80T_\x90a!N\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a-\x01V[\x82T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a!\x89W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua\":V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[``\x80_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"`Wa\"`a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x89W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xA6Wa\"\xA6a1\xB9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_a\"\xEB\x87`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[`\xFF\x88\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x94\x95P\x92\x93\x90\x92\x91\x84\x90\x84\x01[\x82\x82\x10\x15a#kW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a#\x1DV[PPPP\x90P```\x01\x80\x81\x11\x15a#\x85Wa#\x85a2\xEFV[`\xFF\x80\x8B\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a#\xAAWa#\xAAa2\xEFV[\x03a#\xC0Wa#\xB9\x89\x89a-.V[\x90Pa$cV[`\xFF\x89\x16_\x90\x81R`\x04` \x81\x90R`@\x91\x82\x90 \x91Qcxps;`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\xF0\xE0\xE6v\x92a$\x1F\x92\x8D\x92\x91\x01a:[V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$`\x91\x90\x81\x01\x90a:\x7FV[\x90P[_[\x88Q\x81\x10\x15a%\xD9W_[\x84\x81\x10\x15a%iW_\x84\x82\x81Q\x81\x10a$\x8BWa$\x8Ba8\x95V[` \x02` \x01\x01Q\x90P_\x84\x84\x81Q\x81\x10a$\xA8Wa$\xA8a8\x95V[` \x02` \x01\x01Q\x83\x81Q\x81\x10a$\xC1Wa$\xC1a8\x95V[` \x02` \x01\x01Q\x11\x15a%`Wg\r\xE0\xB6\xB3\xA7d\0\0\x81` \x01Q`\x01`\x01``\x1B\x03\x16\x85\x85\x81Q\x81\x10a$\xF8Wa$\xF8a8\x95V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a%\x11Wa%\x11a8\x95V[` \x02` \x01\x01Qa%#\x91\x90a;\x8BV[a%-\x91\x90a;\xA2V[\x88\x84\x81Q\x81\x10a%?Wa%?a8\x95V[` \x02` \x01\x01\x81\x81Qa%S\x91\x90a;\xC1V[`\x01`\x01``\x1B\x03\x16\x90RP[P`\x01\x01a$pV[P`\xFF\x8A\x16_\x90\x81R` \x81\x90R`@\x90 T\x86Q`\x01`\x01``\x1B\x03\x90\x91\x16\x90\x87\x90\x83\x90\x81\x10a%\x9CWa%\x9Ca8\x95V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16\x10\x15\x85\x82\x81Q\x81\x10a%\xC1Wa%\xC1a8\x95V[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a$eV[P\x93\x98\x92\x97P\x91\x95PPPPPPV[_\x81Q\x11a&\nW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a&,\x83\x83a;\xE0V[\x11\x15a&KW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x18\xAAW_[a&a\x82\x84a;\xE0V[\x81\x10\x15a&\xF0W\x84\x82\x81Q\x81\x10a&zWa&za8\x95V[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a&\xB6Wa&\xB6a8\x95V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a&\xE8W`@Qc{t4\x0B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a&WV[P_\x84\x82\x81Q\x81\x10a'\x04Wa'\x04a8\x95V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a'7W`@QcrW\x12Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x85\x16_\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\\Wa'\\a8\x95V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\xC0Wa'\xC0a8\x95V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U_\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a(6Wa(6a8\x95V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a(\x93Wa(\x93a8\x95V[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a(\xB0Wa(\xB0a8\x95V[` \x02` \x01\x01Q` \x01Q`@Qa(\xCA\x92\x91\x90a.\xF2V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a&MV[`\xFF\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\xFF\x82\x16_\x90\x81R`\x05` R`@\x90 \x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a)oWa)oa2\xEFV[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa)\xA3\x91\x90a3\x03V[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\xDFW`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a*\x05WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*\"W`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\x01`\xFF\x80\x84\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a*MWa*Ma2\xEFV[\x14a*kW`@Qc\xA3\xBE%\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c)\x81\xEBw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xEB\x91\x90a;\xF3V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a+\x17W`@Qc\x0B\xD4A\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x81\x16c\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x17\x90\x93U\x84Q\x92\x90\x91\x16\x80\x83R\x92\x82\x01R\x90\x91\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a,!W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a+\xD5`\x01\x84a8\xD6V[\x81T\x81\x10a+\xE5Wa+\xE5a8\x95V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a,\x0FWa,\x06`\x01\x82a8\xD6V[\x92PPPa\x11\\V[\x80a,\x19\x81a<\x0EV[\x91PPa+\xA1V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\\`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a<#V[__\x82\x12\x15a-$Wa-\x13\x82a<BV[a-\x1D\x90\x84a<\\V[\x90Pa\n\xACV[a-\x1D\x82\x84a;\xC1V[`\xFF\x82\x16_\x90\x81R`\x06` R`@\x81 T``\x91\x90a-T\x90c\xFF\xFF\xFF\xFF\x16Ca;\xE0V[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x11\x91\x90a8\xFDV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xFF\x89\x16` \x91\x82\x01\x81\x90R_\x90\x81R`\x04\x91\x82\x90R`@\x90\x81\x90 \x90Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra.[\x93\x92\x8A\x92\x91\x89\x91\x01a<{V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.uW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\":\x91\x90\x81\x01\x90a:\x7FV[\x805`\xFF\x81\x16\x81\x14a.\xACW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a.\xC1W__\xFD[a\x11\\\x82a.\x9CV[__`@\x83\x85\x03\x12\x15a.\xDBW__\xFD[a.\xE4\x83a.\x9CV[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xAFW__\xFD[__`@\x83\x85\x03\x12\x15a/9W__\xFD[a/B\x83a.\x9CV[\x91P` \x83\x015a/R\x81a/\x14V[\x80\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a/mW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x83W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1DVW__\xFD[_____``\x86\x88\x03\x12\x15a/\xB1W__\xFD[a/\xBA\x86a.\x9CV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xD4W__\xFD[a/\xE0\x88\x82\x89\x01a/]V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xFEW__\xFD[a0\n\x88\x82\x89\x01a/]V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a0+W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a0AW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1DVW__\xFD[____``\x85\x87\x03\x12\x15a0kW__\xFD[\x845a0v\x81a/\x14V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x97W__\xFD[a0\xA3\x87\x82\x88\x01a0\x1BV[\x95\x98\x94\x97P\x95PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a0\xE8W\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a0\xC1V[P\x93\x94\x93PPPPV[`@\x81R_a1\x04`@\x83\x01\x85a0\xAFV[\x82\x81\x03` \x84\x01Ra\":\x81\x85a0\xAFV[__`@\x83\x85\x03\x12\x15a1'W__\xFD[\x825\x91Pa17` \x84\x01a.\x9CV[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a1\xAEWa1\x98\x83\x85Qc\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[` \x93\x90\x93\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a1YV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\xEFWa1\xEFa1\xB9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a2\x1DWa2\x1Da1\xB9V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a2=Wa2=a1\xB9V[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a2XW__\xFD[a2a\x83a.\x9CV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2{W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a2\x8BW__\xFD[\x805a2\x9Ea2\x99\x82a2%V[a1\xF5V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a2\xBFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a2\xE1W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a2\xC6V[\x80\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x02\x83\x10a3#WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[_\x82`\x1F\x83\x01\x12a38W__\xFD[\x815a3Fa2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3gW__\xFD[` \x85\x01[\x83\x81\x10\x15a3\x84W\x805\x83R` \x92\x83\x01\x92\x01a3lV[P\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a3\xA0W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xB5W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a3\xC5W__\xFD[\x805a3\xD3a2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a3\xF4W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a4\x1FW\x835a4\x0E\x81a/\x14V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a3\xFBV[\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4<W__\xFD[a4H\x86\x82\x87\x01a3)V[\x92PPa4W`@\x85\x01a.\x9CV[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a1\xAEW\x83Q\x15\x15\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4yV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a.\xACW__\xFD[_\x82`\x1F\x83\x01\x12a4\xBEW__\xFD[\x815a4\xCCa2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a4\xEDW__\xFD[` \x85\x01[\x83\x81\x10\x15a3\x84W`@\x81\x88\x03\x12\x15a5\tW__\xFD[a5\x11a1\xCDV[\x815a5\x1C\x81a/\x14V[\x81Ra5*` \x83\x01a4\x99V[` \x82\x01R\x80\x84RP` \x83\x01\x92P`@\x81\x01\x90Pa4\xF2V[___``\x84\x86\x03\x12\x15a5VW__\xFD[a5_\x84a.\x9CV[\x92Pa5m` \x85\x01a4\x99V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x87W__\xFD[a5\x93\x86\x82\x87\x01a4\xAFV[\x91PP\x92P\x92P\x92V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xAFW__\xFD[___`@\x84\x86\x03\x12\x15a5\xC0W__\xFD[\x835a5\xCB\x81a5\x9DV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xE5W__\xFD[a5\xF1\x86\x82\x87\x01a0\x1BV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a1\xAEW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\x17V[___``\x84\x86\x03\x12\x15a6MW__\xFD[a6V\x84a.\x9CV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[``\x81\x01a\n\xAC\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[__`@\x83\x85\x03\x12\x15a6\xB6W__\xFD[a6\xBF\x83a.\x9CV[\x91Pa17` \x84\x01a4\x99V[___`@\x84\x86\x03\x12\x15a6\xDFW__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xE5W__\xFD[__`@\x83\x85\x03\x12\x15a7\x0CW__\xFD[a7\x15\x83a.\x9CV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7/W__\xFD[a7;\x85\x82\x86\x01a4\xAFV[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a7WW__\xFD[a7`\x84a.\x9CV[\x92P` \x84\x015a7p\x81a5\x9DV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[____`\x80\x85\x87\x03\x12\x15a7\x94W__\xFD[a7\x9D\x85a.\x9CV[\x93Pa7\xAB` \x86\x01a4\x99V[\x92P`@\x85\x015a7\xBB\x81a5\x9DV[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xD5W__\xFD[a7\xE1\x87\x82\x88\x01a4\xAFV[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a7\xFFW__\xFD[\x835\x92Pa8\x0F` \x85\x01a.\x9CV[\x91P`@\x84\x015a8\x1F\x81a5\x9DV[\x80\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a8;W__\xFD[a8D\x83a.\x9CV[\x91P` \x83\x015a/R\x81a5\x9DV[____`\x80\x85\x87\x03\x12\x15a8gW__\xFD[a8p\x85a.\x9CV[\x93P` \x85\x015a8\x80\x81a5\x9DV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a8\xB9W__\xFD[a\x11\\\x82a4\x99V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\xACWa\n\xACa8\xC2V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a9\rW__\xFD[\x81Qa\x11\\\x81a/\x14V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x81\x01a\n\xACV[_` \x82\x84\x03\x12\x15a9NW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\\W__\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\xFF\x83\x16` \x80\x83\x01\x91\x90\x91R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R_\x91\x84\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15a9\xBAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9\x93V[P\x90\x97\x96PPPPPPPV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1C\x19Wa\x1C\x19a8\xC2V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a0\xE8W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a9\xF8V[_\x81T\x80\x84R` \x84\x01\x93P\x82_R` _ _[\x82\x81\x10\x15a0\xE8W\x81T`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x95\x01\x94`\x01\x91\x82\x01\x91\x01a:4V[`@\x81R_a:m`@\x83\x01\x85a9\xE6V[\x82\x81\x03` \x84\x01Ra\":\x81\x85a:\x1FV[_` \x82\x84\x03\x12\x15a:\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a:\xA4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a:\xB4W__\xFD[\x80Qa:\xC2a2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a:\xE3W__\xFD[` \x84\x01[\x83\x81\x10\x15a;\x80W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a;\x05W__\xFD[\x85\x01`?\x81\x01\x89\x13a;\x15W__\xFD[` \x81\x01Qa;&a2\x99\x82a2%V[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15a;IW__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15a;kW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a;PV[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa:\xE8V[P\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xACWa\n\xACa8\xC2V[_\x82a;\xBCWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\n\xACWa\n\xACa8\xC2V[\x80\x82\x01\x80\x82\x11\x15a\n\xACWa\n\xACa8\xC2V[_` \x82\x84\x03\x12\x15a<\x03W__\xFD[\x81Qa\x11\\\x81a5\x9DV[_\x81a<\x1CWa<\x1Ca8\xC2V[P_\x19\x01\x90V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x06\xE8Wa\x06\xE8a8\xC2V[_`\x01`\xFF\x1B\x82\x01a<VWa<Va8\xC2V[P_\x03\x90V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\n\xACWa\n\xACa8\xC2V[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`\xA0`@\x82\x01R_a<\xAD`\xA0\x83\x01\x86a9\xE6V[\x82\x81\x03``\x84\x01Ra<\xBF\x81\x86a:\x1FV[\x91PPc\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x18*\xD3\xDF\xE4\x08MO\x02\xAD\x98\x1A\x17q1(\x83\xD4\xFE\x91;\xEF\xB3m\x1C%\x19\x92\xD7\x84\x7F\xA8dsolcC\0\x08\x1B\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BelowMinimumStakeRequirement()` and selector `0x40fe27c6`.
```solidity
error BelowMinimumStakeRequirement();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BelowMinimumStakeRequirement;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BelowMinimumStakeRequirement>
        for UnderlyingRustTuple<'_> {
            fn from(value: BelowMinimumStakeRequirement) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BelowMinimumStakeRequirement {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BelowMinimumStakeRequirement {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BelowMinimumStakeRequirement()";
            const SELECTOR: [u8; 4] = [64u8, 254u8, 39u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyStakeHistory()` and selector `0xcc646573`.
```solidity
error EmptyStakeHistory();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyStakeHistory;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyStakeHistory> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyStakeHistory) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyStakeHistory {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyStakeHistory {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyStakeHistory()";
            const SELECTOR: [u8; 4] = [204u8, 100u8, 101u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
```solidity
error InputArrayLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputArrayLengthMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputArrayLengthZero()` and selector `0x796cc525`.
```solidity
error InputArrayLengthZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthZero;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputArrayLengthZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthZero()";
            const SELECTOR: [u8; 4] = [121u8, 108u8, 197u8, 37u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputDuplicateStrategy()` and selector `0x7b74340b`.
```solidity
error InputDuplicateStrategy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputDuplicateStrategy;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputDuplicateStrategy> for UnderlyingRustTuple<'_> {
            fn from(value: InputDuplicateStrategy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputDuplicateStrategy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputDuplicateStrategy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputDuplicateStrategy()";
            const SELECTOR: [u8; 4] = [123u8, 116u8, 52u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputMultiplierZero()` and selector `0x72571251`.
```solidity
error InputMultiplierZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputMultiplierZero;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputMultiplierZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputMultiplierZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputMultiplierZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputMultiplierZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputMultiplierZero()";
            const SELECTOR: [u8; 4] = [114u8, 87u8, 18u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBlockNumber()` and selector `0x4e47846c`.
```solidity
error InvalidBlockNumber();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBlockNumber;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBlockNumber()";
            const SELECTOR: [u8; 4] = [78u8, 71u8, 132u8, 108u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `LookAheadPeriodTooLong()` and selector `0x2f5106e4`.
```solidity
error LookAheadPeriodTooLong();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LookAheadPeriodTooLong;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LookAheadPeriodTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: LookAheadPeriodTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LookAheadPeriodTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LookAheadPeriodTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LookAheadPeriodTooLong()";
            const SELECTOR: [u8; 4] = [47u8, 81u8, 6u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlySlashingRegistryCoordinator()` and selector `0xb006c814`.
```solidity
error OnlySlashingRegistryCoordinator();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySlashingRegistryCoordinator;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlySlashingRegistryCoordinator>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlySlashingRegistryCoordinator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlySlashingRegistryCoordinator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySlashingRegistryCoordinator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySlashingRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [176u8, 6u8, 200u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlySlashingRegistryCoordinatorOwner()` and selector `0xce98c24b`.
```solidity
error OnlySlashingRegistryCoordinatorOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySlashingRegistryCoordinatorOwner;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlySlashingRegistryCoordinatorOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlySlashingRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlySlashingRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySlashingRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySlashingRegistryCoordinatorOwner()";
            const SELECTOR: [u8; 4] = [206u8, 152u8, 194u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `QuorumAlreadyExists()` and selector `0x4336945c`.
```solidity
error QuorumAlreadyExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumAlreadyExists;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumAlreadyExists()";
            const SELECTOR: [u8; 4] = [67u8, 54u8, 148u8, 92u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `QuorumDoesNotExist()` and selector `0xe6219fea`.
```solidity
error QuorumDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumDoesNotExist;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumDoesNotExist()";
            const SELECTOR: [u8; 4] = [230u8, 33u8, 159u8, 234u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `QuorumNotSlashable()` and selector `0xa3be2583`.
```solidity
error QuorumNotSlashable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumNotSlashable;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumNotSlashable> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumNotSlashable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumNotSlashable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumNotSlashable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumNotSlashable()";
            const SELECTOR: [u8; 4] = [163u8, 190u8, 37u8, 131u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LookAheadPeriodChanged(uint32,uint32)` and selector `0x28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7`.
```solidity
event LookAheadPeriodChanged(uint32 oldLookAheadBlocks, uint32 newLookAheadBlocks);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LookAheadPeriodChanged {
        #[allow(missing_docs)]
        pub oldLookAheadBlocks: u32,
        #[allow(missing_docs)]
        pub newLookAheadBlocks: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LookAheadPeriodChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "LookAheadPeriodChanged(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 215u8, 53u8, 139u8, 121u8, 240u8, 45u8, 33u8, 184u8, 183u8, 225u8,
                122u8, 239u8, 196u8, 24u8, 90u8, 100u8, 48u8, 138u8, 163u8, 116u8, 6u8,
                250u8, 91u8, 239u8, 192u8, 91u8, 145u8, 147u8, 44u8, 57u8, 199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldLookAheadBlocks: data.0,
                    newLookAheadBlocks: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldLookAheadBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.newLookAheadBlocks),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LookAheadPeriodChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LookAheadPeriodChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LookAheadPeriodChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MinimumStakeForQuorumUpdated(uint8,uint96)` and selector `0x26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf`.
```solidity
event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumStakeForQuorumUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MinimumStakeForQuorumUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "MinimumStakeForQuorumUpdated(uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                38u8, 238u8, 207u8, 242u8, 183u8, 11u8, 10u8, 113u8, 16u8, 79u8, 244u8,
                217u8, 64u8, 186u8, 113u8, 98u8, 210u8, 58u8, 149u8, 194u8, 72u8, 119u8,
                31u8, 196u8, 135u8, 167u8, 190u8, 23u8, 165u8, 150u8, 179u8, 207u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    minimumStake: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumStakeForQuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumStakeForQuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &MinimumStakeForQuorumUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorStakeUpdate(bytes32,uint8,uint96)` and selector `0x2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d`.
```solidity
event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorStakeUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorStakeUpdate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorStakeUpdate(bytes32,uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 82u8, 125u8, 82u8, 126u8, 149u8, 216u8, 254u8, 64u8, 174u8, 197u8,
                83u8, 119u8, 116u8, 59u8, 183u8, 121u8, 8u8, 125u8, 163u8, 246u8, 208u8,
                208u8, 143u8, 18u8, 227u8, 100u8, 68u8, 218u8, 98u8, 50u8, 125u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorId: topics.1,
                    quorumNumber: data.0,
                    stake: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operatorId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorStakeUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorStakeUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorStakeUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `QuorumCreated(uint8)` and selector `0x831a9c86c45bb303caf3f064be2bc2b9fd4ecf19e47c4ac02a61e75dabfe55b4`.
```solidity
event QuorumCreated(uint8 indexed quorumNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumCreated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumCreated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumCreated(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                131u8, 26u8, 156u8, 134u8, 196u8, 91u8, 179u8, 3u8, 202u8, 243u8, 240u8,
                100u8, 190u8, 43u8, 194u8, 185u8, 253u8, 78u8, 207u8, 25u8, 228u8, 124u8,
                74u8, 192u8, 42u8, 97u8, 231u8, 93u8, 171u8, 254u8, 85u8, 180u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { quorumNumber: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `StakeTypeSet(uint8)` and selector `0x7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d`.
```solidity
event StakeTypeSet(IStakeRegistryTypes.StakeType newStakeType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakeTypeSet {
        #[allow(missing_docs)]
        pub newStakeType: <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakeTypeSet {
            type DataTuple<'a> = (IStakeRegistryTypes::StakeType,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StakeTypeSet(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                124u8, 17u8, 46u8, 134u8, 60u8, 207u8, 0u8, 120u8, 98u8, 226u8, 201u8,
                226u8, 88u8, 25u8, 201u8, 51u8, 254u8, 219u8, 201u8, 53u8, 10u8, 100u8,
                67u8, 66u8, 59u8, 74u8, 133u8, 153u8, 194u8, 232u8, 165u8, 45u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newStakeType: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <IStakeRegistryTypes::StakeType as alloy_sol_types::SolType>::tokenize(
                        &self.newStakeType,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakeTypeSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakeTypeSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakeTypeSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `StrategyAddedToQuorum(uint8,address)` and selector `0x10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f5404`.
```solidity
event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyAddedToQuorum {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StrategyAddedToQuorum {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyAddedToQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                16u8, 86u8, 94u8, 86u8, 202u8, 203u8, 243u8, 46u8, 202u8, 38u8, 121u8,
                69u8, 240u8, 84u8, 254u8, 192u8, 46u8, 89u8, 117u8, 0u8, 50u8, 209u8,
                19u8, 211u8, 48u8, 33u8, 130u8, 173u8, 150u8, 127u8, 84u8, 4u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyAddedToQuorum {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyAddedToQuorum> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyAddedToQuorum) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `StrategyMultiplierUpdated(uint8,address,uint256)` and selector `0x11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75`.
```solidity
event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyMultiplierUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StrategyMultiplierUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyMultiplierUpdated(uint8,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                17u8, 165u8, 100u8, 19u8, 34u8, 218u8, 29u8, 255u8, 86u8, 164u8, 182u8,
                110u8, 170u8, 195u8, 31u8, 250u8, 70u8, 82u8, 149u8, 236u8, 233u8, 7u8,
                205u8, 22u8, 52u8, 55u8, 121u8, 59u8, 77u8, 0u8, 154u8, 117u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
                    multiplier: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyMultiplierUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyMultiplierUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyMultiplierUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `StrategyRemovedFromQuorum(uint8,address)` and selector `0x31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f7`.
```solidity
event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyRemovedFromQuorum {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StrategyRemovedFromQuorum {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyRemovedFromQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8, 250u8, 46u8, 44u8, 210u8, 128u8, 201u8, 55u8, 94u8, 19u8, 255u8,
                207u8, 61u8, 129u8, 226u8, 55u8, 129u8, 0u8, 24u8, 110u8, 64u8, 88u8,
                248u8, 211u8, 221u8, 182u8, 144u8, 184u8, 45u8, 205u8, 49u8, 247u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyRemovedFromQuorum {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyRemovedFromQuorum> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyRemovedFromQuorum,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _slashingRegistryCoordinator, address _delegationManager, address _avsDirectory, address _allocationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _delegationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avsDirectory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._slashingRegistryCoordinator,
                        value._delegationManager,
                        value._avsDirectory,
                        value._allocationManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _slashingRegistryCoordinator: tuple.0,
                        _delegationManager: tuple.1,
                        _avsDirectory: tuple.2,
                        _allocationManager: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slashingRegistryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`.
```solidity
function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WEIGHING_FUNCTION_LENGTHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_WEIGHING_FUNCTION_LENGTH()`](MAX_WEIGHING_FUNCTION_LENGTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WEIGHING_FUNCTION_LENGTHReturn {
        #[allow(missing_docs)]
        pub _0: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_WEIGHING_FUNCTION_LENGTHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_WEIGHING_FUNCTION_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_WEIGHING_FUNCTION_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_WEIGHING_FUNCTION_LENGTH()";
            const SELECTOR: [u8; 4] = [124u8, 23u8, 35u8, 71u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MAX_WEIGHING_FUNCTION_LENGTHReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: MAX_WEIGHING_FUNCTION_LENGTHReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`.
```solidity
function WEIGHTING_DIVISOR() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WEIGHTING_DIVISORCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`WEIGHTING_DIVISOR()`](WEIGHTING_DIVISORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WEIGHTING_DIVISORReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<WEIGHTING_DIVISORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for WEIGHTING_DIVISORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<WEIGHTING_DIVISORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for WEIGHTING_DIVISORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WEIGHTING_DIVISORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WEIGHTING_DIVISOR()";
            const SELECTOR: [u8; 4] = [94u8, 90u8, 103u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: WEIGHTING_DIVISORReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: WEIGHTING_DIVISORReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addStrategies(uint8,(address,uint96)[])` and selector `0xc601527d`.
```solidity
function addStrategies(uint8 quorumNumber, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`addStrategies(uint8,(address,uint96)[])`](addStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesCall) -> Self {
                    (value.quorumNumber, value._strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _strategyParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addStrategiesReturn {
            fn _tokenize(
                &self,
            ) -> <addStrategiesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategies(uint8,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [198u8, 1u8, 82u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addStrategiesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
```solidity
function allocationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: allocationManagerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: allocationManagerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
```solidity
function avsDirectory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: avsDirectoryReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: avsDirectoryReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
```solidity
function delegation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: delegationReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: delegationReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `deregisterOperator(bytes32,bytes)` and selector `0xbd29b8cd`.
```solidity
function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(bytes32,bytes)`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumbers: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl deregisterOperatorReturn {
            fn _tokenize(
                &self,
            ) -> <deregisterOperatorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [189u8, 41u8, 184u8, 205u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                deregisterOperatorReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`.
```solidity
function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentStakeCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentStake(bytes32,uint8)`](getCurrentStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentStakeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentStake(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [84u8, 1u8, 237u8, 39u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentStakeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCurrentStakeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`.
```solidity
function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalStakeCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentTotalStake(uint8)`](getCurrentTotalStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalStakeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentTotalStakeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentTotalStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalStakeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentTotalStake(uint8)";
            const SELECTOR: [u8; 4] = [213u8, 236u8, 204u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentTotalStakeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCurrentTotalStakeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`.
```solidity
function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestStakeUpdateCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getLatestStakeUpdate(bytes32,uint8)`](getLatestStakeUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestStakeUpdateReturn {
        #[allow(missing_docs)]
        pub _0: <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestStakeUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestStakeUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestStakeUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestStakeUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestStakeUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestStakeUpdate(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [248u8, 81u8, 225u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IStakeRegistryTypes::StakeUpdate as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getLatestStakeUpdateReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getLatestStakeUpdateReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`.
```solidity
function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getStakeAtBlockNumber(bytes32,uint8,uint32)`](getStakeAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeAtBlockNumber(bytes32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [250u8, 40u8, 198u8, 39u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getStakeAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getStakeAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`.
```solidity
function getStakeAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, bytes32 operatorId, uint256 index) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberAndIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)`](getStakeAtBlockNumberAndIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberAndIndexReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexCall) -> Self {
                    (
                        value.quorumNumber,
                        value.blockNumber,
                        value.operatorId,
                        value.index,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberAndIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                        operatorId: tuple.2,
                        index: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberAndIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeAtBlockNumberAndIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [242u8, 190u8, 148u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getStakeAtBlockNumberAndIndexReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getStakeAtBlockNumberAndIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`.
```solidity
function getStakeHistory(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getStakeHistory(bytes32,uint8)`](getStakeHistoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeHistoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StakeUpdate>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeHistoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeHistoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeHistoryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StakeUpdate>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeHistory(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [44u8, 217u8, 89u8, 64u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StakeUpdate,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getStakeHistoryReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getStakeHistoryReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`.
```solidity
function getStakeHistoryLength(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryLengthCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getStakeHistoryLength(bytes32,uint8)`](getStakeHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryLengthReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeHistoryLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeHistoryLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeHistoryLengthCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeHistoryLength(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [75u8, 210u8, 110u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getStakeHistoryLengthReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getStakeHistoryLengthReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`.
```solidity
function getStakeUpdateAtIndex(uint8 quorumNumber, bytes32 operatorId, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateAtIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getStakeUpdateAtIndex(uint8,bytes32,uint256)`](getStakeUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateAtIndexReturn {
        #[allow(missing_docs)]
        pub _0: <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorId: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeUpdateAtIndex(uint8,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [172u8, 107u8, 251u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IStakeRegistryTypes::StakeUpdate as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getStakeUpdateAtIndexReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getStakeUpdateAtIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`.
```solidity
function getStakeUpdateIndexAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateIndexAtBlockNumberCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)`](getStakeUpdateIndexAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateIndexAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateIndexAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateIndexAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeUpdateIndexAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [221u8, 152u8, 70u8, 185u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getStakeUpdateIndexAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getStakeUpdateIndexAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`.
```solidity
function getTotalStakeAtBlockNumberFromIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeAtBlockNumberFromIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)`](getTotalStakeAtBlockNumberFromIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeAtBlockNumberFromIndexReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                u32,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexCall) -> Self {
                    (value.quorumNumber, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeAtBlockNumberFromIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeAtBlockNumberFromIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeAtBlockNumberFromIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)";
            const SELECTOR: [u8; 4] = [200u8, 41u8, 76u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalStakeAtBlockNumberFromIndexReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTotalStakeAtBlockNumberFromIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`.
```solidity
function getTotalStakeHistoryLength(uint8 quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeHistoryLengthCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTotalStakeHistoryLength(uint8)`](getTotalStakeHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeHistoryLengthReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeHistoryLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeHistoryLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeHistoryLength(uint8)";
            const SELECTOR: [u8; 4] = [4u8, 145u8, 180u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalStakeHistoryLengthReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTotalStakeHistoryLengthReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`.
```solidity
function getTotalStakeIndicesAtBlockNumber(uint32 blockNumber, bytes memory quorumNumbers) external view returns (uint32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeIndicesAtBlockNumberCall {
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTotalStakeIndicesAtBlockNumber(uint32,bytes)`](getTotalStakeIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeIndicesAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<u32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        quorumNumbers: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<u32>;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeIndicesAtBlockNumber(uint32,bytes)";
            const SELECTOR: [u8; 4] = [129u8, 192u8, 117u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalStakeIndicesAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTotalStakeIndicesAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`.
```solidity
function getTotalStakeUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeUpdateAtIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTotalStakeUpdateAtIndex(uint8,uint256)`](getTotalStakeUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeUpdateAtIndexReturn {
        #[allow(missing_docs)]
        pub _0: <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeUpdateAtIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [182u8, 144u8, 75u8, 120u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IStakeRegistryTypes::StakeUpdate as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalStakeUpdateAtIndexReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTotalStakeUpdateAtIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])` and selector `0x75d4173a`.
```solidity
function initializeDelegatedStakeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeDelegatedStakeQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])`](initializeDelegatedStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeDelegatedStakeQuorumReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeDelegatedStakeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake, value._strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeDelegatedStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                        _strategyParams: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeDelegatedStakeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeDelegatedStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeDelegatedStakeQuorumReturn {
            fn _tokenize(
                &self,
            ) -> <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeDelegatedStakeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [117u8, 212u8, 23u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeDelegatedStakeQuorumReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])` and selector `0xcc5a7c20`.
```solidity
function initializeSlashableStakeQuorum(uint8 quorumNumber, uint96 minimumStake, uint32 lookAheadPeriod, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeSlashableStakeQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub lookAheadPeriod: u32,
        #[allow(missing_docs)]
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])`](initializeSlashableStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeSlashableStakeQuorumReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
                u32,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeSlashableStakeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumCall) -> Self {
                    (
                        value.quorumNumber,
                        value.minimumStake,
                        value.lookAheadPeriod,
                        value._strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeSlashableStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                        lookAheadPeriod: tuple.2,
                        _strategyParams: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeSlashableStakeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeSlashableStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeSlashableStakeQuorumReturn {
            fn _tokenize(
                &self,
            ) -> <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeSlashableStakeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [204u8, 90u8, 124u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeSlashableStakeQuorumReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`.
```solidity
function minimumStakeForQuorum(uint8) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumStakeForQuorumCall(pub u8);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`minimumStakeForQuorum(uint8)`](minimumStakeForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumStakeForQuorumReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumStakeForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumStakeForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumStakeForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumStakeForQuorum(uint8)";
            const SELECTOR: [u8; 4] = [196u8, 103u8, 120u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: minimumStakeForQuorumReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: minimumStakeForQuorumReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`.
```solidity
function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategyIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub newMultipliers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
    }
    ///Container type for the return parameters of the [`modifyStrategyParams(uint8,uint256[],uint96[])`](modifyStrategyParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U96,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyStrategyParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsCall) -> Self {
                    (value.quorumNumber, value.strategyIndices, value.newMultipliers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyStrategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        strategyIndices: tuple.1,
                        newMultipliers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyStrategyParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyStrategyParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl modifyStrategyParamsReturn {
            fn _tokenize(
                &self,
            ) -> <modifyStrategyParamsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyStrategyParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyStrategyParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyStrategyParams(uint8,uint256[],uint96[])";
            const SELECTOR: [u8; 4] = [32u8, 182u8, 98u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMultipliers),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                modifyStrategyParamsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`.
```solidity
function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registerOperator(address,bytes32,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorId: tuple.1,
                        quorumNumbers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U96,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U96,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl registerOperatorReturn {
            fn _tokenize(
                &self,
            ) -> <registerOperatorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [37u8, 80u8, 71u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                registerOperatorReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
```solidity
function registryCoordinator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registryCoordinator()";
            const SELECTOR: [u8; 4] = [109u8, 20u8, 169u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: registryCoordinatorReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: registryCoordinatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`.
```solidity
function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub indicesToRemove: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`removeStrategies(uint8,uint256[])`](removeStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesCall) -> Self {
                    (value.quorumNumber, value.indicesToRemove)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        indicesToRemove: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeStrategiesReturn {
            fn _tokenize(
                &self,
            ) -> <removeStrategiesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategies(uint8,uint256[])";
            const SELECTOR: [u8; 4] = [95u8, 31u8, 45u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.indicesToRemove),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeStrategiesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setMinimumStakeForQuorum(uint8,uint96)` and selector `0xbc9a40c3`.
```solidity
function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakeForQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
    }
    ///Container type for the return parameters of the [`setMinimumStakeForQuorum(uint8,uint96)`](setMinimumStakeForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakeForQuorumReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinimumStakeForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMinimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinimumStakeForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMinimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setMinimumStakeForQuorumReturn {
            fn _tokenize(
                &self,
            ) -> <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMinimumStakeForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinimumStakeForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMinimumStakeForQuorum(uint8,uint96)";
            const SELECTOR: [u8; 4] = [188u8, 154u8, 64u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setMinimumStakeForQuorumReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setSlashableStakeLookahead(uint8,uint32)` and selector `0xe086adb3`.
```solidity
function setSlashableStakeLookahead(uint8 quorumNumber, uint32 _lookAheadBlocks) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashableStakeLookaheadCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub _lookAheadBlocks: u32,
    }
    ///Container type for the return parameters of the [`setSlashableStakeLookahead(uint8,uint32)`](setSlashableStakeLookaheadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashableStakeLookaheadReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setSlashableStakeLookaheadCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadCall) -> Self {
                    (value.quorumNumber, value._lookAheadBlocks)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSlashableStakeLookaheadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _lookAheadBlocks: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setSlashableStakeLookaheadReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSlashableStakeLookaheadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setSlashableStakeLookaheadReturn {
            fn _tokenize(
                &self,
            ) -> <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSlashableStakeLookaheadCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSlashableStakeLookaheadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSlashableStakeLookahead(uint8,uint32)";
            const SELECTOR: [u8; 4] = [224u8, 134u8, 173u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._lookAheadBlocks),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setSlashableStakeLookaheadReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `slashableStakeLookAheadPerQuorum(uint8)` and selector `0x9ab4d6ff`.
```solidity
function slashableStakeLookAheadPerQuorum(uint8 quorumNumber) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashableStakeLookAheadPerQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`slashableStakeLookAheadPerQuorum(uint8)`](slashableStakeLookAheadPerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashableStakeLookAheadPerQuorumReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashableStakeLookAheadPerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashableStakeLookAheadPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashableStakeLookAheadPerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashableStakeLookAheadPerQuorum(uint8)";
            const SELECTOR: [u8; 4] = [154u8, 180u8, 214u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: slashableStakeLookAheadPerQuorumReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: slashableStakeLookAheadPerQuorumReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `stakeTypePerQuorum(uint8)` and selector `0x697fbd93`.
```solidity
function stakeTypePerQuorum(uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTypePerQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`stakeTypePerQuorum(uint8)`](stakeTypePerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTypePerQuorumReturn {
        #[allow(missing_docs)]
        pub _0: <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeTypePerQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeTypePerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeTypePerQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeTypePerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeTypePerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeTypePerQuorum(uint8)";
            const SELECTOR: [u8; 4] = [105u8, 127u8, 189u8, 147u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IStakeRegistryTypes::StakeType as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: stakeTypePerQuorumReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: stakeTypePerQuorumReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `strategiesPerQuorum(uint8,uint256)` and selector `0x9f3ccf65`.
```solidity
function strategiesPerQuorum(uint8 quorumNumber, uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesPerQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`strategiesPerQuorum(uint8,uint256)`](strategiesPerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesPerQuorumReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategiesPerQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumCall) -> Self {
                    (value.quorumNumber, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategiesPerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategiesPerQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategiesPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategiesPerQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategiesPerQuorum(uint8,uint256)";
            const SELECTOR: [u8; 4] = [159u8, 60u8, 207u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: strategiesPerQuorumReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: strategiesPerQuorumReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`.
```solidity
function strategyParams(uint8 quorumNumber, uint256) external view returns (address strategy, uint96 multiplier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`strategyParams(uint8,uint256)`](strategyParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsReturn {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsCall) -> Self {
                    (value.quorumNumber, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsReturn) -> Self {
                    (value.strategy, value.multiplier)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategy: tuple.0,
                        multiplier: tuple.1,
                    }
                }
            }
        }
        impl strategyParamsReturn {
            fn _tokenize(
                &self,
            ) -> <strategyParamsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParams(uint8,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 115u8, 36u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                strategyParamsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`.
```solidity
function strategyParamsByIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StrategyParams memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsByIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`strategyParamsByIndex(uint8,uint256)`](strategyParamsByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsByIndexReturn {
        #[allow(missing_docs)]
        pub _0: <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsByIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StrategyParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsByIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StrategyParams,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParamsByIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [173u8, 200u8, 4u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IStakeRegistryTypes::StrategyParams as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: strategyParamsByIndexReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: strategyParamsByIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`.
```solidity
function strategyParamsLength(uint8 quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsLengthCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`strategyParamsLength(uint8)`](strategyParamsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsLengthReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParamsLength(uint8)";
            const SELECTOR: [u8; 4] = [60u8, 165u8, 165u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: strategyParamsLengthReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: strategyParamsLengthReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `updateOperatorsStake(address[],bytes32[],uint8)` and selector `0x6c3fb4bf`.
```solidity
function updateOperatorsStake(address[] memory operators, bytes32[] memory operatorIds, uint8 quorumNumber) external returns (bool[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsStakeCall {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`updateOperatorsStake(address[],bytes32[],uint8)`](updateOperatorsStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsStakeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<bool>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                u8,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsStakeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsStakeCall) -> Self {
                    (value.operators, value.operatorIds, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        operatorIds: tuple.1,
                        quorumNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<bool>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<bool>;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsStake(address[],bytes32[],uint8)";
            const SELECTOR: [u8; 4] = [108u8, 63u8, 180u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bool,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: updateOperatorsStakeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: updateOperatorsStakeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`.
```solidity
function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct weightOfOperatorForQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`weightOfOperatorForQuorum(uint8,address)`](weightOfOperatorForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct weightOfOperatorForQuorumReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<weightOfOperatorForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumCall) -> Self {
                    (value.quorumNumber, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for weightOfOperatorForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<weightOfOperatorForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for weightOfOperatorForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for weightOfOperatorForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weightOfOperatorForQuorum(uint8,address)";
            const SELECTOR: [u8; 4] = [31u8, 155u8, 116u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: weightOfOperatorForQuorumReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: weightOfOperatorForQuorumReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`StakeRegistry`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum StakeRegistryCalls {
        #[allow(missing_docs)]
        MAX_WEIGHING_FUNCTION_LENGTH(MAX_WEIGHING_FUNCTION_LENGTHCall),
        #[allow(missing_docs)]
        WEIGHTING_DIVISOR(WEIGHTING_DIVISORCall),
        #[allow(missing_docs)]
        addStrategies(addStrategiesCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        avsDirectory(avsDirectoryCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        deregisterOperator(deregisterOperatorCall),
        #[allow(missing_docs)]
        getCurrentStake(getCurrentStakeCall),
        #[allow(missing_docs)]
        getCurrentTotalStake(getCurrentTotalStakeCall),
        #[allow(missing_docs)]
        getLatestStakeUpdate(getLatestStakeUpdateCall),
        #[allow(missing_docs)]
        getStakeAtBlockNumber(getStakeAtBlockNumberCall),
        #[allow(missing_docs)]
        getStakeAtBlockNumberAndIndex(getStakeAtBlockNumberAndIndexCall),
        #[allow(missing_docs)]
        getStakeHistory(getStakeHistoryCall),
        #[allow(missing_docs)]
        getStakeHistoryLength(getStakeHistoryLengthCall),
        #[allow(missing_docs)]
        getStakeUpdateAtIndex(getStakeUpdateAtIndexCall),
        #[allow(missing_docs)]
        getStakeUpdateIndexAtBlockNumber(getStakeUpdateIndexAtBlockNumberCall),
        #[allow(missing_docs)]
        getTotalStakeAtBlockNumberFromIndex(getTotalStakeAtBlockNumberFromIndexCall),
        #[allow(missing_docs)]
        getTotalStakeHistoryLength(getTotalStakeHistoryLengthCall),
        #[allow(missing_docs)]
        getTotalStakeIndicesAtBlockNumber(getTotalStakeIndicesAtBlockNumberCall),
        #[allow(missing_docs)]
        getTotalStakeUpdateAtIndex(getTotalStakeUpdateAtIndexCall),
        #[allow(missing_docs)]
        initializeDelegatedStakeQuorum(initializeDelegatedStakeQuorumCall),
        #[allow(missing_docs)]
        initializeSlashableStakeQuorum(initializeSlashableStakeQuorumCall),
        #[allow(missing_docs)]
        minimumStakeForQuorum(minimumStakeForQuorumCall),
        #[allow(missing_docs)]
        modifyStrategyParams(modifyStrategyParamsCall),
        #[allow(missing_docs)]
        registerOperator(registerOperatorCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        removeStrategies(removeStrategiesCall),
        #[allow(missing_docs)]
        setMinimumStakeForQuorum(setMinimumStakeForQuorumCall),
        #[allow(missing_docs)]
        setSlashableStakeLookahead(setSlashableStakeLookaheadCall),
        #[allow(missing_docs)]
        slashableStakeLookAheadPerQuorum(slashableStakeLookAheadPerQuorumCall),
        #[allow(missing_docs)]
        stakeTypePerQuorum(stakeTypePerQuorumCall),
        #[allow(missing_docs)]
        strategiesPerQuorum(strategiesPerQuorumCall),
        #[allow(missing_docs)]
        strategyParams(strategyParamsCall),
        #[allow(missing_docs)]
        strategyParamsByIndex(strategyParamsByIndexCall),
        #[allow(missing_docs)]
        strategyParamsLength(strategyParamsLengthCall),
        #[allow(missing_docs)]
        updateOperatorsStake(updateOperatorsStakeCall),
        #[allow(missing_docs)]
        weightOfOperatorForQuorum(weightOfOperatorForQuorumCall),
    }
    #[automatically_derived]
    impl StakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 145u8, 180u8, 28u8],
            [8u8, 115u8, 36u8, 97u8],
            [31u8, 155u8, 116u8, 224u8],
            [32u8, 182u8, 98u8, 152u8],
            [37u8, 80u8, 71u8, 119u8],
            [44u8, 217u8, 89u8, 64u8],
            [60u8, 165u8, 165u8, 245u8],
            [75u8, 210u8, 110u8, 9u8],
            [84u8, 1u8, 237u8, 39u8],
            [94u8, 90u8, 103u8, 117u8],
            [95u8, 31u8, 45u8, 119u8],
            [105u8, 127u8, 189u8, 147u8],
            [107u8, 58u8, 167u8, 46u8],
            [108u8, 63u8, 180u8, 191u8],
            [109u8, 20u8, 169u8, 135u8],
            [117u8, 212u8, 23u8, 58u8],
            [124u8, 23u8, 35u8, 71u8],
            [129u8, 192u8, 117u8, 2u8],
            [154u8, 180u8, 214u8, 255u8],
            [159u8, 60u8, 207u8, 101u8],
            [172u8, 107u8, 251u8, 3u8],
            [173u8, 200u8, 4u8, 218u8],
            [182u8, 144u8, 75u8, 120u8],
            [188u8, 154u8, 64u8, 195u8],
            [189u8, 41u8, 184u8, 205u8],
            [196u8, 103u8, 120u8, 165u8],
            [198u8, 1u8, 82u8, 125u8],
            [200u8, 41u8, 76u8, 86u8],
            [202u8, 138u8, 167u8, 199u8],
            [204u8, 90u8, 124u8, 32u8],
            [213u8, 236u8, 204u8, 5u8],
            [221u8, 152u8, 70u8, 185u8],
            [223u8, 92u8, 247u8, 35u8],
            [224u8, 134u8, 173u8, 179u8],
            [242u8, 190u8, 148u8, 174u8],
            [248u8, 81u8, 225u8, 152u8],
            [250u8, 40u8, 198u8, 39u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryCalls {
        const NAME: &'static str = "StakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(_) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WEIGHTING_DIVISOR(_) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategies(_) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentStake(_) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentTotalStake(_) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestStakeUpdate(_) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeAtBlockNumber(_) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeAtBlockNumberAndIndex(_) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeHistory(_) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeHistoryLength(_) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeUpdateAtIndex(_) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeUpdateIndexAtBlockNumber(_) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeAtBlockNumberFromIndex(_) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeHistoryLength(_) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeIndicesAtBlockNumber(_) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeUpdateAtIndex(_) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeDelegatedStakeQuorum(_) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeSlashableStakeQuorum(_) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumStakeForQuorum(_) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyStrategyParams(_) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategies(_) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinimumStakeForQuorum(_) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSlashableStakeLookahead(_) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashableStakeLookAheadPerQuorum(_) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeTypePerQuorum(_) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategiesPerQuorum(_) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParams(_) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParamsByIndex(_) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParamsLength(_) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsStake(_) => {
                    <updateOperatorsStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::weightOfOperatorForQuorum(_) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StakeRegistryCalls>] = &[
                {
                    fn getTotalStakeHistoryLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeHistoryLength)
                    }
                    getTotalStakeHistoryLength
                },
                {
                    fn strategyParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::strategyParams)
                    }
                    strategyParams
                },
                {
                    fn weightOfOperatorForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::weightOfOperatorForQuorum)
                    }
                    weightOfOperatorForQuorum
                },
                {
                    fn modifyStrategyParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::modifyStrategyParams)
                    }
                    modifyStrategyParams
                },
                {
                    fn registerOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getStakeHistory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeHistory)
                    }
                    getStakeHistory
                },
                {
                    fn strategyParamsLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::strategyParamsLength)
                    }
                    strategyParamsLength
                },
                {
                    fn getStakeHistoryLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeHistoryLength)
                    }
                    getStakeHistoryLength
                },
                {
                    fn getCurrentStake(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getCurrentStake)
                    }
                    getCurrentStake
                },
                {
                    fn WEIGHTING_DIVISOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::WEIGHTING_DIVISOR)
                    }
                    WEIGHTING_DIVISOR
                },
                {
                    fn removeStrategies(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <removeStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::removeStrategies)
                    }
                    removeStrategies
                },
                {
                    fn stakeTypePerQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::stakeTypePerQuorum)
                    }
                    stakeTypePerQuorum
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn updateOperatorsStake(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <updateOperatorsStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::updateOperatorsStake)
                    }
                    updateOperatorsStake
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn initializeDelegatedStakeQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::initializeDelegatedStakeQuorum)
                    }
                    initializeDelegatedStakeQuorum
                },
                {
                    fn MAX_WEIGHING_FUNCTION_LENGTH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::MAX_WEIGHING_FUNCTION_LENGTH)
                    }
                    MAX_WEIGHING_FUNCTION_LENGTH
                },
                {
                    fn getTotalStakeIndicesAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeIndicesAtBlockNumber)
                    }
                    getTotalStakeIndicesAtBlockNumber
                },
                {
                    fn slashableStakeLookAheadPerQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::slashableStakeLookAheadPerQuorum)
                    }
                    slashableStakeLookAheadPerQuorum
                },
                {
                    fn strategiesPerQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::strategiesPerQuorum)
                    }
                    strategiesPerQuorum
                },
                {
                    fn getStakeUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateAtIndex)
                    }
                    getStakeUpdateAtIndex
                },
                {
                    fn strategyParamsByIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::strategyParamsByIndex)
                    }
                    strategyParamsByIndex
                },
                {
                    fn getTotalStakeUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeUpdateAtIndex)
                    }
                    getTotalStakeUpdateAtIndex
                },
                {
                    fn setMinimumStakeForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::setMinimumStakeForQuorum)
                    }
                    setMinimumStakeForQuorum
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn minimumStakeForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::minimumStakeForQuorum)
                    }
                    minimumStakeForQuorum
                },
                {
                    fn addStrategies(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <addStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::addStrategies)
                    }
                    addStrategies
                },
                {
                    fn getTotalStakeAtBlockNumberFromIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeAtBlockNumberFromIndex)
                    }
                    getTotalStakeAtBlockNumberFromIndex
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initializeSlashableStakeQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::initializeSlashableStakeQuorum)
                    }
                    initializeSlashableStakeQuorum
                },
                {
                    fn getCurrentTotalStake(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getCurrentTotalStake)
                    }
                    getCurrentTotalStake
                },
                {
                    fn getStakeUpdateIndexAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateIndexAtBlockNumber)
                    }
                    getStakeUpdateIndexAtBlockNumber
                },
                {
                    fn delegation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::delegation)
                    }
                    delegation
                },
                {
                    fn setSlashableStakeLookahead(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::setSlashableStakeLookahead)
                    }
                    setSlashableStakeLookahead
                },
                {
                    fn getStakeAtBlockNumberAndIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumberAndIndex)
                    }
                    getStakeAtBlockNumberAndIndex
                },
                {
                    fn getLatestStakeUpdate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getLatestStakeUpdate)
                    }
                    getLatestStakeUpdate
                },
                {
                    fn getStakeAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumber)
                    }
                    getStakeAtBlockNumber
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StakeRegistryCalls>] = &[
                {
                    fn getTotalStakeHistoryLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeHistoryLength)
                    }
                    getTotalStakeHistoryLength
                },
                {
                    fn strategyParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::strategyParams)
                    }
                    strategyParams
                },
                {
                    fn weightOfOperatorForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::weightOfOperatorForQuorum)
                    }
                    weightOfOperatorForQuorum
                },
                {
                    fn modifyStrategyParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::modifyStrategyParams)
                    }
                    modifyStrategyParams
                },
                {
                    fn registerOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getStakeHistory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeHistory)
                    }
                    getStakeHistory
                },
                {
                    fn strategyParamsLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::strategyParamsLength)
                    }
                    strategyParamsLength
                },
                {
                    fn getStakeHistoryLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeHistoryLength)
                    }
                    getStakeHistoryLength
                },
                {
                    fn getCurrentStake(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getCurrentStake)
                    }
                    getCurrentStake
                },
                {
                    fn WEIGHTING_DIVISOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::WEIGHTING_DIVISOR)
                    }
                    WEIGHTING_DIVISOR
                },
                {
                    fn removeStrategies(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <removeStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::removeStrategies)
                    }
                    removeStrategies
                },
                {
                    fn stakeTypePerQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::stakeTypePerQuorum)
                    }
                    stakeTypePerQuorum
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn updateOperatorsStake(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <updateOperatorsStakeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::updateOperatorsStake)
                    }
                    updateOperatorsStake
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn initializeDelegatedStakeQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::initializeDelegatedStakeQuorum)
                    }
                    initializeDelegatedStakeQuorum
                },
                {
                    fn MAX_WEIGHING_FUNCTION_LENGTH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::MAX_WEIGHING_FUNCTION_LENGTH)
                    }
                    MAX_WEIGHING_FUNCTION_LENGTH
                },
                {
                    fn getTotalStakeIndicesAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeIndicesAtBlockNumber)
                    }
                    getTotalStakeIndicesAtBlockNumber
                },
                {
                    fn slashableStakeLookAheadPerQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::slashableStakeLookAheadPerQuorum)
                    }
                    slashableStakeLookAheadPerQuorum
                },
                {
                    fn strategiesPerQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::strategiesPerQuorum)
                    }
                    strategiesPerQuorum
                },
                {
                    fn getStakeUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateAtIndex)
                    }
                    getStakeUpdateAtIndex
                },
                {
                    fn strategyParamsByIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::strategyParamsByIndex)
                    }
                    strategyParamsByIndex
                },
                {
                    fn getTotalStakeUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeUpdateAtIndex)
                    }
                    getTotalStakeUpdateAtIndex
                },
                {
                    fn setMinimumStakeForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::setMinimumStakeForQuorum)
                    }
                    setMinimumStakeForQuorum
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn minimumStakeForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::minimumStakeForQuorum)
                    }
                    minimumStakeForQuorum
                },
                {
                    fn addStrategies(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <addStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::addStrategies)
                    }
                    addStrategies
                },
                {
                    fn getTotalStakeAtBlockNumberFromIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getTotalStakeAtBlockNumberFromIndex)
                    }
                    getTotalStakeAtBlockNumberFromIndex
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initializeSlashableStakeQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::initializeSlashableStakeQuorum)
                    }
                    initializeSlashableStakeQuorum
                },
                {
                    fn getCurrentTotalStake(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getCurrentTotalStake)
                    }
                    getCurrentTotalStake
                },
                {
                    fn getStakeUpdateIndexAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateIndexAtBlockNumber)
                    }
                    getStakeUpdateIndexAtBlockNumber
                },
                {
                    fn delegation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::delegation)
                    }
                    delegation
                },
                {
                    fn setSlashableStakeLookahead(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::setSlashableStakeLookahead)
                    }
                    setSlashableStakeLookahead
                },
                {
                    fn getStakeAtBlockNumberAndIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumberAndIndex)
                    }
                    getStakeAtBlockNumberAndIndex
                },
                {
                    fn getLatestStakeUpdate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getLatestStakeUpdate)
                    }
                    getLatestStakeUpdate
                },
                {
                    fn getStakeAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumber)
                    }
                    getStakeAtBlockNumber
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(inner) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WEIGHTING_DIVISOR(inner) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStrategies(inner) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentStake(inner) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentTotalStake(inner) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestStakeUpdate(inner) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeAtBlockNumber(inner) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeAtBlockNumberAndIndex(inner) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeHistory(inner) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeHistoryLength(inner) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeUpdateAtIndex(inner) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeUpdateIndexAtBlockNumber(inner) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeAtBlockNumberFromIndex(inner) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeHistoryLength(inner) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeIndicesAtBlockNumber(inner) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeUpdateAtIndex(inner) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeDelegatedStakeQuorum(inner) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeSlashableStakeQuorum(inner) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumStakeForQuorum(inner) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyStrategyParams(inner) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeStrategies(inner) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setSlashableStakeLookahead(inner) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashableStakeLookAheadPerQuorum(inner) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeTypePerQuorum(inner) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategiesPerQuorum(inner) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParams(inner) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParamsByIndex(inner) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParamsLength(inner) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsStake(inner) => {
                    <updateOperatorsStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::weightOfOperatorForQuorum(inner) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(inner) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WEIGHTING_DIVISOR(inner) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addStrategies(inner) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentStake(inner) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentTotalStake(inner) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestStakeUpdate(inner) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeAtBlockNumber(inner) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeAtBlockNumberAndIndex(inner) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeHistory(inner) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeHistoryLength(inner) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeUpdateAtIndex(inner) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeUpdateIndexAtBlockNumber(inner) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeAtBlockNumberFromIndex(inner) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeHistoryLength(inner) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeIndicesAtBlockNumber(inner) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeUpdateAtIndex(inner) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeDelegatedStakeQuorum(inner) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeSlashableStakeQuorum(inner) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumStakeForQuorum(inner) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyStrategyParams(inner) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeStrategies(inner) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setSlashableStakeLookahead(inner) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashableStakeLookAheadPerQuorum(inner) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeTypePerQuorum(inner) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategiesPerQuorum(inner) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParams(inner) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParamsByIndex(inner) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParamsLength(inner) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsStake(inner) => {
                    <updateOperatorsStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::weightOfOperatorForQuorum(inner) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StakeRegistry`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum StakeRegistryErrors {
        #[allow(missing_docs)]
        BelowMinimumStakeRequirement(BelowMinimumStakeRequirement),
        #[allow(missing_docs)]
        EmptyStakeHistory(EmptyStakeHistory),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InputArrayLengthZero(InputArrayLengthZero),
        #[allow(missing_docs)]
        InputDuplicateStrategy(InputDuplicateStrategy),
        #[allow(missing_docs)]
        InputMultiplierZero(InputMultiplierZero),
        #[allow(missing_docs)]
        InvalidBlockNumber(InvalidBlockNumber),
        #[allow(missing_docs)]
        LookAheadPeriodTooLong(LookAheadPeriodTooLong),
        #[allow(missing_docs)]
        OnlySlashingRegistryCoordinator(OnlySlashingRegistryCoordinator),
        #[allow(missing_docs)]
        OnlySlashingRegistryCoordinatorOwner(OnlySlashingRegistryCoordinatorOwner),
        #[allow(missing_docs)]
        QuorumAlreadyExists(QuorumAlreadyExists),
        #[allow(missing_docs)]
        QuorumDoesNotExist(QuorumDoesNotExist),
        #[allow(missing_docs)]
        QuorumNotSlashable(QuorumNotSlashable),
    }
    #[automatically_derived]
    impl StakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [47u8, 81u8, 6u8, 228u8],
            [64u8, 254u8, 39u8, 198u8],
            [67u8, 54u8, 148u8, 92u8],
            [67u8, 113u8, 74u8, 253u8],
            [78u8, 71u8, 132u8, 108u8],
            [114u8, 87u8, 18u8, 81u8],
            [121u8, 108u8, 197u8, 37u8],
            [123u8, 116u8, 52u8, 11u8],
            [163u8, 190u8, 37u8, 131u8],
            [176u8, 6u8, 200u8, 20u8],
            [204u8, 100u8, 101u8, 115u8],
            [206u8, 152u8, 194u8, 75u8],
            [230u8, 33u8, 159u8, 234u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryErrors {
        const NAME: &'static str = "StakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BelowMinimumStakeRequirement(_) => {
                    <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyStakeHistory(_) => {
                    <EmptyStakeHistory as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthZero(_) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputDuplicateStrategy(_) => {
                    <InputDuplicateStrategy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputMultiplierZero(_) => {
                    <InputMultiplierZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBlockNumber(_) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LookAheadPeriodTooLong(_) => {
                    <LookAheadPeriodTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlySlashingRegistryCoordinator(_) => {
                    <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlySlashingRegistryCoordinatorOwner(_) => {
                    <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumAlreadyExists(_) => {
                    <QuorumAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumDoesNotExist(_) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumNotSlashable(_) => {
                    <QuorumNotSlashable as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StakeRegistryErrors>] = &[
                {
                    fn LookAheadPeriodTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::LookAheadPeriodTooLong)
                    }
                    LookAheadPeriodTooLong
                },
                {
                    fn BelowMinimumStakeRequirement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::BelowMinimumStakeRequirement)
                    }
                    BelowMinimumStakeRequirement
                },
                {
                    fn QuorumAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::QuorumAlreadyExists)
                    }
                    QuorumAlreadyExists
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn InvalidBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::InvalidBlockNumber)
                    }
                    InvalidBlockNumber
                },
                {
                    fn InputMultiplierZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputMultiplierZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::InputMultiplierZero)
                    }
                    InputMultiplierZero
                },
                {
                    fn InputArrayLengthZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputArrayLengthZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::InputArrayLengthZero)
                    }
                    InputArrayLengthZero
                },
                {
                    fn InputDuplicateStrategy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::InputDuplicateStrategy)
                    }
                    InputDuplicateStrategy
                },
                {
                    fn QuorumNotSlashable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumNotSlashable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::QuorumNotSlashable)
                    }
                    QuorumNotSlashable
                },
                {
                    fn OnlySlashingRegistryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::OnlySlashingRegistryCoordinator)
                    }
                    OnlySlashingRegistryCoordinator
                },
                {
                    fn EmptyStakeHistory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <EmptyStakeHistory as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::EmptyStakeHistory)
                    }
                    EmptyStakeHistory
                },
                {
                    fn OnlySlashingRegistryCoordinatorOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StakeRegistryErrors::OnlySlashingRegistryCoordinatorOwner,
                            )
                    }
                    OnlySlashingRegistryCoordinatorOwner
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StakeRegistryErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StakeRegistryErrors>] = &[
                {
                    fn LookAheadPeriodTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::LookAheadPeriodTooLong)
                    }
                    LookAheadPeriodTooLong
                },
                {
                    fn BelowMinimumStakeRequirement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::BelowMinimumStakeRequirement)
                    }
                    BelowMinimumStakeRequirement
                },
                {
                    fn QuorumAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::QuorumAlreadyExists)
                    }
                    QuorumAlreadyExists
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn InvalidBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::InvalidBlockNumber)
                    }
                    InvalidBlockNumber
                },
                {
                    fn InputMultiplierZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputMultiplierZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::InputMultiplierZero)
                    }
                    InputMultiplierZero
                },
                {
                    fn InputArrayLengthZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputArrayLengthZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::InputArrayLengthZero)
                    }
                    InputArrayLengthZero
                },
                {
                    fn InputDuplicateStrategy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::InputDuplicateStrategy)
                    }
                    InputDuplicateStrategy
                },
                {
                    fn QuorumNotSlashable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumNotSlashable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::QuorumNotSlashable)
                    }
                    QuorumNotSlashable
                },
                {
                    fn OnlySlashingRegistryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::OnlySlashingRegistryCoordinator)
                    }
                    OnlySlashingRegistryCoordinator
                },
                {
                    fn EmptyStakeHistory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <EmptyStakeHistory as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::EmptyStakeHistory)
                    }
                    EmptyStakeHistory
                },
                {
                    fn OnlySlashingRegistryCoordinatorOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StakeRegistryErrors::OnlySlashingRegistryCoordinatorOwner,
                            )
                    }
                    OnlySlashingRegistryCoordinatorOwner
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StakeRegistryErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BelowMinimumStakeRequirement(inner) => {
                    <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyStakeHistory(inner) => {
                    <EmptyStakeHistory as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputDuplicateStrategy(inner) => {
                    <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputMultiplierZero(inner) => {
                    <InputMultiplierZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBlockNumber(inner) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LookAheadPeriodTooLong(inner) => {
                    <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlySlashingRegistryCoordinator(inner) => {
                    <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlySlashingRegistryCoordinatorOwner(inner) => {
                    <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumAlreadyExists(inner) => {
                    <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumNotSlashable(inner) => {
                    <QuorumNotSlashable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BelowMinimumStakeRequirement(inner) => {
                    <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyStakeHistory(inner) => {
                    <EmptyStakeHistory as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputDuplicateStrategy(inner) => {
                    <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputMultiplierZero(inner) => {
                    <InputMultiplierZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBlockNumber(inner) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LookAheadPeriodTooLong(inner) => {
                    <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlySlashingRegistryCoordinator(inner) => {
                    <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlySlashingRegistryCoordinatorOwner(inner) => {
                    <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumAlreadyExists(inner) => {
                    <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumNotSlashable(inner) => {
                    <QuorumNotSlashable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StakeRegistry`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum StakeRegistryEvents {
        #[allow(missing_docs)]
        LookAheadPeriodChanged(LookAheadPeriodChanged),
        #[allow(missing_docs)]
        MinimumStakeForQuorumUpdated(MinimumStakeForQuorumUpdated),
        #[allow(missing_docs)]
        OperatorStakeUpdate(OperatorStakeUpdate),
        #[allow(missing_docs)]
        QuorumCreated(QuorumCreated),
        #[allow(missing_docs)]
        StakeTypeSet(StakeTypeSet),
        #[allow(missing_docs)]
        StrategyAddedToQuorum(StrategyAddedToQuorum),
        #[allow(missing_docs)]
        StrategyMultiplierUpdated(StrategyMultiplierUpdated),
        #[allow(missing_docs)]
        StrategyRemovedFromQuorum(StrategyRemovedFromQuorum),
    }
    #[automatically_derived]
    impl StakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                16u8, 86u8, 94u8, 86u8, 202u8, 203u8, 243u8, 46u8, 202u8, 38u8, 121u8,
                69u8, 240u8, 84u8, 254u8, 192u8, 46u8, 89u8, 117u8, 0u8, 50u8, 209u8,
                19u8, 211u8, 48u8, 33u8, 130u8, 173u8, 150u8, 127u8, 84u8, 4u8,
            ],
            [
                17u8, 165u8, 100u8, 19u8, 34u8, 218u8, 29u8, 255u8, 86u8, 164u8, 182u8,
                110u8, 170u8, 195u8, 31u8, 250u8, 70u8, 82u8, 149u8, 236u8, 233u8, 7u8,
                205u8, 22u8, 52u8, 55u8, 121u8, 59u8, 77u8, 0u8, 154u8, 117u8,
            ],
            [
                38u8, 238u8, 207u8, 242u8, 183u8, 11u8, 10u8, 113u8, 16u8, 79u8, 244u8,
                217u8, 64u8, 186u8, 113u8, 98u8, 210u8, 58u8, 149u8, 194u8, 72u8, 119u8,
                31u8, 196u8, 135u8, 167u8, 190u8, 23u8, 165u8, 150u8, 179u8, 207u8,
            ],
            [
                40u8, 215u8, 53u8, 139u8, 121u8, 240u8, 45u8, 33u8, 184u8, 183u8, 225u8,
                122u8, 239u8, 196u8, 24u8, 90u8, 100u8, 48u8, 138u8, 163u8, 116u8, 6u8,
                250u8, 91u8, 239u8, 192u8, 91u8, 145u8, 147u8, 44u8, 57u8, 199u8,
            ],
            [
                47u8, 82u8, 125u8, 82u8, 126u8, 149u8, 216u8, 254u8, 64u8, 174u8, 197u8,
                83u8, 119u8, 116u8, 59u8, 183u8, 121u8, 8u8, 125u8, 163u8, 246u8, 208u8,
                208u8, 143u8, 18u8, 227u8, 100u8, 68u8, 218u8, 98u8, 50u8, 125u8,
            ],
            [
                49u8, 250u8, 46u8, 44u8, 210u8, 128u8, 201u8, 55u8, 94u8, 19u8, 255u8,
                207u8, 61u8, 129u8, 226u8, 55u8, 129u8, 0u8, 24u8, 110u8, 64u8, 88u8,
                248u8, 211u8, 221u8, 182u8, 144u8, 184u8, 45u8, 205u8, 49u8, 247u8,
            ],
            [
                124u8, 17u8, 46u8, 134u8, 60u8, 207u8, 0u8, 120u8, 98u8, 226u8, 201u8,
                226u8, 88u8, 25u8, 201u8, 51u8, 254u8, 219u8, 201u8, 53u8, 10u8, 100u8,
                67u8, 66u8, 59u8, 74u8, 133u8, 153u8, 194u8, 232u8, 165u8, 45u8,
            ],
            [
                131u8, 26u8, 156u8, 134u8, 196u8, 91u8, 179u8, 3u8, 202u8, 243u8, 240u8,
                100u8, 190u8, 43u8, 194u8, 185u8, 253u8, 78u8, 207u8, 25u8, 228u8, 124u8,
                74u8, 192u8, 42u8, 97u8, 231u8, 93u8, 171u8, 254u8, 85u8, 180u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for StakeRegistryEvents {
        const NAME: &'static str = "StakeRegistryEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <LookAheadPeriodChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LookAheadPeriodChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LookAheadPeriodChanged)
                }
                Some(
                    <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MinimumStakeForQuorumUpdated)
                }
                Some(
                    <OperatorStakeUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorStakeUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OperatorStakeUpdate)
                }
                Some(<QuorumCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::QuorumCreated)
                }
                Some(<StakeTypeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeTypeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::StakeTypeSet)
                }
                Some(
                    <StrategyAddedToQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyAddedToQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::StrategyAddedToQuorum)
                }
                Some(
                    <StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::StrategyMultiplierUpdated)
                }
                Some(
                    <StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::StrategyRemovedFromQuorum)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for StakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::LookAheadPeriodChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakeTypeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyAddedToQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyMultiplierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyRemovedFromQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::LookAheadPeriodChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakeTypeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyAddedToQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyMultiplierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyRemovedFromQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StakeRegistryInstance<P, N> {
        StakeRegistryInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StakeRegistryInstance<P, N>>,
    > {
        StakeRegistryInstance::<
            P,
            N,
        >::deploy(
            provider,
            _slashingRegistryCoordinator,
            _delegationManager,
            _avsDirectory,
            _allocationManager,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        StakeRegistryInstance::<
            P,
            N,
        >::deploy_builder(
            provider,
            _slashingRegistryCoordinator,
            _delegationManager,
            _avsDirectory,
            _allocationManager,
        )
    }
    /**A [`StakeRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StakeRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StakeRegistryInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StakeRegistryInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StakeRegistryInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<StakeRegistryInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _slashingRegistryCoordinator,
                _delegationManager,
                _avsDirectory,
                _allocationManager,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _slashingRegistryCoordinator,
                            _delegationManager,
                            _avsDirectory,
                            _allocationManager,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> StakeRegistryInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StakeRegistryInstance<P, N> {
            StakeRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StakeRegistryInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`MAX_WEIGHING_FUNCTION_LENGTH`] function.
        pub fn MAX_WEIGHING_FUNCTION_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_WEIGHING_FUNCTION_LENGTHCall, N> {
            self.call_builder(&MAX_WEIGHING_FUNCTION_LENGTHCall)
        }
        ///Creates a new call builder for the [`WEIGHTING_DIVISOR`] function.
        pub fn WEIGHTING_DIVISOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, WEIGHTING_DIVISORCall, N> {
            self.call_builder(&WEIGHTING_DIVISORCall)
        }
        ///Creates a new call builder for the [`addStrategies`] function.
        pub fn addStrategies(
            &self,
            quorumNumber: u8,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, addStrategiesCall, N> {
            self.call_builder(
                &addStrategiesCall {
                    quorumNumber,
                    _strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall)
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall)
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, delegationCall, N> {
            self.call_builder(&delegationCall)
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, deregisterOperatorCall, N> {
            self.call_builder(
                &deregisterOperatorCall {
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentStake`] function.
        pub fn getCurrentStake(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentStakeCall, N> {
            self.call_builder(
                &getCurrentStakeCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentTotalStake`] function.
        pub fn getCurrentTotalStake(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentTotalStakeCall, N> {
            self.call_builder(
                &getCurrentTotalStakeCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLatestStakeUpdate`] function.
        pub fn getLatestStakeUpdate(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getLatestStakeUpdateCall, N> {
            self.call_builder(
                &getLatestStakeUpdateCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumber`] function.
        pub fn getStakeAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getStakeAtBlockNumberCall, N> {
            self.call_builder(
                &getStakeAtBlockNumberCall {
                    operatorId,
                    quorumNumber,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumberAndIndex`] function.
        pub fn getStakeAtBlockNumberAndIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getStakeAtBlockNumberAndIndexCall, N> {
            self.call_builder(
                &getStakeAtBlockNumberAndIndexCall {
                    quorumNumber,
                    blockNumber,
                    operatorId,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeHistory`] function.
        pub fn getStakeHistory(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getStakeHistoryCall, N> {
            self.call_builder(
                &getStakeHistoryCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeHistoryLength`] function.
        pub fn getStakeHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getStakeHistoryLengthCall, N> {
            self.call_builder(
                &getStakeHistoryLengthCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeUpdateAtIndex`] function.
        pub fn getStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getStakeUpdateAtIndexCall, N> {
            self.call_builder(
                &getStakeUpdateAtIndexCall {
                    quorumNumber,
                    operatorId,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeUpdateIndexAtBlockNumber`] function.
        pub fn getStakeUpdateIndexAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            getStakeUpdateIndexAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getStakeUpdateIndexAtBlockNumberCall {
                    operatorId,
                    quorumNumber,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeAtBlockNumberFromIndex`] function.
        pub fn getTotalStakeAtBlockNumberFromIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            getTotalStakeAtBlockNumberFromIndexCall,
            N,
        > {
            self.call_builder(
                &getTotalStakeAtBlockNumberFromIndexCall {
                    quorumNumber,
                    blockNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeHistoryLength`] function.
        pub fn getTotalStakeHistoryLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getTotalStakeHistoryLengthCall, N> {
            self.call_builder(
                &getTotalStakeHistoryLengthCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeIndicesAtBlockNumber`] function.
        pub fn getTotalStakeIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            getTotalStakeIndicesAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getTotalStakeIndicesAtBlockNumberCall {
                    blockNumber,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeUpdateAtIndex`] function.
        pub fn getTotalStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getTotalStakeUpdateAtIndexCall, N> {
            self.call_builder(
                &getTotalStakeUpdateAtIndexCall {
                    quorumNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`initializeDelegatedStakeQuorum`] function.
        pub fn initializeDelegatedStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, initializeDelegatedStakeQuorumCall, N> {
            self.call_builder(
                &initializeDelegatedStakeQuorumCall {
                    quorumNumber,
                    minimumStake,
                    _strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`initializeSlashableStakeQuorum`] function.
        pub fn initializeSlashableStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            lookAheadPeriod: u32,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, initializeSlashableStakeQuorumCall, N> {
            self.call_builder(
                &initializeSlashableStakeQuorumCall {
                    quorumNumber,
                    minimumStake,
                    lookAheadPeriod,
                    _strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`minimumStakeForQuorum`] function.
        pub fn minimumStakeForQuorum(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<&P, minimumStakeForQuorumCall, N> {
            self.call_builder(&minimumStakeForQuorumCall(_0))
        }
        ///Creates a new call builder for the [`modifyStrategyParams`] function.
        pub fn modifyStrategyParams(
            &self,
            quorumNumber: u8,
            strategyIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            newMultipliers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, modifyStrategyParamsCall, N> {
            self.call_builder(
                &modifyStrategyParamsCall {
                    quorumNumber,
                    strategyIndices,
                    newMultipliers,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
                    operator,
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall)
        }
        ///Creates a new call builder for the [`removeStrategies`] function.
        pub fn removeStrategies(
            &self,
            quorumNumber: u8,
            indicesToRemove: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, removeStrategiesCall, N> {
            self.call_builder(
                &removeStrategiesCall {
                    quorumNumber,
                    indicesToRemove,
                },
            )
        }
        ///Creates a new call builder for the [`setMinimumStakeForQuorum`] function.
        pub fn setMinimumStakeForQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<&P, setMinimumStakeForQuorumCall, N> {
            self.call_builder(
                &setMinimumStakeForQuorumCall {
                    quorumNumber,
                    minimumStake,
                },
            )
        }
        ///Creates a new call builder for the [`setSlashableStakeLookahead`] function.
        pub fn setSlashableStakeLookahead(
            &self,
            quorumNumber: u8,
            _lookAheadBlocks: u32,
        ) -> alloy_contract::SolCallBuilder<&P, setSlashableStakeLookaheadCall, N> {
            self.call_builder(
                &setSlashableStakeLookaheadCall {
                    quorumNumber,
                    _lookAheadBlocks,
                },
            )
        }
        ///Creates a new call builder for the [`slashableStakeLookAheadPerQuorum`] function.
        pub fn slashableStakeLookAheadPerQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            slashableStakeLookAheadPerQuorumCall,
            N,
        > {
            self.call_builder(
                &slashableStakeLookAheadPerQuorumCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`stakeTypePerQuorum`] function.
        pub fn stakeTypePerQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, stakeTypePerQuorumCall, N> {
            self.call_builder(
                &stakeTypePerQuorumCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`strategiesPerQuorum`] function.
        pub fn strategiesPerQuorum(
            &self,
            quorumNumber: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, strategiesPerQuorumCall, N> {
            self.call_builder(
                &strategiesPerQuorumCall {
                    quorumNumber,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`strategyParams`] function.
        pub fn strategyParams(
            &self,
            quorumNumber: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, strategyParamsCall, N> {
            self.call_builder(
                &strategyParamsCall {
                    quorumNumber,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`strategyParamsByIndex`] function.
        pub fn strategyParamsByIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, strategyParamsByIndexCall, N> {
            self.call_builder(
                &strategyParamsByIndexCall {
                    quorumNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`strategyParamsLength`] function.
        pub fn strategyParamsLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, strategyParamsLengthCall, N> {
            self.call_builder(
                &strategyParamsLengthCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperatorsStake`] function.
        pub fn updateOperatorsStake(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, updateOperatorsStakeCall, N> {
            self.call_builder(
                &updateOperatorsStakeCall {
                    operators,
                    operatorIds,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`weightOfOperatorForQuorum`] function.
        pub fn weightOfOperatorForQuorum(
            &self,
            quorumNumber: u8,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, weightOfOperatorForQuorumCall, N> {
            self.call_builder(
                &weightOfOperatorForQuorumCall {
                    quorumNumber,
                    operator,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StakeRegistryInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`LookAheadPeriodChanged`] event.
        pub fn LookAheadPeriodChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, LookAheadPeriodChanged, N> {
            self.event_filter::<LookAheadPeriodChanged>()
        }
        ///Creates a new event filter for the [`MinimumStakeForQuorumUpdated`] event.
        pub fn MinimumStakeForQuorumUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, MinimumStakeForQuorumUpdated, N> {
            self.event_filter::<MinimumStakeForQuorumUpdated>()
        }
        ///Creates a new event filter for the [`OperatorStakeUpdate`] event.
        pub fn OperatorStakeUpdate_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorStakeUpdate, N> {
            self.event_filter::<OperatorStakeUpdate>()
        }
        ///Creates a new event filter for the [`QuorumCreated`] event.
        pub fn QuorumCreated_filter(
            &self,
        ) -> alloy_contract::Event<&P, QuorumCreated, N> {
            self.event_filter::<QuorumCreated>()
        }
        ///Creates a new event filter for the [`StakeTypeSet`] event.
        pub fn StakeTypeSet_filter(&self) -> alloy_contract::Event<&P, StakeTypeSet, N> {
            self.event_filter::<StakeTypeSet>()
        }
        ///Creates a new event filter for the [`StrategyAddedToQuorum`] event.
        pub fn StrategyAddedToQuorum_filter(
            &self,
        ) -> alloy_contract::Event<&P, StrategyAddedToQuorum, N> {
            self.event_filter::<StrategyAddedToQuorum>()
        }
        ///Creates a new event filter for the [`StrategyMultiplierUpdated`] event.
        pub fn StrategyMultiplierUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, StrategyMultiplierUpdated, N> {
            self.event_filter::<StrategyMultiplierUpdated>()
        }
        ///Creates a new event filter for the [`StrategyRemovedFromQuorum`] event.
        pub fn StrategyRemovedFromQuorum_filter(
            &self,
        ) -> alloy_contract::Event<&P, StrategyRemovedFromQuorum, N> {
            self.event_filter::<StrategyRemovedFromQuorum>()
        }
    }
}
