# CheckStopListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | Front group ID the order must be sent to.    Can be obtained by `/api/1/terminal_groups` operation. | 
**items** | [**Vec<models::RequestCreateOrderItem>**](RequestCreateOrderItem.md) | Order items. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


