# TerminalGroupsIsAliveRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization ID. Deprecated, use \"organizationIds\". | [optional]
**organization_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  Organization IDs.     Can be obtained by `/api/1/organizations` operation. | [optional]
**terminal_group_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | List of terminal groups IDs.                 Can be obtained by `/api/1/terminal_groups` operation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


