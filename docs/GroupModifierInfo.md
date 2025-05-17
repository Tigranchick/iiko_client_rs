# GroupModifierInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID. | 
**min_amount** | **i32** | Minimum quantity. | 
**max_amount** | **i32** | Maximum quantity. | 
**required** | Option<**bool**> | Required availability. | 
**child_modifiers_have_min_max_restrictions** | Option<**bool**> | Presence of max/min quantity limitations of child modifiers. | [optional]
**child_modifiers** | [**Vec<models::ChildModifierInfo>**](ChildModifierInfo.md) | List of child modifiers. | 
**hide_if_default_amount** | Option<**bool**> | Hide if the amount is by default. This field is supported since 7.2.4 iikoRMS version. | [optional]
**default_amount** | Option<**i32**> | Amount by default. This field is supported since 7.2.4 iikoRMS version. | [optional]
**splittable** | Option<**bool**> | Modifier can be split. This field is supported since 7.2.4 iikoRMS version. | [optional]
**free_of_charge_amount** | Option<**i32**> | Free amount. This field is supported since 7.2.4 iikoRMS version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


