# MarketingCampaignInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Marketing campaign id. | [optional]
**program_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Loyalty program id. | [optional]
**name** | Option<**String**> | Loyalty program name. Can be null. | [optional]
**description** | Option<**String**> | Marketing campaign description. Can be null. | [optional]
**is_active** | Option<**bool**> | Marketing campaign is active. | [optional]
**period_from** | Option<**String**> | Marketing campaign works since date. | [optional]
**period_to** | Option<**String**> | Marketing campaign works till date. Null means limitless. | [optional]
**order_action_condition_bindings** | Option<[**Vec<models::MarketingCampaignActionConditionBindingInfo>**](MarketingCampaignActionConditionBindingInfo.md)> | Conditions and actions that will be checked when order is processed. | [optional]
**periodic_action_condition_bindings** | Option<[**Vec<models::MarketingCampaignActionConditionBindingInfo>**](MarketingCampaignActionConditionBindingInfo.md)> | Conditions and actions that will be checked by schedule. | [optional]
**overdraft_action_condition_bindings** | Option<[**Vec<models::MarketingCampaignActionConditionBindingInfo>**](MarketingCampaignActionConditionBindingInfo.md)> | Conditions and actions that will be checked by overdraft. | [optional]
**guest_registration_action_condition_bindings** | Option<[**Vec<models::MarketingCampaignActionConditionBindingInfo>**](MarketingCampaignActionConditionBindingInfo.md)> | Conditions and actions that will be checked by guest registration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


