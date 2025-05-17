# MenuRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_menu_id** | **String** | External menu id                Can be obtained by `api/2/menu` operation. | 
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization IDs.                Can be obtained by `/api/1/organizations` operation. | 
**price_category_id** | Option<**String**> | Price category id.                Can be obtained by `api/2/menu` operation. | [optional]
**version** | Option<**i32**> | Version of the result data model. | [optional]
**language** | Option<**String**> | Language of the external menu. | [optional]
**async_mode** | Option<**bool**> | Async Mode. | [optional]
**start_revision** | Option<**i64**> | Start revision. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


