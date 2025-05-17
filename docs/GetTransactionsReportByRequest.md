# GetTransactionsReportByRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Customer id. | 
**date_from** | **String** | Report since date in UTC. Included. | 
**date_to** | **String** | Report till date in UTC. Included. | 
**page_number** | **i32** | Page number. Zero based. Previous pages will be skipped. | 
**page_size** | **i32** | Page size. Ignored if more than max page size on server. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


