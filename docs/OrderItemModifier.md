# OrderItemModifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product** | [**models::OrderProduct**](OrderProduct.md) | Item. | 
**amount** | **f64** | Quantity. | 
**amount_independent_of_parent_amount** | **bool** | Whether quantity of modifier depends on quantity of item. | 
**product_group** | Option<[**models::OrderProductGroup**](OrderProductGroup.md)> | Group of modifiers (in case of a group modifier). | [optional]
**price** | **f64** | Price per item unit. Can be sent different from the price in RMS. | 
**price_predefined** | **bool** | Whether price is predefined. | 
**result_sum** | **f64** | Total amount per item including tax, discounts/surcharges. | 
**deleted** | Option<[**models::OrderItemDeletedInfo**](OrderItemDeletedInfo.md)> | Item deletion details. If specified, the item is deleted. | [optional]
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the item in the order and for the whole system. | [optional]
**default_amount** | Option<**i32**> | Default amount. | [optional]
**hide_if_default_amount** | Option<**bool**> | Hide modifier in UI if \"amount\" equals \"defaultAmount\". | [optional]
**tax_percent** | Option<**f64**> | Tax rate. | [optional]
**free_of_charge_amount** | Option<**i32**> | Free of charge amount. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


