# TipsType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Tips type ID.                Can be obtained by `/api/1/tips_types` operation. | 
**name** | **String** | Tips type name. | 
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Supported organizations IDs.                Can be obtained by `/api/1/organizations` operation. | 
**order_service_types** | [**Vec<models::DeliveryOrderServiceType>**](DeliveryOrderServiceType.md) | Supported order service types. | 
**payment_types_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Supported payment types IDs. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


