# AvailablePayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Marketing campaign id. | [optional]
**max_sum** | Option<**f64**> | Max sum. | [optional]
**order** | Option<**i32**> | Payment order. In case of partial payment, payments with lesser order should be filled first. | [optional]
**wallet_infos** | Option<[**Vec<models::WalletInfo>**](WalletInfo.md)> | Wallet infos. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


