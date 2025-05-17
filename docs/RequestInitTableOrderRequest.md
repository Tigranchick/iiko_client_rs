# RequestInitTableOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | Terminal group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**table_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Table IDs.                Can be obtained by `/api/1/reserve/available_restaurant_sections` operation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


