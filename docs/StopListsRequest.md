# StopListsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organizations for which out-of-stock lists will be requested.                Can be obtained by `/api/1/organizations` operation. | 
**return_size** | Option<**bool**> | Flag indicating the need to return the sizes of the dish. | [optional]
**terminal_groups_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of terminal groups for which you need to get out-of-stock lists.                Can be obtained by `/api/1/terminal_groups` operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


