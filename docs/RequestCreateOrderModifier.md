# RequestCreateOrderModifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Modifier item ID.                Can be obtained by `/api/1/nomenclature` operation. | 
**amount** | **f64** | Quantity. | 
**product_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Modifiers group ID (for group modifier). Required for a group modifier.                Can be obtained by `/api/1/nomenclature` operation. | [optional]
**price** | Option<**f64**> | Unit price. | [optional]
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the item in the order.  MUST be unique for the whole system. Therefore it must be generated with Guid.NewGuid().  > If sent null, it generates automatically on iikoTransport side. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


