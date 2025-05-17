# MarketingCampaignActionConditionBindingInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id. | [optional]
**stop_further_execution** | Option<**bool**> | Loyalty processing stop after successful execution of binding. So means order of bindings affects. | [optional]
**actions** | Option<[**Vec<models::MarketingCampaignSettingsInfo>**](MarketingCampaignSettingsInfo.md)> | Marketing actions. | [optional]
**conditions** | Option<[**Vec<models::MarketingCampaignSettingsInfo>**](MarketingCampaignSettingsInfo.md)> | Marketing conditions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


