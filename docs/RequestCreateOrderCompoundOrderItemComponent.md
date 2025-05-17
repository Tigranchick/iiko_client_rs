# RequestCreateOrderCompoundOrderItemComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Item ID. | 
**modifiers** | Option<[**Vec<models::RequestCreateOrderModifier>**](RequestCreateOrderModifier.md)> | Modifiers. | [optional]
**price** | Option<**f64**> | Price. | [optional]
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the item in the order.  MUST be unique for the whole system. Therefore it must be generated with Guid.NewGuid().  > If sent null, it generates automatically on iikoTransport side. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


