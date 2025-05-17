# Reserve

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer** | [**models::OrderCustomer**](OrderCustomer.md) | Client that placed the reserve. | 
**guests_count** | **i32** | Estimated guests count. | 
**comment** | Option<**String**> | Optional comment for reserve or banquet. | [optional]
**duration_in_minutes** | **i64** | Estimated banquet duration. | 
**should_remind** | **bool** | Whether to remind staff to prepare table beforehand. | 
**status** | [**models::ReserveStatus**](ReserveStatus.md) | Status of the reserve or banquet. | 
**cancel_reason** | Option<[**models::ReserveCancelReason**](ReserveCancelReason.md)> | The reserve cancellation reason or null if the reserve hasn't been canceled. | [optional]
**table_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Reserved table IDs. | 
**estimated_start_time** | **String** | Estimated time when reserve will be closed or banquet will be started. | 
**guests_coming_time** | Option<**String**> | Time when guests came and reserve was closed or banquet was started. | [optional]
**phone** | Option<**String**> | Telephone number. | [optional]
**event_type** | Option<**String**> | Event type.   > Allowed from version `8.5.6`. | [optional]
**order** | Option<[**models::ReserveOrder**](ReserveOrder.md)> | Order Used only at a banquet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


