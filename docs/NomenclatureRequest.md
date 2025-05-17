# NomenclatureRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**start_revision** | Option<**i64**> | The revision (version) of the menu saved on the integration side.  Use `0` for the first request for each organization. In every subsequent request,  the `startRevision` field should contain the value of the `revision` field received  in the response to the previous request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


