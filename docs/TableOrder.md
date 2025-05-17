# TableOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Table IDs.                Can be obtained by `/api/1/reserve/available_restaurant_sections` operation. | 
**customer** | Option<[**models::OrderRegularCustomer**](OrderRegularCustomer.md)> | Guest.   > Allowed from version `7.5.2`. | [optional]
**phone** | Option<**String**> | Guest phone.   > Allowed from version `7.5.2`. | [optional]
**status** | [**models::OrderStatus**](OrderStatus.md) | Order status. | 
**when_created** | Option<**String**> | Order creation date (terminal time zone). | [optional]
**waiter** | Option<[**models::OrderEmployee**](OrderEmployee.md)> | Order waiter. | [optional]
**tab_name** | Option<**String**> | Tab name (only for fastfood terminals group in tab mode). | [optional]
**split_order_between_cash_registers** | Option<[**models::SplitOrderBetweenCashRegisters**](SplitOrderBetweenCashRegisters.md)> | Need to split order between cash registers.  <remarks>  Not empty for orders in statuses New or Bill.  </remarks> | [optional]
**menu_id** | Option<**String**> | External menu ID. | [optional]
**price_category** | Option<[**models::PriceCategory**](PriceCategory.md)> | Price Category of the order.   > Allowed from version `9.0.5`. | [optional]
**sum** | **f64** | Order amount (after discount or surcharge). | 
**number** | **i32** | Delivery No. | 
**source_key** | Option<**String**> | Delivery source. | [optional]
**when_bill_printed** | Option<**String**> | Invoice printing time (guest bill time). | [optional]
**when_closed** | Option<**String**> | Delivery closing time (Local for delivery terminal). | [optional]
**conception** | Option<[**models::OrderConception**](OrderConception.md)> | Concept. | [optional]
**guests_info** | Option<[**models::OrderGuestsInfo**](OrderGuestsInfo.md)> | Information about order guests. | 
**items** | [**Vec<models::OrderItem>**](OrderItem.md) | Order items. | 
**combos** | Option<[**Vec<models::OrderCombo>**](OrderCombo.md)> | Combo. | [optional]
**payments** | Option<[**Vec<models::OrderPaymentItem>**](OrderPaymentItem.md)> |  | [optional]
**tips** | Option<[**Vec<models::OrderTipsPaymentItem>**](OrderTipsPaymentItem.md)> | Tips. | [optional]
**discounts** | Option<[**Vec<models::OrderDiscountItem>**](OrderDiscountItem.md)> |  | [optional]
**order_type** | Option<[**models::OrderType**](OrderType.md)> | Order type. | 
**terminal_group_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the terminal group where the order is located. | 
**processed_payments_sum** | Option<**f64**> | The amount of processed payments.  <remarks>  null - only for unsupported POS versions.  </remarks>   > Allowed from version `7.6.0`. | 
**loyalty_info** | Option<[**models::OrderLoyaltyInfo**](OrderLoyaltyInfo.md)> | Information about Loyalty app.  <remarks>  null - only for unsupported POS versions.  </remarks> | [optional]
**external_data** | Option<[**Vec<models::OrderExternalData>**](OrderExternalData.md)> | Order external data.   > Allowed from version `8.0.6`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


