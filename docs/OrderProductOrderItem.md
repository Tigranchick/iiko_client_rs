# OrderProductOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product** | [**models::OrderProduct**](OrderProduct.md) | Item. | 
**modifiers** | Option<[**Vec<models::OrderItemModifier>**](OrderItemModifier.md)> | Modifiers. | [optional]
**price** | **f64** | Price per item unit. Can be sent different from the price in the base menu. | 
**cost** | **f64** | Total cost per item without tax, discounts/surcharges. | 
**price_predefined** | **bool** | Whether price is predefined. | 
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the item in the order and for the whole system. | [optional]
**tax_percent** | Option<**f64**> | Tax rate. | [optional]
**result_sum** | Option<**f64**> | Total amount per item including tax, discounts/surcharges. | [optional]
**r#type** | **String** |  | 
**status** | [**models::OrderItemStatus**](OrderItemStatus.md) | Item cooking status. | 
**deleted** | Option<[**models::OrderItemDeletedInfo**](OrderItemDeletedInfo.md)> | Item deletion details. If filled up, item is deleted. | [optional]
**amount** | **f64** | Quantity. | 
**comment** | Option<**String**> | Comment. | [optional]
**when_printed** | Option<**String**> | Printing time (Local for the terminal). | [optional]
**size** | Option<[**models::OrderProductSize**](OrderProductSize.md)> | Size. | [optional]
**combo_information** | Option<[**models::OrderComboItemInformation**](OrderComboItemInformation.md)> | Combo details, if order item is part of combo. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


