# RequestCreateOrderRmsDiscount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**discount_type_id** | [**uuid::Uuid**](uuid::Uuid.md) | Discount type.                 Can be obtained by `/api/1/discounts` operation. | 
**sum** | Option<**f64**> | Discount/surcharge sum. | [optional]
**selective_positions** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Order item positions. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


