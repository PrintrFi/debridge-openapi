# \SingleChainSwapApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**single_swap_controller_v10_get_chain_estimation**](SingleChainSwapApi.md#single_swap_controller_v10_get_chain_estimation) | **GET** /v1.0/chain/estimation | 
[**single_swap_controller_v10_get_chain_transaction**](SingleChainSwapApi.md#single_swap_controller_v10_get_chain_transaction) | **GET** /v1.0/chain/transaction | 



## single_swap_controller_v10_get_chain_estimation

> models::ChainEstimationResponseV10 single_swap_controller_v10_get_chain_estimation(chain_id, token_in, token_in_amount, token_out, slippage, affiliate_fee_percent, affiliate_fee_recipient)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | **String** | An ID of a chain, a chain where the swap must be performed | [required] |
**token_in** | **String** | An address of an input token to swap | [required] |
**token_in_amount** | **String** | An amount of input tokens to swap | [required] |
**token_out** | **String** | An address of a target token. | [required] |
**slippage** | Option<**String**> | A slippage constraint (in %) is a safeguard during swaps (on both source and destination chains, if applicable). It is also used to calculate the minimum possible outcome during estimation. This property can be set to \"auto\" so that the API will suggest the best possible slippage. |  |[default to auto]
**affiliate_fee_percent** | Option<**f64**> |   The share of the input amount to be distributed to the `affiliateFeeRecipient` (if given) address as an affiliate fee.      If you are building on top of deSwap, you might want to take a small fee relative to the amount of a token a user provides for a swap. To achieve this, set this parameter to a desired % (e.g. `0.1` stands for 0.1%; the max value is `10` which stands for 10% of `srcAmountInParam`) and specify your wallet address in the `affiliateFeeRecipient` parameter.      For example, setting this parameter to `0.1` means that when the user gives 5000 USDT then you will receive 5 USDT immediately after transaction is being confirmed.         |  |
**affiliate_fee_recipient** | Option<**String**> | An address (on an origin chain) that will receive affiliate fees according to the `affiliateFeePercent` parameter. |  |

### Return type

[**models::ChainEstimationResponseV10**](ChainEstimationResponseV10.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## single_swap_controller_v10_get_chain_transaction

> models::ChainTransactionResponseV10 single_swap_controller_v10_get_chain_transaction(chain_id, token_in, token_in_amount, token_out, token_out_recipient, slippage, affiliate_fee_percent, affiliate_fee_recipient, sender_address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | **String** | An ID of a chain, a chain where the swap must be performed | [required] |
**token_in** | **String** | An address of an input token to swap | [required] |
**token_in_amount** | **String** | An amount of input tokens to swap | [required] |
**token_out** | **String** | An address of a target token. | [required] |
**token_out_recipient** | **String** |  | [required] |
**slippage** | Option<**String**> | A slippage constraint (in %) is a safeguard during swaps (on both source and destination chains, if applicable). It is also used to calculate the minimum possible outcome during estimation. This property can be set to \"auto\" so that the API will suggest the best possible slippage. |  |[default to auto]
**affiliate_fee_percent** | Option<**f64**> |   The share of the input amount to be distributed to the `affiliateFeeRecipient` (if given) address as an affiliate fee.      If you are building on top of deSwap, you might want to take a small fee relative to the amount of a token a user provides for a swap. To achieve this, set this parameter to a desired % (e.g. `0.1` stands for 0.1%; the max value is `10` which stands for 10% of `srcAmountInParam`) and specify your wallet address in the `affiliateFeeRecipient` parameter.      For example, setting this parameter to `0.1` means that when the user gives 5000 USDT then you will receive 5 USDT immediately after transaction is being confirmed.         |  |
**affiliate_fee_recipient** | Option<**String**> | An address (on an origin chain) that will receive affiliate fees according to the `affiliateFeePercent` parameter. For solana affiliate fee you have to create & provide Jupiter's referral key (https://referral.jup.ag/dashboard). |  |
**sender_address** | Option<**String**> | Address (on the source chain) who submits input tokens for a cross-chain swap |  |

### Return type

[**models::ChainTransactionResponseV10**](ChainTransactionResponseV10.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

