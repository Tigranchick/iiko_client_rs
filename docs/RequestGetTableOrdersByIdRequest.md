# RequestGetTableOrdersByIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization IDs.                Can be obtained by `/api/1/organizations` operation. | 
**order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Order IDs.                > Required if \"posOrderIds\" is null. Must be null if \"posOrderIds\" is not null. | [optional]
**pos_order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | POS order IDs.                > Required if \"orderIds\" is null. Must be null if \"orderIds\" is not null. | [optional]
**return_external_data_keys** | Option<**Vec<String>**> | Keys for retrun external data information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


