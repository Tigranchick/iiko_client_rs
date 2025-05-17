# GetCustomerInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Guest id. | [optional]
**referrer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Guest referrer id. | [optional]
**name** | Option<**String**> | Guest name. Can be null. | [optional]
**surname** | Option<**String**> | Guest surname. Can be null. | [optional]
**middle_name** | Option<**String**> | Guest middle name. Can be null. | [optional]
**comment** | Option<**String**> | Guest comment. Can be null. | [optional]
**phone** | Option<**String**> | Main customer's phone. Can be null. | [optional]
**culture_name** | Option<**String**> | Guest culture name. Can be null. | [optional]
**birthday** | Option<**String**> | Guest birthday. | [optional]
**email** | Option<**String**> | Guest email. Can be null. | [optional]
**sex** | Option<[**models::UserSex**](UserSex.md)> | Sex.  <br>0 - not specified,<br />1 - male,<br />2 - female. | [optional]
**consent_status** | Option<[**models::PersonalDataConsentStatus**](PersonalDataConsentStatus.md)> | Guest consent status.  <br>0 - unknown,<br />1 - given,<br />2 - revoked. | [optional]
**anonymized** | Option<**bool**> | Guest anonymized. | [optional]
**cards** | Option<[**Vec<models::GuestCardInfo>**](GuestCardInfo.md)> | Customer's cards. | [optional]
**categories** | Option<[**Vec<models::GuestCategoryShortInfo>**](GuestCategoryShortInfo.md)> | Customer categories. | [optional]
**wallet_balances** | Option<[**Vec<models::GuestBalanceInfo>**](GuestBalanceInfo.md)> | Customer's user wallets. Contains bonus balances of different loyalty programs. | [optional]
**user_data** | Option<**String**> | Technical user data, customizable by restaurateur. Can be null. | [optional]
**should_receive_promo_actions_info** | Option<**bool**> | Customer get promo messages (email, sms). If null - unknown. | [optional]
**should_receive_loyalty_info** | Option<**bool**> | Guest should receive loyalty info. | [optional]
**should_receive_order_status_info** | Option<**bool**> | Guest should receive order status info. | [optional]
**personal_data_consent_from** | Option<**String**> | Guest personal data consent from. | [optional]
**personal_data_consent_to** | Option<**String**> | Guest personal data consent to. | [optional]
**personal_data_processing_from** | Option<**String**> | Guest personal data processing from. | [optional]
**personal_data_processing_to** | Option<**String**> | Guest personal data processing to. | [optional]
**is_deleted** | Option<**bool**> | Customer marked as deleted. | [optional]
**when_registered** | Option<**String**> | Registration date. | [optional]
**last_processed_order_date** | Option<**String**> | Last order date. | [optional]
**first_order_date** | Option<**String**> | First order date. | [optional]
**last_visited_organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Guest last visited organization id. | [optional]
**registration_organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Guest registration organization id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


