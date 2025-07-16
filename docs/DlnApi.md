# \DlnApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dln_order_controller_v10_cancel_order**](DlnApi.md#dln_order_controller_v10_cancel_order) | **GET** /v1.0/dln/order/{id}/cancel-tx | Generates a transaction that cancels the given order
[**dln_order_controller_v10_create_order**](DlnApi.md#dln_order_controller_v10_create_order) | **GET** /v1.0/dln/order/create-tx | This endpoint returns the data for a transaction to place a cross-chain DLN order.
[**dln_order_controller_v10_ext_call_cancel_order**](DlnApi.md#dln_order_controller_v10_ext_call_cancel_order) | **GET** /v1.0/dln/order/{id}/extcall-cancel-tx | Generates a transaction that cancels external call in the given order
[**dln_order_controller_v10_get_order**](DlnApi.md#dln_order_controller_v10_get_order) | **GET** /v1.0/dln/order/{id} | This endpoint returns the data of order.
[**dln_order_controller_v10_get_order_ids_by_tx**](DlnApi.md#dln_order_controller_v10_get_order_ids_by_tx) | **GET** /v1.0/dln/tx/{hash}/order-ids | This endpoint returns the status of order.
[**dln_order_controller_v10_get_order_status**](DlnApi.md#dln_order_controller_v10_get_order_status) | **GET** /v1.0/dln/order/{id}/status | This endpoint returns the status of order.



## dln_order_controller_v10_cancel_order

> models::DlnOrderCancelTxResponse dln_order_controller_v10_cancel_order(id)
Generates a transaction that cancels the given order

This endpoint generates a transaction that cancels the given order. This transaction must be published to the destination chain of the order. Unlocked funds would be transferred to the address specified as the orderAuthority of the given order on the source chain. This transaction can only be executed by th orderAuthority of the given order on the destination chain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DlnOrderCancelTxResponse**](DlnOrderCancelTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dln_order_controller_v10_create_order

> models::DlnOrderControllerV10CreateOrder200Response dln_order_controller_v10_create_order(src_chain_id, src_chain_token_in, src_chain_token_in_amount, dst_chain_id, dst_chain_token_out, dst_chain_token_out_amount, additional_taker_reward_bps, src_intermediary_token_address, dst_intermediary_token_address, dst_intermediary_token_spender_address, intermediary_token_usd_price, dst_chain_token_out_recipient, sender_address, src_chain_order_authority_address, src_allowed_cancel_beneficiary, referral_code, affiliate_fee_percent, affiliate_fee_recipient, src_chain_token_in_sender_permit, dst_chain_order_authority_address, enable_estimate, allowed_taker, external_call, dln_hook, prepend_operating_expenses, metadata, otc, ptp, skip_solana_recipient_validation, src_chain_priority_level)
This endpoint returns the data for a transaction to place a cross-chain DLN order.

This endpoint returns the data for a transaction to place a cross-chain DLN order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**src_chain_id** | **String** | An ID of a source chain, a chain where the cross-chain swap will start | [required] |
**src_chain_token_in** | **String** | An address (on a source chain) of an input token to swap | [required] |
**src_chain_token_in_amount** | **String** | An amount of input tokens to swap | [required] |
**dst_chain_id** | **String** | An ID of a destination chain, a chain where the cross-chain swap will finish. Must differ from `srcChainId`! | [required] |[default to 56]
**dst_chain_token_out** | **String** | An address (on a destination chain) of a target token. | [required] |
**dst_chain_token_out_amount** | Option<**String**> | Amount of the target asset the market maker expects to receive upon order fulfillment. This property can be set to \"auto\" so that the API will suggest the best possible outcome of the order based on current market conditions and the protocol fees, keeping the order in a reasonable-to-fullfill state |  |
**additional_taker_reward_bps** | Option<**i32**> | `additionalTakerRewardBps` is additionally laid in on top of default taker margin. |  |
**src_intermediary_token_address** | Option<**String**> | An address (on a source chain) of an intermediary token a user's input funds should be swapped to prior order creation. This token must be publicly traded and listed on major DEX aggregators. Examples: USDC on Ethereum, DAI on Ethereum, USDC on Solana. This property can be either a Circle-issued USDC token or ETH, or an arbitrary token; setting an arbitrary token also requires you to specify `dstIntermediaryTokenAddress`, `intermediaryTokenUSDPrice` and `dstIntermediaryTokenSpenderAddress`  properties as well. |  |
**dst_intermediary_token_address** | Option<**String**> | An address (on a destination chain) of an intermediary token whose value assumed to be equal to the value of `srcIntermediaryTokenAddress` . This token must be publicly traded and listed on major DEX aggregators.  Examples: USDC on Ethereum, DAI on Ethereum, USDC on Solana. Setting this property also requires you to specify `srcIntermediaryTokenAddress`, `intermediaryTokenUSDPrice` and `dstIntermediaryTokenSpenderAddress` properties as well. |  |
**dst_intermediary_token_spender_address** | Option<**String**> | Applicable to a EVM-compatible destination chain. An address (on a EVM-compatible destination chain) assumed as a spender of the intermediary token (set as `dstIntermediaryTokenAddress`) during order fulfillment. This address must hold a reasonable amount (of at least $100 in value for testing purposes, or at least 10% of the average order value) of the given intermediary token, and issue on-chain approvals to spent the given  intermediary token by the two trusted DLN contracts (DlnDestination: {CURRENT_ENV[dstChain].pmmDst} and CrosschainForwarder: {CURRENT_ENV[dstChain].evm.forwarderContract}). This property is required when `srcIntermediaryTokenAddress` and `dstIntermediaryTokenAddress` are set. |  |
**intermediary_token_usd_price** | Option<**f64**> | A value (a spot price) of the given intermediary token expressed in US dollars. Keep in mind this price is not used to calculate the spread of an order, but only as a reference price for operating expenses incurred by a taker during and after order fulfillment. The order spread is still calculated using the market price quoted from a major DEX. |  |
**dst_chain_token_out_recipient** | Option<**String**> | Address (on the destination chain) where target tokens should be transferred to after the swap. **Required for transaction construction**, otherwise only the quote is returned! |  |
**sender_address** | Option<**String**> | Address (on the source chain) who submits input tokens for a cross-chain swap |  |
**src_chain_order_authority_address** | Option<**String**> | Address (on the source chain) who submits input tokens for a cross-chain swap. **Required for transaction construction**, otherwise only the quote is returned! |  |
**src_allowed_cancel_beneficiary** | Option<**String**> | Fixed recipient of the funds of an order in case it is being cancelled. If not set, the recipient could be set later upon order cancellation. |  |
**referral_code** | Option<**f64**> | Make sure you pass your referral code which can be generated here: https://app.debridge.finance/refer With your referral code, your address will earn an additional 25% of deBridge points (https://debridge.finance/blog/introducing-debridge-points/), on top of the points generated by users who engage in activity on deBridge through your referral. |  |
**affiliate_fee_percent** | Option<**f64**> |  The share of the input amount to be distributed to the `affiliateFeeRecipient` (if given) address as an affiliate fee.    If you are building on top of deSwap, you might want to take a small fee relative to the amount of a token a user provides for a swap. To achieve this, set this parameter to a desired % (e.g. `0.1` stands for 0.1%; the max value is `10` which stands for 10% of `srcAmountInParam`) and specify your wallet address in the `affiliateFeeRecipient` parameter.    For example, setting this parameter to `0.1` means that when the user gives 5000 USDT then you will receive 5 USDT immediately after transaction is being confirmed.        |  |
**affiliate_fee_recipient** | Option<**String**> | An address (on an origin chain) that will receive affiliate fees according to the `affiliateFeePercent` parameter. |  |
**src_chain_token_in_sender_permit** | Option<**String**> | Typically, a sender is required to approve token transfer to deBridge forwarder for further transfer and swap. For gas-less cases, a sender-signed permit message may be provided to be executed by deBridge forwarder before actual transfer. |  |
**dst_chain_order_authority_address** | Option<**String**> | Address on the destination chain whom should be granted the privileges to manage the order (patch, cancel, etc). **Required for transaction construction**, otherwise only the quote is returned! |  |
**enable_estimate** | Option<**bool**> | This flag forces deSwap API to validate the resulting transaction and estimate its gas consumption. You will find the estimation at `tx.gasLimit` field of the resulting object.  **Important:** to estimate a transaction, you are required to fill the `senderAddress` property with the address that will be used to execute this transaction. It should have enough assets on its balance to cover the amount specified in the `srcChainTokenInAmount` property, and enough ether to cover the protocol global fixed fee.  **Caution:** if the input token (`srcChainTokenIn`) is not a native blockchain currency but an ERC-20 token, it is necessary to provide an approve to spend this token by the `tx.allowanceTarget` contract prior to such estimation. This can be done either by executing `increaseAllowance()` on-chain or by providing the permit envelope via the `srcChainTokenInSenderPermit` property. Failing to provide a correct approve to spend will result an error during transaction. |  |
**allowed_taker** | Option<**String**> | An address (on a destination chain) of a allowed taker. |  |
**external_call** | Option<**String**> | A stringified versioned JSON with data fields that form an encoded transaction call to be attached to a DLN order and would be executed upon order fulfillment on the destination chain. The following schemas are currently supported:       - for Solana as a destination chain: `{     \"version\": \"solana_1\",     \"fields\": {         \"calldata\": \"0x...\"     } }`       - for EVM-compatible chains (gas optional): `{     \"version\": \"evm_1\",     \"fields\": {         \"to\": \"0x...\",         \"data\": \"0x...\",         \"gas\": 0     } }`      |  |
**dln_hook** | Option<**String**> | JSON representing a DLN Hook to be attached to an order. Depending on the destination chain, the following templates are available:        - for Solana as a destination chain: `{     \"type\": \"solana_serialized_instructions\",     \"data\": {       \"instructions\": \"0x...\"     } }`        - for EVM-based destination chains to make an arbitrary transaction call via the universal DLN hook: `{     \"type\": \"evm_transaction_call\",     \"data\": {       \"to\": \"0x...\", \"       calldata\": \"0x...\",       \"gas\": 0     } }`        - for EVM-based destination chains to attach an arbitrary hook: `{   \"type\": \"evm_hook_data_v1\",   \"data\": {     \"fallbackAddress\": \"0x...\",     \"target\": \"0x...\",     \"reward\": \"0\",     \"isNonAtomic\": false,     \"isSuccessRequired\": false,     \"targetPayload\": \"0x\"    } }`  |  |
**prepend_operating_expenses** | Option<**bool**> | Tells API server to prepend operating expenses to the input amount |  |[default to false]
**metadata** | Option<**String**> |  |  |
**otc** | Option<**bool**> | Forces a P2P order where input and output tokens are left intact(renamed to ptp) |  |
**ptp** | Option<**bool**> | Forces a P2P order where input and output tokens are left intact |  |
**skip_solana_recipient_validation** | Option<**bool**> | Skip system address validation `dstChainTokenOutRecipient` in Solana  |  |[default to false]
**src_chain_priority_level** | Option<**String**> | Configures the priority level for transaction fee estimation. |  |[default to normal]

### Return type

[**models::DlnOrderControllerV10CreateOrder200Response**](DlnOrderControllerV10_createOrder_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dln_order_controller_v10_ext_call_cancel_order

> models::DlnOrderCancelTxResponse dln_order_controller_v10_ext_call_cancel_order(id)
Generates a transaction that cancels external call in the given order

This endpoint generates a transaction that cancels external call in the given order. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DlnOrderCancelTxResponse**](DlnOrderCancelTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dln_order_controller_v10_get_order

> models::DlnOrderResponse dln_order_controller_v10_get_order(id)
This endpoint returns the data of order.

This endpoint returns the data of order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DlnOrderResponse**](DlnOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dln_order_controller_v10_get_order_ids_by_tx

> models::DlnOrderidsByTxResponse dln_order_controller_v10_get_order_ids_by_tx(hash)
This endpoint returns the status of order.

This endpoint returns the status of order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

[**models::DlnOrderidsByTxResponse**](DlnOrderidsByTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dln_order_controller_v10_get_order_status

> models::DlnOrderStatusResponse dln_order_controller_v10_get_order_status(id)
This endpoint returns the status of order.

This endpoint returns the status of order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DlnOrderStatusResponse**](DlnOrderStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

