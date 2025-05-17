# ReserveUpdateWebHookEventInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_type** | Option<**String**> | Event type. | [optional]
**event_time** | Option<**String**> | Event date and time (UTC). | [optional]
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization ID.                Can be obtained by `/api/1/organizations` operation. | [optional]
**correlation_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Operation ID. | [optional]
**event_info** | Option<[**models::ReserveInfo**](ReserveInfo.md)> | Event details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


