# RmsDeliveryOrderDraft

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**menu_id** | **String** | External menu ID. | 
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Order ID. Must be unique.  > If sent null, it generates automatically on iikoTransport side. | [optional]
**external_number** | Option<**String**> | Order external number.   > Allowed from version `8.0.6`. | [optional]
**complete_before** | Option<**String**> | Order fulfillment date.  > Date and time must be local for delivery terminal, without time zone (take a look at example).   > If null, order is urgent and time is calculated based on customer settings,  > i.e. the shortest delivery time possible.  > Permissible values: from current day and 60 days on. | [optional]
**phone** | **String** | Telephone number.  > Must begin with symbol \"+\" and must be at least 8 digits. | 
**order_type_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Order type ID.     Can be obtained by `/api/1/deliveries/order_types` operation.    > Only one of the fields must be defined: **orderTypeId** or **orderServiceType**. | [optional]
**order_service_type** | Option<[**models::RequestCreateOrderServiceType**](RequestCreateOrderServiceType.md)> | Order service type.  > Only one of the fields must be defined: **orderTypeId** or **orderServiceType**.   > Allowed from version `7.0.3`. | [optional]
**delivery_point** | Option<[**models::RequestCreateOrderDeliveryPoint**](RequestCreateOrderDeliveryPoint.md)> | Delivery point details.  > Not required in case of customer pickup. Otherwise, required. | [optional]
**comment** | Option<**String**> | Order comment. | [optional]
**customer** | Option<[**models::RequestCreateOrderCustomer**](RequestCreateOrderCustomer.md)> | Customer.                'Regular' customer:    - can be used only if a customer agrees to take part in the store's loyalty programs  - customer details will be added (updated) to the store's customer database  - benefits (accumulation of rewards, etc.) of active loyalty programs will be made available to the customer     'One-time' customer:    - should be used if a customer does not agree to take part in the store's loyalty programs or an aggregator (external system) does not provide customer details  - customer details will NOT be added to the store's customer database and will be used ONLY to complete the current order | [optional]
**guests** | Option<[**models::RequestCreateOrderGuests**](RequestCreateOrderGuests.md)> | Guest details. Not equal to the customer who makes an order. | [optional]
**marketing_source_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Marketing source (advertisement) ID.                 Can be obtained by `/api/1/marketing_sources` operation. | [optional]
**operator_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Operator ID.   > Allowed from version `7.6.3`. | [optional]
**delivery_duration** | Option<**i32**> | Delivery duration.   > Allowed from version `8.8.6`. | [optional]
**delivery_zone** | Option<**String**> | Name of delivery zone.   > Allowed from version `8.8.6`. | [optional]
**items** | [**Vec<models::RequestCreateOrderItem>**](RequestCreateOrderItem.md) | Order items. | 
**combos** | Option<[**Vec<models::RequestCreateOrderCombo>**](RequestCreateOrderCombo.md)> | Combos included in order. | [optional]
**payments** | Option<[**Vec<models::RequestCreateOrderPayment>**](RequestCreateOrderPayment.md)> | Order payment components.   > Type **LoyaltyCard** allowed from version `7.1.5`. | [optional]
**tips** | Option<[**Vec<models::RequestCreateOrderTipsPayment>**](RequestCreateOrderTipsPayment.md)> | Order tips components. | [optional]
**source_key** | Option<**String**> | The string key (marker) of the source (partner - api user) that created the order. Needed to limit the visibility of orders for external integration. | [optional]
**discounts_info** | Option<[**models::RequestCreateOrderDiscountsInfo**](RequestCreateOrderDiscountsInfo.md)> | Discounts/surcharges. | [optional]
**loyalty_info** | Option<[**models::RequestCreateOrderLoyaltyInfo**](RequestCreateOrderLoyaltyInfo.md)> | Information about Loyalty app. | [optional]
**cheque_additional_info** | Option<[**models::ChequeAdditionalInfo**](ChequeAdditionalInfo.md)> | Cheque additional information. | [optional]
**external_data** | Option<[**Vec<models::RequestCreateOrderExternalData>**](RequestCreateOrderExternalData.md)> | Order external data.   > Allowed from version `8.0.6`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


