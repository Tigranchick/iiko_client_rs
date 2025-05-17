# \OrdersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_order_add_customer_post**](OrdersApi.md#api1_order_add_customer_post) | **POST** /api/1/order/add_customer | Add customer to order.
[**api1_order_add_items_post**](OrdersApi.md#api1_order_add_items_post) | **POST** /api/1/order/add_items | Add order items.
[**api1_order_add_payments_post**](OrdersApi.md#api1_order_add_payments_post) | **POST** /api/1/order/add_payments | Add order payments.
[**api1_order_by_id_post**](OrdersApi.md#api1_order_by_id_post) | **POST** /api/1/order/by_id | Retrieve orders by IDs.
[**api1_order_by_table_post**](OrdersApi.md#api1_order_by_table_post) | **POST** /api/1/order/by_table | Retrieve orders by tables.
[**api1_order_cancel_post**](OrdersApi.md#api1_order_cancel_post) | **POST** /api/1/order/cancel | Cancel the table order.
[**api1_order_change_external_data_post**](OrdersApi.md#api1_order_change_external_data_post) | **POST** /api/1/order/change_external_data | Change table order external_data.
[**api1_order_change_payments_post**](OrdersApi.md#api1_order_change_payments_post) | **POST** /api/1/order/change_payments | Change table order's payments.
[**api1_order_close_post**](OrdersApi.md#api1_order_close_post) | **POST** /api/1/order/close | Close order.
[**api1_order_create_post**](OrdersApi.md#api1_order_create_post) | **POST** /api/1/order/create | Create order.
[**api1_order_init_by_pos_order_post**](OrdersApi.md#api1_order_init_by_pos_order_post) | **POST** /api/1/order/init_by_posOrder | Init orders, created on POS, by POS orders.
[**api1_order_init_by_table_post**](OrdersApi.md#api1_order_init_by_table_post) | **POST** /api/1/order/init_by_table | Init orders, created on POS, by tables.



## api1_order_add_customer_post

> models::CorrelationIdResponse api1_order_add_customer_post(authorization, timeout, request_add_customer_to_table_order_request)
Add customer to order.

   > Allowed from version `7.7.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_add_customer_to_table_order_request** | Option<[**RequestAddCustomerToTableOrderRequest**](RequestAddCustomerToTableOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_add_items_post

> models::CorrelationIdResponse api1_order_add_items_post(authorization, timeout, request_add_items_to_table_order_request)
Add order items.

   > Allowed from version `7.4.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_add_items_to_table_order_request** | Option<[**RequestAddItemsToTableOrderRequest**](RequestAddItemsToTableOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_add_payments_post

> models::CorrelationIdResponse api1_order_add_payments_post(authorization, timeout, add_order_payments_request)
Add order payments.

   > Allowed from version `8.2.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order payments: changing`.

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


## api1_order_by_id_post

> models::TableOrdersResponse api1_order_by_id_post(authorization, timeout, request_get_table_orders_by_id_request)
Retrieve orders by IDs.

   > Allowed from version `7.4.6`.   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_get_table_orders_by_id_request** | Option<[**RequestGetTableOrdersByIdRequest**](RequestGetTableOrdersByIdRequest.md)> |  |  |

### Return type

[**models::TableOrdersResponse**](TableOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_by_table_post

> models::TableOrdersResponse api1_order_by_table_post(authorization, timeout, request_get_table_orders_by_table_request)
Retrieve orders by tables.

   > Allowed from version `7.4.6`.   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_get_table_orders_by_table_request** | Option<[**RequestGetTableOrdersByTableRequest**](RequestGetTableOrdersByTableRequest.md)> |  |  |

### Return type

[**models::TableOrdersResponse**](TableOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_cancel_post

> models::CorrelationIdResponse api1_order_cancel_post(authorization, timeout, request_cancel_table_order_request)
Cancel the table order.

   > Allowed from version `9.0.5`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_cancel_table_order_request** | Option<[**RequestCancelTableOrderRequest**](RequestCancelTableOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_change_external_data_post

> models::CorrelationIdResponse api1_order_change_external_data_post(authorization, timeout, request_update_order_change_external_data_request)
Change table order external_data.

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


## api1_order_change_payments_post

> models::CorrelationIdResponse api1_order_change_payments_post(authorization, timeout, request_update_order_change_payments_request)
Change table order's payments.

> Method will fail if there are any processed payments in the order.  > If all payments in the order are unprocessed they will be removed and replaced with new ones.   > Allowed from version `7.7.4`.   > Restriction group: `Order payments: changing`.

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


## api1_order_close_post

> models::CorrelationIdResponse api1_order_close_post(authorization, timeout, request_close_table_order_request)
Close order.

   > Allowed from version `7.4.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_close_table_order_request** | Option<[**RequestCloseTableOrderRequest**](RequestCloseTableOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_create_post

> models::TableOrderResponse api1_order_create_post(authorization, timeout, request_create_table_order_request)
Create order.

   > Allowed from version `7.4.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_create_table_order_request** | Option<[**RequestCreateTableOrderRequest**](RequestCreateTableOrderRequest.md)> |  |  |

### Return type

[**models::TableOrderResponse**](TableOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_init_by_pos_order_post

> models::CorrelationIdResponse api1_order_init_by_pos_order_post(authorization, timeout, request_init_table_order_by_pos_order_request)
Init orders, created on POS, by POS orders.

   > Allowed from version `7.7.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: loading data`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_init_table_order_by_pos_order_request** | Option<[**RequestInitTableOrderByPosOrderRequest**](RequestInitTableOrderByPosOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_order_init_by_table_post

> models::CorrelationIdResponse api1_order_init_by_table_post(authorization, timeout, request_init_table_order_request)
Init orders, created on POS, by tables.

   > Allowed from version `7.7.1`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: loading data`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_init_table_order_request** | Option<[**RequestInitTableOrderRequest**](RequestInitTableOrderRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

