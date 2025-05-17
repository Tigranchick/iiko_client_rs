# CreateOrUpdateCustomerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Customer id. | [optional]
**phone** | Option<**String**> | Customer phone. Can be null. | [optional]
**card_track** | Option<**String**> | Card track. Required if cardNumber set. Can be null. | [optional]
**card_number** | Option<**String**> | Card number. Required if cardTrack set. Can be null. | [optional]
**name** | Option<**String**> | Customer name. Can be null. | [optional]
**middle_name** | Option<**String**> | Customer middle name. Can be null. | [optional]
**sur_name** | Option<**String**> | Customer surname. Can be null. | [optional]
**birthday** | Option<**String**> | Customer birthday. | [optional]
**email** | Option<**String**> | Customer email. Can be null. | [optional]
**sex** | Option<[**models::UserSex**](UserSex.md)> | Customer sex.  <br>0 - not specified,<br />1 - male,<br />2 - female. | [optional]
**consent_status** | Option<[**models::PersonalDataConsentStatus**](PersonalDataConsentStatus.md)> | Customer consent status.  <br>0 - unknown,<br />1 - given,<br />2 - revoked. | [optional]
**should_receive_loyalty_info** | Option<**bool**> | Customer get loyalty messages (email, sms). If the parameter is not specified for new customers, the value 'true' is used. | [optional]
**should_receive_promo_actions_info** | Option<**bool**> | Customer get promo messages (email, sms). If the parameter is not specified for new customers, the value 'true' is used. | [optional]
**referrer_id** | Option<**String**> | Id for referrer guest. Null for old integrations, Guid.Empty - for referrer deletion. Can be null. | [optional]
**user_data** | Option<**String**> | Customer user data. Can be null. | [optional]
**is_deleted** | Option<**bool**> | Customer logical deletion flag. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Customer organization id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


