# RestaurantSection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Restaurant section ID. | 
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | Terminal group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**name** | **String** | Name. | 
**tables** | [**Vec<models::Table>**](Table.md) | Tables. | 
**schema** | Option<[**models::SectionSchema**](SectionSchema.md)> | Table layout. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


