# RequestCreateOrderProductOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of menu item.                Can be obtained by `/api/1/nomenclature` operation. | 
**modifiers** | Option<[**Vec<models::RequestCreateOrderModifier>**](RequestCreateOrderModifier.md)> | Modifiers. | [optional]
**price** | **f64** | Price per item unit. Can be sent different from the price in the base menu. | 
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the item in the order.  MUST be unique for the whole system. Therefore it must be generated with Guid.NewGuid().  > If sent null, it generates automatically on iikoTransport side. | [optional]
**r#type** | **String** |  | 
**amount** | **f64** | Quantity. | 
**product_size_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Size ID. Required if a stock list item has a size scale. | [optional]
**combo_information** | Option<[**models::RequestCreateOrderComboItemInformation**](RequestCreateOrderComboItemInformation.md)> | Combo details if combo includes order item. | [optional]
**comment** | Option<**String**> | Comment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


