# AllowedItemWithDuration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | Terminal group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**delivery_duration_in_minutes** | **i64** | Delivery duration in minutes. | 
**zone** | Option<**String**> | Delivery zone name which this TerminalGroupId belongs to. | 
**delivery_service_product_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Link to \"delivery service payment\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


