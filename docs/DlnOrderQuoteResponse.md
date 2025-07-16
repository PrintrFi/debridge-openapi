# DlnOrderQuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**estimation** | [**models::DlnOrderEstimation**](DlnOrderEstimation.md) |  | 
**prepended_operating_expense_cost** | Option<**String**> |  | [optional]
**tx** | Option<[**models::TxQuote**](TxQuote.md)> |  | [optional]
**order** | [**models::Order**](Order.md) |  | 
**fix_fee** | **String** |  | 
**user_points** | **f64** |  | 
**integrator_points** | **f64** |  | 
**actual_user_points** | Option<**f64**> |  | [optional]
**actual_integrator_points** | Option<**f64**> |  | [optional]
**usd_price_impact** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


