# ReserveInWorkload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Banquet/reserve ID. | 
**table_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Reserved tables. | 
**estimated_start_time** | **String** | Estimated time when reserve will be closed or banquet will be started (Local for the terminal). | 
**duration_in_minutes** | **i64** | Estimated banquet duration. | 
**guests_count** | **i32** | Number of guests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


