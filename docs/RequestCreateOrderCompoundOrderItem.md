# RequestCreateOrderCompoundOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**primary_component** | [**models::RequestCreateOrderCompoundOrderItemComponent**](RequestCreateOrderCompoundOrderItemComponent.md) | Main component. | 
**secondary_component** | Option<[**models::RequestCreateOrderCompoundOrderItemComponent**](RequestCreateOrderCompoundOrderItemComponent.md)> | Minor component. | [optional]
**common_modifiers** | Option<[**Vec<models::RequestCreateOrderModifier>**](RequestCreateOrderModifier.md)> | Indivisible modifiers. | [optional]
**r#type** | **String** |  | 
**amount** | **f64** | Quantity. | 
**product_size_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Size ID. Required if a stock list item has a size scale. | [optional]
**combo_information** | Option<[**models::RequestCreateOrderComboItemInformation**](RequestCreateOrderComboItemInformation.md)> | Combo details if combo includes order item. | [optional]
**comment** | Option<**String**> | Comment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


