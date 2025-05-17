# RequestGetTableOrdersByTableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**table_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Table IDs.                Can be obtained by `/api/1/reserve/available_restaurant_sections` operation. | 
**statuses** | Option<[**Vec<models::OrderStatus>**](OrderStatus.md)> | Order statuses. | [optional]
**date_from** | Option<**String**> | Order creation date (terminal time zone). Lower limit.                Order details are stored for 90 days. | [optional]
**date_to** | Option<**String**> | Order creation date (terminal time zone). Upper limit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


