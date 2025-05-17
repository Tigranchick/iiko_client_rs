# AddOrderPaymentsToBanquetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reserve_id** | [**uuid::Uuid**](uuid::Uuid.md) | Reserve ID. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**payments** | [**Vec<models::RequestCreateOrderPayment>**](RequestCreateOrderPayment.md) | Order payments. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


