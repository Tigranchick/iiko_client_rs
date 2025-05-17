# RequestCreateOrderLoyaltyCardPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_type_kind** | **String** |  | 
**sum** | **f64** | Amount due. | 
**payment_type_id** | [**uuid::Uuid**](uuid::Uuid.md) | Payment type.                 Can be obtained by `/api/1/payment_types` operation. | 
**is_processed_externally** | Option<**bool**> | Whether payment item is processed by external payment system (made from outside). | [optional]
**payment_additional_data** | Option<[**models::PaymentAdditionalData**](PaymentAdditionalData.md)> | Additional payment parameters. | [optional]
**is_fiscalized_externally** | Option<**bool**> | Whether the payment item is externally fiscalized.   > Allowed from version `7.6.3`. | [optional]
**is_prepay** | Option<**bool**> | Whether the payment item is prepay. Unavailable for `paymentKindType.LoyaltyCard`.   > Allowed from version `8.2.6`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


