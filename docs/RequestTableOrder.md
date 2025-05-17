# RequestTableOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Order ID. | [optional]
**external_number** | Option<**String**> | Order external number.   > Allowed from version `8.0.6`. | [optional]
**table_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Table IDs.                Can be obtained by `/api/1/reserve/available_restaurant_sections` operation. | [optional]
**customer** | Option<[**models::RequestCreateOrderRegularCustomer**](RequestCreateOrderRegularCustomer.md)> | Guest.   > Allowed from version `7.5.2`. | [optional]
**phone** | Option<**String**> | Guest phone.   > Allowed from version `7.5.2`. | [optional]
**guest_count** | Option<**i32**> | Amount of guests in the order.   > Allowed from version `7.6.1`. | [optional]
**guests** | Option<[**models::RequestGuestsInfo**](RequestGuestsInfo.md)> | Guests information.   > Allowed from version `7.6.1`. | [optional]
**tab_name** | Option<**String**> | Tab name (only for fastfood terminals group in tab mode).   > Allowed from version `7.6.1`. | [optional]
**menu_id** | Option<**String**> | External menu ID. | [optional]
**price_category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Price category id of the order.    Can be obtained by `/api/2/menu` operation.   > Allowed from version `9.0.5`. | [optional]
**items** | [**Vec<models::RequestCreateOrderItem>**](RequestCreateOrderItem.md) | Order items. | 
**combos** | Option<[**Vec<models::RequestCreateOrderCombo>**](RequestCreateOrderCombo.md)> | Combos included in order. | [optional]
**payments** | Option<[**Vec<models::RequestCreateOrderPayment>**](RequestCreateOrderPayment.md)> | Order payment components.   > Type **LoyaltyCard** allowed from version `7.1.5`. | [optional]
**tips** | Option<[**Vec<models::RequestCreateOrderTipsPayment>**](RequestCreateOrderTipsPayment.md)> | Order tips components. | [optional]
**source_key** | Option<**String**> | The string key (marker) of the source (partner - api user) that created the order. Needed to limit the visibility of orders for external integration. | [optional]
**discounts_info** | Option<[**models::RequestCreateOrderDiscountsInfo**](RequestCreateOrderDiscountsInfo.md)> | Discounts/surcharges. | [optional]
**loyalty_info** | Option<[**models::RequestCreateOrderLoyaltyInfo**](RequestCreateOrderLoyaltyInfo.md)> | Information about Loyalty app. | [optional]
**order_type_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Order type ID.                 Can be obtained by `/api/1/deliveries/order_types` operation | [optional]
**cheque_additional_info** | Option<[**models::ChequeAdditionalInfo**](ChequeAdditionalInfo.md)> | Cheque additional information. | [optional]
**external_data** | Option<[**Vec<models::RequestCreateOrderExternalData>**](RequestCreateOrderExternalData.md)> | Order external data.   > Allowed from version `8.0.6`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


