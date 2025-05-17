# OpenPersonalSessionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | Delivery group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**employee_id** | [**uuid::Uuid**](uuid::Uuid.md) | Employee ID. | 
**role_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Employee role ID.                Must be null if the restaurant doesn't use roles, otherwise not-null role must be specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


