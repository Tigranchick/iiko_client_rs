# OrderTipsPaymentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tips_type** | Option<[**models::OrderTipsType**](OrderTipsType.md)> | Tips type. | [optional]
**payment_type** | [**models::OrderPaymentType**](OrderPaymentType.md) | Payment type.                 Can be obtained by `/api/1/payment_types` operation. | 
**sum** | **f64** | Amount due. | 
**is_preliminary** | **bool** | Whether payment item is preliminary. | 
**is_external** | **bool** | Payment item is external (created via biz.API). | 
**is_processed_externally** | **bool** | Payment item is processed by external payment system. | 
**is_fiscalized_externally** | Option<**bool**> | Whether the payment item is externally fiscalized.   > Allowed from version `7.6.3`. | [optional]
**is_prepay** | **bool** | Whether the payment item is prepay.   > Allowed from version `7.7.6`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


