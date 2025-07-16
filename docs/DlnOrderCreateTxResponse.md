# DlnOrderCreateTxResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**estimation** | [**models::DlnOrderEstimation**](DlnOrderEstimation.md) |  | 
**tx** | [**models::DlnTx**](DlnTx.md) |  | 
**order_id** | **String** |  | 
**prepended_operating_expense_cost** | Option<**String**> |  | [optional]
**order** | [**models::Order**](Order.md) |  | 
**fix_fee** | **String** |  | 
**user_points** | **f64** |  | 
**integrator_points** | **f64** |  | 
**actual_user_points** | Option<**f64**> |  | [optional]
**actual_integrator_points** | Option<**f64**> |  | [optional]
**estimated_transaction_fee** | Option<[**models::FeeInfo**](FeeInfo.md)> |  | [optional]
**usd_price_impact** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


