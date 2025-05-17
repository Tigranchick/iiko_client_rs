# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_delivery_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of delivery serving as source for splitting by FCRs. | [optional]
**customer** | Option<[**models::OrderCustomer**](OrderCustomer.md)> | Delivery customer. | [optional]
**phone** | **String** | Delivery phone number. | 
**delivery_point** | Option<[**models::OrderDeliveryPoint**](OrderDeliveryPoint.md)> | Delivery point details.  <remarks>  Not required if order type is customer pickup. Otherwise, required.  </remarks> | [optional]
**status** | [**models::DeliveryStatus**](DeliveryStatus.md) | Delivery status.                > Delivery status `ReadyForCooking` is deprecated from version `9.0.6`. | 
**cancel_info** | Option<[**models::OrderCancelInfo**](OrderCancelInfo.md)> | Delivery cancellation details.  <remarks>  Required only if delivery is canceled, i.e. status=Canceled.  </remarks> | [optional]
**courier_info** | Option<[**models::OrderCourierInfo**](OrderCourierInfo.md)> | Driver information. | [optional]
**complete_before** | **String** | Order fulfillment time (Local for the terminal). | 
**when_created** | **String** | Delivery creation time in iikoFront (Local for the terminal). | 
**when_confirmed** | Option<**String**> | Delivery confirmation time (Local for the terminal). | [optional]
**when_printed** | Option<**String**> | Service printing time (Local for the terminal). | [optional]
**when_cooking_completed** | Option<**String**> | Cooking completion time (Local for the terminal). | [optional]
**when_sended** | Option<**String**> | Delivery dispatch time (Local for the terminal). | [optional]
**when_delivered** | Option<**String**> | Actual delivery time (Local for delivery terminal). | [optional]
**comment** | Option<**String**> | Order comment. | [optional]
**problem** | Option<[**models::OrderProblem**](OrderProblem.md)> | Problem flag. | [optional]
**operator** | Option<[**models::OrderEmployee**](OrderEmployee.md)> | Operator that took order. | [optional]
**marketing_source** | Option<[**models::OrderMarketingSource**](OrderMarketingSource.md)> | Marketing source. | [optional]
**delivery_duration** | Option<**i64**> | Duration of delivery (in minutes). | [optional]
**index_in_courier_route** | Option<**i32**> | Ordinal number in route list.  <remarks>  Field is filled up at the time of delivery allocation by logistics in iikoFront.  If logistics is not in use, the field is not filled up.  </remarks> | [optional]
**cooking_start_time** | **String** | The time when you need to start cooking an order (Local for the terminal). | 
**is_deleted** | Option<**bool**> | Order is deleted. | [optional]
**when_received_by_api** | Option<**String**> | Moment of time when CloudAPI received the request to create the order (UTC). | [optional]
**when_received_from_front** | Option<**String**> | Moment of time when the order first received and saved from iikoFront (UTC). | [optional]
**moved_from_delivery_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Tells that this delivery has been moved from terminal group  with id *MovedFromTerminalGroupId* by cancelling delivery with deliveryId *MovedFromDeliveryId*.   > Allowed from version `7.5.4`. | [optional]
**moved_from_terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Tells that this delivery has been moved from terminal group  with id *MovedFromTerminalGroupId* by cancelling delivery with deliveryId *MovedFromDeliveryId*.   > Allowed from version `7.5.4`. | [optional]
**moved_from_organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Tells that this delivery has been moved from terminal group  with id *MovedFromTerminalGroupId* by cancelling delivery with deliveryId *MovedFromDeliveryId*.   > Allowed from version `7.5.4`. | [optional]
**external_courier_service** | Option<[**models::OrderExternalCourierService**](OrderExternalCourierService.md)> | ECS info.   > Allowed from version `7.7.7`. | [optional]
**moved_to_delivery_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Tells that this delivery has been canceled and moved to terminal group  with id *MovedToTerminalGroupId*. | [optional]
**moved_to_terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**moved_to_organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**menu_id** | Option<**String**> | External menu ID. | [optional]
**delivery_zone** | Option<**String**> | Name of delivery zone. | [optional]
**estimated_time** | Option<**String**> | Delivery estimated time. | [optional]
**is_asap** | Option<**bool**> | Whether to deliver as soon as possible. | [optional]
**when_packed** | Option<**String**> | Delivery packing time (Local for the terminal). | [optional]
**price_category** | Option<[**models::PriceCategory**](PriceCategory.md)> | Price category of the order.   > Allowed from version `9.0.5`. | [optional]
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


