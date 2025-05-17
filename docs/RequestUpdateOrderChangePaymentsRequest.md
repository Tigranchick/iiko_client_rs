# RequestUpdateOrderChangePaymentsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**payments** | [**Vec<models::RequestCreateOrderPayment>**](RequestCreateOrderPayment.md) | Order payments. | 
**tips** | Option<[**Vec<models::RequestCreateOrderTipsPayment>**](RequestCreateOrderTipsPayment.md)> | Order tips. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


