# CourierLocationsByTimeOffsetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | List of organizations for drivers coordinates of which will be retrieved. | 
**offset_in_seconds** | Option<**i32**> | Interval in seconds from current server time.   If driver coordinates were recorded in server storage   within interval: (\"current server time\" - *OffsetInSeconds*, \"current server time\"],  driver and their coordinates will be retrieved. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


