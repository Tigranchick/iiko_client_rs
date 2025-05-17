# RequestCreateOrderCombo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Combo ID.  MUST be unique for the whole system. Therefore it must be generated with Guid.NewGuid(). | 
**name** | **String** | Name of combo. | 
**amount** | **i32** | Quantity. | 
**price** | **f64** | Price of one combo. | 
**source_id** | [**uuid::Uuid**](uuid::Uuid.md) | Combo validity ID. | 
**program_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Card program ID.   > Allowed from version `7.6.1`. | [optional]
**size_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Size ID. Required if combo has a size scale.   > Allowed from version `8.5.6`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


