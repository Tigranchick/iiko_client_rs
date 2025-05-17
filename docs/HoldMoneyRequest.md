# HoldMoneyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Predefined transaction id. Random if empty. | [optional]
**customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Customer id. | 
**wallet_id** | [**uuid::Uuid**](uuid::Uuid.md) | Wallet id. | 
**sum** | **f64** | Sum. | 
**comment** | Option<**String**> | Additional information about holding. Can be null. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


