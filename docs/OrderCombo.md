# OrderCombo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Combo ID. | 
**name** | **String** | Name of combo. | 
**amount** | **i32** | Number of combos. | 
**price** | **f64** | Price of combo. Given for 1 combo, without regard to amount. | 
**source_id** | [**uuid::Uuid**](uuid::Uuid.md) | Combo action ID. | 
**size** | Option<[**models::OrderProductSize**](OrderProductSize.md)> | Size. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


