# ChangeUserBalanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Customer id. | [optional]
**wallet_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Wallet id. | [optional]
**sum** | Option<**f64**> | Sum of balance change. Must be possible. | [optional]
**comment** | Option<**String**> | Comment. Can be null. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


