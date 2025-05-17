# OrderServiceOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service** | [**models::OrderProduct**](OrderProduct.md) | Item. | 
**cost** | **f64** | Total cost per item without tax, discounts/surcharges. | 
**r#type** | **String** |  | 
**status** | [**models::OrderItemStatus**](OrderItemStatus.md) | Item cooking status. | 
**deleted** | Option<[**models::OrderItemDeletedInfo**](OrderItemDeletedInfo.md)> | Item deletion details. If filled up, item is deleted. | [optional]
**amount** | **f64** | Quantity. | 
**comment** | Option<**String**> | Comment. | [optional]
**when_printed** | Option<**String**> | Printing time (Local for the terminal). | [optional]
**size** | Option<[**models::OrderProductSize**](OrderProductSize.md)> | Size. | [optional]
**combo_information** | Option<[**models::OrderComboItemInformation**](OrderComboItemInformation.md)> | Combo details, if order item is part of combo. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


