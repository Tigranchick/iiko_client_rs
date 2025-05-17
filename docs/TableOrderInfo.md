# TableOrderInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**pos_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | POS order ID. | [optional]
**external_number** | Option<**String**> | Order external number. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**timestamp** | **i64** | Timestamp of most recent order change that took place on iikoTransport server. | 
**creation_status** | [**models::OrderCreationStatus**](OrderCreationStatus.md) | Order creation status. In case of asynchronous creation, it allows to track the instance an order was validated/created in iikoFront. | 
**error_info** | Option<[**models::ErrorInfo**](ErrorInfo.md)> | Order creation error details.  > Required only if \"creationStatus\"=\"Error\". | [optional]
**order** | Option<[**models::TableOrder**](TableOrder.md)> | Order creation details.  > Field is filled up if \"creationStatus\"=\"Success\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


