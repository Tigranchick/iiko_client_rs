# TerminalGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Delivery group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID to which delivery group belongs.                Can be obtained by `/api/1/organizations` operation. | 
**name** | **String** | Terminal group name. | 
**address** | Option<**String**> | Group address. Not used. | 
**time_zone** | **String** | Terminal group time zone. | 
**external_data** | Option<[**Vec<models::ExternalData>**](ExternalData.md)> | Terminal group external data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


