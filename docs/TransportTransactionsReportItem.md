# TransportTransactionsReportItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_client_login** | Option<**String**> | Api client login. Can be null. | [optional]
**balance_after** | Option<**f64**> | Balance after. | [optional]
**balance_before** | Option<**f64**> | Balance before. | [optional]
**block_reason** | Option<**String**> | Block reason. Can be null. | [optional]
**certificate** | Option<[**models::TransportTransactionsCertificateReportItem**](TransportTransactionsCertificateReportItem.md)> | Certificate. | [optional]
**comment** | Option<**String**> | Comment. Can be null. | [optional]
**counteragent** | Option<**String**> | Counteragent. Can be null. | [optional]
**counteragent_type** | Option<[**models::CertificateCounteragentType**](CertificateCounteragentType.md)> | Counteragent type. | [optional]
**counteragent_type_name** | Option<**String**> | Counteragent type name. Can be null. | [optional]
**coupon** | Option<[**models::TransportTransactionsCouponReportItem**](TransportTransactionsCouponReportItem.md)> | Coupon. | [optional]
**emitent_name** | Option<**String**> | Emitent name. Can be null. | [optional]
**loyalty_user** | Option<**String**> | Loyalty user. Can be null. | [optional]
**marketing_campaign_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Marketing campaign id. | [optional]
**nominal** | Option<**f64**> | Nominal. | [optional]
**order_number** | Option<**i32**> | Order number. | [optional]
**order_sum** | Option<**f64**> | Order sum. | [optional]
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization id. | 
**pos_balance_before** | Option<**f64**> | Pos balance before. | [optional]
**program_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Program id. | [optional]
**sum** | Option<**f64**> | Sum. | [optional]
**r#type** | Option<[**models::TransactionType**](TransactionType.md)> | Type. | [optional]
**type_name** | Option<**String**> | Type name. Can be null. | [optional]
**wallet_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Wallet id. | [optional]
**when_created** | Option<**String**> | When created. | [optional]
**when_created_order** | Option<**String**> | When created order. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id. | 
**is_delivery** | Option<**bool**> | Is delivery. | [optional]
**is_ignored** | Option<**bool**> | Is ignored. | [optional]
**pos_order_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Pos order id. | [optional]
**revision** | **i64** | Revision. | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Terminal group id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


