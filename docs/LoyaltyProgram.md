# LoyaltyProgram

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Program id. | [optional]
**name** | Option<**String**> | Program name. Can be null. | [optional]
**description** | Option<**String**> | Program description. Can be null. | [optional]
**service_from** | Option<**String**> | Program works since date. | [optional]
**service_to** | Option<**String**> | Program works till date. | [optional]
**notify_about_balance_changes** | Option<**bool**> | Notify customer when balance has changed (sms/push). | [optional]
**program_type** | Option<[**models::ProgramType**](ProgramType.md)> | Program type.  <br>0 - deposit or corporate nutrition,<br />1 - bonus program,<br />2 - products program,<br />3 - discount program,<br />4 - certificate program. | [optional]
**is_active** | Option<**bool**> | Program is active. | [optional]
**wallet_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Wallet id. Program has only wallet that means global payment type for customers. | [optional]
**marketing_campaigns** | Option<[**Vec<models::MarketingCampaignInfo>**](MarketingCampaignInfo.md)> | Program marketing campaigns. | [optional]
**applied_organizations** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Program applied organizations. | [optional]
**template_type** | Option<[**models::EnumsTemplateType**](EnumsTemplateType.md)> | Program template type.  <br>0 - None,<br />1 - BonusProgram,<br />2 - DiscountProgram,<br />3 - NthDishProgram,<br />4 - ManualOrderAnonymousDiscount,<br />5 - AutoOrderAnonymousDiscount,<br />6 - AutoDishAnonymousDiscount,<br />7 - PromotionsProgram,<br />8 - NthDishPromotionsProgram. | [optional]
**is_exchange_rate_enabled** | Option<**bool**> | Exchange rate for bonuses and real currency. | [optional]
**refill_type** | Option<[**models::EnumsRefillType**](EnumsRefillType.md)> | Refill type with payment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


