# GetTransactionsReportByRevisionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Customer id. | 
**revision** | Option<**i64**> | Report since revision. Included if LastTransactionId set.. | [optional]
**last_transaction_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Report since transaction. Excluded. Can't be used without revision.. | [optional]
**page_size** | **i32** | Page size. Ignored if more than max size on server.. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


