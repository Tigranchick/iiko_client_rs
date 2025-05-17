# CalculateCheckinResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**loyalty_program_results** | Option<[**Vec<models::LoyaltyProgramResult>**](LoyaltyProgramResult.md)> | Loyalty program results. | [optional]
**available_payments** | Option<[**Vec<models::AvailablePayment>**](AvailablePayment.md)> | Marketing campaigns with available payments. | [optional]
**validation_warnings** | Option<**Vec<String>**> | Warnings about errors, not blocking loyalty calculation. | [optional]
**warnings** | Option<[**Vec<models::WarningInfo>**](WarningInfo.md)> | Warnings about errors, not blocking loyalty calculation. | [optional]
**loyalty_trace** | Option<**String**> | Loyalty trace. Can be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


