# MarketingSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Marketing source ID. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**name** | **String** | Marketing source name. | 
**external_revision** | Option<**i64**> | External system revision number. | [optional]
**is_deleted** | Option<**bool**> | IsDeleted attribute of marketing source. | [optional]
**attached_sources** | **Vec<String>** | Attached marketing sources. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


