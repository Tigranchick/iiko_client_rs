# GetTransactionsReportByRevisionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transactions** | Option<[**Vec<models::TransportTransactionsReportItem>**](TransportTransactionsReportItem.md)> | Transactions. | [optional]
**last_revision** | Option<**i64**> | Last known transaction revision. | [optional]
**last_transaction_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Last known transaction id. | [optional]
**page_size** | Option<**i32**> | Page size. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


