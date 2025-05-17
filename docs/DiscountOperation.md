# DiscountOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<[**models::OperationCode**](OperationCode.md)> | Operation Type Code.  <br>0 - fixed discount for the entire order,<br />1 - fixed discount for the item,<br />2 - free product,<br />3 - other type of discounts. | [optional]
**order_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Deprecated, use positionId. | [optional]
**position_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id of item the discount is applied to. If null - discount applied to whole orders. | [optional]
**discount_sum** | Option<**f64**> | Discount sum. | [optional]
**amount** | Option<**f64**> | Amount. | [optional]
**comment** | Option<**String**> | Comment. Can be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


