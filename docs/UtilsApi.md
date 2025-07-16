# \UtilsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_controller_v10_get_supported_chain_info_response**](UtilsApi.md#app_controller_v10_get_supported_chain_info_response) | **GET** /v1.0/supported-chains-info | 
[**app_controller_v10_get_supported_chain_response**](UtilsApi.md#app_controller_v10_get_supported_chain_response) | **GET** /v1.0/supported-chains | 
[**app_controller_v10_get_tokens**](UtilsApi.md#app_controller_v10_get_tokens) | **GET** /v1.0/token-list | 



## app_controller_v10_get_supported_chain_info_response

> models::SupportedChainsInfoResponse app_controller_v10_get_supported_chain_info_response()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SupportedChainsInfoResponse**](SupportedChainsInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_controller_v10_get_supported_chain_response

> models::SupportedChainsResponse app_controller_v10_get_supported_chain_response()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SupportedChainsResponse**](SupportedChainsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_controller_v10_get_tokens

> models::TokenListResponse app_controller_v10_get_tokens(chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | **String** | ID of a chain | [required] |

### Return type

[**models::TokenListResponse**](TokenListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

