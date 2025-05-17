# PersonalShift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Employee ID. | 
**role_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Employee Role ID. | [optional]
**opened** | **bool** | Personal shift state flag (true - shift is opened, false - shift is closed). | 
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the terminal group where the personal shift is opened/closed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


