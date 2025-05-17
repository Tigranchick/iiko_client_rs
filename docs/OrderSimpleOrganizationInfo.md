# OrderSimpleOrganizationInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response_type** | **String** |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**name** | Option<**String**> | Organization name. | 
**code** | Option<**String**> | Organization`s code. | [optional]
**external_data** | Option<[**Vec<models::ExternalData>**](ExternalData.md)> | Organization`s external data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


