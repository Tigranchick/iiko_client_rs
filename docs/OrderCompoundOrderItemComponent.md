# OrderCompoundOrderItemComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product** | [**models::OrderProduct**](OrderProduct.md) | Item. | 
**modifiers** | Option<[**Vec<models::OrderItemModifier>**](OrderItemModifier.md)> | Modifiers. | [optional]
**price** | **f64** | Price per item unit. Can be sent different from the price in the base menu. | 
**cost** | **f64** | Item total including tax, discounts/surcharges. | 
**price_predefined** | **bool** | Whether price is predefined. | 
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the item in the order and for the whole system. | [optional]
**tax_percent** | Option<**f64**> | Tax rate. | [optional]
**result_sum** | Option<**f64**> | Total amount per item including tax, discounts/surcharges. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


