#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]
use IndexRegistry::*;

///Module containing a contract's types and functions.
/**

```solidity
library IIndexRegistryTypes {
    struct OperatorUpdate { uint32 fromBlockNumber; bytes32 operatorId; }
    struct QuorumUpdate { uint32 fromBlockNumber; uint32 numOperators; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IIndexRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OperatorUpdate { uint32 fromBlockNumber; bytes32 operatorId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorUpdate {
        #[allow(missing_docs)]
        pub fromBlockNumber: u32,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
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
        impl ::core::convert::From<OperatorUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorUpdate) -> Self {
                (value.fromBlockNumber, value.operatorId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    fromBlockNumber: tuple.0,
                    operatorId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromBlockNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
        impl alloy_sol_types::SolType for OperatorUpdate {
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
        impl alloy_sol_types::SolStruct for OperatorUpdate {
            const NAME: &'static str = "OperatorUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorUpdate(uint32 fromBlockNumber,bytes32 operatorId)",
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
                            &self.fromBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fromBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
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
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fromBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
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
struct QuorumUpdate { uint32 fromBlockNumber; uint32 numOperators; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumUpdate {
        #[allow(missing_docs)]
        pub fromBlockNumber: u32,
        #[allow(missing_docs)]
        pub numOperators: u32,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u32);
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
        impl ::core::convert::From<QuorumUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumUpdate) -> Self {
                (value.fromBlockNumber, value.numOperators)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    fromBlockNumber: tuple.0,
                    numOperators: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.numOperators),
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
        impl alloy_sol_types::SolType for QuorumUpdate {
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
        impl alloy_sol_types::SolStruct for QuorumUpdate {
            const NAME: &'static str = "QuorumUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumUpdate(uint32 fromBlockNumber,uint32 numOperators)",
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
                            &self.fromBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numOperators)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fromBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numOperators,
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
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fromBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numOperators,
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
    /**Creates a new wrapper around an on-chain [`IIndexRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IIndexRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIndexRegistryTypesInstance<P, N> {
        IIndexRegistryTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IIndexRegistryTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IIndexRegistryTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIndexRegistryTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IIndexRegistryTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIndexRegistryTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IIndexRegistryTypesInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IIndexRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IIndexRegistryTypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IIndexRegistryTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IIndexRegistryTypesInstance<P, N> {
            IIndexRegistryTypesInstance {
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
    > IIndexRegistryTypesInstance<P, N> {
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
    > IIndexRegistryTypesInstance<P, N> {
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
library IIndexRegistryTypes {
    struct OperatorUpdate {
        uint32 fromBlockNumber;
        bytes32 operatorId;
    }
    struct QuorumUpdate {
        uint32 fromBlockNumber;
        uint32 numOperators;
    }
}

interface IndexRegistry {
    error OnlyRegistryCoordinator();
    error OperatorIdDoesNotExist();
    error QuorumDoesNotExist();

    event Initialized(uint8 version);
    event QuorumIndexUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint32 newOperatorIndex);

    constructor(address _slashingRegistryCoordinator);

    function OPERATOR_DOES_NOT_EXIST_ID() external view returns (bytes32);
    function currentOperatorIndex(uint8, bytes32) external view returns (uint32);
    function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
    function getLatestOperatorUpdate(uint8 quorumNumber, uint32 operatorIndex) external view returns (IIndexRegistryTypes.OperatorUpdate memory);
    function getLatestQuorumUpdate(uint8 quorumNumber) external view returns (IIndexRegistryTypes.QuorumUpdate memory);
    function getOperatorListAtBlockNumber(uint8 quorumNumber, uint32 blockNumber) external view returns (bytes32[] memory);
    function getOperatorUpdateAtIndex(uint8 quorumNumber, uint32 operatorIndex, uint32 arrayIndex) external view returns (IIndexRegistryTypes.OperatorUpdate memory);
    function getQuorumUpdateAtIndex(uint8 quorumNumber, uint32 quorumIndex) external view returns (IIndexRegistryTypes.QuorumUpdate memory);
    function initializeQuorum(uint8 quorumNumber) external;
    function registerOperator(bytes32 operatorId, bytes memory quorumNumbers) external returns (uint32[] memory);
    function registryCoordinator() external view returns (address);
    function totalOperatorsForQuorum(uint8 quorumNumber) external view returns (uint32);
    function totalOperatorsForQuorumAtBlockNumber(uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
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
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "OPERATOR_DOES_NOT_EXIST_ID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentOperatorIndex",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "getLatestOperatorUpdate",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistryTypes.OperatorUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestQuorumUpdate",
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
        "type": "tuple",
        "internalType": "struct IIndexRegistryTypes.QuorumUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numOperators",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorListAtBlockNumber",
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
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "arrayIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistryTypes.OperatorUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "quorumIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistryTypes.QuorumUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numOperators",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
        "type": "uint32[]",
        "internalType": "uint32[]"
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
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "totalOperatorsForQuorum",
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
    "name": "totalOperatorsForQuorumAtBlockNumber",
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
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumIndexUpdate",
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
        "name": "newOperatorIndex",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorIdDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumDoesNotExist",
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
pub mod IndexRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561000f575f5ffd5b5060405161116338038061116383398101604081905261002e91610107565b6001600160a01b0381166080528061004461004b565b5050610134565b5f54610100900460ff16156100b65760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff90811614610105575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b5f60208284031215610117575f5ffd5b81516001600160a01b038116811461012d575f5ffd5b9392505050565b6080516110106101535f395f8181610158015261078d01526110105ff3fe608060405234801561000f575f5ffd5b50600436106100ca575f3560e01c80638121906f11610088578063bd29b8cd11610063578063bd29b8cd14610224578063caa3cd7614610237578063e2e685801461024c578063f34109221461027c575f5ffd5b80638121906f146101ba57806389026245146101f1578063a48bb0ac14610211575f5ffd5b8062bff04d146100ce57806312d1d74d146100f757806326d941f21461012b5780632ed583e5146101405780636d14a987146101535780637b5d26be14610192575b5f5ffd5b6100e16100dc366004610d6e565b61028f565b6040516100ee9190610de5565b60405180910390f35b61010a610105366004610e55565b610398565b60408051825163ffffffff16815260209283015192810192909252016100ee565b61013e610139366004610e86565b6103dd565b005b61010a61014e366004610e9f565b610476565b61017a7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100ee565b6101a56101a0366004610e55565b6104f9565b60405163ffffffff90911681526020016100ee565b6101cd6101c8366004610e86565b610504565b60408051825163ffffffff90811682526020938401511692810192909252016100ee565b6102046101ff366004610e55565b61054a565b6040516100ee9190610edf565b6101cd61021f366004610e55565b61062a565b61013e610232366004610d6e565b61069e565b61023e5f81565b6040519081526020016100ee565b6101a561025a366004610f16565b600160209081525f928352604080842090915290825290205463ffffffff1681565b6101a561028a366004610e86565b610764565b6060610299610782565b5f8267ffffffffffffffff8111156102b3576102b3610f3e565b6040519080825280602002602001820160405280156102dc578160200160208202803683370190505b5090505f5b8381101561038d575f8585838181106102fc576102fc610f52565b919091013560f81c5f8181526003602052604081205491935090915081900361033857604051637310cff560e11b815260040160405180910390fd5b5f610342836107cd565b90506103598984610354600185610f7a565b6108c4565b8085858151811061036c5761036c610f52565b63ffffffff92909216602092830291909101909101525050506001016102e1565b5090505b9392505050565b604080518082019091525f80825260208201526103b5838361094c565b60408051808201909152815463ffffffff168152600190910154602082015290505b92915050565b6103e5610782565b60ff81165f908152600360205260409020541561041557604051637310cff560e11b815260040160405180910390fd5b60ff165f908152600360209081526040808320815180830190925263ffffffff438116835282840185815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055565b604080518082019091525f808252602082015260ff84165f90815260026020908152604080832063ffffffff808816855292529091208054909184169081106104c1576104c1610f52565b5f91825260209182902060408051808201909152600290920201805463ffffffff168252600101549181019190915290509392505050565b5f61039183836109a1565b604080518082019091525f808252602082015261052082610ad7565b60408051808201909152905463ffffffff8082168352600160201b90910416602082015292915050565b60605f61055784846109a1565b90505f8163ffffffff1667ffffffffffffffff81111561057957610579610f3e565b6040519080825280602002602001820160405280156105a2578160200160208202803683370190505b5090505f5b8263ffffffff16811015610621576105c0868287610b16565b8282815181106105d2576105d2610f52565b6020026020010181815250505f5f1b8282815181106105f3576105f3610f52565b60200260200101510361061957604051637f12098d60e11b815260040160405180910390fd5b6001016105a7565b50949350505050565b604080518082019091525f808252602082015260ff83165f908152600360205260409020805463ffffffff841690811061066657610666610f52565b5f9182526020918290206040805180820190915291015463ffffffff8082168352600160201b90910416918101919091529392505050565b6106a6610782565b5f5b8181101561075e575f8383838181106106c3576106c3610f52565b919091013560f81c5f818152600360205260408120549193509091508190036106ff57604051637310cff560e11b815260040160405180910390fd5b60ff82165f90815260016020908152604080832089845290915281205463ffffffff169061072c84610be9565b90505f6107398583610c21565b905080891461074d5761074d8186856108c4565b5050600190930192506106a8915050565b50505050565b5f61076e82610ad7565b54600160201b900463ffffffff1692915050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146107cb57604051634394dbdf60e11b815260040160405180910390fd5b565b5f5f6107d883610ad7565b80549091505f906107f790600160201b900463ffffffff166001610f96565b9050610804848383610c49565b60ff84165f90815260026020526040812090610821600184610f7a565b63ffffffff16815260208101919091526040015f9081205490036103915760ff84165f9081526002602052604081209061085c600184610f7a565b63ffffffff908116825260208083019390935260409182015f908120835180850190945243831684528385018281528154600180820184559284529590922093516002909502909301805463ffffffff19169490921693909317815591519101559392505050565b5f6108cf838361094c565b90506108dd83838387610ce6565b60ff83165f818152600160209081526040808320888452825291829020805463ffffffff191663ffffffff871690811790915582519384529083015285917f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6910160405180910390a250505050565b60ff82165f90815260026020908152604080832063ffffffff85168452909152812080549061097c600183610fb2565b8154811061098c5761098c610f52565b905f5260205f20906002020191505092915050565b60ff82165f90815260036020526040812054805b8015610a465760ff85165f9081526003602052604081206109d7600184610fb2565b815481106109e7576109e7610f52565b5f9182526020918290206040805180820190915291015463ffffffff808216808452600160201b90920481169383019390935290925090861610610a33576020015192506103d7915050565b5080610a3e81610fc5565b9150506109b5565b5060405162461bcd60e51b815260206004820152605560248201527f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c60448201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060648201527430ba1033b4bb32b710313637b1b590373ab6b132b960591b608482015260a40160405180910390fd5b60ff81165f908152600360205260408120805490610af6600183610fb2565b81548110610b0657610b06610f52565b905f5260205f2001915050919050565b60ff83165f90815260026020908152604080832063ffffffff86168452909152812054805b8015610bde5760ff86165f90815260026020908152604080832063ffffffff891684529091528120610b6e600184610fb2565b81548110610b7e57610b7e610f52565b5f91825260209182902060408051808201909152600290920201805463ffffffff9081168084526001909201549383019390935290925090861610610bcb57602001519250610391915050565b5080610bd681610fc5565b915050610b3b565b505f95945050505050565b5f5f610bf483610ad7565b80549091505f90610c1490600190600160201b900463ffffffff16610f7a565b9050610391848383610c49565b5f5f610c2d848461094c565b6001810154909150610c418585845f610ce6565b949350505050565b815463ffffffff438116911603610c7e57815463ffffffff8216600160201b0267ffffffff0000000019909116178255505050565b60ff83165f908152600360209081526040808320815180830190925263ffffffff438116835285811683850190815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055505050565b815463ffffffff438116911603610d03576001820181905561075e565b60ff939093165f90815260026020818152604080842063ffffffff968716855282528084208151808301909252438716825281830197885280546001808201835591865292909420905191909202909101805463ffffffff1916919094161783559251919092015550565b5f5f5f60408486031215610d80575f5ffd5b83359250602084013567ffffffffffffffff811115610d9d575f5ffd5b8401601f81018613610dad575f5ffd5b803567ffffffffffffffff811115610dc3575f5ffd5b866020828401011115610dd4575f5ffd5b939660209190910195509293505050565b602080825282518282018190525f918401906040840190835b81811015610e2257835163ffffffff16835260209384019390920191600101610dfe565b509095945050505050565b803560ff81168114610e3d575f5ffd5b919050565b803563ffffffff81168114610e3d575f5ffd5b5f5f60408385031215610e66575f5ffd5b610e6f83610e2d565b9150610e7d60208401610e42565b90509250929050565b5f60208284031215610e96575f5ffd5b61039182610e2d565b5f5f5f60608486031215610eb1575f5ffd5b610eba84610e2d565b9250610ec860208501610e42565b9150610ed660408501610e42565b90509250925092565b602080825282518282018190525f918401906040840190835b81811015610e22578351835260209384019390920191600101610ef8565b5f5f60408385031215610f27575f5ffd5b610f3083610e2d565b946020939093013593505050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b63ffffffff82811682821603908111156103d7576103d7610f66565b63ffffffff81811683821601908111156103d7576103d7610f66565b818103818111156103d7576103d7610f66565b5f81610fd357610fd3610f66565b505f19019056fea2646970667358221220a71d38b1c4eaee7a2304107558d2f809cb2cf83665850ab95ca02f97568e761564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x11c8\x03\x80a\x11c\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80a\0Da\0KV[PPa\x014V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x01\x05W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[_` \x82\x84\x03\x12\x15a\x01\x17W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01-W__\xFD[\x93\x92PPPV[`\x80Qa\x10\x10a\x01S_9_\x81\x81a\x01X\x01Ra\x07\x8D\x01Ra\x10\x10_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCAW_5`\xE0\x1C\x80c\x81!\x90o\x11a\0\x88W\x80c\xBD)\xB8\xCD\x11a\0cW\x80c\xBD)\xB8\xCD\x14a\x02$W\x80c\xCA\xA3\xCDv\x14a\x027W\x80c\xE2\xE6\x85\x80\x14a\x02LW\x80c\xF3A\t\"\x14a\x02|W__\xFD[\x80c\x81!\x90o\x14a\x01\xBAW\x80c\x89\x02bE\x14a\x01\xF1W\x80c\xA4\x8B\xB0\xAC\x14a\x02\x11W__\xFD[\x80b\xBF\xF0M\x14a\0\xCEW\x80c\x12\xD1\xD7M\x14a\0\xF7W\x80c&\xD9A\xF2\x14a\x01+W\x80c.\xD5\x83\xE5\x14a\x01@W\x80cm\x14\xA9\x87\x14a\x01SW\x80c{]&\xBE\x14a\x01\x92W[__\xFD[a\0\xE1a\0\xDC6`\x04a\rnV[a\x02\x8FV[`@Qa\0\xEE\x91\x90a\r\xE5V[`@Q\x80\x91\x03\x90\xF3[a\x01\na\x01\x056`\x04a\x0EUV[a\x03\x98V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xEEV[a\x01>a\x0196`\x04a\x0E\x86V[a\x03\xDDV[\0[a\x01\na\x01N6`\x04a\x0E\x9FV[a\x04vV[a\x01z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\x01\xA5a\x01\xA06`\x04a\x0EUV[a\x04\xF9V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xEEV[a\x01\xCDa\x01\xC86`\x04a\x0E\x86V[a\x05\x04V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\0\xEEV[a\x02\x04a\x01\xFF6`\x04a\x0EUV[a\x05JV[`@Qa\0\xEE\x91\x90a\x0E\xDFV[a\x01\xCDa\x02\x1F6`\x04a\x0EUV[a\x06*V[a\x01>a\x0226`\x04a\rnV[a\x06\x9EV[a\x02>_\x81V[`@Q\x90\x81R` \x01a\0\xEEV[a\x01\xA5a\x02Z6`\x04a\x0F\x16V[`\x01` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xA5a\x02\x8A6`\x04a\x0E\x86V[a\x07dV[``a\x02\x99a\x07\x82V[_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xB3Wa\x02\xB3a\x0F>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xDCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x03\x8DW_\x85\x85\x83\x81\x81\x10a\x02\xFCWa\x02\xFCa\x0FRV[\x91\x90\x91\x015`\xF8\x1C_\x81\x81R`\x03` R`@\x81 T\x91\x93P\x90\x91P\x81\x90\x03a\x038W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03B\x83a\x07\xCDV[\x90Pa\x03Y\x89\x84a\x03T`\x01\x85a\x0FzV[a\x08\xC4V[\x80\x85\x85\x81Q\x81\x10a\x03lWa\x03la\x0FRV[c\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPPP`\x01\x01a\x02\xE1V[P\x90P[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x03\xB5\x83\x83a\tLV[`@\x80Q\x80\x82\x01\x90\x91R\x81Tc\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01T` \x82\x01R\x90P[\x92\x91PPV[a\x03\xE5a\x07\x82V[`\xFF\x81\x16_\x90\x81R`\x03` R`@\x90 T\x15a\x04\x15W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x82\x84\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x84\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x92R\x90\x91 \x80T\x90\x91\x84\x16\x90\x81\x10a\x04\xC1Wa\x04\xC1a\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x90P\x93\x92PPPV[_a\x03\x91\x83\x83a\t\xA1V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x05 \x82a\n\xD7V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16` \x82\x01R\x92\x91PPV[``_a\x05W\x84\x84a\t\xA1V[\x90P_\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05yWa\x05ya\x0F>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xA2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x06!Wa\x05\xC0\x86\x82\x87a\x0B\x16V[\x82\x82\x81Q\x81\x10a\x05\xD2Wa\x05\xD2a\x0FRV[` \x02` \x01\x01\x81\x81RPP__\x1B\x82\x82\x81Q\x81\x10a\x05\xF3Wa\x05\xF3a\x0FRV[` \x02` \x01\x01Q\x03a\x06\x19W`@Qc\x7F\x12\t\x8D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x05\xA7V[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x06fWa\x06fa\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x06\xA6a\x07\x82V[_[\x81\x81\x10\x15a\x07^W_\x83\x83\x83\x81\x81\x10a\x06\xC3Wa\x06\xC3a\x0FRV[\x91\x90\x91\x015`\xF8\x1C_\x81\x81R`\x03` R`@\x81 T\x91\x93P\x90\x91P\x81\x90\x03a\x06\xFFW`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x81 Tc\xFF\xFF\xFF\xFF\x16\x90a\x07,\x84a\x0B\xE9V[\x90P_a\x079\x85\x83a\x0C!V[\x90P\x80\x89\x14a\x07MWa\x07M\x81\x86\x85a\x08\xC4V[PP`\x01\x90\x93\x01\x92Pa\x06\xA8\x91PPV[PPPPV[_a\x07n\x82a\n\xD7V[T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xCBW`@QcC\x94\xDB\xDF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[__a\x07\xD8\x83a\n\xD7V[\x80T\x90\x91P_\x90a\x07\xF7\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x0F\x96V[\x90Pa\x08\x04\x84\x83\x83a\x0CIV[`\xFF\x84\x16_\x90\x81R`\x02` R`@\x81 \x90a\x08!`\x01\x84a\x0FzV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T\x90\x03a\x03\x91W`\xFF\x84\x16_\x90\x81R`\x02` R`@\x81 \x90a\x08\\`\x01\x84a\x0FzV[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01_\x90\x81 \x83Q\x80\x85\x01\x90\x94RC\x83\x16\x84R\x83\x85\x01\x82\x81R\x81T`\x01\x80\x82\x01\x84U\x92\x84R\x95\x90\x92 \x93Q`\x02\x90\x95\x02\x90\x93\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x91Q\x91\x01U\x93\x92PPPV[_a\x08\xCF\x83\x83a\tLV[\x90Pa\x08\xDD\x83\x83\x83\x87a\x0C\xE6V[`\xFF\x83\x16_\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x85\x91\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x90a\t|`\x01\x83a\x0F\xB2V[\x81T\x81\x10a\t\x8CWa\t\x8Ca\x0FRV[\x90_R` _ \x90`\x02\x02\x01\x91PP\x92\x91PPV[`\xFF\x82\x16_\x90\x81R`\x03` R`@\x81 T\x80[\x80\x15a\nFW`\xFF\x85\x16_\x90\x81R`\x03` R`@\x81 a\t\xD7`\x01\x84a\x0F\xB2V[\x81T\x81\x10a\t\xE7Wa\t\xE7a\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\n3W` \x01Q\x92Pa\x03\xD7\x91PPV[P\x80a\n>\x81a\x0F\xC5V[\x91PPa\t\xB5V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x03` R`@\x81 \x80T\x90a\n\xF6`\x01\x83a\x0F\xB2V[\x81T\x81\x10a\x0B\x06Wa\x0B\x06a\x0FRV[\x90_R` _ \x01\x91PP\x91\x90PV[`\xFF\x83\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a\x0B\xDEW`\xFF\x86\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 a\x0Bn`\x01\x84a\x0F\xB2V[\x81T\x81\x10a\x0B~Wa\x0B~a\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\x0B\xCBW` \x01Q\x92Pa\x03\x91\x91PPV[P\x80a\x0B\xD6\x81a\x0F\xC5V[\x91PPa\x0B;V[P_\x95\x94PPPPPV[__a\x0B\xF4\x83a\n\xD7V[\x80T\x90\x91P_\x90a\x0C\x14\x90`\x01\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x0FzV[\x90Pa\x03\x91\x84\x83\x83a\x0CIV[__a\x0C-\x84\x84a\tLV[`\x01\x81\x01T\x90\x91Pa\x0CA\x85\x85\x84_a\x0C\xE6V[\x94\x93PPPPV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a\x0C~W\x81Tc\xFF\xFF\xFF\xFF\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x17\x82UPPPV[`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x85\x81\x16\x83\x85\x01\x90\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPPPV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a\r\x03W`\x01\x82\x01\x81\x90Ua\x07^V[`\xFF\x93\x90\x93\x16_\x90\x81R`\x02` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x96\x87\x16\x85R\x82R\x80\x84 \x81Q\x80\x83\x01\x90\x92RC\x87\x16\x82R\x81\x83\x01\x97\x88R\x80T`\x01\x80\x82\x01\x83U\x91\x86R\x92\x90\x94 \x90Q\x91\x90\x92\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x94\x16\x17\x83U\x92Q\x91\x90\x92\x01UPV[___`@\x84\x86\x03\x12\x15a\r\x80W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x9DW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\r\xADW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xC3W__\xFD[\x86` \x82\x84\x01\x01\x11\x15a\r\xD4W__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x0E\"W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\xFEV[P\x90\x95\x94PPPPPV[\x805`\xFF\x81\x16\x81\x14a\x0E=W__\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E=W__\xFD[__`@\x83\x85\x03\x12\x15a\x0EfW__\xFD[a\x0Eo\x83a\x0E-V[\x91Pa\x0E}` \x84\x01a\x0EBV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x0E\x96W__\xFD[a\x03\x91\x82a\x0E-V[___``\x84\x86\x03\x12\x15a\x0E\xB1W__\xFD[a\x0E\xBA\x84a\x0E-V[\x92Pa\x0E\xC8` \x85\x01a\x0EBV[\x91Pa\x0E\xD6`@\x85\x01a\x0EBV[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x0E\"W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\xF8V[__`@\x83\x85\x03\x12\x15a\x0F'W__\xFD[a\x0F0\x83a\x0E-V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x0FfV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x0FfV[\x81\x81\x03\x81\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x0FfV[_\x81a\x0F\xD3Wa\x0F\xD3a\x0FfV[P_\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \xA7\x1D8\xB1\xC4\xEA\xEEz#\x04\x10uX\xD2\xF8\t\xCB,\xF86e\x85\n\xB9\\\xA0/\x97V\x8Ev\x15dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100ca575f3560e01c80638121906f11610088578063bd29b8cd11610063578063bd29b8cd14610224578063caa3cd7614610237578063e2e685801461024c578063f34109221461027c575f5ffd5b80638121906f146101ba57806389026245146101f1578063a48bb0ac14610211575f5ffd5b8062bff04d146100ce57806312d1d74d146100f757806326d941f21461012b5780632ed583e5146101405780636d14a987146101535780637b5d26be14610192575b5f5ffd5b6100e16100dc366004610d6e565b61028f565b6040516100ee9190610de5565b60405180910390f35b61010a610105366004610e55565b610398565b60408051825163ffffffff16815260209283015192810192909252016100ee565b61013e610139366004610e86565b6103dd565b005b61010a61014e366004610e9f565b610476565b61017a7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100ee565b6101a56101a0366004610e55565b6104f9565b60405163ffffffff90911681526020016100ee565b6101cd6101c8366004610e86565b610504565b60408051825163ffffffff90811682526020938401511692810192909252016100ee565b6102046101ff366004610e55565b61054a565b6040516100ee9190610edf565b6101cd61021f366004610e55565b61062a565b61013e610232366004610d6e565b61069e565b61023e5f81565b6040519081526020016100ee565b6101a561025a366004610f16565b600160209081525f928352604080842090915290825290205463ffffffff1681565b6101a561028a366004610e86565b610764565b6060610299610782565b5f8267ffffffffffffffff8111156102b3576102b3610f3e565b6040519080825280602002602001820160405280156102dc578160200160208202803683370190505b5090505f5b8381101561038d575f8585838181106102fc576102fc610f52565b919091013560f81c5f8181526003602052604081205491935090915081900361033857604051637310cff560e11b815260040160405180910390fd5b5f610342836107cd565b90506103598984610354600185610f7a565b6108c4565b8085858151811061036c5761036c610f52565b63ffffffff92909216602092830291909101909101525050506001016102e1565b5090505b9392505050565b604080518082019091525f80825260208201526103b5838361094c565b60408051808201909152815463ffffffff168152600190910154602082015290505b92915050565b6103e5610782565b60ff81165f908152600360205260409020541561041557604051637310cff560e11b815260040160405180910390fd5b60ff165f908152600360209081526040808320815180830190925263ffffffff438116835282840185815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055565b604080518082019091525f808252602082015260ff84165f90815260026020908152604080832063ffffffff808816855292529091208054909184169081106104c1576104c1610f52565b5f91825260209182902060408051808201909152600290920201805463ffffffff168252600101549181019190915290509392505050565b5f61039183836109a1565b604080518082019091525f808252602082015261052082610ad7565b60408051808201909152905463ffffffff8082168352600160201b90910416602082015292915050565b60605f61055784846109a1565b90505f8163ffffffff1667ffffffffffffffff81111561057957610579610f3e565b6040519080825280602002602001820160405280156105a2578160200160208202803683370190505b5090505f5b8263ffffffff16811015610621576105c0868287610b16565b8282815181106105d2576105d2610f52565b6020026020010181815250505f5f1b8282815181106105f3576105f3610f52565b60200260200101510361061957604051637f12098d60e11b815260040160405180910390fd5b6001016105a7565b50949350505050565b604080518082019091525f808252602082015260ff83165f908152600360205260409020805463ffffffff841690811061066657610666610f52565b5f9182526020918290206040805180820190915291015463ffffffff8082168352600160201b90910416918101919091529392505050565b6106a6610782565b5f5b8181101561075e575f8383838181106106c3576106c3610f52565b919091013560f81c5f818152600360205260408120549193509091508190036106ff57604051637310cff560e11b815260040160405180910390fd5b60ff82165f90815260016020908152604080832089845290915281205463ffffffff169061072c84610be9565b90505f6107398583610c21565b905080891461074d5761074d8186856108c4565b5050600190930192506106a8915050565b50505050565b5f61076e82610ad7565b54600160201b900463ffffffff1692915050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146107cb57604051634394dbdf60e11b815260040160405180910390fd5b565b5f5f6107d883610ad7565b80549091505f906107f790600160201b900463ffffffff166001610f96565b9050610804848383610c49565b60ff84165f90815260026020526040812090610821600184610f7a565b63ffffffff16815260208101919091526040015f9081205490036103915760ff84165f9081526002602052604081209061085c600184610f7a565b63ffffffff908116825260208083019390935260409182015f908120835180850190945243831684528385018281528154600180820184559284529590922093516002909502909301805463ffffffff19169490921693909317815591519101559392505050565b5f6108cf838361094c565b90506108dd83838387610ce6565b60ff83165f818152600160209081526040808320888452825291829020805463ffffffff191663ffffffff871690811790915582519384529083015285917f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6910160405180910390a250505050565b60ff82165f90815260026020908152604080832063ffffffff85168452909152812080549061097c600183610fb2565b8154811061098c5761098c610f52565b905f5260205f20906002020191505092915050565b60ff82165f90815260036020526040812054805b8015610a465760ff85165f9081526003602052604081206109d7600184610fb2565b815481106109e7576109e7610f52565b5f9182526020918290206040805180820190915291015463ffffffff808216808452600160201b90920481169383019390935290925090861610610a33576020015192506103d7915050565b5080610a3e81610fc5565b9150506109b5565b5060405162461bcd60e51b815260206004820152605560248201527f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c60448201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060648201527430ba1033b4bb32b710313637b1b590373ab6b132b960591b608482015260a40160405180910390fd5b60ff81165f908152600360205260408120805490610af6600183610fb2565b81548110610b0657610b06610f52565b905f5260205f2001915050919050565b60ff83165f90815260026020908152604080832063ffffffff86168452909152812054805b8015610bde5760ff86165f90815260026020908152604080832063ffffffff891684529091528120610b6e600184610fb2565b81548110610b7e57610b7e610f52565b5f91825260209182902060408051808201909152600290920201805463ffffffff9081168084526001909201549383019390935290925090861610610bcb57602001519250610391915050565b5080610bd681610fc5565b915050610b3b565b505f95945050505050565b5f5f610bf483610ad7565b80549091505f90610c1490600190600160201b900463ffffffff16610f7a565b9050610391848383610c49565b5f5f610c2d848461094c565b6001810154909150610c418585845f610ce6565b949350505050565b815463ffffffff438116911603610c7e57815463ffffffff8216600160201b0267ffffffff0000000019909116178255505050565b60ff83165f908152600360209081526040808320815180830190925263ffffffff438116835285811683850190815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055505050565b815463ffffffff438116911603610d03576001820181905561075e565b60ff939093165f90815260026020818152604080842063ffffffff968716855282528084208151808301909252438716825281830197885280546001808201835591865292909420905191909202909101805463ffffffff1916919094161783559251919092015550565b5f5f5f60408486031215610d80575f5ffd5b83359250602084013567ffffffffffffffff811115610d9d575f5ffd5b8401601f81018613610dad575f5ffd5b803567ffffffffffffffff811115610dc3575f5ffd5b866020828401011115610dd4575f5ffd5b939660209190910195509293505050565b602080825282518282018190525f918401906040840190835b81811015610e2257835163ffffffff16835260209384019390920191600101610dfe565b509095945050505050565b803560ff81168114610e3d575f5ffd5b919050565b803563ffffffff81168114610e3d575f5ffd5b5f5f60408385031215610e66575f5ffd5b610e6f83610e2d565b9150610e7d60208401610e42565b90509250929050565b5f60208284031215610e96575f5ffd5b61039182610e2d565b5f5f5f60608486031215610eb1575f5ffd5b610eba84610e2d565b9250610ec860208501610e42565b9150610ed660408501610e42565b90509250925092565b602080825282518282018190525f918401906040840190835b81811015610e22578351835260209384019390920191600101610ef8565b5f5f60408385031215610f27575f5ffd5b610f3083610e2d565b946020939093013593505050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b63ffffffff82811682821603908111156103d7576103d7610f66565b63ffffffff81811683821601908111156103d7576103d7610f66565b818103818111156103d7576103d7610f66565b5f81610fd357610fd3610f66565b505f19019056fea2646970667358221220a71d38b1c4eaee7a2304107558d2f809cb2cf83665850ab95ca02f97568e761564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCAW_5`\xE0\x1C\x80c\x81!\x90o\x11a\0\x88W\x80c\xBD)\xB8\xCD\x11a\0cW\x80c\xBD)\xB8\xCD\x14a\x02$W\x80c\xCA\xA3\xCDv\x14a\x027W\x80c\xE2\xE6\x85\x80\x14a\x02LW\x80c\xF3A\t\"\x14a\x02|W__\xFD[\x80c\x81!\x90o\x14a\x01\xBAW\x80c\x89\x02bE\x14a\x01\xF1W\x80c\xA4\x8B\xB0\xAC\x14a\x02\x11W__\xFD[\x80b\xBF\xF0M\x14a\0\xCEW\x80c\x12\xD1\xD7M\x14a\0\xF7W\x80c&\xD9A\xF2\x14a\x01+W\x80c.\xD5\x83\xE5\x14a\x01@W\x80cm\x14\xA9\x87\x14a\x01SW\x80c{]&\xBE\x14a\x01\x92W[__\xFD[a\0\xE1a\0\xDC6`\x04a\rnV[a\x02\x8FV[`@Qa\0\xEE\x91\x90a\r\xE5V[`@Q\x80\x91\x03\x90\xF3[a\x01\na\x01\x056`\x04a\x0EUV[a\x03\x98V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xEEV[a\x01>a\x0196`\x04a\x0E\x86V[a\x03\xDDV[\0[a\x01\na\x01N6`\x04a\x0E\x9FV[a\x04vV[a\x01z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\x01\xA5a\x01\xA06`\x04a\x0EUV[a\x04\xF9V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xEEV[a\x01\xCDa\x01\xC86`\x04a\x0E\x86V[a\x05\x04V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\0\xEEV[a\x02\x04a\x01\xFF6`\x04a\x0EUV[a\x05JV[`@Qa\0\xEE\x91\x90a\x0E\xDFV[a\x01\xCDa\x02\x1F6`\x04a\x0EUV[a\x06*V[a\x01>a\x0226`\x04a\rnV[a\x06\x9EV[a\x02>_\x81V[`@Q\x90\x81R` \x01a\0\xEEV[a\x01\xA5a\x02Z6`\x04a\x0F\x16V[`\x01` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xA5a\x02\x8A6`\x04a\x0E\x86V[a\x07dV[``a\x02\x99a\x07\x82V[_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xB3Wa\x02\xB3a\x0F>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xDCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x03\x8DW_\x85\x85\x83\x81\x81\x10a\x02\xFCWa\x02\xFCa\x0FRV[\x91\x90\x91\x015`\xF8\x1C_\x81\x81R`\x03` R`@\x81 T\x91\x93P\x90\x91P\x81\x90\x03a\x038W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03B\x83a\x07\xCDV[\x90Pa\x03Y\x89\x84a\x03T`\x01\x85a\x0FzV[a\x08\xC4V[\x80\x85\x85\x81Q\x81\x10a\x03lWa\x03la\x0FRV[c\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPPP`\x01\x01a\x02\xE1V[P\x90P[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x03\xB5\x83\x83a\tLV[`@\x80Q\x80\x82\x01\x90\x91R\x81Tc\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01T` \x82\x01R\x90P[\x92\x91PPV[a\x03\xE5a\x07\x82V[`\xFF\x81\x16_\x90\x81R`\x03` R`@\x90 T\x15a\x04\x15W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x82\x84\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x84\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x92R\x90\x91 \x80T\x90\x91\x84\x16\x90\x81\x10a\x04\xC1Wa\x04\xC1a\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x90P\x93\x92PPPV[_a\x03\x91\x83\x83a\t\xA1V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x05 \x82a\n\xD7V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16` \x82\x01R\x92\x91PPV[``_a\x05W\x84\x84a\t\xA1V[\x90P_\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05yWa\x05ya\x0F>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xA2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x06!Wa\x05\xC0\x86\x82\x87a\x0B\x16V[\x82\x82\x81Q\x81\x10a\x05\xD2Wa\x05\xD2a\x0FRV[` \x02` \x01\x01\x81\x81RPP__\x1B\x82\x82\x81Q\x81\x10a\x05\xF3Wa\x05\xF3a\x0FRV[` \x02` \x01\x01Q\x03a\x06\x19W`@Qc\x7F\x12\t\x8D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x05\xA7V[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x06fWa\x06fa\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x06\xA6a\x07\x82V[_[\x81\x81\x10\x15a\x07^W_\x83\x83\x83\x81\x81\x10a\x06\xC3Wa\x06\xC3a\x0FRV[\x91\x90\x91\x015`\xF8\x1C_\x81\x81R`\x03` R`@\x81 T\x91\x93P\x90\x91P\x81\x90\x03a\x06\xFFW`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x81 Tc\xFF\xFF\xFF\xFF\x16\x90a\x07,\x84a\x0B\xE9V[\x90P_a\x079\x85\x83a\x0C!V[\x90P\x80\x89\x14a\x07MWa\x07M\x81\x86\x85a\x08\xC4V[PP`\x01\x90\x93\x01\x92Pa\x06\xA8\x91PPV[PPPPV[_a\x07n\x82a\n\xD7V[T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xCBW`@QcC\x94\xDB\xDF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[__a\x07\xD8\x83a\n\xD7V[\x80T\x90\x91P_\x90a\x07\xF7\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x0F\x96V[\x90Pa\x08\x04\x84\x83\x83a\x0CIV[`\xFF\x84\x16_\x90\x81R`\x02` R`@\x81 \x90a\x08!`\x01\x84a\x0FzV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T\x90\x03a\x03\x91W`\xFF\x84\x16_\x90\x81R`\x02` R`@\x81 \x90a\x08\\`\x01\x84a\x0FzV[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01_\x90\x81 \x83Q\x80\x85\x01\x90\x94RC\x83\x16\x84R\x83\x85\x01\x82\x81R\x81T`\x01\x80\x82\x01\x84U\x92\x84R\x95\x90\x92 \x93Q`\x02\x90\x95\x02\x90\x93\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x91Q\x91\x01U\x93\x92PPPV[_a\x08\xCF\x83\x83a\tLV[\x90Pa\x08\xDD\x83\x83\x83\x87a\x0C\xE6V[`\xFF\x83\x16_\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x85\x91\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x90a\t|`\x01\x83a\x0F\xB2V[\x81T\x81\x10a\t\x8CWa\t\x8Ca\x0FRV[\x90_R` _ \x90`\x02\x02\x01\x91PP\x92\x91PPV[`\xFF\x82\x16_\x90\x81R`\x03` R`@\x81 T\x80[\x80\x15a\nFW`\xFF\x85\x16_\x90\x81R`\x03` R`@\x81 a\t\xD7`\x01\x84a\x0F\xB2V[\x81T\x81\x10a\t\xE7Wa\t\xE7a\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\n3W` \x01Q\x92Pa\x03\xD7\x91PPV[P\x80a\n>\x81a\x0F\xC5V[\x91PPa\t\xB5V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x03` R`@\x81 \x80T\x90a\n\xF6`\x01\x83a\x0F\xB2V[\x81T\x81\x10a\x0B\x06Wa\x0B\x06a\x0FRV[\x90_R` _ \x01\x91PP\x91\x90PV[`\xFF\x83\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a\x0B\xDEW`\xFF\x86\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 a\x0Bn`\x01\x84a\x0F\xB2V[\x81T\x81\x10a\x0B~Wa\x0B~a\x0FRV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\x0B\xCBW` \x01Q\x92Pa\x03\x91\x91PPV[P\x80a\x0B\xD6\x81a\x0F\xC5V[\x91PPa\x0B;V[P_\x95\x94PPPPPV[__a\x0B\xF4\x83a\n\xD7V[\x80T\x90\x91P_\x90a\x0C\x14\x90`\x01\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x0FzV[\x90Pa\x03\x91\x84\x83\x83a\x0CIV[__a\x0C-\x84\x84a\tLV[`\x01\x81\x01T\x90\x91Pa\x0CA\x85\x85\x84_a\x0C\xE6V[\x94\x93PPPPV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a\x0C~W\x81Tc\xFF\xFF\xFF\xFF\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x17\x82UPPPV[`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x85\x81\x16\x83\x85\x01\x90\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPPPV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a\r\x03W`\x01\x82\x01\x81\x90Ua\x07^V[`\xFF\x93\x90\x93\x16_\x90\x81R`\x02` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x96\x87\x16\x85R\x82R\x80\x84 \x81Q\x80\x83\x01\x90\x92RC\x87\x16\x82R\x81\x83\x01\x97\x88R\x80T`\x01\x80\x82\x01\x83U\x91\x86R\x92\x90\x94 \x90Q\x91\x90\x92\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x94\x16\x17\x83U\x92Q\x91\x90\x92\x01UPV[___`@\x84\x86\x03\x12\x15a\r\x80W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x9DW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\r\xADW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xC3W__\xFD[\x86` \x82\x84\x01\x01\x11\x15a\r\xD4W__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x0E\"W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\xFEV[P\x90\x95\x94PPPPPV[\x805`\xFF\x81\x16\x81\x14a\x0E=W__\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E=W__\xFD[__`@\x83\x85\x03\x12\x15a\x0EfW__\xFD[a\x0Eo\x83a\x0E-V[\x91Pa\x0E}` \x84\x01a\x0EBV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x0E\x96W__\xFD[a\x03\x91\x82a\x0E-V[___``\x84\x86\x03\x12\x15a\x0E\xB1W__\xFD[a\x0E\xBA\x84a\x0E-V[\x92Pa\x0E\xC8` \x85\x01a\x0EBV[\x91Pa\x0E\xD6`@\x85\x01a\x0EBV[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x0E\"W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\xF8V[__`@\x83\x85\x03\x12\x15a\x0F'W__\xFD[a\x0F0\x83a\x0E-V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x0FfV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x0FfV[\x81\x81\x03\x81\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x0FfV[_\x81a\x0F\xD3Wa\x0F\xD3a\x0FfV[P_\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \xA7\x1D8\xB1\xC4\xEA\xEEz#\x04\x10uX\xD2\xF8\t\xCB,\xF86e\x85\n\xB9\\\xA0/\x97V\x8Ev\x15dsolcC\0\x08\x1B\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyRegistryCoordinator()` and selector `0x8729b7be`.
```solidity
error OnlyRegistryCoordinator();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinator;
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
        impl ::core::convert::From<OnlyRegistryCoordinator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [135u8, 41u8, 183u8, 190u8];
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
    /**Custom error with signature `OperatorIdDoesNotExist()` and selector `0xfe24131a`.
```solidity
error OperatorIdDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorIdDoesNotExist;
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
        impl ::core::convert::From<OperatorIdDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorIdDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorIdDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorIdDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorIdDoesNotExist()";
            const SELECTOR: [u8; 4] = [254u8, 36u8, 19u8, 26u8];
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
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
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
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
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
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `QuorumIndexUpdate(bytes32,uint8,uint32)` and selector `0x6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6`.
```solidity
event QuorumIndexUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint32 newOperatorIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumIndexUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub newOperatorIndex: u32,
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
        impl alloy_sol_types::SolEvent for QuorumIndexUpdate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "QuorumIndexUpdate(bytes32,uint8,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8, 225u8, 228u8, 244u8, 7u8, 95u8, 61u8, 6u8, 113u8, 118u8, 20u8,
                13u8, 52u8, 232u8, 120u8, 116u8, 36u8, 77u8, 210u8, 115u8, 41u8, 76u8,
                5u8, 178u8, 33u8, 129u8, 51u8, 228u8, 154u8, 43u8, 166u8, 246u8,
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
                    newOperatorIndex: data.1,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.newOperatorIndex),
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
        impl alloy_sol_types::private::IntoLogData for QuorumIndexUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumIndexUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumIndexUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _slashingRegistryCoordinator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _slashingRegistryCoordinator: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._slashingRegistryCoordinator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _slashingRegistryCoordinator: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `OPERATOR_DOES_NOT_EXIST_ID()` and selector `0xcaa3cd76`.
```solidity
function OPERATOR_DOES_NOT_EXIST_ID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_DOES_NOT_EXIST_IDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`OPERATOR_DOES_NOT_EXIST_ID()`](OPERATOR_DOES_NOT_EXIST_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_DOES_NOT_EXIST_IDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<OPERATOR_DOES_NOT_EXIST_IDCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_DOES_NOT_EXIST_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_DOES_NOT_EXIST_IDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<OPERATOR_DOES_NOT_EXIST_IDReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_DOES_NOT_EXIST_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_DOES_NOT_EXIST_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_DOES_NOT_EXIST_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_DOES_NOT_EXIST_ID()";
            const SELECTOR: [u8; 4] = [202u8, 163u8, 205u8, 118u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
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
                        let r: OPERATOR_DOES_NOT_EXIST_IDReturn = r.into();
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
                        let r: OPERATOR_DOES_NOT_EXIST_IDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `currentOperatorIndex(uint8,bytes32)` and selector `0xe2e68580`.
```solidity
function currentOperatorIndex(uint8, bytes32) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentOperatorIndexCall {
        #[allow(missing_docs)]
        pub _0: u8,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`currentOperatorIndex(uint8,bytes32)`](currentOperatorIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentOperatorIndexReturn {
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<currentOperatorIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentOperatorIndexCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentOperatorIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<currentOperatorIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentOperatorIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentOperatorIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentOperatorIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentOperatorIndex(uint8,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 230u8, 133u8, 128u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
                        let r: currentOperatorIndexReturn = r.into();
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
                        let r: currentOperatorIndexReturn = r.into();
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
    /**Function with signature `getLatestOperatorUpdate(uint8,uint32)` and selector `0x12d1d74d`.
```solidity
function getLatestOperatorUpdate(uint8 quorumNumber, uint32 operatorIndex) external view returns (IIndexRegistryTypes.OperatorUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestOperatorUpdateCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorIndex: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getLatestOperatorUpdate(uint8,uint32)`](getLatestOperatorUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestOperatorUpdateReturn {
        #[allow(missing_docs)]
        pub _0: <IIndexRegistryTypes::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestOperatorUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorUpdateCall) -> Self {
                    (value.quorumNumber, value.operatorIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestOperatorUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorIndex: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistryTypes::OperatorUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistryTypes::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestOperatorUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestOperatorUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestOperatorUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IIndexRegistryTypes::OperatorUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IIndexRegistryTypes::OperatorUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestOperatorUpdate(uint8,uint32)";
            const SELECTOR: [u8; 4] = [18u8, 209u8, 215u8, 77u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IIndexRegistryTypes::OperatorUpdate as alloy_sol_types::SolType>::tokenize(
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
                        let r: getLatestOperatorUpdateReturn = r.into();
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
                        let r: getLatestOperatorUpdateReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getLatestQuorumUpdate(uint8)` and selector `0x8121906f`.
```solidity
function getLatestQuorumUpdate(uint8 quorumNumber) external view returns (IIndexRegistryTypes.QuorumUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestQuorumUpdateCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getLatestQuorumUpdate(uint8)`](getLatestQuorumUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestQuorumUpdateReturn {
        #[allow(missing_docs)]
        pub _0: <IIndexRegistryTypes::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestQuorumUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestQuorumUpdateCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestQuorumUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistryTypes::QuorumUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistryTypes::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestQuorumUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestQuorumUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestQuorumUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestQuorumUpdateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IIndexRegistryTypes::QuorumUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IIndexRegistryTypes::QuorumUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestQuorumUpdate(uint8)";
            const SELECTOR: [u8; 4] = [129u8, 33u8, 144u8, 111u8];
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
                    <IIndexRegistryTypes::QuorumUpdate as alloy_sol_types::SolType>::tokenize(
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
                        let r: getLatestQuorumUpdateReturn = r.into();
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
                        let r: getLatestQuorumUpdateReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorListAtBlockNumber(uint8,uint32)` and selector `0x89026245`.
```solidity
function getOperatorListAtBlockNumber(uint8 quorumNumber, uint32 blockNumber) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorListAtBlockNumberCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorListAtBlockNumber(uint8,uint32)`](getOperatorListAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorListAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getOperatorListAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorListAtBlockNumberCall) -> Self {
                    (value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorListAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getOperatorListAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorListAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorListAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorListAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorListAtBlockNumber(uint8,uint32)";
            const SELECTOR: [u8; 4] = [137u8, 2u8, 98u8, 69u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getOperatorListAtBlockNumberReturn = r.into();
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
                        let r: getOperatorListAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorUpdateAtIndex(uint8,uint32,uint32)` and selector `0x2ed583e5`.
```solidity
function getOperatorUpdateAtIndex(uint8 quorumNumber, uint32 operatorIndex, uint32 arrayIndex) external view returns (IIndexRegistryTypes.OperatorUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorUpdateAtIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorIndex: u32,
        #[allow(missing_docs)]
        pub arrayIndex: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorUpdateAtIndex(uint8,uint32,uint32)`](getOperatorUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorUpdateAtIndexReturn {
        #[allow(missing_docs)]
        pub _0: <IIndexRegistryTypes::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32, u32);
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
            impl ::core::convert::From<getOperatorUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorIndex, value.arrayIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorIndex: tuple.1,
                        arrayIndex: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistryTypes::OperatorUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistryTypes::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IIndexRegistryTypes::OperatorUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IIndexRegistryTypes::OperatorUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorUpdateAtIndex(uint8,uint32,uint32)";
            const SELECTOR: [u8; 4] = [46u8, 213u8, 131u8, 229u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.arrayIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IIndexRegistryTypes::OperatorUpdate as alloy_sol_types::SolType>::tokenize(
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
                        let r: getOperatorUpdateAtIndexReturn = r.into();
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
                        let r: getOperatorUpdateAtIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getQuorumUpdateAtIndex(uint8,uint32)` and selector `0xa48bb0ac`.
```solidity
function getQuorumUpdateAtIndex(uint8 quorumNumber, uint32 quorumIndex) external view returns (IIndexRegistryTypes.QuorumUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumUpdateAtIndexCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub quorumIndex: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getQuorumUpdateAtIndex(uint8,uint32)`](getQuorumUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumUpdateAtIndexReturn {
        #[allow(missing_docs)]
        pub _0: <IIndexRegistryTypes::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQuorumUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.quorumIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        quorumIndex: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistryTypes::QuorumUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistryTypes::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQuorumUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IIndexRegistryTypes::QuorumUpdate as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IIndexRegistryTypes::QuorumUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumUpdateAtIndex(uint8,uint32)";
            const SELECTOR: [u8; 4] = [164u8, 139u8, 176u8, 172u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IIndexRegistryTypes::QuorumUpdate as alloy_sol_types::SolType>::tokenize(
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
                        let r: getQuorumUpdateAtIndexReturn = r.into();
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
                        let r: getQuorumUpdateAtIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initializeQuorum(uint8)` and selector `0x26d941f2`.
```solidity
function initializeQuorum(uint8 quorumNumber) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`initializeQuorum(uint8)`](initializeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumReturn {}
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
            impl ::core::convert::From<initializeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<initializeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeQuorumReturn {
            fn _tokenize(
                &self,
            ) -> <initializeQuorumCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeQuorum(uint8)";
            const SELECTOR: [u8; 4] = [38u8, 217u8, 65u8, 242u8];
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
                initializeQuorumReturn::_tokenize(ret)
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
    /**Function with signature `registerOperator(bytes32,bytes)` and selector `0x00bff04d`.
```solidity
function registerOperator(bytes32 operatorId, bytes memory quorumNumbers) external returns (uint32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registerOperator(bytes32,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
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
            impl ::core::convert::From<registerOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
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
            impl ::core::convert::From<registerOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
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
            const SIGNATURE: &'static str = "registerOperator(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [0u8, 191u8, 240u8, 77u8];
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
                        let r: registerOperatorReturn = r.into();
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
                        let r: registerOperatorReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `totalOperatorsForQuorum(uint8)` and selector `0xf3410922`.
```solidity
function totalOperatorsForQuorum(uint8 quorumNumber) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`totalOperatorsForQuorum(uint8)`](totalOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumReturn {
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
            impl ::core::convert::From<totalOperatorsForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalOperatorsForQuorumCall {
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
            impl ::core::convert::From<totalOperatorsForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalOperatorsForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalOperatorsForQuorum(uint8)";
            const SELECTOR: [u8; 4] = [243u8, 65u8, 9u8, 34u8];
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
                        let r: totalOperatorsForQuorumReturn = r.into();
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
                        let r: totalOperatorsForQuorumReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `totalOperatorsForQuorumAtBlockNumber(uint8,uint32)` and selector `0x7b5d26be`.
```solidity
function totalOperatorsForQuorumAtBlockNumber(uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumAtBlockNumberCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`totalOperatorsForQuorumAtBlockNumber(uint8,uint32)`](totalOperatorsForQuorumAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumAtBlockNumberReturn {
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
            impl ::core::convert::From<totalOperatorsForQuorumAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumAtBlockNumberCall) -> Self {
                    (value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalOperatorsForQuorumAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
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
            impl ::core::convert::From<totalOperatorsForQuorumAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalOperatorsForQuorumAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalOperatorsForQuorumAtBlockNumberCall {
            type Parameters<'a> = (
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
            const SIGNATURE: &'static str = "totalOperatorsForQuorumAtBlockNumber(uint8,uint32)";
            const SELECTOR: [u8; 4] = [123u8, 93u8, 38u8, 190u8];
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
                        let r: totalOperatorsForQuorumAtBlockNumberReturn = r.into();
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
                        let r: totalOperatorsForQuorumAtBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`IndexRegistry`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum IndexRegistryCalls {
        #[allow(missing_docs)]
        OPERATOR_DOES_NOT_EXIST_ID(OPERATOR_DOES_NOT_EXIST_IDCall),
        #[allow(missing_docs)]
        currentOperatorIndex(currentOperatorIndexCall),
        #[allow(missing_docs)]
        deregisterOperator(deregisterOperatorCall),
        #[allow(missing_docs)]
        getLatestOperatorUpdate(getLatestOperatorUpdateCall),
        #[allow(missing_docs)]
        getLatestQuorumUpdate(getLatestQuorumUpdateCall),
        #[allow(missing_docs)]
        getOperatorListAtBlockNumber(getOperatorListAtBlockNumberCall),
        #[allow(missing_docs)]
        getOperatorUpdateAtIndex(getOperatorUpdateAtIndexCall),
        #[allow(missing_docs)]
        getQuorumUpdateAtIndex(getQuorumUpdateAtIndexCall),
        #[allow(missing_docs)]
        initializeQuorum(initializeQuorumCall),
        #[allow(missing_docs)]
        registerOperator(registerOperatorCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        totalOperatorsForQuorum(totalOperatorsForQuorumCall),
        #[allow(missing_docs)]
        totalOperatorsForQuorumAtBlockNumber(totalOperatorsForQuorumAtBlockNumberCall),
    }
    #[automatically_derived]
    impl IndexRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 191u8, 240u8, 77u8],
            [18u8, 209u8, 215u8, 77u8],
            [38u8, 217u8, 65u8, 242u8],
            [46u8, 213u8, 131u8, 229u8],
            [109u8, 20u8, 169u8, 135u8],
            [123u8, 93u8, 38u8, 190u8],
            [129u8, 33u8, 144u8, 111u8],
            [137u8, 2u8, 98u8, 69u8],
            [164u8, 139u8, 176u8, 172u8],
            [189u8, 41u8, 184u8, 205u8],
            [202u8, 163u8, 205u8, 118u8],
            [226u8, 230u8, 133u8, 128u8],
            [243u8, 65u8, 9u8, 34u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IndexRegistryCalls {
        const NAME: &'static str = "IndexRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_DOES_NOT_EXIST_ID(_) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentOperatorIndex(_) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestOperatorUpdate(_) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestQuorumUpdate(_) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorListAtBlockNumber(_) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorUpdateAtIndex(_) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumUpdateAtIndex(_) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeQuorum(_) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalOperatorsForQuorum(_) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalOperatorsForQuorumAtBlockNumber(_) => {
                    <totalOperatorsForQuorumAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<IndexRegistryCalls>] = &[
                {
                    fn registerOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getLatestOperatorUpdate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::getLatestOperatorUpdate)
                    }
                    getLatestOperatorUpdate
                },
                {
                    fn initializeQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::initializeQuorum)
                    }
                    initializeQuorum
                },
                {
                    fn getOperatorUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::getOperatorUpdateAtIndex)
                    }
                    getOperatorUpdateAtIndex
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn totalOperatorsForQuorumAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <totalOperatorsForQuorumAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IndexRegistryCalls::totalOperatorsForQuorumAtBlockNumber,
                            )
                    }
                    totalOperatorsForQuorumAtBlockNumber
                },
                {
                    fn getLatestQuorumUpdate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::getLatestQuorumUpdate)
                    }
                    getLatestQuorumUpdate
                },
                {
                    fn getOperatorListAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::getOperatorListAtBlockNumber)
                    }
                    getOperatorListAtBlockNumber
                },
                {
                    fn getQuorumUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::getQuorumUpdateAtIndex)
                    }
                    getQuorumUpdateAtIndex
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn OPERATOR_DOES_NOT_EXIST_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::OPERATOR_DOES_NOT_EXIST_ID)
                    }
                    OPERATOR_DOES_NOT_EXIST_ID
                },
                {
                    fn currentOperatorIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::currentOperatorIndex)
                    }
                    currentOperatorIndex
                },
                {
                    fn totalOperatorsForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryCalls::totalOperatorsForQuorum)
                    }
                    totalOperatorsForQuorum
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
            ) -> alloy_sol_types::Result<IndexRegistryCalls>] = &[
                {
                    fn registerOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getLatestOperatorUpdate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::getLatestOperatorUpdate)
                    }
                    getLatestOperatorUpdate
                },
                {
                    fn initializeQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::initializeQuorum)
                    }
                    initializeQuorum
                },
                {
                    fn getOperatorUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::getOperatorUpdateAtIndex)
                    }
                    getOperatorUpdateAtIndex
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn totalOperatorsForQuorumAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <totalOperatorsForQuorumAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IndexRegistryCalls::totalOperatorsForQuorumAtBlockNumber,
                            )
                    }
                    totalOperatorsForQuorumAtBlockNumber
                },
                {
                    fn getLatestQuorumUpdate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::getLatestQuorumUpdate)
                    }
                    getLatestQuorumUpdate
                },
                {
                    fn getOperatorListAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::getOperatorListAtBlockNumber)
                    }
                    getOperatorListAtBlockNumber
                },
                {
                    fn getQuorumUpdateAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::getQuorumUpdateAtIndex)
                    }
                    getQuorumUpdateAtIndex
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn OPERATOR_DOES_NOT_EXIST_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::OPERATOR_DOES_NOT_EXIST_ID)
                    }
                    OPERATOR_DOES_NOT_EXIST_ID
                },
                {
                    fn currentOperatorIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::currentOperatorIndex)
                    }
                    currentOperatorIndex
                },
                {
                    fn totalOperatorsForQuorum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryCalls::totalOperatorsForQuorum)
                    }
                    totalOperatorsForQuorum
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
                Self::OPERATOR_DOES_NOT_EXIST_ID(inner) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentOperatorIndex(inner) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestOperatorUpdate(inner) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestQuorumUpdate(inner) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorListAtBlockNumber(inner) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorUpdateAtIndex(inner) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumUpdateAtIndex(inner) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::totalOperatorsForQuorum(inner) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::totalOperatorsForQuorumAtBlockNumber(inner) => {
                    <totalOperatorsForQuorumAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OPERATOR_DOES_NOT_EXIST_ID(inner) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentOperatorIndex(inner) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getLatestOperatorUpdate(inner) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestQuorumUpdate(inner) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorListAtBlockNumber(inner) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorUpdateAtIndex(inner) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumUpdateAtIndex(inner) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::totalOperatorsForQuorum(inner) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::totalOperatorsForQuorumAtBlockNumber(inner) => {
                    <totalOperatorsForQuorumAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IndexRegistry`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum IndexRegistryErrors {
        #[allow(missing_docs)]
        OnlyRegistryCoordinator(OnlyRegistryCoordinator),
        #[allow(missing_docs)]
        OperatorIdDoesNotExist(OperatorIdDoesNotExist),
        #[allow(missing_docs)]
        QuorumDoesNotExist(QuorumDoesNotExist),
    }
    #[automatically_derived]
    impl IndexRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [135u8, 41u8, 183u8, 190u8],
            [230u8, 33u8, 159u8, 234u8],
            [254u8, 36u8, 19u8, 26u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IndexRegistryErrors {
        const NAME: &'static str = "IndexRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OnlyRegistryCoordinator(_) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorIdDoesNotExist(_) => {
                    <OperatorIdDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumDoesNotExist(_) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<IndexRegistryErrors>] = &[
                {
                    fn OnlyRegistryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryErrors> {
                        <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryErrors::OnlyRegistryCoordinator)
                    }
                    OnlyRegistryCoordinator
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
                {
                    fn OperatorIdDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryErrors> {
                        <OperatorIdDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IndexRegistryErrors::OperatorIdDoesNotExist)
                    }
                    OperatorIdDoesNotExist
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
            ) -> alloy_sol_types::Result<IndexRegistryErrors>] = &[
                {
                    fn OnlyRegistryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryErrors> {
                        <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryErrors::OnlyRegistryCoordinator)
                    }
                    OnlyRegistryCoordinator
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
                {
                    fn OperatorIdDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IndexRegistryErrors> {
                        <OperatorIdDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IndexRegistryErrors::OperatorIdDoesNotExist)
                    }
                    OperatorIdDoesNotExist
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
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorIdDoesNotExist(inner) => {
                    <OperatorIdDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorIdDoesNotExist(inner) => {
                    <OperatorIdDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`IndexRegistry`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum IndexRegistryEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        QuorumIndexUpdate(QuorumIndexUpdate),
    }
    #[automatically_derived]
    impl IndexRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                110u8, 225u8, 228u8, 244u8, 7u8, 95u8, 61u8, 6u8, 113u8, 118u8, 20u8,
                13u8, 52u8, 232u8, 120u8, 116u8, 36u8, 77u8, 210u8, 115u8, 41u8, 76u8,
                5u8, 178u8, 33u8, 129u8, 51u8, 228u8, 154u8, 43u8, 166u8, 246u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IndexRegistryEvents {
        const NAME: &'static str = "IndexRegistryEvents";
        const COUNT: usize = 2usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <QuorumIndexUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <QuorumIndexUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::QuorumIndexUpdate)
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
    impl alloy_sol_types::private::IntoLogData for IndexRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumIndexUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumIndexUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IndexRegistry`](self) contract instance.

See the [wrapper's documentation](`IndexRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IndexRegistryInstance<P, N> {
        IndexRegistryInstance::<P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IndexRegistryInstance<P, N>>,
    > {
        IndexRegistryInstance::<P, N>::deploy(provider, _slashingRegistryCoordinator)
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
    ) -> alloy_contract::RawCallBuilder<P, N> {
        IndexRegistryInstance::<
            P,
            N,
        >::deploy_builder(provider, _slashingRegistryCoordinator)
    }
    /**A [`IndexRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IndexRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IndexRegistryInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IndexRegistryInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IndexRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IndexRegistryInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IndexRegistry`](self) contract instance.

See the [wrapper's documentation](`IndexRegistryInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IndexRegistryInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _slashingRegistryCoordinator,
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
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _slashingRegistryCoordinator,
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
    impl<P: ::core::clone::Clone, N> IndexRegistryInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IndexRegistryInstance<P, N> {
            IndexRegistryInstance {
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
    > IndexRegistryInstance<P, N> {
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
        ///Creates a new call builder for the [`OPERATOR_DOES_NOT_EXIST_ID`] function.
        pub fn OPERATOR_DOES_NOT_EXIST_ID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, OPERATOR_DOES_NOT_EXIST_IDCall, N> {
            self.call_builder(&OPERATOR_DOES_NOT_EXIST_IDCall)
        }
        ///Creates a new call builder for the [`currentOperatorIndex`] function.
        pub fn currentOperatorIndex(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, currentOperatorIndexCall, N> {
            self.call_builder(&currentOperatorIndexCall { _0, _1 })
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
        ///Creates a new call builder for the [`getLatestOperatorUpdate`] function.
        pub fn getLatestOperatorUpdate(
            &self,
            quorumNumber: u8,
            operatorIndex: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getLatestOperatorUpdateCall, N> {
            self.call_builder(
                &getLatestOperatorUpdateCall {
                    quorumNumber,
                    operatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getLatestQuorumUpdate`] function.
        pub fn getLatestQuorumUpdate(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, getLatestQuorumUpdateCall, N> {
            self.call_builder(
                &getLatestQuorumUpdateCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorListAtBlockNumber`] function.
        pub fn getOperatorListAtBlockNumber(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorListAtBlockNumberCall, N> {
            self.call_builder(
                &getOperatorListAtBlockNumberCall {
                    quorumNumber,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorUpdateAtIndex`] function.
        pub fn getOperatorUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorIndex: u32,
            arrayIndex: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorUpdateAtIndexCall, N> {
            self.call_builder(
                &getOperatorUpdateAtIndexCall {
                    quorumNumber,
                    operatorIndex,
                    arrayIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumUpdateAtIndex`] function.
        pub fn getQuorumUpdateAtIndex(
            &self,
            quorumNumber: u8,
            quorumIndex: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getQuorumUpdateAtIndexCall, N> {
            self.call_builder(
                &getQuorumUpdateAtIndexCall {
                    quorumNumber,
                    quorumIndex,
                },
            )
        }
        ///Creates a new call builder for the [`initializeQuorum`] function.
        pub fn initializeQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, initializeQuorumCall, N> {
            self.call_builder(
                &initializeQuorumCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
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
        ///Creates a new call builder for the [`totalOperatorsForQuorum`] function.
        pub fn totalOperatorsForQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<&P, totalOperatorsForQuorumCall, N> {
            self.call_builder(
                &totalOperatorsForQuorumCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`totalOperatorsForQuorumAtBlockNumber`] function.
        pub fn totalOperatorsForQuorumAtBlockNumber(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            totalOperatorsForQuorumAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &totalOperatorsForQuorumAtBlockNumberCall {
                    quorumNumber,
                    blockNumber,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IndexRegistryInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`QuorumIndexUpdate`] event.
        pub fn QuorumIndexUpdate_filter(
            &self,
        ) -> alloy_contract::Event<&P, QuorumIndexUpdate, N> {
            self.event_filter::<QuorumIndexUpdate>()
        }
    }
}
