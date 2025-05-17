# PaymentType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Payment type ID | [optional]
**code** | Option<**String**> | Payment type code | [optional]
**name** | Option<**String**> | Payment type name | [optional]
**comment** | Option<**String**> | Payment type comment | [optional]
**combinable** | Option<**bool**> | Combinability attribute | [optional]
**external_revision** | Option<**i64**> | External system revision number. | [optional]
**applicable_marketing_campaigns** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Array of marketing campaigns associated with LoyaltyApp payment type applicable to this organization. | 
**is_deleted** | Option<**bool**> | IsDeleted attribute of payment type. | [optional]
**print_cheque** | Option<**bool**> | If true, payment type is fiscal and bill will be printed. | [optional]
**payment_processing_type** | Option<[**models::PaymentProcessingType**](PaymentProcessingType.md)> | Describes operation processing type. | [optional]
**payment_type_kind** | Option<[**models::PaymentTypeKind**](PaymentTypeKind.md)> | Payment type category. | [optional]
**terminal_groups** | [**Vec<models::TerminalGroup>**](TerminalGroup.md) | Terminal groups where this payment type is available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


