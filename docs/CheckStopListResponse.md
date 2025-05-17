# CheckStopListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Operation ID. | 
**rejected_items** | Option<[**Vec<models::StopListItem>**](StopListItem.md)> | Set of items in out-of-stock list.                If null, none of requested items are in out-of-stock list.  > Present in response only if **not null**. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


