# \DeliveriesCreateAndUpdateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_deliveries_add_items_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_add_items_post) | **POST** /api/1/deliveries/add_items | Add order items.
[**api1_deliveries_add_payments_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_add_payments_post) | **POST** /api/1/deliveries/add_payments | Add order payments.
[**api1_deliveries_cancel_confirmation_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_cancel_confirmation_post) | **POST** /api/1/deliveries/cancel_confirmation | Cancel delivery confirmation.
[**api1_deliveries_cancel_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_cancel_post) | **POST** /api/1/deliveries/cancel | Cancel delivery order.
[**api1_deliveries_change_comment_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_comment_post) | **POST** /api/1/deliveries/change_comment | Change delivery comment.
[**api1_deliveries_change_complete_before_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_complete_before_post) | **POST** /api/1/deliveries/change_complete_before | Change time when client wants the order to be delivered.
[**api1_deliveries_change_delivery_point_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_delivery_point_post) | **POST** /api/1/deliveries/change_delivery_point | Change order's delivery point information.
[**api1_deliveries_change_driver_info_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_driver_info_post) | **POST** /api/1/deliveries/change_driver_info | Change driver info.
[**api1_deliveries_change_external_data_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_external_data_post) | **POST** /api/1/deliveries/change_external_data | Change delivery external data.
[**api1_deliveries_change_operator_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_operator_post) | **POST** /api/1/deliveries/change_operator | Assign/change the order operator.
[**api1_deliveries_change_payments_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_payments_post) | **POST** /api/1/deliveries/change_payments | Change order's payments.
[**api1_deliveries_change_service_type_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_change_service_type_post) | **POST** /api/1/deliveries/change_service_type | Change order's delivery type.
[**api1_deliveries_close_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_close_post) | **POST** /api/1/deliveries/close | Close order.
[**api1_deliveries_confirm_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_confirm_post) | **POST** /api/1/deliveries/confirm | Confirm delivery.
[**api1_deliveries_create_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_create_post) | **POST** /api/1/deliveries/create | Create delivery.
[**api1_deliveries_print_delivery_bill_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_print_delivery_bill_post) | **POST** /api/1/deliveries/print_delivery_bill | Print delivery bill.
[**api1_deliveries_update_order_courier_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_update_order_courier_post) | **POST** /api/1/deliveries/update_order_courier | Update order courier.
[**api1_deliveries_update_order_delivery_status_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_update_order_delivery_status_post) | **POST** /api/1/deliveries/update_order_delivery_status | Update delivery status.
[**api1_deliveries_update_order_payments_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_update_order_payments_post) | **POST** /api/1/deliveries/update_order_payments | Update order payment details.
[**api1_deliveries_update_order_problem_post**](DeliveriesCreateAndUpdateApi.md#api1_deliveries_update_order_problem_post) | **POST** /api/1/deliveries/update_order_problem | Update order problem.



## api1_deliveries_add_items_post

> models::CorrelationIdResponse api1_deliveries_add_items_post(authorization, timeout, request_add_order_items_request)
Add order items.

   > Allowed from version `7.4.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_add_order_items_request** | Option<[**RequestAddOrderItemsRequest**](RequestAddOrderItemsRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_add_payments_post

> models::CorrelationIdResponse api1_deliveries_add_payments_post(authorization, timeout, add_order_payments_request)
Add order payments.

   > Allowed from version `8.4.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order payments: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**add_order_payments_request** | Option<[**AddOrderPaymentsRequest**](AddOrderPaymentsRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_cancel_confirmation_post

> models::CorrelationIdResponse api1_deliveries_cancel_confirmation_post(authorization, timeout, request_update_order_cancel_delivery_confirmation_request)
Cancel delivery confirmation.

   > Allowed from version `7.6.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_cancel_delivery_confirmation_request** | Option<[**RequestUpdateOrderCancelDeliveryConfirmationRequest**](RequestUpdateOrderCancelDeliveryConfirmationRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_cancel_post

> models::CorrelationIdResponse api1_deliveries_cancel_post(authorization, timeout, request_cancel_order_request)
Cancel delivery order.

   > Allowed from version `7.5.4`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_cancel_order_request** | Option<[**RequestCancelOrderRequest**](RequestCancelOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_comment_post

> models::CorrelationIdResponse api1_deliveries_change_comment_post(authorization, timeout, request_update_order_change_delivery_comment_request)
Change delivery comment.

   > Allowed from version `7.6.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_delivery_comment_request** | Option<[**RequestUpdateOrderChangeDeliveryCommentRequest**](RequestUpdateOrderChangeDeliveryCommentRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_complete_before_post

> models::CorrelationIdResponse api1_deliveries_change_complete_before_post(authorization, timeout, request_update_order_change_complete_before_request)
Change time when client wants the order to be delivered.

   > Allowed from version `7.5.4`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_complete_before_request** | Option<[**RequestUpdateOrderChangeCompleteBeforeRequest**](RequestUpdateOrderChangeCompleteBeforeRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_delivery_point_post

> models::CorrelationIdResponse api1_deliveries_change_delivery_point_post(authorization, timeout, request_update_order_change_delivery_point_request)
Change order's delivery point information.

   > Allowed from version `7.5.4`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_delivery_point_request** | Option<[**RequestUpdateOrderChangeDeliveryPointRequest**](RequestUpdateOrderChangeDeliveryPointRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_driver_info_post

> models::CorrelationIdResponse api1_deliveries_change_driver_info_post(authorization, timeout, request_change_driver_info_request)
Change driver info.

   > Allowed from version `8.6.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order driver: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_change_driver_info_request** | Option<[**RequestChangeDriverInfoRequest**](RequestChangeDriverInfoRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_external_data_post

> models::CorrelationIdResponse api1_deliveries_change_external_data_post(authorization, timeout, request_update_order_change_external_data_request)
Change delivery external data.

   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_external_data_request** | Option<[**RequestUpdateOrderChangeExternalDataRequest**](RequestUpdateOrderChangeExternalDataRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_operator_post

> models::CorrelationIdResponse api1_deliveries_change_operator_post(authorization, timeout, request_update_order_change_delivery_operator_request)
Assign/change the order operator.

   > Allowed from version `7.6.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_delivery_operator_request** | Option<[**RequestUpdateOrderChangeDeliveryOperatorRequest**](RequestUpdateOrderChangeDeliveryOperatorRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_payments_post

> models::CorrelationIdResponse api1_deliveries_change_payments_post(authorization, timeout, request_update_order_change_payments_request)
Change order's payments.

> Method will fail if there are any processed payments in the order.  > If all payments in the order are unprocessed they will be removed and replaced with new ones.   > Allowed from version `7.6.3`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order payments: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_payments_request** | Option<[**RequestUpdateOrderChangePaymentsRequest**](RequestUpdateOrderChangePaymentsRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_change_service_type_post

> models::CorrelationIdResponse api1_deliveries_change_service_type_post(authorization, timeout, request_update_order_change_service_type_request)
Change order's delivery type.

   > Allowed from version `7.5.4`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_change_service_type_request** | Option<[**RequestUpdateOrderChangeServiceTypeRequest**](RequestUpdateOrderChangeServiceTypeRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_close_post

> models::CorrelationIdResponse api1_deliveries_close_post(authorization, timeout, request_close_delivery_order_request)
Close order.

> Before version `8.0.6` it's possible to close deliveries with `DeliveryByClient`  orderServiceType only, starting from version `8.0.6` it's also possible to close  `DeliveryByCourier` deiveries in the DeliveryStatus `OnWay` or `Delivered` .   > Allowed from version `7.4.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_close_delivery_order_request** | Option<[**RequestCloseDeliveryOrderRequest**](RequestCloseDeliveryOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_confirm_post

> models::CorrelationIdResponse api1_deliveries_confirm_post(authorization, timeout, request_update_order_confirm_delivery_request)
Confirm delivery.

   > Allowed from version `7.6.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_confirm_delivery_request** | Option<[**RequestUpdateOrderConfirmDeliveryRequest**](RequestUpdateOrderConfirmDeliveryRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_create_post

> models::OrderResponse api1_deliveries_create_post(authorization, timeout, request_create_order_request)
Create delivery.

   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_create_order_request** | Option<[**RequestCreateOrderRequest**](RequestCreateOrderRequest.md)> |  |  |

### Return type

[**models::OrderResponse**](OrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_print_delivery_bill_post

> models::CorrelationIdResponse api1_deliveries_print_delivery_bill_post(authorization, timeout, request_print_delivery_bill_request)
Print delivery bill.

   > Allowed from version `7.6.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_print_delivery_bill_request** | Option<[**RequestPrintDeliveryBillRequest**](RequestPrintDeliveryBillRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_update_order_courier_post

> models::CorrelationIdResponse api1_deliveries_update_order_courier_post(authorization, timeout, request_update_order_courier_request)
Update order courier.

   > Allowed from version `7.1.5`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order driver: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_courier_request** | Option<[**RequestUpdateOrderCourierRequest**](RequestUpdateOrderCourierRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_update_order_delivery_status_post

> models::CorrelationIdResponse api1_deliveries_update_order_delivery_status_post(authorization, timeout, request_update_delivery_status_request)
Update delivery status.

   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_delivery_status_request** | Option<[**RequestUpdateDeliveryStatusRequest**](RequestUpdateDeliveryStatusRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_update_order_payments_post

> models::CorrelationIdResponse api1_deliveries_update_order_payments_post(authorization, timeout, request_update_order_payments_request)
Update order payment details.

> Deprecated, use `api/1/deliveries/change_payments` method instead.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Deprecated`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_payments_request** | Option<[**RequestUpdateOrderPaymentsRequest**](RequestUpdateOrderPaymentsRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_update_order_problem_post

> models::CorrelationIdResponse api1_deliveries_update_order_problem_post(authorization, timeout, request_update_order_problem_request)
Update order problem.

   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_problem_request** | Option<[**RequestUpdateOrderProblemRequest**](RequestUpdateOrderProblemRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

