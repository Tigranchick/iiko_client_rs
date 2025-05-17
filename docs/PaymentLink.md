# PaymentLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique key. | 
**integration_type** | Option<**String**> | Integration type code. | [optional]
**status** | [**models::PaymentLinkStatus**](PaymentLinkStatus.md) | Payment link transport status. | 
**url** | Option<**String**> | Payload. | [optional]
**updated_at** | **String** | Last update date (UTC). | 
**status_text** | Option<**String**> | Payment status (equals Payment link status if empty). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


