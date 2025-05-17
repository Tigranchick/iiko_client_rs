# ChildModifierInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID. | 
**default_amount** | Option<**i32**> | Default quantity. | [optional]
**min_amount** | **i32** | Minimum quantity. | 
**max_amount** | **i32** | Maximum quantity. | 
**required** | Option<**bool**> | Required availability. | [optional]
**hide_if_default_amount** | Option<**bool**> | Hide if default amount applied. This field is supported since 7.2.4 iikoRMS version. | [optional]
**splittable** | Option<**bool**> | Modifier can be split. This field is supported since 7.2.4 iikoRMS version. | [optional]
**free_of_charge_amount** | Option<**i32**> | Free of charge amount. This field is supported since 7.2.4 iikoRMS version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


